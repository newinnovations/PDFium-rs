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

pub mod boundaries;

use crate::{
    PdfiumColor, PdfiumMatrix, PdfiumRect,
    bitmap::{PdfiumBitmap, PdfiumBitmapFormat},
    error::{PdfiumError, PdfiumResult},
    guard::lib,
    page::boundaries::PdfiumPageBoundaries,
    pdfium_constants,
    pdfium_types::{FPDF_PAGE, FS_MATRIX, FS_RECTF},
};

/// # Rust interface to FPDF_PAGE
pub struct PdfiumPage {
    handle: FPDF_PAGE,
}

use bitflags::bitflags;

bitflags! {
    /// Flags controlling the PDFium rendering behavior
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PdfiumRenderFlags: i32 {
        /// ANNOT: Render annotations (comments, highlights, etc.)
        const ANNOT = pdfium_constants::FPDF_ANNOT;
        /// LCD_TEXT: Use LCD text rendering for better clarity on LCD screens
        const LCD_TEXT = pdfium_constants::FPDF_LCD_TEXT;
        /// NO_NATIVETEXT: Don't use native text rendering optimizations
        const NO_NATIVETEXT = pdfium_constants::FPDF_NO_NATIVETEXT;
        /// GRAYSCALE: Render in grayscale
        const GRAYSCALE = pdfium_constants::FPDF_GRAYSCALE;
        /// REVERSE_BYTE_ORDER: Reverse byte order for pixel data
        const REVERSE_BYTE_ORDER = pdfium_constants::FPDF_REVERSE_BYTE_ORDER;
        /// CONVERT_FILL_TO_STROKE: Convert filled paths to stroked paths
        const CONVERT_FILL_TO_STROKE = pdfium_constants::FPDF_CONVERT_FILL_TO_STROKE;
        /// DEBUG_INFO: Include debug information
        const DEBUG_INFO = pdfium_constants::FPDF_DEBUG_INFO;
        /// NO_CATCH: Don't catch exceptions (for debugging)
        const NO_CATCH = pdfium_constants::FPDF_NO_CATCH;
        /// RENDER_LIMITEDIMAGECACHE: Limit image cache size
        const RENDER_LIMITEDIMAGECACHE = pdfium_constants::FPDF_RENDER_LIMITEDIMAGECACHE;
        /// RENDER_FORCEHALFTONE: Force halftone rendering
        const RENDER_FORCEHALFTONE = pdfium_constants::FPDF_RENDER_FORCEHALFTONE;
        /// PRINTING: Optimize for printing
        const PRINTING = pdfium_constants::FPDF_PRINTING;
        /// RENDER_NO_SMOOTHTEXT*: Disable anti-aliasing for text
        const RENDER_NO_SMOOTHTEXT = pdfium_constants::FPDF_RENDER_NO_SMOOTHTEXT;
        /// RENDER_NO_SMOOTHIMAGE: Disable anti-aliasing for images
        const RENDER_NO_SMOOTHIMAGE = pdfium_constants::FPDF_RENDER_NO_SMOOTHIMAGE;
        /// RENDER_NO_SMOOTHPATH: Disable anti-aliasing for paths
        const RENDER_NO_SMOOTHPATH = pdfium_constants::FPDF_RENDER_NO_SMOOTHPATH;
    }
}

impl PdfiumPage {
    pub(crate) fn new_from_handle(handle: FPDF_PAGE) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            println!("New page {handle:?}");
            Ok(Self { handle })
        }
    }

    pub fn boundaries(&self) -> PdfiumPageBoundaries {
        PdfiumPageBoundaries::new(self)
    }

    /// # Renders this [`PdfiumPage`] into a new [`PdfiumBitmap`] using a transformation matrix.
    ///
    /// This function provides fine-grained control over the rendering process by allowing
    /// you to directly specify a transformation matrix, background color, render flags,
    /// and clipping rectangle. The transformation matrix can be used to apply scaling,
    /// rotation or translation transformations during rendering.
    ///
    /// ## Arguments
    ///
    /// * `width` - The width of the target bitmap in pixels
    /// * `height` - The height of the target bitmap in pixels
    /// * `format` - The pixel format for the target bitmap
    /// * `background` - Optional background color to fill the bitmap before rendering
    /// * `matrix` - The transformation matrix to apply during rendering
    /// * `render_flags` - Flags controlling the rendering behavior (0 for most use cases)
    /// * `clipping` - Optional clipping rectangle to restrict rendering to a specific area
    ///
    /// ## Returns
    ///
    /// Returns a [`PdfiumBitmap`] containing the rendered page, or a [PdfiumError] if rendering fails.
    ///
    /// ## Examples
    ///
    /// ```
    /// use pdfium::*;
    ///
    /// fn page_to_bitmap(
    ///     document: &PdfiumDocument,
    ///     index: i32,
    ///     height: i32,
    /// ) -> PdfiumResult<PdfiumBitmap> {
    ///     let page = document.page(index)?;
    ///     let bounds = page.boundaries().media()?;
    ///     let zoom = height as f32 / bounds.height();
    ///     let width = (bounds.width() * zoom) as i32;
    ///     let matrix = PdfiumMatrix::new_scale(zoom);
    ///     page.render_with_matrix(
    ///         width,
    ///         height,
    ///         PdfiumBitmapFormat::Bgra,
    ///         Some(PdfiumColor::WHITE),
    ///         &matrix,
    ///         PdfiumRenderFlags::empty(),
    ///         None,
    ///     )
    /// }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn render_with_matrix(
        &self,
        width: i32,
        height: i32,
        format: PdfiumBitmapFormat,
        background: Option<PdfiumColor>,
        matrix: &PdfiumMatrix,
        render_flags: PdfiumRenderFlags,
        clipping: Option<PdfiumRect>,
    ) -> PdfiumResult<PdfiumBitmap> {
        let mut bitmap = PdfiumBitmap::empty(width, height, format)?;

        if let Some(color) = background {
            bitmap.fill(&color)?;
        }

        self.render_into_bitmap_with_matrix(&mut bitmap, matrix, render_flags, clipping);

        Ok(bitmap)
    }

    /// # Renders this [`PdfiumPage`] into the given [`PdfiumBitmap`] using a transformation matrix.
    ///
    /// This function provides fine-grained control over the rendering process by allowing
    /// you to directly specify a transformation matrix, render flags, and clipping rectangle.
    /// The transformation matrix can be used to apply scaling, rotation or translation
    /// transformations during rendering.
    ///
    /// ## Arguments
    ///
    /// * `bitmap` - The target bitmap to render into
    /// * `matrix` - The transformation matrix to apply during rendering
    /// * `render_flags` - Flags controlling the rendering behavior (0 for most use cases)
    /// * `clipping` - Optional clipping rectangle to restrict rendering to a specific area.
    ///   If None, defaults to the full bitmap dimensions.
    ///
    /// ## Returns
    ///
    /// Returns `Ok(())` if rendering succeeds, or a [PdfiumError] if rendering fails.
    pub fn render_into_bitmap_with_matrix(
        &self,
        bitmap: &mut PdfiumBitmap,
        matrix: &PdfiumMatrix,
        render_flags: PdfiumRenderFlags,
        clipping: Option<PdfiumRect>,
    ) {
        let clipping = clipping.unwrap_or(PdfiumRect::new(
            0.0,
            0.0,
            bitmap.width() as f32,
            bitmap.height() as f32,
        ));
        let clipping: FS_RECTF = (&clipping).into();
        let matrix: FS_MATRIX = matrix.into();
        lib().FPDF_RenderPageBitmapWithMatrix(
            bitmap,
            self,
            &matrix,
            &clipping,
            render_flags.bits(),
        );
    }
}

impl From<&PdfiumPage> for FPDF_PAGE {
    #[inline]
    fn from(value: &PdfiumPage) -> Self {
        value.handle
    }
}

impl Drop for PdfiumPage {
    /// # Closes this [`PdfiumPage`], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        println!("Closing page {:?}", self.handle);
        lib().FPDF_ClosePage(self);
    }
}

#[cfg(test)]
mod tests {
    use crate::document::PdfiumDocument;

    #[test]
    fn test_load_sequential_pages() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0);
        assert!(page.is_ok());
        let page = document.page(1);
        assert!(page.is_ok());
        let page = document.page(0);
        assert!(page.is_ok());
        let page = document.page(1);
        assert!(page.is_ok());
        let page = document.page(0);
        assert!(page.is_ok());
        let page = document.page(1);
        assert!(page.is_ok());
    }

    #[test]
    fn test_parallel_load_1() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0);
        assert!(page.is_ok());
    }

    #[test]
    fn test_parallel_load_2() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(1);
        assert!(page.is_ok());
    }

    #[test]
    fn test_parallel_load_3() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0);
        assert!(page.is_ok());
    }

    #[test]
    fn test_parallel_load_4() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(1);
        assert!(page.is_ok());
    }

    #[test]
    fn test_parallel_load_5() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0);
        assert!(page.is_ok());
    }

    #[test]
    fn test_parallel_load_6() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(1);
        assert!(page.is_ok());
    }

    #[test]
    fn test_parallel_load_7() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0);
        assert!(page.is_ok());
    }

    #[test]
    fn test_parallel_load_8() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(1);
        assert!(page.is_ok());
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
