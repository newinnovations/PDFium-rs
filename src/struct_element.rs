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
    PdfiumStructElementAttr,
    error::{PdfiumError, PdfiumResult},
    lib,
    pdfium_types::{FPDF_STRUCTELEMENT, Handle, StructElementHandle},
};

/// # Rust interface to FPDF_STRUCTELEMENT
#[derive(Debug, Clone)]
pub struct PdfiumStructElement {
    handle: StructElementHandle,
}

impl PdfiumStructElement {
    pub(crate) fn new_from_handle(handle: FPDF_STRUCTELEMENT) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            Ok(Self {
                handle: Handle::new(handle, None), // TODO: check close is not needed
            })
        }
    }

    /// Returns the number of children for this structure element.
    pub fn count_children(&self) -> i32 {
        lib().FPDF_StructElement_CountChildren(self)
    }

    /// Returns the child element at the given index.
    pub fn get_child(&self, index: i32) -> PdfiumResult<PdfiumStructElement> {
        lib().FPDF_StructElement_GetChildAtIndex(self, index)
    }

    /// Returns the type (/S) for this element as a String (e.g. "H1", "P", "Sect").
    pub fn element_type(&self) -> Option<String> {
        let len = lib().FPDF_StructElement_GetType(self, None, 0);
        if len > 0 {
            let mut buffer = vec![0u8; len as usize];
            lib().FPDF_StructElement_GetType(self, Some(&mut buffer), len);

            // FPDF_StructElement_GetType returns UTF-16LE, NUL-terminated
            let u16_buffer: Vec<u16> = buffer
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .take_while(|&c| c != 0)
                .collect();

            Some(String::from_utf16_lossy(&u16_buffer))
        } else {
            None
        }
    }

    /// Returns the actual text for this element.
    pub fn actual_text(&self) -> Option<String> {
        let len = lib().FPDF_StructElement_GetActualText(self, None, 0);
        if len > 0 {
            let mut buffer = vec![0u8; len as usize];
            lib().FPDF_StructElement_GetActualText(self, Some(&mut buffer), len);

            let u16_buffer: Vec<u16> = buffer
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .take_while(|&c| c != 0)
                .collect();

            Some(String::from_utf16_lossy(&u16_buffer))
        } else {
            None
        }
    }

    /// Returns the alternate text for this element.
    pub fn alt_text(&self) -> Option<String> {
        let len = lib().FPDF_StructElement_GetAltText(self, None, 0);
        if len > 0 {
            let mut buffer = vec![0u8; len as usize];
            lib().FPDF_StructElement_GetAltText(self, Some(&mut buffer), len);

            let u16_buffer: Vec<u16> = buffer
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .take_while(|&c| c != 0)
                .collect();

            Some(String::from_utf16_lossy(&u16_buffer))
        } else {
            None
        }
    }

    /// Returns the title (/T) for this element.
    pub fn title(&self) -> Option<String> {
        let len = lib().FPDF_StructElement_GetTitle(self, None, 0);
        if len > 0 {
            let mut buffer = vec![0u8; len as usize];
            lib().FPDF_StructElement_GetTitle(self, Some(&mut buffer), len);

            let u16_buffer: Vec<u16> = buffer
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .take_while(|&c| c != 0)
                .collect();

            Some(String::from_utf16_lossy(&u16_buffer))
        } else {
            None
        }
    }

    /// Returns the ID for this element.
    pub fn id(&self) -> Option<String> {
        let len = lib().FPDF_StructElement_GetID(self, None, 0);
        if len > 0 {
            let mut buffer = vec![0u8; len as usize];
            lib().FPDF_StructElement_GetID(self, Some(&mut buffer), len);

            let u16_buffer: Vec<u16> = buffer
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .take_while(|&c| c != 0)
                .collect();

            Some(String::from_utf16_lossy(&u16_buffer))
        } else {
            None
        }
    }

    /// Returns the language code for this element.
    pub fn lang(&self) -> Option<String> {
        let len = lib().FPDF_StructElement_GetLang(self, None, 0);
        if len > 0 {
            let mut buffer = vec![0u8; len as usize];
            lib().FPDF_StructElement_GetLang(self, Some(&mut buffer), len);

            let u16_buffer: Vec<u16> = buffer
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .take_while(|&c| c != 0)
                .collect();

            Some(String::from_utf16_lossy(&u16_buffer))
        } else {
            None
        }
    }

    /// Returns the object type for this element.
    pub fn obj_type(&self) -> Option<String> {
        let len = lib().FPDF_StructElement_GetObjType(self, None, 0);
        if len > 0 {
            let mut buffer = vec![0u8; len as usize];
            lib().FPDF_StructElement_GetObjType(self, Some(&mut buffer), len);

            let u16_buffer: Vec<u16> = buffer
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .take_while(|&c| c != 0)
                .collect();

            Some(String::from_utf16_lossy(&u16_buffer))
        } else {
            None
        }
    }

    /// Returns the string attribute for this element.
    pub fn string_attribute(&self, attr_name: &str) -> Option<String> {
        let c_attr_name = std::ffi::CString::new(attr_name).ok()?;
        let len = lib().FPDF_StructElement_GetStringAttribute(self, &c_attr_name, None, 0);
        if len > 0 {
            let mut buffer = vec![0u8; len as usize];
            lib().FPDF_StructElement_GetStringAttribute(self, &c_attr_name, Some(&mut buffer), len);

            let u16_buffer: Vec<u16> = buffer
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .take_while(|&c| c != 0)
                .collect();

            Some(String::from_utf16_lossy(&u16_buffer))
        } else {
            None
        }
    }

    /// Returns the marked content ID for this element.
    pub fn marked_content_id(&self) -> i32 {
        lib().FPDF_StructElement_GetMarkedContentID(self)
    }

    /// Returns the number of marked content IDs for this element.
    pub fn marked_content_id_count(&self) -> i32 {
        lib().FPDF_StructElement_GetMarkedContentIdCount(self)
    }

    /// Returns the marked content ID at the given index.
    pub fn marked_content_id_at_index(&self, index: i32) -> i32 {
        lib().FPDF_StructElement_GetMarkedContentIdAtIndex(self, index)
    }

    /// Returns the child marked content ID at the given index.
    pub fn child_marked_content_id(&self, index: i32) -> i32 {
        lib().FPDF_StructElement_GetChildMarkedContentID(self, index)
    }

    /// Returns the parent of this element.
    pub fn parent(&self) -> PdfiumResult<PdfiumStructElement> {
        lib().FPDF_StructElement_GetParent(self)
    }

    /// Returns the number of attributes for this element.
    pub fn attribute_count(&self) -> i32 {
        lib().FPDF_StructElement_GetAttributeCount(self)
    }

    /// Returns the attribute at the given index.
    pub fn attribute_at_index(&self, index: i32) -> PdfiumResult<PdfiumStructElementAttr> {
        lib().FPDF_StructElement_GetAttributeAtIndex(self, index)
    }
}

impl From<&PdfiumStructElement> for FPDF_STRUCTELEMENT {
    fn from(struct_element: &PdfiumStructElement) -> Self {
        struct_element.handle.handle()
    }
}
