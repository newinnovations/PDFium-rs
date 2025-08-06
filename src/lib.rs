// PDFium-rs -- Modern Rust interface to PDFium, the PDF library from Google
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

mod action;
mod annotation;
mod attachment;
mod availability;
mod bitmap;
mod bookmark;
mod c_api;
mod clip_path;
mod color;
mod destination;
mod document;
mod error;
mod font;
mod form;
mod glyph_path;
mod javascript_action;
mod link;
mod matrix;
mod page;
mod path_segment;
mod rect;
mod signature;
mod struct_element;
mod struct_element_attr;
mod struct_element_attr_value;
mod struct_tree;
mod xobject;

pub use action::PdfiumAction;
pub use annotation::PdfiumAnnotation;
pub use attachment::PdfiumAttachment;
pub use availability::PdfiumAvailability;
pub use bitmap::PdfiumBitmap;
pub use bitmap::PdfiumBitmapFormat;
pub use bookmark::PdfiumBookmark;
pub use c_api::guard::lib;
pub use c_api::guard::set_library_location;
pub use c_api::guard::set_use_skia;
pub use c_api::guard::try_lib;
pub use c_api::pdfium_bindings::Pdfium;
pub use c_api::pdfium_constants;
pub use c_api::pdfium_types;
pub use clip_path::PdfiumClipPath;
pub use color::PdfiumColor;
pub use destination::PdfiumDestination;
pub use document::reader::PdfiumReader;
pub use document::PdfiumDocument;
pub use error::PdfiumError;
pub use error::PdfiumResult;
pub use font::PdfiumFont;
pub use form::PdfiumForm;
pub use glyph_path::PdfiumGlyphPath;
pub use javascript_action::PdfiumJavascriptAction;
pub use link::PdfiumLink;
pub use matrix::PdfiumMatrix;
pub use page::boundaries::PdfiumPageBoundaries;
pub use page::link::PdfiumPageLink;
pub use page::object::mark::PdfiumPageObjectMark;
pub use page::object::PdfiumPageObject;
pub use page::range::PdfiumPageRange;
pub use page::render::PdfiumRenderConfig;
pub use page::render::PdfiumRenderFlags;
pub use page::text::search::PdfiumSearch;
pub use page::text::search::PdfiumSearchFlags;
pub use page::text::PdfiumTextPage;
pub use page::PdfiumPage;
pub use path_segment::PdfiumPathSegment;
pub use rect::PdfiumRect;
pub use signature::PdfiumSignature;
pub use struct_element::PdfiumStructElement;
pub use struct_element_attr::PdfiumStructElementAttr;
pub use struct_element_attr_value::PdfiumStructElementAttrValue;
pub use struct_tree::PdfiumStructTree;
pub use xobject::PdfiumXObject;
