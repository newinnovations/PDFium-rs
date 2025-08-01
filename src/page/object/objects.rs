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

use std::cell::OnceCell;

use crate::{PdfiumPage, PdfiumPageObject, PdfiumResult};

/// Iterator for [`PdfiumPageObject`]
pub struct PdfiumPageObjects<'a> {
    page: &'a PdfiumPage,
    object_count: OnceCell<i32>,
    current_object: i32,
}

impl<'a> PdfiumPageObjects<'a> {
    pub(crate) fn new(page: &'a PdfiumPage) -> PdfiumPageObjects<'a> {
        Self {
            page,
            object_count: OnceCell::new(),
            current_object: 0,
        }
    }

    /// Returns the number of objects in the [`PdfiumPage`].
    pub fn object_count(&self) -> i32 {
        *self.object_count.get_or_init(|| self.page.object_count())
    }

    /// Returns the [`PdfiumPageObject`] indicated by `index` from the [`PdfiumPage`].
    pub fn get(&self, index: i32) -> PdfiumResult<PdfiumPageObject> {
        self.page.object(index)
    }
}

impl<'a> Iterator for PdfiumPageObjects<'a> {
    type Item = PdfiumResult<PdfiumPageObject>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_object >= self.object_count() {
            None
        } else {
            let page = self.page.object(self.current_object);
            self.current_object += 1;
            Some(page)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.object_count() as usize;
        let remaining = len.saturating_sub(self.current_object as usize);
        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.object_count() as usize - self.current_object as usize
    }

    fn last(mut self) -> Option<Self::Item> {
        let len = self.object_count();
        if len == 0 || self.current_object >= len {
            None
        } else {
            self.current_object = len - 1;
            Some(self.page.object(self.current_object))
        }
    }
}

impl<'a> DoubleEndedIterator for PdfiumPageObjects<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.object_count();
        if self.current_object >= len {
            None
        } else {
            let page = self.page.object(len - 1);
            self.object_count = OnceCell::from(len - 1);
            Some(page)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_object_count() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();

        let page = document.page(0).unwrap();

        let mut objects = page.objects();

        assert_eq!(objects.object_count(), 721);

        let _ = objects.next().unwrap().unwrap();

        assert_eq!(objects.object_count(), 721);
        assert_eq!(objects.count(), 720); // remaining in iterator
    }
}
