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

mod action;
mod annotation;
mod attachment;
mod availability;
mod bitmap;
mod bookmark;
mod clip_path;
mod color;
mod destination;
mod document;
mod error;
mod font;
mod form;
mod glyph_path;
mod guard;
mod javascript_action;
mod link;
mod matrix;
mod page;
mod page_link;
mod page_object;
mod page_object_mark;
mod page_range;
mod path_segment;
mod pdfium_sys;
mod rect;
mod search;
mod signature;
mod struct_element;
mod struct_element_attr;
mod struct_element_attr_value;
mod struct_tree;
mod text_page;
mod xobject;

pub use action::PdfiumAction;
pub use annotation::PdfiumAnnotation;
pub use attachment::PdfiumAttachment;
pub use availability::PdfiumAvailability;
pub use bitmap::PdfiumBitmap;
pub use bitmap::PdfiumBitmapFormat;
pub use bookmark::PdfiumBookmark;
pub use clip_path::PdfiumClipPath;
pub use color::PdfiumColor;
pub use destination::PdfiumDestination;
pub use document::PdfiumDocument;
pub use document::reader::PdfiumReader;
pub use error::PdfiumError;
pub use error::PdfiumResult;
pub use font::PdfiumFont;
pub use form::PdfiumForm;
pub use glyph_path::PdfiumGlyphPath;
pub use guard::lib;
pub use guard::set_library_location;
pub use guard::set_use_skia;
pub use guard::try_lib;
pub use javascript_action::PdfiumJavascriptAction;
pub use link::PdfiumLink;
pub use matrix::PdfiumMatrix;
pub use page::PdfiumPage;
pub use page::PdfiumRenderFlags;
pub use page::boundaries::PdfiumPageBoundaries;
pub use page_link::PdfiumPageLink;
pub use page_object::PdfiumPageObject;
pub use page_object_mark::PdfiumPageObjectMark;
pub use page_range::PdfiumPageRange;
pub use path_segment::PdfiumPathSegment;
pub use pdfium_sys::pdfium_bindings::Pdfium;
pub use pdfium_sys::pdfium_constants;
pub use pdfium_sys::pdfium_types;
pub use rect::PdfiumRect;
pub use search::PdfiumSearch;
pub use signature::PdfiumSignature;
pub use struct_element::PdfiumStructElement;
pub use struct_element_attr::PdfiumStructElementAttr;
pub use struct_element_attr_value::PdfiumStructElementAttrValue;
pub use struct_tree::PdfiumStructTree;
pub use text_page::PdfiumTextPage;
pub use xobject::PdfiumXObject;
