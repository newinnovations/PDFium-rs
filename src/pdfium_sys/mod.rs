// PDFium-rs -- Safe Rust wrapper to PDFium, the PDF library from Google
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

pub mod pdfium;
pub mod pdfium_constants;
pub mod pdfium_types;

use std::{
    ffi::{CString, OsStr, OsString},
    path::Path,
};

use libloading::{Library, Symbol};

use crate::{PdfiumError, pdfium_sys::pdfium::PdfiumBindings};

pub struct Pdfium {
    bindings: Box<PdfiumBindings>,
}

impl Pdfium {
    /// Tries to load the PDFium dynamic library from the system
    ///
    /// The locations in which the library is searched for are platform specific and cannot
    /// be adjusted in a portable manner.
    pub fn load() -> Result<Box<PdfiumBindings>, PdfiumError> {
        Self::load_with_filename(Self::library_filename())
    }

    /// Tries to load the PDFium dynamic library from the specifed directory
    pub fn load_from_directory<P: AsRef<Path>>(
        directory: P,
    ) -> Result<Box<PdfiumBindings>, PdfiumError> {
        let filename = directory.as_ref().join(Self::library_filename());
        Self::load_with_filename(filename)
    }

    /// Returns the [`PdfiumBindings`] wrapped by this instance of [`Pdfium`].
    pub fn bindings(&self) -> &PdfiumBindings {
        self.bindings.as_ref()
    }

    /// Creates a new [Pdfium] instance from the given external Pdfium library bindings.
    pub fn new(bindings: Box<PdfiumBindings>) -> Self {
        bindings.FPDF_InitLibrary();
        Self { bindings }
    }

    /// Tries to load the PDFium dynamic library.
    ///
    /// The `filename` argument may be either:
    ///
    /// * The PDFium library filename;
    /// * The absolute path to the library;
    /// * A relative (to the current working directory) path to the library.
    ///
    /// # Platform-specific behaviour
    ///
    /// When a plain library filename is supplied, the locations in which the library is searched for
    /// are platform specific and cannot be adjusted in a portable manner.
    fn load_with_filename<P: AsRef<OsStr>>(
        filename: P,
    ) -> Result<Box<PdfiumBindings>, PdfiumError> {
        // let lib_name = library_filename("pdfium");
        let lib = unsafe { Library::new(filename) };
        let bindings = match lib {
            Ok(lib) => PdfiumBindings::new(lib),
            Err(e) => Err(PdfiumError::LibraryError(e.to_string())),
        };
        bindings.map(Box::new)
    }

    /// Generates the PDFium library filename appropriate for this system.
    ///
    /// This function will prepend prefixes (such as `lib`) and suffixes (such as `.so`) to
    /// the library name to construct the filename.
    fn library_filename() -> OsString {
        libloading::library_filename("pdfium")
    }
}

/// Retrieves function entries from the PDFium dynamic library
pub fn lib_get<'a, T>(library: &'a Library, function: &str) -> Result<Symbol<'a, T>, PdfiumError> {
    let function = CString::new(function)
        .map_err(|_| PdfiumError::LibraryError("NULL in string".to_string()))?;
    unsafe {
        library
            .get(function.as_bytes_with_nul())
            .map_err(|e| PdfiumError::LibraryError(e.to_string()))
    }
}
