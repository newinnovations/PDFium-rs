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
    PdfiumDestination, PdfiumDocument,
    error::{PdfiumError, PdfiumResult},
    lib,
    pdfium_types::{ActionHandle, FPDF_ACTION, Handle},
};

/// Action type returned by FPDFAction_GetType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PdfiumActionType {
    /// Unsupported action type.
    #[default]
    Unsupported,
    /// Go to a destination within current document.
    Goto,
    /// Go to a destination within another document.
    RemoteGoto,
    /// URI, including web pages and other Internet resources.
    #[allow(clippy::upper_case_acronyms)] // PDFium API uses uppercase URI
    URI,
    /// Launch an application or open a file.
    Launch,
    /// Go to a destination in an embedded file.
    EmbeddedGoto,
}

/// # Rust interface to FPDF_ACTION
#[derive(Debug, Clone)]
pub struct PdfiumAction {
    handle: ActionHandle,
}

impl PdfiumAction {
    pub(crate) fn new_from_handle(handle: FPDF_ACTION) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            Ok(Self {
                handle: Handle::new(handle, None), // TODO: check close is not needed
            })
        }
    }

    /// Returns the type of this action.
    pub fn r#type(&self) -> PdfiumActionType {
        match lib().FPDFAction_GetType(self) {
            0 => PdfiumActionType::Unsupported,
            1 => PdfiumActionType::Goto,
            2 => PdfiumActionType::RemoteGoto,
            3 => PdfiumActionType::URI,
            4 => PdfiumActionType::Launch,
            5 => PdfiumActionType::EmbeddedGoto,
            _ => PdfiumActionType::Unsupported,
        }
    }

    /// Returns the destination associated with this action, if any.
    ///
    /// Returns an error if the action is not a Goto or RemoteGoto action.
    ///
    /// Note: In the case of [PdfiumActionType::RemoteGoto], you must first call [PdfiumAction::file_path()],
    /// then load the document at that path, then pass the document handle from that document as
    /// |document| to [PdfiumAction::dest()].
    pub fn dest(&self, document: &PdfiumDocument) -> PdfiumResult<PdfiumDestination> {
        lib().FPDFAction_GetDest(document, self)
    }

    /// Returns the file path associated with this action, if any.
    ///
    /// Returns an error if the action is not a RemoteGoto or Launch action.
    pub fn file_path(&self) -> PdfiumResult<String> {
        let lib = lib();
        let buf_len = lib.FPDFAction_GetFilePath(self, None, 0);
        if buf_len == 0 {
            // Same as the length checking below
            return Err(PdfiumError::InvokationFailed);
        }
        let mut buffer = vec![0u8; buf_len as usize];
        // Note from the library: Regardless of the platform, the buffer is always in UTF-8 encoding.
        let actual_len = lib.FPDFAction_GetFilePath(self, Some(&mut buffer), buf_len);
        if actual_len == 0 {
            // PDFium returns 0 on error
            return Err(PdfiumError::InvokationFailed);
        }
        let content_len = (actual_len as usize).saturating_sub(1);
        buffer.truncate(content_len);
        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }

    /// Returns the URI Path of the action, in raw bytes.
    ///
    /// Note: the returned raw bytes represents a String, but we do not know what encoding the PDF
    /// file adopts. If you believe the String is encoded in UTF-8, you can simply call
    /// String::from_utf8() to convert the raw bytes into a String.
    pub fn uri_path(&self, doc: &PdfiumDocument) -> PdfiumResult<Vec<u8>> {
        let lib = lib();
        let buf_len = lib.FPDFAction_GetURIPath(doc, self, None, 0);
        if buf_len == 0 {
            return Err(PdfiumError::InvokationFailed);
        }
        let mut buffer = vec![0u8; buf_len as usize];
        let actual_len = lib.FPDFAction_GetURIPath(doc, self, Some(&mut buffer), buf_len);
        if actual_len == 0 {
            return Err(PdfiumError::InvokationFailed);
        }
        Ok(buffer)
    }
}

impl From<&PdfiumAction> for FPDF_ACTION {
    fn from(action: &PdfiumAction) -> Self {
        action.handle.handle()
    }
}
