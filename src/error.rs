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

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub type PdfiumResult<T> = Result<T, PdfiumError>;

#[derive(Debug)]
pub enum PdfiumError {
    /// An unknown or unspecified error occurred
    Unknown,

    /// Failed to open, read, or access the PDF file
    FileError,

    /// The PDF file format is invalid or corrupted
    FormatError,

    /// The PDF requires a password that was not provided or is incorrect
    PasswordError,

    /// Security restrictions prevent the requested operation
    SecurityError,

    /// Error related to page operations (invalid page number, page not found, etc.)
    PageError,

    /// Error in color space or color profile processing
    ColorError,

    /// Input/output error with additional details
    IoError(String),

    /// A handle or pointer is null when it should be valid
    NullHandle,

    /// The requested image format is not supported for export/conversion
    UnsupportedImageFormat,

    /// The requested resource, object, or element was not found
    NotFound,

    /// Error loading or initializing the PDFium library
    LibraryError(String),

    /// An error occurred in the image create
    ImageError,
}

impl Error for PdfiumError {}

impl Display for PdfiumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}
