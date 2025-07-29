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

#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{c_api::pdfium::to_result, pdfium_types::*, Pdfium, PdfiumResult};

// These functions are internal to the crate
impl Pdfium {
    /// C documentation for FPDFAvail_Destroy:
    ///
    /// ```text
    /// Destroy the |avail| document availability provider.
    ///
    ///   avail - handle to document availability provider to be destroyed.
    /// ```
    #[inline]
    pub(crate) fn FPDFAvail_Destroy(&self, avail: FPDF_AVAIL) {
        unsafe { (self.fn_FPDFAvail_Destroy)(avail) }
    }

    /// C documentation for FPDFBitmap_Destroy:
    ///
    /// ```text
    /// Function: FPDFBitmap_Destroy
    ///          Destroy a bitmap and release all related buffers.
    /// Parameters:
    ///          bitmap      -   Handle to the bitmap. Returned by FPDFBitmap_Create
    ///                          or FPDFImageObj_GetBitmap.
    /// Return value:
    ///          None.
    /// Comments:
    ///          This function will not destroy any external buffers provided when
    ///          the bitmap was created.
    /// ```
    #[inline]
    pub(crate) fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP) {
        unsafe { (self.fn_FPDFBitmap_Destroy)(bitmap) }
    }

    /// C documentation for FPDFBitmap_GetBuffer:
    ///
    /// ```text
    /// Function: FPDFBitmap_GetBuffer
    ///          Get data buffer of a bitmap.
    /// Parameters:
    ///          bitmap      -   Handle to the bitmap. Returned by FPDFBitmap_Create
    ///                          or FPDFImageObj_GetBitmap.
    /// Return value:
    ///          The pointer to the first byte of the bitmap buffer.
    /// Comments:
    ///          The stride may be more than width * number of bytes per pixel
    ///
    ///          Applications can use this function to get the bitmap buffer pointer,
    ///          then manipulate any color and/or alpha values for any pixels in the
    ///          bitmap.
    ///
    ///          Use FPDFBitmap_GetFormat() to find out the format of the data.
    /// ```
    #[inline]
    pub(crate) fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut ::std::os::raw::c_void {
        unsafe { (self.fn_FPDFBitmap_GetBuffer)(bitmap) }
    }

    /// C documentation for FPDFDoc_CloseJavaScriptAction:
    ///
    /// ```text
    ///   javascript - Handle to a JavaScript action.
    /// ```
    #[inline]
    pub(crate) fn FPDFDoc_CloseJavaScriptAction(&self, javascript: FPDF_JAVASCRIPT_ACTION) {
        unsafe { (self.fn_FPDFDoc_CloseJavaScriptAction)(javascript) }
    }

    /// C documentation for FPDFFont_Close:
    ///
    /// ```text
    /// Close a loaded PDF font.
    ///
    /// font   - Handle to the loaded font.
    /// ```
    #[inline]
    pub(crate) fn FPDFFont_Close(&self, font: FPDF_FONT) {
        unsafe { (self.fn_FPDFFont_Close)(font) }
    }

    /// C documentation for FPDFImageObj_LoadJpegFile:
    ///
    /// ```text
    /// Load an image from a JPEG image file and then set it into |image_object|.
    ///
    ///   pages        - pointer to the start of all loaded pages, may be NULL.
    ///   count        - number of |pages|, may be 0.
    ///   image_object - handle to an image object.
    ///   file_access  - file access handler which specifies the JPEG image file.
    ///
    /// Returns TRUE on success.
    ///
    /// The image object might already have an associated image, which is shared and
    /// cached by the loaded pages. In that case, we need to clear the cached image
    /// for all the loaded pages. Pass |pages| and page count (|count|) to this API
    /// to clear the image cache. If the image is not previously shared, or NULL is a
    /// valid |pages| value.
    /// ```
    #[inline]
    pub(crate) fn FPDFImageObj_LoadJpegFile(
        &self,
        pages: *mut FPDF_PAGE,
        count: ::std::os::raw::c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFImageObj_LoadJpegFile)(pages, count, image_object, file_access)
        })
    }

    /// C documentation for FPDFImageObj_LoadJpegFileInline:
    ///
    /// ```text
    /// Load an image from a JPEG image file and then set it into |image_object|.
    ///
    ///   pages        - pointer to the start of all loaded pages, may be NULL.
    ///   count        - number of |pages|, may be 0.
    ///   image_object - handle to an image object.
    ///   file_access  - file access handler which specifies the JPEG image file.
    ///
    /// Returns TRUE on success.
    ///
    /// The image object might already have an associated image, which is shared and
    /// cached by the loaded pages. In that case, we need to clear the cached image
    /// for all the loaded pages. Pass |pages| and page count (|count|) to this API
    /// to clear the image cache. If the image is not previously shared, or NULL is a
    /// valid |pages| value. This function loads the JPEG image inline, so the image
    /// content is copied to the file. This allows |file_access| and its associated
    /// data to be deleted after this function returns.
    /// ```
    #[inline]
    pub(crate) fn FPDFImageObj_LoadJpegFileInline(
        &self,
        pages: *mut FPDF_PAGE,
        count: ::std::os::raw::c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFImageObj_LoadJpegFileInline)(pages, count, image_object, file_access)
        })
    }

    /// C documentation for FPDFImageObj_SetBitmap:
    ///
    /// ```text
    /// Set |bitmap| to |image_object|.
    ///
    ///   pages        - pointer to the start of all loaded pages, may be NULL.
    ///   count        - number of |pages|, may be 0.
    ///   image_object - handle to an image object.
    ///   bitmap       - handle of the bitmap.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub(crate) fn FPDFImageObj_SetBitmap(
        &self,
        pages: *mut FPDF_PAGE,
        count: ::std::os::raw::c_int,
        image_object: FPDF_PAGEOBJECT,
        bitmap: FPDF_BITMAP,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFImageObj_SetBitmap)(pages, count, image_object, bitmap) })
    }

    /// C documentation for FPDFLink_CloseWebLinks:
    ///
    /// ```text
    /// Function: FPDFLink_CloseWebLinks
    ///          Release resources used by weblink feature.
    /// Parameters:
    ///          link_page   -   Handle returned by FPDFLink_LoadWebLinks.
    /// Return Value:
    ///          None.
    /// ```
    #[inline]
    pub(crate) fn FPDFLink_CloseWebLinks(&self, link_page: FPDF_PAGELINK) {
        unsafe { (self.fn_FPDFLink_CloseWebLinks)(link_page) }
    }

    /// C documentation for FPDFLink_Enumerate:
    ///
    /// ```text
    /// Enumerates all the link annotations in |page|.
    ///
    ///   page       - handle to the page.
    ///   start_pos  - the start position, should initially be 0 and is updated with
    ///                the next start position on return.
    ///   link_annot - the link handle for |startPos|.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub(crate) fn FPDFLink_Enumerate(
        &self,
        page: FPDF_PAGE,
        start_pos: *mut ::std::os::raw::c_int,
        link_annot: *mut FPDF_LINK,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFLink_Enumerate)(page, start_pos, link_annot) })
    }

    /// C documentation for FPDFPageObj_Destroy:
    ///
    /// ```text
    /// Destroy |page_object| by releasing its resources. |page_object| must have
    /// been created by FPDFPageObj_CreateNew{Path|Rect}() or
    /// FPDFPageObj_New{Text|Image}Obj(). This function must be called on
    /// newly-created objects if they are not added to a page through
    /// FPDFPage_InsertObject() or to an annotation through FPDFAnnot_AppendObject().
    ///
    ///   page_object - handle to a page object.
    /// ```
    #[inline]
    pub(crate) fn FPDFPageObj_Destroy(&self, page_object: FPDF_PAGEOBJECT) {
        unsafe { (self.fn_FPDFPageObj_Destroy)(page_object) }
    }

    /// C documentation for FPDFPage_CloseAnnot:
    ///
    /// ```text
    /// Experimental API.
    /// Close an annotation. Must be called when the annotation returned by
    /// FPDFPage_CreateAnnot() or FPDFPage_GetAnnot() is no longer needed. This
    /// function does not remove the annotation from the document.
    ///
    ///   annot  - handle to an annotation.
    /// ```
    #[inline]
    pub(crate) fn FPDFPage_CloseAnnot(&self, annot: FPDF_ANNOTATION) {
        unsafe { (self.fn_FPDFPage_CloseAnnot)(annot) }
    }

    /// C documentation for FPDFText_ClosePage:
    ///
    /// ```text
    /// Function: FPDFText_ClosePage
    ///          Release all resources allocated for a text page information
    ///          structure.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    /// Return Value:
    ///          None.
    /// ```
    #[inline]
    pub(crate) fn FPDFText_ClosePage(&self, text_page: FPDF_TEXTPAGE) {
        unsafe { (self.fn_FPDFText_ClosePage)(text_page) }
    }

    /// C documentation for FPDFText_FindClose:
    ///
    /// ```text
    /// Function: FPDFText_FindClose
    ///          Release a search context.
    /// Parameters:
    ///          handle      -   A search context handle returned by
    ///                          FPDFText_FindStart.
    /// Return Value:
    ///          None.
    /// ```
    #[inline]
    pub(crate) fn FPDFText_FindClose(&self, handle: FPDF_SCHHANDLE) {
        unsafe { (self.fn_FPDFText_FindClose)(handle) }
    }

    /// C documentation for FPDF_CloseDocument:
    ///
    /// ```text
    /// Function: FPDF_CloseDocument
    ///          Close a loaded PDF document.
    /// Parameters:
    ///          document    -   Handle to the loaded document.
    /// Return value:
    ///          None.
    /// ```
    #[inline]
    pub(crate) fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT) {
        unsafe { (self.fn_FPDF_CloseDocument)(document) }
    }

    /// C documentation for FPDF_ClosePage:
    ///
    /// ```text
    /// Function: FPDF_ClosePage
    ///          Close a loaded PDF page.
    /// Parameters:
    ///          page        -   Handle to the loaded page.
    /// Return value:
    ///          None.
    /// ```
    #[inline]
    pub(crate) fn FPDF_ClosePage(&self, page: FPDF_PAGE) {
        unsafe { (self.fn_FPDF_ClosePage)(page) }
    }

    /// C documentation for FPDF_CloseXObject:
    ///
    /// ```text
    /// Experimental API.
    /// Close an FPDF_XOBJECT handle created by FPDF_NewXObjectFromPage().
    /// FPDF_PAGEOBJECTs created from the FPDF_XOBJECT handle are not affected.
    /// ```
    #[inline]
    pub(crate) fn FPDF_CloseXObject(&self, xobject: FPDF_XOBJECT) {
        unsafe { (self.fn_FPDF_CloseXObject)(xobject) }
    }

    /// C documentation for FPDF_DestroyClipPath:
    ///
    /// ```text
    /// Destroy the clip path.
    ///
    /// clipPath - A handle to the clip path. It will be invalid after this call.
    /// ```
    #[inline]
    pub(crate) fn FPDF_DestroyClipPath(&self, clipPath: FPDF_CLIPPATH) {
        unsafe { (self.fn_FPDF_DestroyClipPath)(clipPath) }
    }

    /// C documentation for FPDF_FreeDefaultSystemFontInfo:
    ///
    /// ```text
    /// Function: FPDF_FreeDefaultSystemFontInfo
    ///           Free a default system font info interface
    /// Parameters:
    ///           font_info       -   Pointer to a FPDF_SYSFONTINFO structure
    /// Return Value:
    ///           None
    /// Comments:
    ///           This function should be called on the output from
    ///           FPDF_GetDefaultSystemFontInfo() once it is no longer needed.
    /// ```
    #[inline]
    pub(crate) fn FPDF_FreeDefaultSystemFontInfo(&self, font_info: *mut FPDF_SYSFONTINFO) {
        unsafe { (self.fn_FPDF_FreeDefaultSystemFontInfo)(font_info) }
    }

    /// C documentation for FPDF_GetDefaultSystemFontInfo:
    ///
    /// ```text
    /// Function: FPDF_GetDefaultSystemFontInfo
    ///          Get default system font info interface for current platform
    /// Parameters:
    ///          None
    /// Return Value:
    ///          Pointer to a FPDF_SYSFONTINFO structure describing the default
    ///          interface, or NULL if the platform doesn't have a default interface.
    ///          Application should call FPDF_FreeDefaultSystemFontInfo to free the
    ///          returned pointer.
    /// Comments:
    ///          For some platforms, PDFium implements a default version of system
    ///          font info interface. The default implementation can be passed to
    ///          FPDF_SetSystemFontInfo().
    /// ```
    #[inline]
    pub(crate) fn FPDF_GetDefaultSystemFontInfo(&self) -> *mut FPDF_SYSFONTINFO {
        unsafe { (self.fn_FPDF_GetDefaultSystemFontInfo)() }
    }

    /// C documentation for FPDF_GetDefaultTTFMap:
    ///
    /// ```text
    /// Function: FPDF_GetDefaultTTFMap
    ///    Returns a pointer to the default character set to TT Font name map. The
    ///    map is an array of FPDF_CharsetFontMap structs, with its end indicated
    ///    by a { -1, NULL } entry.
    /// Parameters:
    ///     None.
    /// Return Value:
    ///     Pointer to the Charset Font Map.
    /// Note:
    ///     Once FPDF_GetDefaultTTFMapCount() and FPDF_GetDefaultTTFMapEntry() are no
    ///     longer experimental, this API will be marked as deprecated.
    ///     See https://crbug.com/348468114
    /// ```
    #[inline]
    pub(crate) fn FPDF_GetDefaultTTFMap(&self) -> *const FPDF_CharsetFontMap {
        unsafe { (self.fn_FPDF_GetDefaultTTFMap)() }
    }

    /// C documentation for FPDF_GetDefaultTTFMapEntry:
    ///
    /// ```text
    /// Experimental API.
    ///
    /// Function: FPDF_GetDefaultTTFMapEntry
    ///    Returns an entry in the default character set to TT Font name map.
    /// Parameters:
    ///    index    -   The index to the entry in the map to retrieve.
    /// Return Value:
    ///     A pointer to the entry, if it is in the map, or NULL if the index is out
    ///     of bounds.
    /// ```
    #[inline]
    pub(crate) fn FPDF_GetDefaultTTFMapEntry(&self, index: usize) -> *const FPDF_CharsetFontMap {
        unsafe { (self.fn_FPDF_GetDefaultTTFMapEntry)(index) }
    }

    /// C documentation for FPDF_SaveAsCopy:
    ///
    /// ```text
    /// Function: FPDF_SaveAsCopy
    ///          Saves the copy of specified document in custom way.
    /// Parameters:
    ///          document        -   Handle to document, as returned by
    ///                              FPDF_LoadDocument() or FPDF_CreateNewDocument().
    ///          pFileWrite      -   A pointer to a custom file write structure.
    ///          flags           -   Flags above that affect how the PDF gets saved.
    ///                              Pass in 0 when there are no flags.
    /// Return value:
    ///          TRUE for succeed, FALSE for failed.
    /// ```
    #[inline]
    pub(crate) fn FPDF_SaveAsCopy(
        &self,
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_SaveAsCopy)(document, pFileWrite, flags) })
    }

    /// C documentation for FPDF_SaveWithVersion:
    ///
    /// ```text
    /// Function: FPDF_SaveWithVersion
    ///          Same as FPDF_SaveAsCopy(), except the file version of the
    ///          saved document can be specified by the caller.
    /// Parameters:
    ///          document        -   Handle to document.
    ///          pFileWrite      -   A pointer to a custom file write structure.
    ///          flags           -   The creating flags.
    ///          fileVersion     -   The PDF file version. File version: 14 for 1.4,
    ///                              15 for 1.5, ...
    /// Return value:
    ///          TRUE if succeed, FALSE if failed.
    /// ```
    #[inline]
    pub(crate) fn FPDF_SaveWithVersion(
        &self,
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
        fileVersion: ::std::os::raw::c_int,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_SaveWithVersion)(document, pFileWrite, flags, fileVersion)
        })
    }

    /// C documentation for FPDF_SetSystemFontInfo:
    ///
    /// ```text
    /// Function: FPDF_SetSystemFontInfo
    ///          Set the system font info interface into PDFium
    /// Parameters:
    ///          font_info       -   Pointer to a FPDF_SYSFONTINFO structure
    /// Return Value:
    ///          None
    /// Comments:
    ///          Platform support implementation should implement required methods of
    ///          FFDF_SYSFONTINFO interface, then call this function during PDFium
    ///          initialization process.
    ///
    ///          Call this with NULL to tell PDFium to stop using a previously set
    ///          |FPDF_SYSFONTINFO|.
    /// ```
    #[inline]
    pub(crate) fn FPDF_SetSystemFontInfo(&self, font_info: *mut FPDF_SYSFONTINFO) {
        unsafe { (self.fn_FPDF_SetSystemFontInfo)(font_info) }
    }

    /// C documentation for FPDF_StructTree_Close:
    ///
    /// ```text
    /// Function: FPDF_StructTree_Close
    ///          Release a resource allocated by FPDF_StructTree_GetForPage().
    /// Parameters:
    ///          struct_tree -   Handle to the structure tree, as returned by
    ///                          FPDF_StructTree_LoadPage().
    /// Return value:
    ///          None.
    /// ```
    #[inline]
    pub(crate) fn FPDF_StructTree_Close(&self, struct_tree: FPDF_STRUCTTREE) {
        unsafe { (self.fn_FPDF_StructTree_Close)(struct_tree) }
    }
}
