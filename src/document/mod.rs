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

pub mod reader;

use std::{
    ffi::CString,
    fs::File,
    io::{Read, Seek},
    path::Path,
};

use crate::{
    document::reader::PdfiumReader,
    error::{PdfiumError, PdfiumResult},
    lib,
    page::PdfiumPage,
    pdfium_types::FPDF_DOCUMENT,
    try_lib,
};

/// Rust interface to FPDF_DOCUMENT
pub struct PdfiumDocument {
    handle: FPDF_DOCUMENT,
    _reader: Option<Box<PdfiumReader>>,
}

impl PdfiumDocument {
    fn new_from_handle(
        handle: FPDF_DOCUMENT,
        reader: Option<Box<PdfiumReader>>,
    ) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(match try_lib()?.FPDF_GetLastError() as i32 {
                crate::pdfium_constants::FPDF_ERR_FILE => PdfiumError::FileError,
                crate::pdfium_constants::FPDF_ERR_FORMAT => PdfiumError::FormatError,
                crate::pdfium_constants::FPDF_ERR_PASSWORD => PdfiumError::PasswordError,
                crate::pdfium_constants::FPDF_ERR_SECURITY => PdfiumError::SecurityError,
                crate::pdfium_constants::FPDF_ERR_PAGE => PdfiumError::PageError,
                _ => PdfiumError::Unknown,
            })
        } else {
            #[cfg(feature = "debug_print")]
            println!("New document {handle:?}");
            Ok(Self {
                handle,
                _reader: reader,
            })
        }
    }

    pub fn new_from_path<P: AsRef<Path>>(path: P, password: Option<&str>) -> PdfiumResult<Self> {
        let reader = File::open(path).map_err(|e| PdfiumError::IoError(e.to_string()))?;
        Self::new_from_reader(reader, password)
    }

    pub fn new_from_reader<R: Read + Seek + 'static>(
        reader: R,
        password: Option<&str>,
    ) -> PdfiumResult<Self> {
        let lib = try_lib()?;
        let mut reader = PdfiumReader::new(reader);
        let password = CString::new(password.unwrap_or("")).unwrap();
        let handle = lib.FPDF_LoadCustomDocument(&mut reader, &password);
        Self::new_from_handle(handle, Some(reader))
    }

    pub fn page_count(&self) -> i32 {
        lib().FPDF_GetPageCount(self)
    }

    /// Returns the [`PdfiumPage`] indicated by `index` from this [`PdfiumDocument`].
    pub fn page(&self, index: i32) -> PdfiumResult<PdfiumPage> {
        lib().FPDF_LoadPage(self, index)
    }
}

impl From<&PdfiumDocument> for FPDF_DOCUMENT {
    #[inline]
    fn from(value: &PdfiumDocument) -> Self {
        value.handle
    }
}

impl Drop for PdfiumDocument {
    /// Closes this [PdfiumDocument], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "debug_print")]
        println!("Closing document {:?}", self.handle);
        lib().FPDF_CloseDocument(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_non_existing() {
        let document = PdfiumDocument::new_from_path("resources/non_existing.pdf", None);
        assert!(document.is_err());
    }

    #[test]
    fn test_load_existing() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None);
        assert!(document.is_ok());
    }

    #[test]
    fn test_page_count() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page_count = document.page_count();
        assert_eq!(page_count, 2);
    }
}
