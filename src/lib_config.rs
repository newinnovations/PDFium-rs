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

use crate::{
    pdfium_constants::{
        FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_AGG, FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_SKIA,
    },
    pdfium_types::{FPDF_LIBRARY_CONFIG, FPDF_RENDERER_TYPE},
};

/// Rust interface to FPDF_LIBRARY_CONFIG
#[derive(Debug, Copy, Clone)]
pub struct PdfiumLibraryConfig {
    use_skia: bool,
}

impl Default for PdfiumLibraryConfig {
    fn default() -> Self {
        Self::new(false)
    }
}

impl PdfiumLibraryConfig {
    pub fn new(use_skia: bool) -> Self {
        Self { use_skia }
    }

    pub fn renderer_type(&self) -> FPDF_RENDERER_TYPE {
        match self.use_skia {
            true => FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_SKIA,
            false => FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_AGG,
        }
    }
}

impl From<&PdfiumLibraryConfig> for FPDF_LIBRARY_CONFIG {
    fn from(config: &PdfiumLibraryConfig) -> Self {
        FPDF_LIBRARY_CONFIG {
            version: 2,
            m_pUserFontPaths: std::ptr::null_mut(), // default paths
            m_pIsolate: std::ptr::null_mut(),       // let PDFium create one
            m_v8EmbedderSlot: 0,                    // 0 is fine for most embedders
            m_pPlatform: std::ptr::null_mut(),
            m_RendererType: config.renderer_type(),
        }
    }
}
