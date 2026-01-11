// PDFium-rs -- Modern Rust interface to PDFium, the PDF library from Google
//
// Copyright (c) 2025-2026 Martin van der Werff <github (at) newinnovations.nl>
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
    PdfiumDocument, PdfiumPageObject, PdfiumTextPage,
    error::{PdfiumError, PdfiumResult},
    lib,
    page::{boundaries::PdfiumPageBoundaries, object::objects::PdfiumPageObjects},
    pdfium_constants::FALSE,
    pdfium_types::{FPDF_PAGE, Handle, PageHandle},
};

/// Rotation transformation that can be applied to a [`Page`] and during rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfiumRotation {
    /// No rotation
    None = 0,
    /// 90° clockwise
    Cw90 = 1,
    /// 180° clockwise
    Cw180 = 2,
    /// 270° clockwise (90° counterclockwise)
    Cw270 = 3,
}

impl PdfiumRotation {
    /// Checks if rotation is 90°/270° and height/width needs to be flipped
    pub fn needs_transpose(&self) -> bool {
        *self == PdfiumRotation::Cw90 || *self == PdfiumRotation::Cw270
    }
}

impl From<i32> for PdfiumRotation {
    fn from(value: i32) -> Self {
        match value % 4 {
            1 => Self::Cw90,
            2 => Self::Cw180,
            3 => Self::Cw270,
            _ => Self::None,
        }
    }
}

impl std::ops::Add for PdfiumRotation {
    type Output = PdfiumRotation;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::from(self as i32 + rhs as i32)
    }
}

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

    /// Get the rotation of this [`PdfiumPage`].
    ///
    /// Returns a [`PdfiumRotation`] indicating the page rotation
    pub fn rotation(&self) -> PdfiumRotation {
        PdfiumRotation::from(lib().FPDFPage_GetRotation(self))
    }

    /// Set rotation for this [`PdfiumPage`].
    ///
    /// rotate - the rotation value as [`PdfiumRotation`]
    pub fn set_rotation(&self, rotate: PdfiumRotation) {
        lib().FPDFPage_SetRotation(self, rotate as i32)
    }

    /// Get page height.
    ///
    /// Return value:
    /// * Page height (excluding non-displayable area) measured in points.
    ///   One point is 1/72 inch (around 0.3528 mm)
    ///
    /// Comments:
    /// * Changing the rotation of |page| affects the return value.
    pub fn height(&self) -> f32 {
        lib().FPDF_GetPageHeightF(self)
    }

    /// Get page width.
    ///
    /// Return value:
    /// * Page width (excluding non-displayable area) measured in points.
    ///   One point is 1/72 inch (around 0.3528 mm).
    ///
    /// Comments:
    /// * Changing the rotation of |page| affects the return value.
    pub fn width(&self) -> f32 {
        lib().FPDF_GetPageWidthF(self)
    }

    /// Check for landscape orientation.
    ///
    /// Return value:
    /// * `true` if width is larger than height, otherwise `false`
    ///
    /// Comments:
    /// * Square is neither landscape nor portrait
    /// * Changing the rotation of the page affects the return value.
    pub fn is_landscape(&self) -> bool {
        self.width() > self.height()
    }

    /// Check for portrait orientation.
    ///
    /// Return value:
    /// * `true` if width is smaller than height, otherwise `false`
    ///
    /// Comments:
    /// * Square is neither landscape nor portrait
    /// * Changing the rotation of the page affects the return value.
    pub fn is_portrait(&self) -> bool {
        self.width() < self.height()
    }

    /// Checks if this [`PdfiumPage`] contains transparency.
    ///
    /// Returns true if this contains transparency.
    pub fn has_transparency(&self) -> bool {
        lib().FPDFPage_HasTransparency(self) != FALSE
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
    use crate::{PdfiumDocument, PdfiumRotation};

    #[test]
    fn test_rotations() {
        assert_eq!(
            PdfiumRotation::Cw90 + PdfiumRotation::Cw90,
            PdfiumRotation::Cw180
        );
        assert_eq!(
            PdfiumRotation::Cw90 + PdfiumRotation::Cw270,
            PdfiumRotation::None
        );
        assert_eq!(
            PdfiumRotation::Cw270 + PdfiumRotation::Cw270,
            PdfiumRotation::Cw180
        );
    }

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
