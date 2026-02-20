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

use crate::{
    document::PdfiumDocument,
    error::{PdfiumError, PdfiumResult},
    lib,
    pdfium_types::{DestinationHandle, FPDF_DEST, Handle},
};

/// # Rust interface to FPDF_DEST
#[derive(Debug, Clone)]
pub struct PdfiumDestination {
    handle: DestinationHandle,
}

impl PdfiumDestination {
    pub(crate) fn new_from_handle(handle: FPDF_DEST) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            Ok(Self {
                handle: Handle::new(handle, None), // TODO: check close is not needed
            })
        }
    }

    /// Returns the zero-based page index this destination points to,
    /// or `None` on failure.
    pub fn index(&self, document: &PdfiumDocument) -> Option<i32> {
        let val = lib().FPDFDest_GetDestPageIndex(document, self);
        if val >= 0 { Some(val) } else { None }
    }

    /// Returns `(view_mode, view_pos)` for this destination.
    ///
    /// `view_mode` is a `PDFDEST_VIEW_*` constant; `view_pos` contains 0–4
    /// floats whose meaning depends on `view_mode`.
    pub fn view(&self) -> (u64, Vec<f32>) {
        let mut n_params: u64 = 0;
        let mut params = [0f32; 4];
        let mode = lib().FPDFDest_GetView(self, &mut n_params, &mut params[0]);
        let pos = params[..n_params as usize].to_vec();
        (mode, pos)
    }
}

impl From<&PdfiumDestination> for FPDF_DEST {
    fn from(destination: &PdfiumDestination) -> Self {
        destination.handle.handle()
    }
}
