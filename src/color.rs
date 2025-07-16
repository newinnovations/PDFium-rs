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

use crate::pdfium_types::FPDF_DWORD;

/// A 32-bit color value with alpha channel.
///
/// PDFium uses BGRA by default
#[derive(Debug, Copy, Clone)]
pub struct PdfiumColor {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl PdfiumColor {
    pub const WHITE: PdfiumColor = PdfiumColor::new(255, 255, 255, 255);
    pub const BLACK: PdfiumColor = PdfiumColor::new(0, 0, 0, 255);

    /// Constructs a new [`PdfiumColor`] instance from the given components.
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl From<&PdfiumColor> for FPDF_DWORD {
    /// Converts the [`PdfiumColor`] to the BGRA value
    fn from(value: &PdfiumColor) -> Self {
        let red = value.red as FPDF_DWORD;
        let green = value.green as FPDF_DWORD;
        let blue = value.blue as FPDF_DWORD;
        let alpha = value.alpha as FPDF_DWORD;
        (alpha << 24) | (red << 16) | (green << 8) | blue
    }
}
