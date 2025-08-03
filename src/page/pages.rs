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

use std::{cell::OnceCell, ffi::CString, os::raw::c_ulong, ptr::null};

use crate::{lib, PdfiumDocument, PdfiumPage, PdfiumResult};

/// Iterator for [`PdfiumPage`]
pub struct PdfiumPages<'a> {
    doc: &'a PdfiumDocument,
    page_count: OnceCell<i32>,
    current_page: i32,
}

impl<'a> PdfiumPages<'a> {
    pub(crate) fn new(doc: &'a PdfiumDocument) -> PdfiumPages<'a> {
        Self {
            doc,
            page_count: OnceCell::new(),
            current_page: 0,
        }
    }

    /// Returns the number of pages in the [`PdfiumDocument`].
    pub fn page_count(&self) -> i32 {
        *self.page_count.get_or_init(|| self.doc.page_count())
    }

    /// Returns the [`PdfiumPage`] indicated by `index` from the [`PdfiumDocument`].
    pub fn get(&self, index: i32) -> PdfiumResult<PdfiumPage> {
        self.doc.page(index)
    }

    /// Import pages to this [`PdfiumDocument`].
    ///
    /// # Arguments
    /// * `src_doc` - The document to be imported.
    /// * `pagerange` - A page range string, Such as "1,3,5-7". The first page is one.
    ///   If `pagerange` is empty, all pages from `src_doc` are imported.
    /// * index - The page index at which to insert the first imported page into
    ///   dest_doc. The first page is zero.
    ///
    /// Returns `Err` if any pages in `pagerange` is invalid or cannot be read.
    #[inline]
    pub fn import(
        &self,
        src_doc: &PdfiumDocument,
        pagerange: &str,
        index: i32,
    ) -> PdfiumResult<()> {
        let pagerange = CString::new(pagerange)?;
        lib().FPDF_ImportPages(self.doc, src_doc, &pagerange, index)
    }

    /// Import pages to this [`PdfiumDocument`] by index.
    ///
    /// # Arguments
    /// * `src_doc` - The document to be imported.
    /// * `src_indices` - An array of page indices to be imported. The first page is
    ///   zero. If `src_indices` is None, all pages from `src_doc` are imported.
    /// * `index` - The page index at which to insert the first imported page
    ///   into `dest_doc`. The first page is zero.
    ///
    /// Returns `Err` if any pages in `src_indices` is invalid or cannot be read.
    #[inline]
    pub fn import_by_index(
        &self,
        src_doc: &PdfiumDocument,
        src_indices: Option<&[i32]>,
        index: i32,
    ) -> PdfiumResult<()> {
        match src_indices {
            Some(indices) => lib().FPDF_ImportPagesByIndex(
                self.doc.into(),
                src_doc.into(),
                indices.as_ptr(),
                indices.len() as c_ulong,
                index,
            ),
            None => {
                lib().FPDF_ImportPagesByIndex(self.doc.into(), src_doc.into(), null(), 0, index)
            }
        }
    }
}

impl<'a> Iterator for PdfiumPages<'a> {
    type Item = PdfiumResult<PdfiumPage>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page >= self.page_count() {
            None
        } else {
            let page = self.doc.page(self.current_page);
            self.current_page += 1;
            Some(page)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.page_count() as usize;
        let remaining = len.saturating_sub(self.current_page as usize);
        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.page_count() as usize - self.current_page as usize
    }

    fn last(mut self) -> Option<Self::Item> {
        let len = self.page_count();
        if len == 0 || self.current_page >= len {
            None
        } else {
            self.current_page = len - 1;
            Some(self.doc.page(self.current_page))
        }
    }
}

impl<'a> DoubleEndedIterator for PdfiumPages<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.page_count();
        if self.current_page >= len {
            None
        } else {
            let page = self.doc.page(len - 1);
            self.page_count = OnceCell::from(len - 1); // Update page_count to reflect last page accessed
            Some(page)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();

        let mut pages = document.pages();

        assert_eq!(pages.page_count(), 2);

        let _ = pages.next().unwrap().unwrap();

        assert_eq!(pages.page_count(), 2);
        assert_eq!(pages.count(), 1); // remaining in iterator
    }

    #[test]
    fn test_import_pages() {
        let document = PdfiumDocument::new().unwrap();
        let src_doc = PdfiumDocument::new_from_path("resources/pg1342-images-3.pdf", None).unwrap();
        document.pages().import(&src_doc, "12,14,30-34", 0).unwrap();
        document.save_to_path("pride-1.pdf", None).unwrap();
        let document = PdfiumDocument::new_from_path("pride-1.pdf", None).unwrap();
        let page_count = document.page_count();
        assert_eq!(page_count, 7);
    }

    #[test]
    fn test_import_pages_by_index() {
        let document = PdfiumDocument::new().unwrap();
        let src_doc = PdfiumDocument::new_from_path("resources/pg1342-images-3.pdf", None).unwrap();
        document
            .pages()
            .import_by_index(&src_doc, Some(&[11, 13, 29, 30, 31, 32, 33]), 0)
            .unwrap();
        document.save_to_path("pride-2.pdf", None).unwrap();
        let document = PdfiumDocument::new_from_path("pride-2.pdf", None).unwrap();
        let page_count = document.page_count();
        assert_eq!(page_count, 7);
    }
}
