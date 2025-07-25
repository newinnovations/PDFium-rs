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



mod page_link;
pub use page_link::PdfiumPageLink;

use crate::{
    error::{PdfiumError, PdfiumResult},
    guard::lib,
    pdfium_types::FPDF_PAGELINK,
};

/// # Rust interface to FPDF_PAGELINK
pub struct PdfiumPageLink {
    handle: FPDF_PAGELINK,
}

impl PdfiumPageLink {
    pub(crate) fn new_from_handle(handle: FPDF_PAGELINK) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            #[cfg(feature = "debug_print")]
            println!("New page_link {handle:?}");
            Ok(Self { handle })
        }
    }
}

impl From<&PdfiumPageLink> for FPDF_PAGELINK {
    fn from(value: &PdfiumPageLink) -> Self {
        value.handle
    }
}

// TODO: check lifecycle FPDF_PAGELINK

impl Drop for PdfiumPageLink {
    /// Closes this [`PdfiumPageLink`], releasing held memory.
    fn drop(&mut self) {
        #[cfg(feature = "debug_print")]
        println!("Closing page_link {:?}", self.handle);
        lib().FPDF_Close(self);
    }
}
