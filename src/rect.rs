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

use crate::pdfium_types::FS_RECTF;

/// Rust interface to FS_RECTF
#[derive(Debug, Copy, Clone)]
pub struct PdfiumRect {
    /// The x-coordinate of the left-top corner.
    pub left: f32,
    /// The y-coordinate of the left-top corner.
    pub top: f32,
    /// The x-coordinate of the right-bottom corner.
    pub right: f32,
    /// The y-coordinate of the right-bottom corner.
    pub bottom: f32,
}

impl PdfiumRect {
    /// Creates a new [`PdfiumRect`] object with all values set to `0.0`.
    pub fn zero() -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }

    /// Creates a new [`PdfiumRect`] with the given values.
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Creates a new [`PdfiumRect`] from a (left, bottom, right, top) tuple.
    pub fn new_from_lbrt(value: (f32, f32, f32, f32)) -> Self {
        Self {
            left: value.0,
            top: value.3,
            right: value.2,
            bottom: value.1,
        }
    }

    /// Returns the width of this [`PdfiumRect`].
    #[inline]
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    /// Returns the height of this [`PdfiumRect`].
    #[inline]
    pub fn height(&self) -> f32 {
        self.top - self.bottom
    }
}

impl From<FS_RECTF> for PdfiumRect {
    fn from(rect: FS_RECTF) -> Self {
        Self::new(rect.left, rect.top, rect.right, rect.bottom)
    }
}

impl From<&PdfiumRect> for FS_RECTF {
    fn from(rect: &PdfiumRect) -> Self {
        FS_RECTF {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

impl From<&PdfiumRect> for *const FS_RECTF {
    fn from(rect: &PdfiumRect) -> Self {
        let rect: FS_RECTF = rect.into();
        &rect
    }
}
