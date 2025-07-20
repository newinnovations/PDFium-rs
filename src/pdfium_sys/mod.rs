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

pub mod pdfium;
pub mod pdfium_bindings;
pub mod pdfium_constants;
pub mod pdfium_init;
pub mod pdfium_types;

use std::{
    ffi::{CString, OsStr, OsString},
    path::Path,
};

use libloading::{Library, Symbol};

use crate::{
    PdfiumError,
    pdfium_constants::{
        FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_AGG, FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_SKIA,
    },
    pdfium_sys::pdfium_bindings::Pdfium,
    pdfium_types::FPDF_LIBRARY_CONFIG,
};

impl Pdfium {
    /// Tries to load the PDFium dynamic library from the system
    ///
    /// The locations in which the library is searched for are platform specific and cannot
    /// be adjusted in a portable manner.
    pub fn load() -> Result<Box<Pdfium>, PdfiumError> {
        Self::load_with_filename(Self::library_filename())
    }

    /// Tries to load the PDFium dynamic library from the specifed directory
    pub fn load_from_directory<P: AsRef<Path>>(directory: P) -> Result<Box<Pdfium>, PdfiumError> {
        let filename = directory.as_ref().join(Self::library_filename());
        Self::load_with_filename(filename)
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
    fn load_with_filename<P: AsRef<OsStr>>(filename: P) -> Result<Box<Pdfium>, PdfiumError> {
        let lib = unsafe { Library::new(filename) };
        let bindings = match lib {
            Ok(lib) => Pdfium::new(lib),
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

    /// Initializes the PDFium library
    ///
    /// The `use_skia` parameter controls the use of the Skia renderer. Default renderer
    /// for PDFium is AGG (Aggregated Graphics).
    ///
    /// You have to call this function before you can call any PDF processing function.
    pub fn init(&self, use_skia: bool) {
        let config = FPDF_LIBRARY_CONFIG {
            version: 2,
            m_pUserFontPaths: std::ptr::null_mut(), // default paths
            m_pIsolate: std::ptr::null_mut(),       // let PDFium create one
            m_v8EmbedderSlot: 0,                    // 0 is fine for most embedders
            m_pPlatform: std::ptr::null_mut(),
            m_RendererType: match use_skia {
                true => FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_SKIA,
                false => FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_AGG,
            },
        };
        self.FPDF_InitLibraryWithConfig(&config);
    }
}

/// Retrieves function entries from the PDFium dynamic library
pub fn lib_get<'a, T>(library: &'a Library, function: &str) -> Result<Symbol<'a, T>, PdfiumError> {
    let function_c = CString::new(function).unwrap();
    let symbol = unsafe { library.get(function_c.as_bytes_with_nul()) };
    match symbol {
        Ok(symbol) => Ok(symbol),
        Err(e) => {
            eprintln!("== Failed to get entry '{function}' from dynamic library");
            let e = e.to_string().replace("\n", "");
            eprintln!("== {e}");
            Err(PdfiumError::LibraryError(e))
        }
    }
}
