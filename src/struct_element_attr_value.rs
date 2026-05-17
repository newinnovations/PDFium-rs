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
    pdfium_types::{FPDF_STRUCTELEMENT_ATTR_VALUE, Handle, StructElementAttrValueHandle},
};

/// # Rust interface to FPDF_STRUCTELEMENT_ATTR_VALUE
#[derive(Debug, Clone)]
pub struct PdfiumStructElementAttrValue {
    handle: StructElementAttrValueHandle,
    owner: Option<PdfiumStructElementAttr>,
}

impl PdfiumStructElementAttrValue {
    pub(crate) fn new_from_handle(handle: FPDF_STRUCTELEMENT_ATTR_VALUE) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            Ok(Self {
                handle: Handle::new_const(handle),
                owner: None,
            })
        }
    }

    pub(crate) fn set_owner(&mut self, owner: PdfiumStructElementAttr) {
        self.owner = Some(owner);
    }

    /// Returns the type of this attribute value.
    pub fn value_type(&self) -> i32 {
        lib().FPDF_StructElement_Attr_GetType(self)
    }

    /// Returns the boolean value.
    pub fn boolean_value(&self) -> Option<bool> {
        let mut value = 0;
        if lib()
            .FPDF_StructElement_Attr_GetBooleanValue(self, &mut value)
            .is_ok()
        {
            Some(value != 0)
        } else {
            None
        }
    }

    /// Returns the number value.
    pub fn number_value(&self) -> Option<f32> {
        let mut value = 0.0;
        if lib()
            .FPDF_StructElement_Attr_GetNumberValue(self, &mut value)
            .is_ok()
        {
            Some(value)
        } else {
            None
        }
    }

    /// Returns the string value.
    pub fn string_value(&self) -> Option<String> {
        let mut len: std::os::raw::c_ulong = 0;
        if lib()
            .FPDF_StructElement_Attr_GetStringValue(self, None, 0, &mut len)
            .is_ok()
            && len > 0
        {
            let mut buffer = vec![0u8; len as usize];
            if lib()
                .FPDF_StructElement_Attr_GetStringValue(self, Some(&mut buffer), len, &mut len)
                .is_ok()
            {
                let u16_buffer: Vec<u16> = buffer
                    .chunks_exact(2)
                    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                    .take_while(|&c| c != 0)
                    .collect();
                return Some(String::from_utf16_lossy(&u16_buffer));
            }
        }
        None
    }

    /// Returns the blob value.
    pub fn blob_value(&self) -> Option<Vec<u8>> {
        let mut len: std::os::raw::c_ulong = 0;
        if lib()
            .FPDF_StructElement_Attr_GetBlobValue(self, None, 0, &mut len)
            .is_ok()
            && len > 0
        {
            let mut buffer = vec![0u8; len as usize];
            if lib()
                .FPDF_StructElement_Attr_GetBlobValue(self, Some(&mut buffer), len, &mut len)
                .is_ok()
            {
                return Some(buffer);
            }
        }
        None
    }

    /// Returns the number of children for this attribute value.
    pub fn count_children(&self) -> i32 {
        lib().FPDF_StructElement_Attr_CountChildren(self)
    }

    /// Returns the child at the given index.
    pub fn get_child_at_index(&self, index: i32) -> PdfiumResult<PdfiumStructElementAttrValue> {
        let mut child = lib().FPDF_StructElement_Attr_GetChildAtIndex(self, index)?;
        if let Some(owner) = &self.owner {
            child.set_owner(owner.clone());
        }
        Ok(child)
    }
}

impl From<&PdfiumStructElementAttrValue> for FPDF_STRUCTELEMENT_ATTR_VALUE {
    fn from(struct_element_attr_value: &PdfiumStructElementAttrValue) -> Self {
        struct_element_attr_value.handle.handle()
    }
}
