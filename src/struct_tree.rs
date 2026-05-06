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
    PdfiumPage, PdfiumStructElement,
    error::{PdfiumError, PdfiumResult},
    lib,
    pdfium_types::{FPDF_STRUCTTREE, Handle, StructTreeHandle},
};

/// # Rust interface to FPDF_STRUCTTREE
#[derive(Debug, Clone)]
pub struct PdfiumStructTree {
    handle: StructTreeHandle,
    owner: Option<PdfiumPage>,
}

impl PdfiumStructTree {
    pub(crate) fn new_from_handle(handle: FPDF_STRUCTTREE) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            Ok(Self {
                handle: Handle::new(handle, Some(close_struct_tree)),
                owner: None,
            })
        }
    }

    pub(crate) fn set_owner(&mut self, owner: PdfiumPage) {
        self.owner = Some(owner);
    }

    /// Returns the number of children for this structure tree.
    pub fn count_children(&self) -> i32 {
        lib().FPDF_StructTree_CountChildren(self)
    }

    /// Returns the child element at the given index.
    pub fn get_child(&self, index: i32) -> PdfiumResult<PdfiumStructElement> {
        let mut child = lib().FPDF_StructTree_GetChildAtIndex(self, index)?;
        child.set_owner(self.clone());
        Ok(child)
    }
}

impl From<&PdfiumStructTree> for FPDF_STRUCTTREE {
    fn from(struct_tree: &PdfiumStructTree) -> Self {
        struct_tree.handle.handle()
    }
}

fn close_struct_tree(struct_tree: FPDF_STRUCTTREE) {
    lib().FPDF_StructTree_Close(struct_tree);
}

#[cfg(test)]
mod tests {
    use crate::PdfiumDocument;

    #[test]
    fn test_struct_tree() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        if let Ok(page) = document.page(0) {
            let tree = page.struct_tree();
            if let Some(tree) = tree {
                let children = tree.count_children();
                assert!(children >= 0);
                if children > 0 {
                    let child = tree.get_child(0);
                    assert!(child.is_ok());
                    let child = child.unwrap();

                    // Test struct element methods
                    let _ = child.obj_type();
                    let _ = child.title();
                    let _ = child.id();
                    let _ = child.lang();
                    let _ = child.alt_text();
                    let _ = child.actual_text();

                    let attr_count = child.attribute_count();
                    if attr_count > 0 {
                        let attr = child.attribute_at_index(0);
                        assert!(attr.is_ok());
                        let attr = attr.unwrap();
                        let attr_count = attr.count();
                        assert!(attr_count >= 0);
                        if attr_count > 0 {
                            if let Some(name) = attr.name(0) {
                                let _ = attr.value(&name);
                            }
                        }
                    }
                }
            }
        }
    }
}
