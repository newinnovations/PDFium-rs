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

#![doc = include_str!("../README.md")]

mod bitmap;
mod color;
mod document;
mod error;
mod guard;
mod matrix;
mod page;
mod pdfium_sys;
mod rect;

pub use pdfium_sys::Pdfium;
pub use pdfium_sys::pdfium_constants;
pub use pdfium_sys::pdfium_types;

pub use bitmap::PdfiumBitmap;
pub use bitmap::PdfiumBitmapFormat;
pub use color::PdfiumColor;
pub use document::PdfiumDocument;
pub use document::reader::PdfiumReader;
pub use error::PdfiumError;
pub use error::PdfiumResult;
pub use guard::lib;
pub use guard::set_library_location;
pub use matrix::PdfiumMatrix;
pub use page::PdfiumPage;
pub use page::PdfiumRenderFlags;
pub use page::boundaries::PdfiumPageBoundaries;
pub use rect::PdfiumRect;
