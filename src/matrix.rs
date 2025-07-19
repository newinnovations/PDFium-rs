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

use crate::pdfium_types::FS_MATRIX;

/// Rust interface to FS_MATRIX
#[derive(Debug, Copy, Clone)]
pub struct PdfiumMatrix {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl PdfiumMatrix {
    /// Creates a new [`PdfiumMatrix`] with the given matrix values.
    pub const fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        Self { a, b, c, d, e, f }
    }

    /// Creates a new [`PdfiumMatrix`] with the given `scale` and `pan` values.
    pub const fn new_scale_pan(scale: f32, pan_x: f32, pan_y: f32) -> Self {
        Self {
            a: scale,
            b: 0.0,
            c: 0.0,
            d: scale,
            e: pan_x,
            f: pan_y,
        }
    }

    /// Creates a new [`PdfiumMatrix`] with the given `scale` value.
    pub const fn new_scale(scale: f32) -> Self {
        Self {
            a: scale,
            b: 0.0,
            c: 0.0,
            d: scale,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Creates a new [`PdfiumMatrix`] object with all matrix values set to `0.0`.
    pub const fn zero() -> Self {
        Self {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Creates a new [`PdfiumMatrix`] object with matrix values `a` and `d` set to `1.0`
    /// and all other values set to `0.0`.
    pub const fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }
}

impl From<&PdfiumMatrix> for FS_MATRIX {
    fn from(matrix: &PdfiumMatrix) -> Self {
        FS_MATRIX {
            a: matrix.a,
            b: matrix.b,
            c: matrix.c,
            d: matrix.d,
            e: matrix.e,
            f: matrix.f,
        }
    }
}

impl From<FS_MATRIX> for PdfiumMatrix {
    #[inline]
    fn from(matrix: FS_MATRIX) -> Self {
        Self::new(matrix.a, matrix.b, matrix.c, matrix.d, matrix.e, matrix.f)
    }
}
