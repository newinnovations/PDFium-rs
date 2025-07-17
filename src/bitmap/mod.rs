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

use image::{DynamicImage, ImageFormat, RgbaImage};

use crate::{
    PdfiumColor,
    error::{PdfiumError, PdfiumResult},
    guard::{PdfiumGuard, lib, try_lib},
    pdfium_constants,
    pdfium_types::FPDF_BITMAP,
};

/// Rust interface to FPDF_BITMAP
pub struct PdfiumBitmap {
    handle: FPDF_BITMAP,
}

impl PdfiumBitmap {
    pub(crate) fn new_from_handle(handle: FPDF_BITMAP) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            println!("New bitmap {handle:?}");
            Ok(Self { handle })
        }
    }

    /// Creates a new [`PdfiumBitmap`] with the given `width`, `height` and [`PdfiumBitmapFormat`].
    pub fn empty(width: i32, height: i32, format: PdfiumBitmapFormat) -> PdfiumResult<Self> {
        let lib = try_lib()?;
        lib.FPDFBitmap_CreateEx(
            width,
            height,
            format.into(),
            None, // If this parameter is NULL, then PDFium will create its own buffer.
            0,    // Number of bytes for each scan line, for external buffer only
        )
    }

    /// Fills this entire [`PdfiumBitmap`] with the given [`PdfiumColor`].
    pub fn fill(&self, color: &PdfiumColor) {
        let lib = lib();
        lib.FPDFBitmap_FillRect(
            self,
            0,
            0,
            lib.FPDFBitmap_GetWidth(self),
            lib.FPDFBitmap_GetHeight(self),
            color.into(),
        );
    }

    /// Returns the width of the image in the bitmap buffer backing this [`PdfiumBitmap`].
    #[inline]
    pub fn width(&self) -> i32 {
        lib().FPDFBitmap_GetWidth(self) as i32
    }

    /// Returns the height of the image in the bitmap buffer backing this [`PdfiumBitmap`].
    #[inline]
    pub fn height(&self) -> i32 {
        lib().FPDFBitmap_GetHeight(self) as i32
    }

    /// Returns the pixel format of the image in the bitmap buffer backing this [`PdfiumBitmap`].
    #[inline]
    pub fn format(&self) -> PdfiumBitmapFormat {
        lib().FPDFBitmap_GetFormat(self).into()
    }

    /// Returns an immutable reference to the bitmap buffer backing this [`PdfiumBitmap`].
    ///
    /// This function does not attempt any color channel normalization.
    pub fn as_raw_bytes<'a>(&self, lib: &'a PdfiumGuard) -> &'a [u8] {
        let buffer = lib.FPDFBitmap_GetBuffer(self);
        let len = lib.FPDFBitmap_GetStride(self) * lib.FPDFBitmap_GetHeight(self);
        unsafe { std::slice::from_raw_parts(buffer as *const u8, len as usize) }
    }

    /// Returns an owned copy of the bitmap buffer backing this [`PdfiumBitmap`] as RGBA.
    ///
    /// Normalizing all color channels into RGBA irrespective of the original pixel format.
    pub fn as_rgba_bytes(&self) -> PdfiumResult<Vec<u8>> {
        let lib = lib();
        match self.format() {
            PdfiumBitmapFormat::Bgra => Ok(self
                .as_raw_bytes(&lib)
                .chunks_exact(4)
                .flat_map(|pixel| [pixel[2], pixel[1], pixel[0], pixel[3]]) // B,G,R,A -> R,G,B,A
                .collect()),
            PdfiumBitmapFormat::Unknown
            | PdfiumBitmapFormat::Gray
            | PdfiumBitmapFormat::Bgr
            | PdfiumBitmapFormat::Bgrx
            | PdfiumBitmapFormat::BgraPremul => Err(PdfiumError::UnsupportedImageFormat),
        }
    }

    /// Returns a copy of this a bitmap as a [`DynamicImage::ImageRgba8`]
    pub fn as_image(&self) -> PdfiumResult<DynamicImage> {
        let rgba_bytes = self.as_rgba_bytes()?;
        match RgbaImage::from_raw(self.width() as u32, self.height() as u32, rgba_bytes) {
            Some(image) => Ok(DynamicImage::ImageRgba8(image)),
            None => Err(PdfiumError::ImageError),
        }
    }

    /// Saves this bitmap to the given path.
    ///
    /// As the underlying image contains an alpha channel, the [`ImageFormat`] needs to support it.
    pub fn save(&self, path: &str, format: ImageFormat) -> PdfiumResult<()> {
        let image = self.as_image()?;
        image
            .save_with_format(path, format)
            .or(Err(PdfiumError::ImageError))
    }
}

impl From<&PdfiumBitmap> for FPDF_BITMAP {
    #[inline]
    fn from(value: &PdfiumBitmap) -> Self {
        value.handle
    }
}

impl From<&mut PdfiumBitmap> for FPDF_BITMAP {
    #[inline]
    fn from(value: &mut PdfiumBitmap) -> Self {
        value.handle
    }
}
impl Drop for PdfiumBitmap {
    /// Closes this [PdfiumBitmap], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        println!("Closing bitmap {:?}", self.handle);
        lib().FPDFBitmap_Destroy(self);
    }
}

/// The pixel format of the backing buffer of a [PdfiumBitmap].
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(i32)]
pub enum PdfiumBitmapFormat {
    Unknown = pdfium_constants::FPDFBitmap_Unknown,
    Gray = pdfium_constants::FPDFBitmap_Gray,
    Bgr = pdfium_constants::FPDFBitmap_BGR,
    Bgrx = pdfium_constants::FPDFBitmap_BGRx,
    #[default]
    Bgra = pdfium_constants::FPDFBitmap_BGRA,
    BgraPremul = pdfium_constants::FPDFBitmap_BGRA_Premul,
}

impl From<i32> for PdfiumBitmapFormat {
    fn from(value: i32) -> Self {
        match value {
            pdfium_constants::FPDFBitmap_Gray => PdfiumBitmapFormat::Gray,
            pdfium_constants::FPDFBitmap_BGR => PdfiumBitmapFormat::Bgr,
            pdfium_constants::FPDFBitmap_BGRx => PdfiumBitmapFormat::Bgrx,
            pdfium_constants::FPDFBitmap_BGRA => PdfiumBitmapFormat::Bgra,
            pdfium_constants::FPDFBitmap_BGRA_Premul => PdfiumBitmapFormat::BgraPremul,
            _ => PdfiumBitmapFormat::Unknown,
        }
    }
}

impl From<PdfiumBitmapFormat> for i32 {
    fn from(value: PdfiumBitmapFormat) -> Self {
        match value {
            PdfiumBitmapFormat::Unknown => pdfium_constants::FPDFBitmap_Unknown,
            PdfiumBitmapFormat::Gray => pdfium_constants::FPDFBitmap_Gray,
            PdfiumBitmapFormat::Bgr => pdfium_constants::FPDFBitmap_BGR,
            PdfiumBitmapFormat::Bgrx => pdfium_constants::FPDFBitmap_BGRx,
            PdfiumBitmapFormat::Bgra => pdfium_constants::FPDFBitmap_BGRA,
            PdfiumBitmapFormat::BgraPremul => pdfium_constants::FPDFBitmap_BGRA_Premul,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_render_to_image() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(1).unwrap();
        let bounds = page.boundaries().media().unwrap();
        let height = 1080;
        let zoom = height as f32 / bounds.height();
        let width = (bounds.width() * zoom) as i32;
        let matrix = PdfiumMatrix::new_scale(zoom);
        let bitmap = page
            .render_with_matrix(
                width,
                height,
                PdfiumBitmapFormat::Bgra,
                Some(PdfiumColor::WHITE),
                &matrix,
                0,
                None,
            )
            .unwrap();
        bitmap
            .save("groningen-page-2.png", image::ImageFormat::Png)
            .unwrap();
    }
}
