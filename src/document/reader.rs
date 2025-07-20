// PDFium-rs -- Modern Rust wrapper to PDFium, the PDF library from Google
//
// Copyright (c) 2025 Martin van der Werff <github (at) newinnovations.nl>
//
// This file is part of PDFium-rs.
//
// PDFium-rs is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either version 3
// of the License, or (at your option) any later version.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
// BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::{
    ffi::{c_int, c_uchar, c_ulong, c_void},
    io::{Read, Seek, SeekFrom},
    ptr::null_mut,
    slice,
};

use crate::pdfium_types::FPDF_FILEACCESS;

/// Enables Rust based readers (implementing `Read` + `Seek`) with PDFium.
#[repr(C)]
pub struct PdfiumReader {
    file_access_ptr: FPDF_FILEACCESS,
    reader: Box<dyn PdfiumReaderExt>, // Type-erased equivalent of <R: Read + Seek>
}

impl PdfiumReader {
    pub(crate) fn new<R: Read + Seek + 'static>(mut reader: R) -> Box<Self> {
        let content_length = reader.seek(SeekFrom::End(0)).unwrap_or(0) as c_ulong;

        let file_access = FPDF_FILEACCESS {
            m_FileLen: content_length,
            m_GetBlock: Some(read_callback),
            m_Param: null_mut(),
        };

        let mut pdfium_reader = Box::new(PdfiumReader {
            file_access_ptr: file_access,
            reader: Box::new(reader),
        });

        // Store a pointer to this PdfiumReader instance in m_Param. This pointer
        // will be passed to the read_callback function that Pdfium invokes, allowing
        // the callback to retrieve the PdfiumReader struct and access the boxed Rust reader.
        let pdfium_reader_ptr: *const PdfiumReader = pdfium_reader.as_ref();
        pdfium_reader.as_mut().file_access_ptr.m_Param = pdfium_reader_ptr as *mut c_void;
        pdfium_reader
    }
}

/// Converts a mutable reference to PdfiumReader into a raw pointer to FPDF_FILEACCESS
/// for FFI purposes. This allows passing the file access structure to PDFium's C API.
impl From<&mut PdfiumReader> for *mut FPDF_FILEACCESS {
    fn from(value: &mut PdfiumReader) -> Self {
        unsafe { &mut *(&mut value.file_access_ptr as *mut FPDF_FILEACCESS) }
    }
}

/// A trait that enables type-erasure on the user-provided Rust reader.
/// This allows PdfiumReader to store any reader that implements Read + Seek
/// without requiring generic parameters, which simplifies the API for consumers.
trait PdfiumReaderExt: Read + Seek {}

impl<R: Read + Seek> PdfiumReaderExt for R {}

/// The callback function invoked by PDFium to read data from the underlying reader.
///
/// # Parameters
/// - `param`: Pointer to the PdfiumReader instance (cast from m_Param)
/// - `position`: File position to seek to before reading
/// - `buf`: Buffer to write the read data into
/// - `size`: Number of bytes to read
///
/// # Returns
/// The number of bytes actually read, or 0 on error.
extern "C" fn read_callback(
    param: *mut c_void,
    position: c_ulong,
    buf: *mut c_uchar,
    size: c_ulong,
) -> c_int {
    // Cast the void pointer back to PdfiumReader to access the underlying reader
    let pdfium_reader: &mut PdfiumReader = unsafe { &mut *(param as *mut PdfiumReader) };
    let reader = pdfium_reader.reader.as_mut();

    #[allow(clippy::unnecessary_cast)]
    // c_ulong isn't guaranteed to be u64 on all platforms
    let result = match reader.seek(SeekFrom::Start(position as u64)) {
        Ok(_) => reader
            .read(unsafe { slice::from_raw_parts_mut(buf, size as usize) })
            .unwrap_or(0),
        Err(_) => 0,
    };

    result as c_int
}
