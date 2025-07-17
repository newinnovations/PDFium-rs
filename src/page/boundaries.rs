// PDFium-rs -- Safe Rust wrapper to PDFium, the PDF library from Google
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

use crate::{PdfiumRect, error::PdfiumResult, guard::lib, page::PdfiumPage};

/// Rust interface to the boundary boxes of a page
pub struct PdfiumPageBoundaries<'a> {
    page: &'a PdfiumPage,
}

impl<'a> PdfiumPageBoundaries<'a> {
    pub fn new(page: &'a PdfiumPage) -> PdfiumPageBoundaries<'a> {
        Self { page }
    }

    /// Gets "MediaBox" entry from the page dictionary. The Media box is the full page
    /// size, equivalent to the target paper size when the document is printed.
    #[inline]
    pub fn media(&self) -> PdfiumResult<PdfiumRect> {
        Ok(PdfiumRect::new_from_lbrt(
            lib().FPDFPage_GetMediaBox(self.page)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::document::PdfiumDocument;

    #[test]
    fn test_media_boundary() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let media = page.boundaries().media().unwrap();

        assert!(media.left == 0.0);
        assert!(media.top == 841.92);
        assert!(media.bottom == 0.0);
        assert!(media.right == 594.95996);
    }
}
