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

#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{
    Pdfium, PdfiumAnnotation, PdfiumAvailability, PdfiumBitmap, PdfiumClipPath, PdfiumDocument,
    PdfiumFont, PdfiumJavascriptAction, PdfiumPage, PdfiumPageLink, PdfiumPageObject, PdfiumSearch,
    PdfiumStructTree, PdfiumTextPage, PdfiumXObject, pdfium_types::*,
};

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
    pub(crate) fn FPDFAvail_Destroy(&self, avail: &PdfiumAvailability) {
        unsafe { (self.fn_FPDFAvail_Destroy)(avail.into()) }
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
    pub(crate) fn FPDFBitmap_Destroy(&self, bitmap: &PdfiumBitmap) {
        unsafe { (self.fn_FPDFBitmap_Destroy)(bitmap.into()) }
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
    pub(crate) fn FPDFBitmap_GetBuffer(
        &self,
        bitmap: &PdfiumBitmap,
    ) -> *mut ::std::os::raw::c_void {
        unsafe { (self.fn_FPDFBitmap_GetBuffer)(bitmap.into()) }
    }
    /// C documentation for FPDFDoc_CloseJavaScriptAction:
    ///
    /// ```text
    ///   javascript - Handle to a JavaScript action.
    /// ```
    #[inline]
    pub(crate) fn FPDFDoc_CloseJavaScriptAction(&self, javascript: &PdfiumJavascriptAction) {
        unsafe { (self.fn_FPDFDoc_CloseJavaScriptAction)(javascript.into()) }
    }
    /// C documentation for FPDFFont_Close:
    ///
    /// ```text
    /// Close a loaded PDF font.
    ///
    /// font   - Handle to the loaded font.
    /// ```
    #[inline]
    pub(crate) fn FPDFFont_Close(&self, font: &PdfiumFont) {
        unsafe { (self.fn_FPDFFont_Close)(font.into()) }
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
    pub(crate) fn FPDFLink_CloseWebLinks(&self, link_page: &PdfiumPageLink) {
        unsafe { (self.fn_FPDFLink_CloseWebLinks)(link_page.into()) }
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
    pub(crate) fn FPDFPageObj_Destroy(&self, page_object: &PdfiumPageObject) {
        unsafe { (self.fn_FPDFPageObj_Destroy)(page_object.into()) }
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
    pub(crate) fn FPDFPage_CloseAnnot(&self, annot: &PdfiumAnnotation) {
        unsafe { (self.fn_FPDFPage_CloseAnnot)(annot.into()) }
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
    pub(crate) fn FPDFText_ClosePage(&self, text_page: &PdfiumTextPage) {
        unsafe { (self.fn_FPDFText_ClosePage)(text_page.into()) }
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
    pub(crate) fn FPDFText_FindClose(&self, handle: &PdfiumSearch) {
        unsafe { (self.fn_FPDFText_FindClose)(handle.into()) }
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
    pub(crate) fn FPDF_CloseDocument(&self, document: &PdfiumDocument) {
        unsafe { (self.fn_FPDF_CloseDocument)(document.into()) }
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
    pub(crate) fn FPDF_ClosePage(&self, page: &PdfiumPage) {
        unsafe { (self.fn_FPDF_ClosePage)(page.into()) }
    }
    /// C documentation for FPDF_CloseXObject:
    ///
    /// ```text
    /// Experimental API.
    /// Close an FPDF_XOBJECT handle created by FPDF_NewXObjectFromPage().
    /// FPDF_PAGEOBJECTs created from the FPDF_XOBJECT handle are not affected.
    /// ```
    #[inline]
    pub(crate) fn FPDF_CloseXObject(&self, xobject: &PdfiumXObject) {
        unsafe { (self.fn_FPDF_CloseXObject)(xobject.into()) }
    }
    /// C documentation for FPDF_DestroyClipPath:
    ///
    /// ```text
    /// Destroy the clip path.
    ///
    /// clipPath - A handle to the clip path. It will be invalid after this call.
    /// ```
    #[inline]
    pub(crate) fn FPDF_DestroyClipPath(&self, clipPath: &PdfiumClipPath) {
        unsafe { (self.fn_FPDF_DestroyClipPath)(clipPath.into()) }
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
    pub(crate) fn FPDF_StructTree_Close(&self, struct_tree: &PdfiumStructTree) {
        unsafe { (self.fn_FPDF_StructTree_Close)(struct_tree.into()) }
    }
}
