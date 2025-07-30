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

pub mod reader;
pub mod writer;

use std::{
    cell::OnceCell,
    ffi::CString,
    fmt::Debug,
    fs::File,
    io::{Cursor, Read, Seek, Write},
    path::Path,
    ptr::null,
    rc::Rc,
};

use crate::{
    document::{reader::PdfiumReader, writer::PdfiumWriter},
    error::{PdfiumError, PdfiumResult},
    lib,
    page::PdfiumPage,
    pdfium_types::{DocumentHandle, Handle, FPDF_DOCUMENT},
    try_lib,
};

/// Rust interface to FPDF_DOCUMENT
#[derive(Clone)]
pub struct PdfiumDocument {
    handle: DocumentHandle,
    #[allow(clippy::redundant_allocation)]
    _reader: Option<Rc<Box<PdfiumReader>>>,
}

impl Debug for PdfiumDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PdfiumDocument")
            .field("handle", &self.handle)
            .finish()
    }
}

impl PdfiumDocument {
    fn new_from_handle(
        handle: FPDF_DOCUMENT,
        reader: Option<Box<PdfiumReader>>,
    ) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(match try_lib()?.FPDF_GetLastError() as i32 {
                crate::pdfium_constants::FPDF_ERR_FILE => PdfiumError::FileError,
                crate::pdfium_constants::FPDF_ERR_FORMAT => PdfiumError::FormatError,
                crate::pdfium_constants::FPDF_ERR_PASSWORD => PdfiumError::PasswordError,
                crate::pdfium_constants::FPDF_ERR_SECURITY => PdfiumError::SecurityError,
                crate::pdfium_constants::FPDF_ERR_PAGE => PdfiumError::PageError,
                _ => PdfiumError::Unknown,
            })
        } else {
            Ok(Self {
                handle: Handle::new(handle, Some(close_document)),
                _reader: reader.map(Rc::new),
            })
        }
    }

    pub fn new() -> PdfiumResult<Self> {
        let handle = try_lib()?.FPDF_CreateNewDocument();
        Self::new_from_handle(handle, None)
    }

    pub fn new_from_path<P: AsRef<Path>>(path: P, password: Option<&str>) -> PdfiumResult<Self> {
        let reader = File::open(path)?;
        Self::new_from_reader(reader, password)
    }

    pub fn new_from_reader<R: Read + Seek + 'static>(
        reader: R,
        password: Option<&str>,
    ) -> PdfiumResult<Self> {
        let lib = try_lib()?;
        let mut reader = PdfiumReader::new(reader);
        let password = CString::new(password.unwrap_or("")).unwrap();
        let handle = lib.FPDF_LoadCustomDocument(&mut reader, &password);
        Self::new_from_handle(handle, Some(reader))
    }

    /// Saves this [`PdfiumDocument`] to a file at the specified path.
    ///
    /// This is a convenience method that creates a new file at the given path and writes
    /// the PDF document to it. The file will be created if it doesn't exist, or truncated
    /// if it does exist.
    ///
    /// # Arguments
    ///
    /// * `path` - A path-like type (String, &str, Path, PathBuf, etc.) that specifies
    ///   where to save the PDF file. Uses AsRef<Path> for maximum flexibility.
    /// * `version` - Optional PDF version to save as. If None, saves as a copy of the
    ///   original document preserving its version. If Some(version), converts
    ///   the document to the specified PDF version (e.g., 14 for PDF 1.4).
    ///
    /// # Returns
    ///
    /// * `PdfiumResult<()>` - Ok(()) on success, or an error if file creation fails
    ///   or the PDF save operation encounters an issue.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Save to current directory preserving original PDF version
    /// document.save_to_path("document.pdf", None)?;
    ///
    /// // Save as PDF 1.4 to a specific path
    /// document.save_to_path("document_v14.pdf", Some(14))?;
    /// ```
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// * The file cannot be created (permissions, invalid path, disk full, etc.)
    /// * The underlying PDF save operation fails (corrupt document, unsupported features, etc.)
    pub fn save_to_path<P: AsRef<Path>>(&self, path: P, version: Option<i32>) -> PdfiumResult<()> {
        self.save_to_writer(File::create(path)?, version)?;
        Ok(())
    }

    /// Saves this [`PdfiumDocument`] to a byte vector in memory.
    ///
    /// This method is useful when you need the PDF data as bytes rather than writing
    /// directly to a file. Common use cases include:
    /// * Serving PDF content over HTTP without creating temporary files
    /// * Storing PDF data in a database as a BLOB
    /// * Further processing the PDF bytes (compression, encryption, etc.)
    /// * Testing scenarios where you want to verify PDF content
    ///
    /// # Arguments
    ///
    /// * `version` - Optional PDF version to save as. If None, preserves the original
    ///   document's PDF version. If Some(version), converts to the specified
    ///   version (e.g., 17 for PDF 1.7).
    ///
    /// # Returns
    ///
    /// * `PdfiumResult<Vec<u8>>` - On success, returns a Vec<u8> containing the complete
    ///   PDF file data. On failure, returns a PdfiumResult error.
    ///
    /// # Memory Considerations
    ///
    /// The entire PDF is loaded into memory, so this method may use significant RAM
    /// for large documents. Consider `save_to_writer()` with a streaming writer for
    /// very large PDFs.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Get PDF bytes preserving original version
    /// let pdf_bytes = document.save_to_bytes(None)?;
    ///
    /// // Convert to PDF 1.5 and get bytes
    /// let pdf_v15_bytes = document.save_to_bytes(Some(15))?;
    ///
    /// // Use the bytes (e.g., send over HTTP)
    /// response.set_body(pdf_bytes);
    /// ```
    pub fn save_to_bytes(&self, version: Option<i32>) -> PdfiumResult<Vec<u8>> {
        let cursor = Cursor::new(Vec::new());
        let cursor = self.save_to_writer(cursor, version)?;
        Ok(cursor.into_inner())
    }

    /// Writes this [`PdfiumDocument`] to the given writer.
    ///
    /// This is the core implementation method that all other save methods delegate to.
    /// It accepts any type that implements the Write trait, providing maximum flexibility
    /// for different output destinations (files, network streams, in-memory buffers, etc.).
    ///
    /// The method wraps the provided writer in a PdfiumWriter, which handles
    /// the low-level details of interfacing with the Pdfium C library, such as:
    /// - Implements the callback interface expected by Pdfium's C API
    /// - Handles buffering and error propagation
    /// - Manages the lifetime and ownership of the underlying writer
    ///
    /// # Arguments
    ///
    /// * `writer` - Any type implementing Write + 'static. The 'static lifetime bound
    ///   ensures the writer can be stored and moved around safely without
    ///   lifetime issues. Common types include File, TcpStream, Cursor<Vec<u8>>, etc.
    /// * `version` - Optional PDF version specification:
    ///   - None: Save as copy preserving original document version and structure
    ///   - Some(version): Convert document to specified PDF version (10-20 typical range)
    ///
    /// # Returns
    ///
    /// * `PdfiumResult<Box<W>>` - On success, returns the original writer wrapped in a Box.
    ///   This allows you to continue using the writer after the save
    ///   operation completes (e.g., to write additional data).
    ///
    /// # PDF Version Notes
    ///
    /// PDF versions are typically specified as integers:
    /// * 10 = PDF 1.0, 11 = PDF 1.1, ..., 17 = PDF 1.7, 20 = PDF 2.0
    /// * Converting to an older version may lose features not supported in that version
    /// * Converting to a newer version may enable additional features but reduce compatibility
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Save to a file
    /// let file = File::create("document.pdf")?;
    /// let file = document.save_to_writer(file, None)?;
    ///
    /// // Save to a network stream
    /// let stream = TcpStream::connect("server:8080")?;
    /// let stream = document.save_to_writer(stream, Some(17))?;
    ///
    /// // Save to memory buffer
    /// let buffer = Cursor::new(Vec::new());
    /// let buffer = document.save_to_writer(buffer, None)?;
    /// ```
    ///
    /// # Implementation Details
    ///
    /// The method uses the Pdfium library's C API functions:
    /// * `FPDF_SaveWithVersion()` - When a specific version is requested
    /// * `FPDF_SaveAsCopy()` - When preserving the original version
    ///
    /// Both functions use a callback-based approach where Pdfium calls back into our
    /// PdfiumWriter to actually write the data chunks as they're generated.
    pub fn save_to_writer<W: Write + 'static>(
        &self,
        writer: W,
        version: Option<i32>,
    ) -> PdfiumResult<Box<W>> {
        // Set flags to 0 - this typically means "use default behavior" in Pdfium.
        // Other flag values might control incremental updates, linearization,
        // object stream compression, etc., but aren't used in this implementation.
        let flags = 0;

        let mut pdfium_writer = PdfiumWriter::new(writer);

        // Choose the appropriate Pdfium API function based on whether a version was specified
        match version {
            Some(version) => {
                // Save with a specific PDF version.
                lib().FPDF_SaveWithVersion(
                    self.into(),
                    pdfium_writer.as_mut().into(),
                    flags,
                    version,
                )
            }
            None => lib().FPDF_SaveAsCopy(self.into(), pdfium_writer.as_mut().into(), flags),
        }?;

        // Ensure all buffered data is written to the underlying writer.
        // This is crucial because the PdfiumWriter may buffer data for performance,
        // and we need to guarantee everything is written before returning.
        pdfium_writer.flush()?;

        // Extract and return the original writer. The take_writer() method
        // consumes the PdfiumWriter and returns ownership of the wrapped writer,
        // allowing the caller to continue using it if needed.
        Ok(pdfium_writer.take_writer())
    }

    /// Returns the number of pages in this [`PdfiumDocument`].
    pub fn page_count(&self) -> i32 {
        lib().FPDF_GetPageCount(self)
    }

    /// Returns the [`PdfiumPage`] indicated by `index` from this [`PdfiumDocument`].
    pub fn page(&self, index: i32) -> PdfiumResult<PdfiumPage> {
        let mut page = lib().FPDF_LoadPage(self, index)?;
        page.set_owner(self.clone());
        Ok(page)
    }

    pub fn pages(&self) -> PdfiumPages {
        PdfiumPages::new(self)
    }

    /// Import pages to this [`PdfiumDocument`].
    ///
    /// # Arguments
    /// * `src_doc` - The document to be imported.
    /// * `pagerange` - A page range string, Such as "1,3,5-7". The first page is one.
    ///   If `pagerange` is empty, all pages from `src_doc` are imported.
    /// * index - The page index at which to insert the first imported page into
    ///   dest_doc. The first page is zero.
    ///
    /// Returns `Err` if any pages in `pagerange` is invalid or cannot be read.
    /// ```
    #[inline]
    pub fn import_pages(
        &self,
        src_doc: &PdfiumDocument,
        pagerange: &str,
        index: i32,
    ) -> PdfiumResult<()> {
        let pagerange = CString::new(pagerange)?;
        lib().FPDF_ImportPages(self, src_doc, &pagerange, index)
    }

    /// Import pages to this [`PdfiumDocument`] by index.
    ///
    /// # Arguments
    /// * `src_doc` - The document to be imported.
    /// * `src_indices` - An array of page indices to be imported. The first page is
    ///   zero. If `src_indices` is None, all pages from `src_doc` are imported.
    /// * `index` - The page index at which to insert the first imported page
    ///   into `dest_doc`. The first page is zero.
    ///
    /// Returns `Err` if any pages in `src_indices` is invalid or cannot be read.
    /// ```
    #[inline]
    pub fn import_pages_by_index(
        &self,
        src_doc: &PdfiumDocument,
        src_indices: Option<&[i32]>,
        index: i32,
    ) -> PdfiumResult<()> {
        match src_indices {
            Some(indices) => lib().FPDF_ImportPagesByIndex(
                self.into(),
                src_doc.into(),
                indices.as_ptr(),
                indices.len() as u64,
                index,
            ),
            None => lib().FPDF_ImportPagesByIndex(self.into(), src_doc.into(), null(), 0, index),
        }
    }
}

/// Iterator for [`PdfiumPage`]
pub struct PdfiumPages<'a> {
    doc: &'a PdfiumDocument,
    page_count: OnceCell<i32>,
    current_page: i32,
}

impl<'a> PdfiumPages<'a> {
    fn new(doc: &'a PdfiumDocument) -> PdfiumPages<'a> {
        Self {
            doc,
            page_count: OnceCell::new(),
            current_page: 0,
        }
    }

    /// Returns the number of pages in the [`PdfiumDocument`].
    ///
    /// Use `len` instead of `count` as the latter will consume
    /// the iterator and load all the pages in the process.
    pub fn len(&self) -> i32 {
        *self.page_count.get_or_init(|| self.doc.page_count())
    }

    /// Returns the [`PdfiumPage`] indicated by `index` from the [`PdfiumDocument`].
    pub fn get(&self, index: i32) -> PdfiumResult<PdfiumPage> {
        self.doc.page(index)
    }
}

impl<'a> Iterator for PdfiumPages<'a> {
    type Item = PdfiumPage;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page >= self.len() {
            None
        } else {
            match self.doc.page(self.current_page) {
                Ok(page) => {
                    self.current_page += 1;
                    Some(page)
                }
                Err(_) => None,
            }
        }
    }
}

impl From<&PdfiumDocument> for FPDF_DOCUMENT {
    #[inline]
    fn from(value: &PdfiumDocument) -> Self {
        value.handle.handle()
    }
}

/// Closes this [PdfiumDocument], releasing held memory.
fn close_document(document: FPDF_DOCUMENT) {
    lib().FPDF_CloseDocument(document);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_non_existing() {
        let document = PdfiumDocument::new_from_path("resources/non_existing.pdf", None);
        assert!(document.is_err());
    }

    #[test]
    fn test_load_existing() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None);
        assert!(document.is_ok());
    }

    #[test]
    fn test_page_count() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page_count = document.page_count();
        assert_eq!(page_count, 2);

        let pages = document.pages();
        let page_count = pages.len();
        assert_eq!(page_count, 2);

        // Don't use this as it will load all the pages in the process
        let page_count = pages.count();
        assert_eq!(page_count, 2);
    }

    #[test]
    fn test_doc_save() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        document.save_to_path("groningen_copy.pdf", None).unwrap();
        let document = PdfiumDocument::new_from_path("groningen_copy.pdf", None).unwrap();
        let page_count = document.page_count();
        assert_eq!(page_count, 2);
    }

    #[test]
    fn test_import_pages() {
        let document = PdfiumDocument::new().unwrap();
        let src_doc = PdfiumDocument::new_from_path("resources/pg1342-images-3.pdf", None).unwrap();
        document.import_pages(&src_doc, "12,14,30-34", 0).unwrap();
        document.save_to_path("pride-1.pdf", None).unwrap();
        let document = PdfiumDocument::new_from_path("pride-1.pdf", None).unwrap();
        let page_count = document.page_count();
        assert_eq!(page_count, 7);
    }

    #[test]
    fn test_import_pages_by_index() {
        let document = PdfiumDocument::new().unwrap();
        let src_doc = PdfiumDocument::new_from_path("resources/pg1342-images-3.pdf", None).unwrap();
        document
            .import_pages_by_index(&src_doc, Some(&[11, 13, 29, 30, 31, 32, 33]), 0)
            .unwrap();
        document.save_to_path("pride-2.pdf", None).unwrap();
        let document = PdfiumDocument::new_from_path("pride-2.pdf", None).unwrap();
        let page_count = document.page_count();
        assert_eq!(page_count, 7);
    }
}
