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

use crate::{pdfium_types::FS_MATRIX, PdfiumRotation};

/// Rust interface to FS_MATRIX
#[derive(Debug, Copy, Clone)]
pub struct PdfiumMatrix {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
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

    /// Creates a new [`PdfiumMatrix`] with the given `scale` and optional `pan` value.
    pub const fn new_scale_opt_pan(scale: f32, pan: Option<(f32, f32)>) -> Self {
        if let Some((pan_x, pan_y)) = pan {
            Self::new_scale_pan(scale, pan_x, pan_y)
        } else {
            Self::new_scale(scale)
        }
    }

    /// Null matrix.
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

    /// Identity matrix.
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

    /// Pure rotations (clockwise) about the origin specified by `rotation`
    pub const fn rotation(rotation: PdfiumRotation) -> Self {
        match rotation {
            PdfiumRotation::None => Self {
                a: 1.0,
                b: 0.0,
                c: 0.0,
                d: 1.0,
                e: 0.0,
                f: 0.0,
            },
            PdfiumRotation::Cw90 => Self {
                a: 0.0,
                b: -1.0,
                c: 1.0,
                d: 0.0,
                e: 0.0,
                f: 0.0,
            },
            PdfiumRotation::Cw180 => Self {
                a: -1.0,
                b: 0.0,
                c: 0.0,
                d: -1.0,
                e: 0.0,
                f: 0.0,
            },
            PdfiumRotation::Cw270 => Self {
                a: 0.0,
                b: 1.0,
                c: -1.0,
                d: 0.0,
                e: 0.0,
                f: 0.0,
            },
        }
    }

    /// General clockwise rotation by `degrees` about the origin.
    pub fn rotation_degrees(degrees: f32) -> Self {
        // clockwise => negative angle
        let theta = -degrees.to_radians();
        let (cos_t, sin_t) = (theta.cos(), theta.sin());
        Self {
            a: cos_t,
            b: sin_t,
            c: -sin_t,
            d: cos_t,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Apply to a point (x, y) -> (x', y')
    pub fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.a * x + self.c * y + self.e,
            self.b * x + self.d * y + self.f,
        )
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

/// Compose `self` with `other` (apply `other` first, then `self`).
/// This matches PDF's `cm` operator semantics: new_ctm = M * CTM.
impl std::ops::Mul for PdfiumMatrix {
    type Output = PdfiumMatrix;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a * rhs.a + self.c * rhs.b,
            b: self.b * rhs.a + self.d * rhs.b,
            c: self.a * rhs.c + self.c * rhs.d,
            d: self.b * rhs.c + self.d * rhs.d,
            e: self.a * rhs.e + self.c * rhs.f + self.e,
            f: self.b * rhs.e + self.d * rhs.f + self.f,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_matrix_scale_pan() {
        let matrix = PdfiumMatrix::new_scale(2.0);
        assert_eq!(matrix.apply(3.0, 4.0), (6.0, 8.0));
        let matrix = PdfiumMatrix::new_scale_pan(3.0, 2.5, 2.8);
        assert_eq!(matrix.apply(3.0, 4.0), (11.5, 14.8));
    }

    #[test]
    fn test_matrix_rotate() {
        let matrix = PdfiumMatrix::rotation(PdfiumRotation::None);
        assert_eq!(matrix.apply(3.0, 4.0), (3.0, 4.0));

        let matrix = PdfiumMatrix::rotation(PdfiumRotation::Cw90);
        assert_eq!(matrix.apply(3.0, 4.0), (4.0, -3.0));

        let matrix = PdfiumMatrix::rotation(PdfiumRotation::Cw180);
        assert_eq!(matrix.apply(3.0, 4.0), (-3.0, -4.0));

        let matrix = PdfiumMatrix::rotation(PdfiumRotation::Cw270);
        assert_eq!(matrix.apply(3.0, 4.0), (-4.0, 3.0));
    }

    #[test]
    fn test_matrix_mul() {
        let matrix = PdfiumMatrix::new_scale(2.0) * PdfiumMatrix::rotation(PdfiumRotation::Cw90);
        assert_eq!(matrix.apply(3.0, 4.0), (8.0, -6.0));
        let matrix = PdfiumMatrix::rotation(PdfiumRotation::Cw90) * PdfiumMatrix::new_scale(2.0);
        assert_eq!(matrix.apply(3.0, 4.0), (8.0, -6.0));
        let matrix = PdfiumMatrix::rotation(PdfiumRotation::Cw180)
            * PdfiumMatrix::new_scale_pan(3.0, 2.5, 2.8);
        assert_eq!(matrix.apply(3.0, 4.0), (-11.5, -14.8));
        let matrix = PdfiumMatrix::new_scale_pan(3.0, 2.5, 2.8)
            * PdfiumMatrix::rotation(PdfiumRotation::Cw180);
        assert_eq!(matrix.apply(3.0, 4.0), (-6.5, -9.2));
    }
}
