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

use crate::{PdfiumRect, error::PdfiumResult, lib, page::PdfiumPage};

/// Rust interface to the boundary boxes of a page
///
/// PDF pages define multiple nested boundary boxes that serve different purposes in the
/// document lifecycle, from creation to final output.
///
/// **MediaBox** The MediaBox represents the full physical size of a PDF page. It includes every
/// possible part of the layout—bleed area, crop marks, and artwork. It’s essentially the outermost
/// boundary and acts like the canvas that all other boxes sit on. Every PDF must have a MediaBox,
/// and it's the largest of the five.
///
/// **CropBox** The CropBox defines the visible area of the page when viewed on a screen or printed
/// by default. It acts as a “window” through which the page content is displayed in most PDF viewers.
/// This box doesn’t usually affect professional printing, but it’s important for digital previews and
/// general display.
///
/// **BleedBox** The BleedBox extends slightly beyond the final trim size to include artwork that
/// "bleeds" off the edge of the page. This is especially important in printing, as it prevents unwanted
/// white margins if the paper is cut slightly off-target. Designers typically allow 3 to 5 mm of bleed
/// outside the TrimBox.
///
/// **TrimBox** The TrimBox defines the finished size of the printed page after it’s been trimmed. In
/// professional printing workflows, this box is the most critical—it determines how the page will be cut
/// and positioned. For example, if you're printing a business card or a poster, the TrimBox sets the
/// final dimensions.
///
/// **ArtBox** The ArtBox outlines the area that contains the meaningful content—like text, logos, or
/// illustrations. It's useful in cases where you want to mark a "safe zone" so that nothing important
/// sits too close to the edge. While not used as often, it’s handy for laying out advertisements or design
/// elements within a page.
pub struct PdfiumPageBoundaries<'a> {
    page: &'a PdfiumPage,
}

impl<'a> PdfiumPageBoundaries<'a> {
    pub fn new(page: &'a PdfiumPage) -> PdfiumPageBoundaries<'a> {
        Self { page }
    }

    /// Gets the "ArtBox" entry from the page dictionary.
    ///
    /// The ArtBox defines the extent of the page's meaningful content (including potential
    /// white space) as intended by the page's creator. This is typically used by print
    /// production software and represents the "artistic" boundary of the page content.
    /// It should be contained within or equal to the CropBox.
    #[inline]
    pub fn art(&self) -> PdfiumResult<PdfiumRect> {
        let mut rect = PdfiumRect::zero();
        lib().FPDFPage_GetArtBox(
            self.page,
            &mut rect.left,
            &mut rect.bottom,
            &mut rect.right,
            &mut rect.top,
        )?;
        Ok(rect)
    }

    /// Gets the "BleedBox" entry from the page dictionary.
    ///
    /// The BleedBox defines the region to which the contents of the page should be clipped
    /// when output in a production environment. This may include any extra bleed area needed
    /// to accommodate the physical limitations of cutting, folding, and trimming equipment.
    /// The bleed box should be larger than or equal to the TrimBox.
    #[inline]
    pub fn bleed(&self) -> PdfiumResult<PdfiumRect> {
        let mut rect = PdfiumRect::zero();
        lib().FPDFPage_GetBleedBox(
            self.page,
            &mut rect.left,
            &mut rect.bottom,
            &mut rect.right,
            &mut rect.top,
        )?;
        Ok(rect)
    }

    /// Gets the "CropBox" entry from the page dictionary.
    ///
    /// The CropBox defines the visible region of default user space. When the page is displayed
    /// or printed, its contents should be clipped to this rectangle and then imposed on the
    /// output medium. This is what viewers typically show as the "page" and defaults to the
    /// MediaBox if not specified. The CropBox should be contained within the MediaBox.
    #[inline]
    pub fn crop(&self) -> PdfiumResult<PdfiumRect> {
        let mut rect = PdfiumRect::zero();
        lib().FPDFPage_GetCropBox(
            self.page,
            &mut rect.left,
            &mut rect.bottom,
            &mut rect.right,
            &mut rect.top,
        )?;
        Ok(rect)
    }

    /// Gets the "MediaBox" entry from the page dictionary.
    ///
    /// The MediaBox defines the boundaries of the physical medium on which the page is to be
    /// printed. This represents the largest possible page size and is required for every page.
    /// All other page boundary boxes should be contained within or equal to the MediaBox.
    /// This is typically the paper size (e.g., A4, Letter, etc.).
    #[inline]
    pub fn media(&self) -> PdfiumResult<PdfiumRect> {
        let mut rect = PdfiumRect::zero();
        lib().FPDFPage_GetMediaBox(
            self.page,
            &mut rect.left,
            &mut rect.bottom,
            &mut rect.right,
            &mut rect.top,
        )?;
        Ok(rect)
    }

    /// Gets the "TrimBox" entry from the page dictionary.
    ///
    /// The TrimBox defines the intended dimensions of the finished page after trimming.
    /// This represents the final size of the page as it will appear to the end user after
    /// any production cutting/trimming processes. It should be contained within or equal to
    /// the BleedBox and is commonly used in professional printing workflows.
    #[inline]
    pub fn trim(&self) -> PdfiumResult<PdfiumRect> {
        let mut rect = PdfiumRect::zero();
        lib().FPDFPage_GetTrimBox(
            self.page,
            &mut rect.left,
            &mut rect.bottom,
            &mut rect.right,
            &mut rect.top,
        )?;
        Ok(rect)
    }

    /// Gets the default boundary for use by PDF viewers
    ///
    /// Returns the most appropriate boundary box for displaying the page in a PDF viewer
    /// or similar application. This method implements a fallback hierarchy to determine
    /// the best viewing area:
    ///
    /// 1. **CropBox** - The primary choice, as it defines the visible region intended
    ///    for display and is what most PDF viewers show by default
    /// 2. **TrimBox** - Used if CropBox is not available, representing the final
    ///    page dimensions after trimming
    /// 3. **MediaBox** - The fallback option, representing the full physical page size
    ///
    /// This method ensures that viewer applications always have a valid boundary to work
    /// with, since MediaBox is required to exist in every PDF page. The returned rectangle
    /// represents the area that should be visible to end users when viewing the document.
    pub fn default(&self) -> PdfiumResult<PdfiumRect> {
        self.crop()
            .or_else(|_| self.trim())
            .or_else(|_| self.media())
    }
}

#[cfg(test)]
mod tests {
    use crate::document::PdfiumDocument;

    #[test]
    fn test_media_boundary() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let boundary = page.boundaries().media().unwrap();

        assert_eq!(boundary.left, 0.0);
        assert_eq!(boundary.top, 841.92);
        assert_eq!(boundary.bottom, 0.0);
        assert_eq!(boundary.right, 594.95996);
    }

    #[test]
    fn test_default_boundary() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let boundary = page.boundaries().default().unwrap();

        assert_eq!(boundary.left, 0.0);
        assert_eq!(boundary.top, 841.92);
        assert_eq!(boundary.bottom, 0.0);
        assert_eq!(boundary.right, 594.95996);
    }
}
