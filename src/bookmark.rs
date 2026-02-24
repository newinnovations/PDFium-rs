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
    PdfiumAction, PdfiumDestination,
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

    #[allow(dead_code)]
    pub(crate) fn is_null(&self) -> bool {
        self.handle.handle().is_null()
    }

    /// The bookmark's title string.
    pub fn title(&self) -> PdfiumResult<String> {
        let lib = lib();
        let buf_len = lib.FPDFBookmark_GetTitle(self, None, 0);
        if buf_len < 2 {
            // We need at least two bytes for a UTF-16 null terminator
            return Ok(String::new());
        }
        let mut buffer = vec![0u8; buf_len as usize];
        // Note from the library: Regardless of the platform, the buffer is always in UTF-16LE encoding.
        // Safety: buffer is valid and large enough, no need to check the returned byte length
        lib.FPDFBookmark_GetTitle(self, Some(&mut buffer), buf_len);

        // The buffer contains UTF-16LE encoded data, but the last two bytes are always a null terminator.
        let utf16_codes: Vec<_> = buffer[..buffer.len().saturating_sub(2)]
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]])) // Convert bytes to u16
            .collect();

        String::from_utf16(&utf16_codes).map_err(|_| PdfiumError::StringEncodingError)
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

    /// Get the first child of the current [`PdfiumBookmark`].
    /// Returns `None` if the bookmark has no children.
    pub fn first_child(&self, document: &PdfiumDocument) -> Option<PdfiumBookmark> {
        // The original C++ API returns a null pointer if there is no first child.
        // We convert this to an Option<PdfiumBookmark> for Rust safety.
        lib().FPDFBookmark_GetFirstChild(document, self).ok()
    }

    /// Get the next sibling of the current [`PdfiumBookmark`].
    /// Returns `None` if the bookmark has no next sibling.
    pub fn next_sibling(&self, document: &PdfiumDocument) -> Option<PdfiumBookmark> {
        // The original C++ API returns a null pointer if there is no next sibling.
        // We convert this to an Option<PdfiumBookmark> for Rust safety.
        lib().FPDFBookmark_GetNextSibling(document, self).ok()
    }

    /// Returns the action associated with this [`PdfiumBookmark`].
    /// Returns `None` if the bookmark has no action and you should check [`PdfiumBookmark::dest()`] instead.
    pub fn action(&self) -> Option<PdfiumAction> {
        // The original C++ API returns a null pointer if there is no action.
        // We convert this to an Option<PdfiumAction> for Rust safety.
        lib().FPDFBookmark_GetAction(self).ok()
    }
}

impl From<&PdfiumBookmark> for FPDF_BOOKMARK {
    fn from(bookmark: &PdfiumBookmark) -> Self {
        bookmark.handle.handle()
    }
}
