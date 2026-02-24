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

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum PdfiumDestinationView {
    #[default]
    Unknown,
    /// X, Y, Zoom
    #[allow(clippy::upper_case_acronyms)] // PDFium uses uppercase acronyms
    XYZ(f32, f32, f32),
    /// Fits the page within the window.
    Fit,
    /// Fits the horizontal width of the page at the top of the window.
    ///
    /// Parameters:
    /// * (top_y) - The y coordinate of the top of the window.
    FitH(f32),
    /// Fits the vertical height of the page at the left of the window.
    ///
    /// Parameters:
    /// * (left_x) - The x coordinate of the left of the window.
    FitV(f32),
    /// Fits a specific rectangle
    ///
    /// Parameters:
    /// * (left, bottom, right, top) - The coordinates of the rectangle.
    FitR(f32, f32, f32, f32),
    /// Fits the page bounding box
    FitB,
    /// Fits the horizontal width of the page bounding box at the top of the window.
    ///
    /// Parameters:
    /// * (top_y) - The y coordinate of the top of the page bounding box.
    FitBH(f32),
    /// Fits the vertical height of the page bounding box at the left of the window.
    ///
    /// Parameters:
    /// * (left_x) - The x coordinate of the left of the page bounding box.
    FitBV(f32),
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

    /// Returns the view for this destination.
    pub fn view(&self) -> PdfiumDestinationView {
        let mut n_params: std::os::raw::c_ulong = 0; // The value is indeed not used, because the number of parameters is determined by the view mode, not dynamically returned
        let mut params = [0f32; 4]; // Safety: Maximum number of parameters for any destination view is 4, so no out-of-bounds access possible
        let mode: std::os::raw::c_ulong =
            lib().FPDFDest_GetView(self, &mut n_params, &mut params[0]);
        // PDFium guarantees that the number of parameters stays consistent with the view mode, so we
        // can safely index into the params array.
        match mode {
            0 => PdfiumDestinationView::Unknown,
            1 => PdfiumDestinationView::XYZ(params[0], params[1], params[2]),
            2 => PdfiumDestinationView::Fit,
            3 => PdfiumDestinationView::FitH(params[0]),
            4 => PdfiumDestinationView::FitV(params[0]),
            5 => PdfiumDestinationView::FitR(params[0], params[1], params[2], params[3]),
            6 => PdfiumDestinationView::FitB,
            7 => PdfiumDestinationView::FitBH(params[0]),
            8 => PdfiumDestinationView::FitBV(params[0]),
            // Impossible view mode enum variant to reach
            _ => unreachable!("Unknown destination view mode {}", mode),
        }
    }
}

impl From<&PdfiumDestination> for FPDF_DEST {
    fn from(destination: &PdfiumDestination) -> Self {
        destination.handle.handle()
    }
}
