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

pub mod reader;
pub mod writer;

use std::{
    collections::{HashMap, HashSet},
    ffi::CString,
    fmt::Debug,
    fs::File,
    io::{Cursor, Read, Seek, Write},
    path::Path,
    rc::Rc,
    str::FromStr,
};

use crate::{
    PdfiumAttachment, PdfiumBookmark, PdfiumXObject,
    document::{reader::PdfiumReader, writer::PdfiumWriter},
    error::{PdfiumError, PdfiumResult},
    lib,
    page::{PdfiumPage, pages::PdfiumPages},
    pdfium_types::{DocumentHandle, FPDF_DOCUMENT, FPDF_FILEIDTYPE, FS_SIZEF, Handle},
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

    /// Creates a new empty [`PdfiumDocument`]
    pub fn new() -> PdfiumResult<Self> {
        let handle = try_lib()?.FPDF_CreateNewDocument();
        Self::new_from_handle(handle, None)
    }

    /// Load a [`PdfiumDocument`] from a `Path`
    pub fn new_from_path<P: AsRef<Path>>(path: P, password: Option<&str>) -> PdfiumResult<Self> {
        let reader = File::open(path)?;
        Self::new_from_reader(reader, password)
    }

    /// Load a [`PdfiumDocument`] using a reader implementing `Read` and `Seek`
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
    ///   where to save the PDF file. Uses `AsRef<Path>` for maximum flexibility.
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
    /// * `PdfiumResult<Vec<u8>>` - On success, returns a `Vec<u8>` containing the complete
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
    ///   lifetime issues. Common types include `File`, `TcpStream`, `Cursor<Vec<u8>>`, etc.
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

    /// Return an [`Iterator`] for the pages in this [`PdfiumDocument`].
    pub fn pages(&self) -> PdfiumPages<'_> {
        PdfiumPages::new(self)
    }

    /// Imports pages from another [`PdfiumDocument`] into this [`PdfiumDocument`].
    ///
    /// # Examples
    /// ```ignore
    /// let dest_document = PdfiumDocument::new_from_path("output.pdf", None)?;
    /// let src_document = PdfiumDocument::new_from_path("input.pdf", None)?;
    /// dest_document.import_pages(&src_document, "1,3,5-7", 0)?;
    /// ```
    pub fn import_pages(&self, src_doc: &Self, page_range: &str, index: i32) -> PdfiumResult<()> {
        lib().FPDF_ImportPages(self, src_doc, &CString::from_str(page_range)?, index)
    }

    /// Helper function for recursively traversing the table of contents.
    fn get_toc_helper(
        &self,
        max_depth: u32,
        level: u32,
        parent: PdfiumBookmark,
        result: &mut Vec<PdfiumBookmark>,
        seen: &mut HashSet<crate::pdfium_types::FPDF_BOOKMARK>,
    ) -> PdfiumResult<()> {
        let lib = lib();
        let mut bm = lib.FPDFBookmark_GetFirstChild(self, &parent)?;
        while !bm.is_null() {
            let ptr = crate::pdfium_types::FPDF_BOOKMARK::from(&bm);
            if seen.contains(&ptr) {
                return Err(PdfiumError::CircularReferenceError);
            }
            seen.insert(ptr);
            bm.set_level(level);
            let next_bm = lib.FPDFBookmark_GetNextSibling(self, &bm)?;
            let bm_dup = bm.clone();
            result.push(bm);
            if level < max_depth - 1 {
                self.get_toc_helper(max_depth, level + 1, bm_dup, result, seen)?;
            }
            bm = next_bm;
        }
        Ok(())
    }

    /// Returns the type of form contained in this [`PdfiumDocument`] (`FORMTYPE_*` constant).
    pub fn form_type(&self) -> i32 {
        lib().FPDF_GetFormType(self)
    }

    /// Returns the page mode of this [`PdfiumDocument`] (`PAGEMODE_*` constant).
    pub fn page_mode(&self) -> i32 {
        lib().FPDFDoc_GetPageMode(self)
    }

    /// Returns whether this [`PdfiumDocument`] is a tagged PDF.
    pub fn is_tagged(&self) -> bool {
        lib().FPDFCatalog_IsTagged(self).is_ok()
    }

    /// Returns the unique file identifier from the PDF's trailer dictionary.
    ///
    /// `id_type` is either `FPDF_FILEIDTYPE_FILEIDTYPE_PERMANENT` or
    /// `FPDF_FILEIDTYPE_FILEIDTYPE_CHANGING`.
    pub fn identifier(&self, id_type: FPDF_FILEIDTYPE) -> Vec<u8> {
        let lib = lib();
        let n_bytes = lib.FPDF_GetFileIdentifier(self, id_type, None, 0) as usize;
        if n_bytes <= 2 {
            return Vec::new();
        }
        let mut buffer = vec![0u8; n_bytes];
        lib.FPDF_GetFileIdentifier(self, id_type, Some(&mut buffer), n_bytes as u64);
        buffer.truncate(n_bytes - 2);
        buffer
    }

    /// Returns the PDF version of this [`PdfiumDocument`] (e.g. 14 for PDF 1.4),
    /// or `None` if the document is new or the version could not be determined.
    pub fn version(&self) -> Option<i32> {
        let mut version = 0i32;
        lib().FPDF_GetFileVersion(self, &mut version).ok()?;
        Some(version)
    }

    /// Returns the value of a metadata key from this [`PdfiumDocument`].
    /// Returns an empty string if the key is not present.
    pub fn metadata_value(&self, key: &str) -> PdfiumResult<String> {
        let lib = lib();
        let tag = CString::new(key).map_err(|_| PdfiumError::NulError)?;
        let buf_len = lib.FPDF_GetMetaText(self, &tag, None, 0);
        if buf_len == 0 {
            return Ok(String::new());
        }
        let mut buffer = vec![0u16; buf_len as usize / 2];
        let (_prefix, u8_slice, _suffix) = unsafe { buffer.align_to_mut::<u8>() };
        lib.FPDF_GetMetaText(self, &tag, Some(u8_slice), buf_len);
        String::from_utf16(&buffer[..buf_len as usize / 2 - 1])
            .map_err(|_| PdfiumError::StringEncodingError)
    }

    /// Standard PDF metadata keys.
    pub const METADATA_KEYS: &'static [&'static str] = &[
        "Title",
        "Author",
        "Subject",
        "Keywords",
        "Creator",
        "Producer",
        "CreationDate",
        "ModDate",
    ];

    /// Returns all metadata from this [`PdfiumDocument`] as a `HashMap`.
    ///
    /// If `skip_empty` is `true`, keys with empty values are omitted.
    pub fn metadata_dict(&self, skip_empty: bool) -> PdfiumResult<HashMap<String, String>> {
        let mut map = HashMap::new();
        for &key in Self::METADATA_KEYS {
            let value = self.metadata_value(key)?;
            if !skip_empty || !value.is_empty() {
                map.insert(key.to_string(), value);
            }
        }
        Ok(map)
    }

    /// Returns the number of embedded files in this [`PdfiumDocument`].
    pub fn count_attachments(&self) -> i32 {
        lib().FPDFDoc_GetAttachmentCount(self)
    }

    /// Returns the [`PdfiumAttachment`] at the given zero-based index.
    pub fn attachment(&self, index: i32) -> PdfiumResult<PdfiumAttachment> {
        lib().FPDFDoc_GetAttachment(self, index)
    }

    /// Adds a new attachment with the given name to this [`PdfiumDocument`].
    pub fn new_attachment(&self, name: &str) -> PdfiumResult<PdfiumAttachment> {
        lib().FPDFDoc_AddAttachment(self, name)
    }

    /// Removes the attachment at the given zero-based index from this [`PdfiumDocument`].
    /// Following attachments shift one slot to the left.
    pub fn del_attachment(&self, index: i32) -> PdfiumResult<()> {
        lib().FPDFDoc_DeleteAttachment(self, index)
    }

    /// Inserts a new empty page into this [`PdfiumDocument`].
    ///
    /// If `index` is `None` or beyond the last page, the page is appended.
    pub fn new_page(
        &self,
        width: f64,
        height: f64,
        index: Option<i32>,
    ) -> PdfiumResult<PdfiumPage> {
        let index = index.unwrap_or_else(|| self.page_count());
        lib().FPDFPage_New(self, index, width, height)
    }

    /// Removes the page at the given zero-based index from this [`PdfiumDocument`].
    pub fn del_page(&self, index: i32) {
        lib().FPDFPage_Delete(self, index);
    }

    /// Returns the size `(width, height)` in PDF canvas units of the page at the given
    /// zero-based index.
    pub fn page_size(&self, index: i32) -> PdfiumResult<(f32, f32)> {
        let mut size = FS_SIZEF {
            width: 0.0,
            height: 0.0,
        };
        lib().FPDF_GetPageSizeByIndexF(self, index, &mut size)?;
        Ok((size.width, size.height))
    }

    /// Returns the label string of the page at the given zero-based index.
    pub fn page_label(&self, index: i32) -> PdfiumResult<String> {
        let lib = lib();
        let buf_len = lib.FPDF_GetPageLabel(self, index, None, 0);
        if buf_len == 0 {
            return Ok(String::new());
        }
        let mut buffer = vec![0u16; buf_len as usize / 2];
        let (_prefix, u8_slice, _suffix) = unsafe { buffer.align_to_mut::<u8>() };
        lib.FPDF_GetPageLabel(self, index, Some(u8_slice), buf_len);
        String::from_utf16(&buffer[..buf_len as usize / 2 - 1])
            .map_err(|_| PdfiumError::StringEncodingError)
    }

    /// Captures the page at the given zero-based index as a [`PdfiumXObject`] attached to
    /// `dest_doc`'s resources.
    pub fn page_as_xobject(
        &self,
        index: i32,
        dest_doc: &PdfiumDocument,
    ) -> PdfiumResult<PdfiumXObject> {
        lib().FPDF_NewXObjectFromPage(dest_doc, self, index)
    }

    /// Iterate through the bookmarks in the document's table of contents (TOC).
    pub fn toc(&self, max_depth: u32) -> PdfiumResult<Vec<PdfiumBookmark>> {
        let mut result = Vec::new();
        let mut seen = HashSet::new();
        self.get_toc_helper(max_depth, 0, PdfiumBookmark::null(), &mut result, &mut seen)?;
        Ok(result)
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
        let page_count = pages.page_count();
        assert_eq!(page_count, 2);

        let page_count = pages.count();
        assert_eq!(page_count, 2);
    }

    #[test]
    fn test_doc_save() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        document.save_to_path("groningen-copy.pdf", None).unwrap();
        let document = PdfiumDocument::new_from_path("groningen-copy.pdf", None).unwrap();
        let page_count = document.page_count();
        assert_eq!(page_count, 2);
    }
}
