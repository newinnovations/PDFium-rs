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

use crate::{
    c_api::i32_to_bool,
    error::{PdfiumError, PdfiumResult},
    lib, pdfium_constants,
    pdfium_types::{Handle, SearchHandle, FPDF_SCHHANDLE},
};

use bitflags::bitflags;

// Define a bitflags struct for PDF search configuration options
bitflags! {
    /// A bitflag type representing various search options for PDF text searching.
    ///
    /// This struct wraps PDFium library constants to provide a type-safe way
    /// to combine multiple search flags using bitwise operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pdfium::*;
    ///
    /// // Search with no flags
    /// let flags = PdfiumSearchFlags::empty();
    ///
    /// // Search with case sensitivity
    /// let flags = PdfiumSearchFlags::MATCH_CASE;
    ///
    /// // Search for whole words only, case-sensitive
    /// let flags = PdfiumSearchFlags::MATCH_CASE | PdfiumSearchFlags::MATCH_WHOLE_WORD;
    ///
    /// // Use all search options
    /// let flags = PdfiumSearchFlags::all();
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PdfiumSearchFlags: i32 {
        /// Enables case-sensitive text matching during PDF search operations.
        /// When this flag is set, "Hello" will not match "hello".
        const MATCH_CASE = pdfium_constants::FPDF_MATCHCASE;

        /// Enables whole word matching during PDF search operations.
        /// When this flag is set, searching for "cat" will not match "category".
        /// The search term must be bounded by word boundaries (spaces, punctuation, etc.).
        const MATCH_WHOLE_WORD = pdfium_constants::FPDF_MATCHWHOLEWORD;

        /// The `CONSECUTIVE` flag modifies the behavior of the text search to start the next
        /// or previous search from the position immediately following or preceding the current
        /// match, rather than skipping to the end or start of the current match. This is
        /// particularly useful for finding overlapping or consecutive matches of the search term
        /// in the text.
        ///
        /// - **Behavior with `CONSECUTIVE`**:
        ///   - When this flag is set, the search continues from the character immediately after the
        ///     start of the current match (for `FPDFText_FindNext`) or immediately before the end of
        ///     the current match (for `FPDFText_FindPrev`).
        ///   - This allows the search to potentially find overlapping instances of the search term.
        ///     For example, searching for "ana" in "banana" could find matches at positions 1 ("ana")
        ///     and 3 ("ana") if `CONSECUTIVE` is enabled.
        ///   - Without this flag, the search would typically skip to the position after the end of
        ///     the current match (for `FPDFText_FindNext`) or before the start of the current match
        ///    (for `FPDFText_FindPrev`), potentially missing overlapping matches.
        ///
        /// - **Default Behavior (without `CONSECUTIVE`)**:
        ///   - If the flag is not set, after finding a match, `FPDFText_FindNext` starts the next
        ///     search from the character after the end of the current match (`m_resEnd + 1`), and
        ///     `FPDFText_FindPrev` starts from the character before the start of the current match
        ///     (`m_resStart - 1`). This prevents overlapping matches.
        const CONSECUTIVE = pdfium_constants::FPDF_CONSECUTIVE;
    }
}

/// # Rust interface to FPDF_SCHHANDLE
#[derive(Debug, Clone)]
pub struct PdfiumSearch {
    handle: SearchHandle,
}

impl PdfiumSearch {
    pub(crate) fn new_from_handle(handle: FPDF_SCHHANDLE) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            Ok(Self {
                handle: Handle::new(handle, Some(close_search)),
            })
        }
    }

    /// Search in the direction from page start to end.
    ///
    /// Returns:
    /// * Whether a match is found.
    pub fn find_next(&self) -> bool {
        i32_to_bool(lib().FPDFText_FindNext(self))
    }

    /// Search in the direction from page end to start.
    ///
    /// Returns:
    /// * Whether a match is found.
    pub fn find_prev(&self) -> bool {
        i32_to_bool(lib().FPDFText_FindPrev(self))
    }

    /// Get the number of matched characters in the search result.
    ///
    /// Returns:
    /// * Number of matched characters.
    pub fn get_count(&self) -> i32 {
        lib().FPDFText_GetSchCount(self)
    }

    /// Get the starting character index of the search result.
    ///
    /// Returns:
    /// * Index for the starting character.
    pub fn get_index(&self) -> i32 {
        lib().FPDFText_GetSchResultIndex(self)
    }
}

impl From<&PdfiumSearch> for FPDF_SCHHANDLE {
    fn from(search: &PdfiumSearch) -> Self {
        search.handle.handle()
    }
}

fn close_search(search: FPDF_SCHHANDLE) {
    lib().FPDFText_FindClose(search);
}

pub struct PdfiumSearchResult {
    index: i32,
    count: i32,
}

impl PdfiumSearchResult {
    pub fn index(&self) -> i32 {
        self.index
    }
    pub fn count(&self) -> i32 {
        self.count
    }
}

pub struct PdfiumSearchIterator {
    pub(crate) inner: PdfiumResult<PdfiumSearch>,
}

impl Iterator for PdfiumSearchIterator {
    type Item = PdfiumSearchResult;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.inner {
            Ok(search) => {
                if search.find_next() {
                    Some(PdfiumSearchResult {
                        index: search.get_index(),
                        count: search.get_count(),
                    })
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}
