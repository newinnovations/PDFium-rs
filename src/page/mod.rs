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

pub mod boundaries;
pub mod link;
pub mod object;
pub mod pages;
pub mod range;
pub mod render;
pub mod text;

use crate::{
    error::{PdfiumError, PdfiumResult},
    lib,
    page::{boundaries::PdfiumPageBoundaries, object::objects::PdfiumPageObjects},
    pdfium_types::{Handle, PageHandle, FPDF_PAGE},
    PdfiumDocument, PdfiumPageObject, PdfiumTextPage,
};

/// # Rust interface to FPDF_PAGE
#[derive(Debug, Clone)]
pub struct PdfiumPage {
    handle: PageHandle,
    owner: Option<PdfiumDocument>,
}

impl PdfiumPage {
    pub(crate) fn new_from_handle(handle: FPDF_PAGE) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            Ok(Self {
                handle: Handle::new(handle, Some(close_page)),
                owner: None,
            })
        }
    }

    pub(crate) fn set_owner(&mut self, owner: PdfiumDocument) {
        self.owner = Some(owner);
    }

    /// Rust interface to the boundary boxes of a page
    pub fn boundaries(&self) -> PdfiumPageBoundaries<'_> {
        PdfiumPageBoundaries::new(self)
    }

    /// Get number of page objects inside this [`PdfiumPage`].
    pub fn object_count(&self) -> i32 {
        lib().FPDFPage_CountObjects(self)
    }

    /// Returns the [`PdfiumPageObject`] indicated by `index` from this [`PdfiumPage`].
    pub fn object(&self, index: i32) -> PdfiumResult<PdfiumPageObject> {
        let mut object = lib().FPDFPage_GetObject(self, index)?;
        object.set_owner(self.clone());
        Ok(object)
    }

    /// Return an [`Iterator`] for the ojects in this [`PdfiumPage`].
    pub fn objects(&self) -> PdfiumPageObjects<'_> {
        PdfiumPageObjects::new(self)
    }

    /// Get text page information structure
    ///
    /// Contains information about all characters in a page.
    pub fn text(&self) -> PdfiumResult<PdfiumTextPage> {
        lib().FPDFText_LoadPage(self)
    }
}

impl From<&PdfiumPage> for FPDF_PAGE {
    #[inline]
    fn from(page: &PdfiumPage) -> Self {
        page.handle.handle()
    }
}

/// Closes this [`PdfiumPage`], releasing held memory.
fn close_page(page: FPDF_PAGE) {
    lib().FPDF_ClosePage(page);
}

#[cfg(test)]
mod tests {
    use crate::document::PdfiumDocument;

    #[test]
    fn test_sequential_page_access() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let _: Vec<_> = (0..8)
            .map(|i| {
                let page = document.page(i % 2);
                assert!(page.is_ok());
            })
            .collect();
    }

    #[test]
    fn test_concurrent_page_access() {
        use std::thread;

        let handles: Vec<_> = (0..8)
            .map(|i| {
                thread::spawn(move || {
                    let document =
                        PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
                    let page = document.page(i % 2);
                    assert!(page.is_ok());
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_load_pages_out_of_range() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(-1);
        assert!(page.is_err());
        let page = document.page(2);
        assert!(page.is_err());
    }
}
