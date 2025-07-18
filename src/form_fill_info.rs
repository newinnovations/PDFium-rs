// PDFium-rs -- Modern Rust wrapper to PDFium, the PDF library from Google
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

#![allow(unreachable_code)]

use crate::pdfium_types::FPDF_FORMFILLINFO;

/// # Rust interface to FPDF_FORMFILLINFO
pub struct PdfiumFormFillInfo {}

impl Default for PdfiumFormFillInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfiumFormFillInfo {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<FPDF_FORMFILLINFO> for PdfiumFormFillInfo {
    fn from(_info: FPDF_FORMFILLINFO) -> Self {
        Self::new()
    }
}

impl From<&PdfiumFormFillInfo> for FPDF_FORMFILLINFO {
    fn from(_info: &PdfiumFormFillInfo) -> Self {
        FPDF_FORMFILLINFO {
            version: todo!(),
            Release: todo!(),
            FFI_Invalidate: todo!(),
            FFI_OutputSelectedRect: todo!(),
            FFI_SetCursor: todo!(),
            FFI_SetTimer: todo!(),
            FFI_KillTimer: todo!(),
            FFI_GetLocalTime: todo!(),
            FFI_OnChange: todo!(),
            FFI_GetPage: todo!(),
            FFI_GetCurrentPage: todo!(),
            FFI_GetRotation: todo!(),
            FFI_ExecuteNamedAction: todo!(),
            FFI_SetTextFieldFocus: todo!(),
            FFI_DoURIAction: todo!(),
            FFI_DoGoToAction: todo!(),
            m_pJsPlatform: todo!(),
            xfa_disabled: todo!(),
            FFI_DisplayCaret: todo!(),
            FFI_GetCurrentPageIndex: todo!(),
            FFI_SetCurrentPage: todo!(),
            FFI_GotoURL: todo!(),
            FFI_GetPageViewRect: todo!(),
            FFI_PageEvent: todo!(),
            FFI_PopupMenu: todo!(),
            FFI_OpenFile: todo!(),
            FFI_EmailTo: todo!(),
            FFI_UploadTo: todo!(),
            FFI_GetPlatform: todo!(),
            FFI_GetLanguage: todo!(),
            FFI_DownloadFromURL: todo!(),
            FFI_PostRequestURL: todo!(),
            FFI_PutRequestURL: todo!(),
            FFI_OnFocusChange: todo!(),
            FFI_DoURIActionWithKeyboardModifier: todo!(),
        }
    }
}

impl From<&mut PdfiumFormFillInfo> for FPDF_FORMFILLINFO {
    fn from(_info: &mut PdfiumFormFillInfo) -> Self {
        FPDF_FORMFILLINFO {
            version: todo!(),
            Release: todo!(),
            FFI_Invalidate: todo!(),
            FFI_OutputSelectedRect: todo!(),
            FFI_SetCursor: todo!(),
            FFI_SetTimer: todo!(),
            FFI_KillTimer: todo!(),
            FFI_GetLocalTime: todo!(),
            FFI_OnChange: todo!(),
            FFI_GetPage: todo!(),
            FFI_GetCurrentPage: todo!(),
            FFI_GetRotation: todo!(),
            FFI_ExecuteNamedAction: todo!(),
            FFI_SetTextFieldFocus: todo!(),
            FFI_DoURIAction: todo!(),
            FFI_DoGoToAction: todo!(),
            m_pJsPlatform: todo!(),
            xfa_disabled: todo!(),
            FFI_DisplayCaret: todo!(),
            FFI_GetCurrentPageIndex: todo!(),
            FFI_SetCurrentPage: todo!(),
            FFI_GotoURL: todo!(),
            FFI_GetPageViewRect: todo!(),
            FFI_PageEvent: todo!(),
            FFI_PopupMenu: todo!(),
            FFI_OpenFile: todo!(),
            FFI_EmailTo: todo!(),
            FFI_UploadTo: todo!(),
            FFI_GetPlatform: todo!(),
            FFI_GetLanguage: todo!(),
            FFI_DownloadFromURL: todo!(),
            FFI_PostRequestURL: todo!(),
            FFI_PutRequestURL: todo!(),
            FFI_OnFocusChange: todo!(),
            FFI_DoURIActionWithKeyboardModifier: todo!(),
        }
    }
}
