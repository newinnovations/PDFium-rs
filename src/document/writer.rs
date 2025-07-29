// PDFium-rs -- Modern Rust interface to PDFium, the PDF library from Google
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
    ffi::{c_int, c_ulong, c_void},
    io::Write,
    ptr::null_mut,
    slice,
};

use crate::{pdfium_types::FPDF_FILEWRITE, PdfiumResult};

/// Enables Rust based writers (implementing `Write` + `Seek`) with PDFium.
#[repr(C)]
pub struct PdfiumWriter<W: Write + 'static> {
    custom_file_write: CustomFileWrite<W>,
    pdfium_writer: Box<W>,
}

impl<W: Write + 'static> PdfiumWriter<W> {
    pub(crate) fn new(writer: W) -> Box<Self> {
        let custom_file_write = CustomFileWrite {
            version: 1,
            write_block: Some(write_callback),
            pdfium_writer: null_mut(),
        };

        let mut pdfium_writer = Box::new(PdfiumWriter {
            custom_file_write,
            pdfium_writer: Box::new(writer),
        });

        // Store a pointer to this PdfiumWriter instance in `custom_file_write``. This pointer
        // will be passed to the write_callback function that Pdfium invokes, allowing
        // the callback to retrieve the PdfiumWriter struct and access the boxed Rust writer.
        let pdfium_writer_ptr: *const PdfiumWriter<W> = pdfium_writer.as_ref();
        pdfium_writer.as_mut().custom_file_write.pdfium_writer =
            pdfium_writer_ptr as *mut PdfiumWriter<W>;

        pdfium_writer
    }

    pub(crate) fn flush(&mut self) -> PdfiumResult<()> {
        self.pdfium_writer.flush()?;
        Ok(())
    }

    pub(crate) fn take_writer(self) -> Box<W> {
        self.pdfium_writer
    }
}

/// Converts a mutable reference to PdfiumWriter into a raw pointer to FPDF_FILEWRITE
/// for FFI purposes. This allows passing the custom write structure to PDFium's C API.
// impl From<&mut PdfiumWriter> for *mut FPDF_FILEWRITE {
impl<W: Write + 'static> From<&mut PdfiumWriter<W>> for *mut FPDF_FILEWRITE {
    fn from(value: &mut PdfiumWriter<W>) -> Self {
        &mut value.custom_file_write as *mut CustomFileWrite<W> as *mut FPDF_FILEWRITE
    }
}

/// Structure for custom file write
///
/// This custom struct is identical to FPDF_FILEWRITE_, but extended
/// with a reference to the PdfiumWriter. We need that to access the
/// underlying writer in the write_callback function.
#[repr(C)]
struct CustomFileWrite<W: Write + 'static> {
    /// Version number of the interface. Currently must be 1.
    pub version: ::std::os::raw::c_int,
    /// Method: WriteBlock
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: WriteBlock
    ///          Output a block of data in your custom way.
    /// Interface Version:
    ///          1
    /// Implementation Required:
    ///          Yes
    /// Comments:
    ///          Called by function FPDF_SaveDocument
    /// Parameters:
    ///          pThis       -   Pointer to the structure itself
    ///          pData       -   Pointer to a buffer to output
    ///          size        -   The size of the buffer.
    /// Return value:
    ///          Should be non-zero if successful, zero for error.
    /// ```
    pub write_block: Option<
        unsafe extern "C" fn(
            p_this: *mut CustomFileWrite<W>,
            p_data: *const c_void,
            size: c_ulong,
        ) -> c_int,
    >,
    /// The reference to the PdfiumWriter. We need that to access the
    /// actual underlying Rust writer in the write_callback function.
    pdfium_writer: *mut PdfiumWriter<W>,
}

/// The callback function invoked by PDFium to write data to the underlying writer.
///
/// # Parameters
/// - `pThis`: Pointer to the CustomFileWrite instance
/// - `pData`: Buffer containing the data to write
/// - `size`: Number of bytes to write
///
/// # Returns
/// non-zero (1) if successful, zero for error
extern "C" fn write_callback(
    p_this: *mut CustomFileWrite<impl Write + 'static>,
    p_data: *const c_void,
    size: c_ulong,
) -> c_int {
    unsafe {
        match (*(*p_this).pdfium_writer)
            .pdfium_writer
            .write_all(slice::from_raw_parts(p_data as *const u8, size as usize))
        {
            Ok(()) => 1,
            Err(_) => 0,
        }
    }
}
