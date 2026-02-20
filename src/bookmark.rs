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
    PdfiumDestination,
    document::PdfiumDocument,
    error::{PdfiumError, PdfiumResult},
    lib,
    pdfium_types::{BookmarkHandle, FPDF_BOOKMARK, Handle},
};

/// # Rust interface to FPDF_BOOKMARK
#[derive(Debug, Clone)]
pub struct PdfiumBookmark {
    handle: BookmarkHandle,
    level: Option<u32>,
}

impl PdfiumBookmark {
    pub(crate) fn new_from_handle(handle: FPDF_BOOKMARK) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            Ok(Self {
                handle: Handle::new(handle, None), // TODO: check close is not needed
                level: None,
            })
        }
    }

    pub(crate) fn null() -> Self {
        Self {
            handle: Handle::new(std::ptr::null_mut(), None),
            level: None,
        }
    }

    pub(crate) fn is_null(&self) -> bool {
        self.handle.handle().is_null()
    }

    /// The bookmark's title string.
    pub fn title(&self) -> PdfiumResult<String> {
        let lib = lib();
        let buf_len = lib.FPDFBookmark_GetTitle(self, None, 0);
        if buf_len == 0 {
            Ok(String::new())
        } else {
            let mut buffer = vec![0u16; buf_len as usize / 2];
            // Safety: The alignment of u8 is less than or equal to u16, so this is safe.
            let (_prefix, u8_slice, _suffix) = unsafe { buffer.align_to_mut::<u8>() };
            lib.FPDFBookmark_GetTitle(self, Some(u8_slice), buf_len);
            Ok(String::from_utf16(&buffer[..buf_len as usize / 2 - 1])
                .map_err(|_| PdfiumError::StringEncodingError)?)
        }
    }

    /// Signed number of child bookmarks that would be visible if the bookmark were open (i.e. recursively counting children of open children).
    pub fn count(&self) -> i32 {
        lib().FPDFBookmark_GetCount(self)
    }

    /// The bookmark's nesting level (0 = top-level).
    pub fn level(&self) -> Option<u32> {
        self.level
    }

    pub(crate) fn set_level(&mut self, level: u32) {
        self.level = Some(level);
    }

    /// Returns the destination associated with this [`PdfiumBookmark`]
    pub fn dest(&self, document: &PdfiumDocument) -> PdfiumResult<PdfiumDestination> {
        lib().FPDFBookmark_GetDest(document, self)
    }
}

impl From<&PdfiumBookmark> for FPDF_BOOKMARK {
    fn from(bookmark: &PdfiumBookmark) -> Self {
        bookmark.handle.handle()
    }
}
