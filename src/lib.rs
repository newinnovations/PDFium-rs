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

#![doc = include_str!("../README.md")]

mod annotation;
mod bitmap;
mod clippath;
mod color;
mod document;
mod error;
mod form;
mod form_fill_info;
mod guard;
mod lib_config;
mod matrix;
mod page;
mod page_object;
mod pdfium_sys;
mod rect;
mod x_object;

pub use pdfium_sys::Pdfium;
pub use pdfium_sys::pdfium::PdfiumBindings;
pub use pdfium_sys::pdfium_constants;
pub use pdfium_sys::pdfium_types;

pub use annotation::PdfiumAnnotation;
pub use bitmap::PdfiumBitmap;
pub use bitmap::PdfiumBitmapFormat;
pub use clippath::PdfiumClipPath;
pub use color::PdfiumColor;
pub use document::PdfiumDocument;
pub use document::reader::PdfiumReader;
pub use error::PdfiumError;
pub use error::PdfiumResult;
pub use form::PdfiumForm;
pub use form_fill_info::PdfiumFormFillInfo;
pub use guard::lib;
pub use guard::set_library_location;
pub use guard::set_use_skia;
pub use guard::try_lib;
pub use lib_config::PdfiumLibraryConfig;
pub use matrix::PdfiumMatrix;
pub use page::PdfiumPage;
pub use page::PdfiumRenderFlags;
pub use page::boundaries::PdfiumPageBoundaries;
pub use page_object::PdfiumPageObject;
pub use rect::PdfiumRect;
pub use x_object::PdfiumXObject;
