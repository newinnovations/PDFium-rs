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
    PdfiumStructElement, PdfiumStructElementAttrValue,
    error::{PdfiumError, PdfiumResult},
    lib,
    pdfium_types::{FPDF_STRUCTELEMENT_ATTR, Handle, StructElementAttrHandle},
};

/// # Rust interface to FPDF_STRUCTELEMENT_ATTR
#[derive(Debug, Clone)]
pub struct PdfiumStructElementAttr {
    handle: StructElementAttrHandle,
    owner: Option<PdfiumStructElement>,
}

impl PdfiumStructElementAttr {
    pub(crate) fn new_from_handle(handle: FPDF_STRUCTELEMENT_ATTR) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            Ok(Self {
                handle: Handle::new_const(handle),
                owner: None,
            })
        }
    }

    pub(crate) fn set_owner(&mut self, owner: PdfiumStructElement) {
        self.owner = Some(owner);
    }

    /// Returns the number of attributes in this map.
    pub fn count(&self) -> i32 {
        lib().FPDF_StructElement_Attr_GetCount(self)
    }

    /// Returns the name of the attribute at the given index.
    pub fn name(&self, index: i32) -> Option<String> {
        let mut len: std::os::raw::c_ulong = 0;
        if lib()
            .FPDF_StructElement_Attr_GetName(self, index, None, 0, &mut len)
            .is_ok()
            && len > 0
        {
            let mut buffer = vec![0u8; len as usize];
            if lib()
                .FPDF_StructElement_Attr_GetName(self, index, Some(&mut buffer), len, &mut len)
                .is_ok()
            {
                let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
                return Some(String::from_utf8_lossy(&buffer[..end]).into_owned());
            }
        }
        None
    }

    /// Returns the value for the attribute with the given name.
    pub fn value(&self, name: &str) -> Option<PdfiumStructElementAttrValue> {
        let c_name = std::ffi::CString::new(name).ok()?;
        let mut val = lib().FPDF_StructElement_Attr_GetValue(self, &c_name).ok()?;
        val.set_owner(self.clone());
        Some(val)
    }
}

impl From<&PdfiumStructElementAttr> for FPDF_STRUCTELEMENT_ATTR {
    fn from(struct_element_attr: &PdfiumStructElementAttr) -> Self {
        struct_element_attr.handle.handle()
    }
}
