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

use crate::{
    error::{PdfiumError, PdfiumResult},
    guard::lib,
    pdfium_types::FPDF_ANNOTATION,
};

/// # Rust interface to FPDF_ANNOTATION
pub struct PdfiumAnnotation {
    handle: FPDF_ANNOTATION,
}

impl PdfiumAnnotation {
    pub(crate) fn new_from_handle(handle: FPDF_ANNOTATION) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            println!("New page {handle:?}");
            Ok(Self { handle })
        }
    }
}

impl From<&PdfiumAnnotation> for FPDF_ANNOTATION {
    fn from(value: &PdfiumAnnotation) -> Self {
        value.handle
    }
}

impl Drop for PdfiumAnnotation {
    /// # Closes this [`PdfiumAnnotation`], releasing held memory.
    fn drop(&mut self) {
        println!("Closing annotation {:?}", self.handle);
        lib().FPDFPage_CloseAnnot(self);
    }
}
