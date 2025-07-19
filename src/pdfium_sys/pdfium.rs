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
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};

use super::lib_get;
use crate::{
    PdfiumAnnotation, PdfiumBitmap, PdfiumClipPath, PdfiumDocument, PdfiumError, PdfiumForm,
    PdfiumFormFillInfo, PdfiumLibraryConfig, PdfiumPage, PdfiumPageObject, PdfiumResult,
    PdfiumXObject, pdfium_types::*,
};
use libloading::Library;

/// # Rust bindings to the PDFium C library
///
/// These functions aim to fully follow the original API, while also providing a safe interface by replacing
/// all pointer based parameters with safe Rust replacements. The same applies to the function results
/// whenever possible.
#[allow(non_snake_case)]
pub struct Pdfium {
    fn_FPDF_InitLibraryWithConfig: unsafe extern "C" fn(config: *const FPDF_LIBRARY_CONFIG),
    fn_FPDF_InitLibrary: unsafe extern "C" fn(),
    fn_FPDF_DestroyLibrary: unsafe extern "C" fn(),
    fn_FPDF_LoadCustomDocument: unsafe extern "C" fn(
        pFileAccess: *mut FPDF_FILEACCESS,
        password: FPDF_BYTESTRING,
    ) -> FPDF_DOCUMENT,
    fn_FPDF_GetLastError: unsafe extern "C" fn() -> c_ulong,
    fn_FPDF_GetPageCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    fn_FPDF_LoadPage: unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE,
    fn_FPDF_RenderPageBitmapWithMatrix: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipping: *const FS_RECTF,
        flags: c_int,
    ),
    fn_FPDF_ClosePage: unsafe extern "C" fn(page: FPDF_PAGE),
    fn_FPDF_CloseDocument: unsafe extern "C" fn(document: FPDF_DOCUMENT),
    fn_FPDFBitmap_Create:
        unsafe extern "C" fn(width: c_int, height: c_int, alpha: c_int) -> FPDF_BITMAP,
    fn_FPDFBitmap_CreateEx: unsafe extern "C" fn(
        width: c_int,
        height: c_int,
        format: c_int,
        first_scan: *mut c_void,
        stride: c_int,
    ) -> FPDF_BITMAP,
    fn_FPDFBitmap_GetFormat: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    fn_FPDFBitmap_FillRect: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        color: FPDF_DWORD,
    ) -> FPDF_BOOL,
    fn_FPDFBitmap_GetBuffer: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> *mut c_void,
    fn_FPDFBitmap_GetWidth: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    fn_FPDFBitmap_GetHeight: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    fn_FPDFBitmap_GetStride: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    fn_FPDFBitmap_Destroy: unsafe extern "C" fn(bitmap: FPDF_BITMAP),
    fn_FPDFDOC_InitFormFillEnvironment: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        formInfo: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE,
    fn_FPDFPage_HasFormFieldAtPoint: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        page_x: f64,
        page_y: f64,
    ) -> c_int,
    fn_FPDFPage_FormFieldZOrderAtPoint: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        page_x: f64,
        page_y: f64,
    ) -> c_int,
    fn_FPDFPage_CreateAnnot:
        unsafe extern "C" fn(page: FPDF_PAGE, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_ANNOTATION,
    fn_FPDFPage_GetAnnotCount: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    fn_FPDFPage_GetAnnot: unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_ANNOTATION,
    fn_FPDFPage_GetAnnotIndex:
        unsafe extern "C" fn(page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> c_int,
    fn_FPDFPage_CloseAnnot: unsafe extern "C" fn(annot: FPDF_ANNOTATION),
    fn_FPDFPage_RemoveAnnot: unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_BOOL,
    fn_FPDFPage_New: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        width: f64,
        height: f64,
    ) -> FPDF_PAGE,
    fn_FPDFPage_Delete: unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: c_int),
    fn_FPDFPage_GetRotation: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    fn_FPDFPage_SetRotation: unsafe extern "C" fn(page: FPDF_PAGE, rotate: c_int),
    fn_FPDFPage_InsertObject: unsafe extern "C" fn(page: FPDF_PAGE, page_object: FPDF_PAGEOBJECT),
    fn_FPDFPage_RemoveObject:
        unsafe extern "C" fn(page: FPDF_PAGE, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    fn_FPDFPage_CountObjects: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    fn_FPDFPage_GetObject: unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT,
    fn_FPDFPage_HasTransparency: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL,
    fn_FPDFPage_GenerateContent: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL,
    fn_FPDFPage_TransformAnnots:
        unsafe extern "C" fn(page: FPDF_PAGE, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64),
    fn_FPDFPage_Flatten: unsafe extern "C" fn(page: FPDF_PAGE, nFlag: c_int) -> c_int,
    fn_FPDF_NewXObjectFromPage: unsafe extern "C" fn(
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        src_page_index: c_int,
    ) -> FPDF_XOBJECT,
    fn_FPDF_CloseXObject: unsafe extern "C" fn(xobject: FPDF_XOBJECT),
    fn_FPDFPage_GetDecodedThumbnailData:
        unsafe extern "C" fn(page: FPDF_PAGE, buffer: *mut c_void, buflen: c_ulong) -> c_ulong,
    fn_FPDFPage_GetRawThumbnailData:
        unsafe extern "C" fn(page: FPDF_PAGE, buffer: *mut c_void, buflen: c_ulong) -> c_ulong,
    fn_FPDFPage_GetThumbnailAsBitmap: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BITMAP,
    fn_FPDFPage_SetMediaBox:
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
    fn_FPDFPage_SetCropBox:
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
    fn_FPDFPage_SetBleedBox:
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
    fn_FPDFPage_SetTrimBox:
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
    fn_FPDFPage_SetArtBox:
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
    fn_FPDFPage_GetMediaBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    fn_FPDFPage_GetCropBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    fn_FPDFPage_GetBleedBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    fn_FPDFPage_GetTrimBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    fn_FPDFPage_GetArtBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    fn_FPDFPage_TransFormWithClip: unsafe extern "C" fn(
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipRect: *const FS_RECTF,
    ) -> FPDF_BOOL,
    fn_FPDF_CreateClipPath:
        unsafe extern "C" fn(left: f32, bottom: f32, right: f32, top: f32) -> FPDF_CLIPPATH,
    fn_FPDF_DestroyClipPath: unsafe extern "C" fn(clipPath: FPDF_CLIPPATH),
    fn_FPDFPage_InsertClipPath: unsafe extern "C" fn(page: FPDF_PAGE, clipPath: FPDF_CLIPPATH),
    lib: Library,
}

impl Pdfium {
    pub fn new(lib: Library) -> Result<Self, PdfiumError> {
        Ok(Self {
            fn_FPDF_InitLibraryWithConfig: *(lib_get(&lib, "FPDF_InitLibraryWithConfig")?),
            fn_FPDF_InitLibrary: *(lib_get(&lib, "FPDF_InitLibrary")?),
            fn_FPDF_DestroyLibrary: *(lib_get(&lib, "FPDF_DestroyLibrary")?),
            fn_FPDF_LoadCustomDocument: *(lib_get(&lib, "FPDF_LoadCustomDocument")?),
            fn_FPDF_GetLastError: *(lib_get(&lib, "FPDF_GetLastError")?),
            fn_FPDF_GetPageCount: *(lib_get(&lib, "FPDF_GetPageCount")?),
            fn_FPDF_LoadPage: *(lib_get(&lib, "FPDF_LoadPage")?),
            fn_FPDF_RenderPageBitmapWithMatrix: *(lib_get(
                &lib,
                "FPDF_RenderPageBitmapWithMatrix",
            )?),
            fn_FPDF_ClosePage: *(lib_get(&lib, "FPDF_ClosePage")?),
            fn_FPDF_CloseDocument: *(lib_get(&lib, "FPDF_CloseDocument")?),
            fn_FPDFBitmap_Create: *(lib_get(&lib, "FPDFBitmap_Create")?),
            fn_FPDFBitmap_CreateEx: *(lib_get(&lib, "FPDFBitmap_CreateEx")?),
            fn_FPDFBitmap_GetFormat: *(lib_get(&lib, "FPDFBitmap_GetFormat")?),
            fn_FPDFBitmap_FillRect: *(lib_get(&lib, "FPDFBitmap_FillRect")?),
            fn_FPDFBitmap_GetBuffer: *(lib_get(&lib, "FPDFBitmap_GetBuffer")?),
            fn_FPDFBitmap_GetWidth: *(lib_get(&lib, "FPDFBitmap_GetWidth")?),
            fn_FPDFBitmap_GetHeight: *(lib_get(&lib, "FPDFBitmap_GetHeight")?),
            fn_FPDFBitmap_GetStride: *(lib_get(&lib, "FPDFBitmap_GetStride")?),
            fn_FPDFBitmap_Destroy: *(lib_get(&lib, "FPDFBitmap_Destroy")?),
            fn_FPDFDOC_InitFormFillEnvironment: *(lib_get(
                &lib,
                "FPDFDOC_InitFormFillEnvironment",
            )?),
            fn_FPDFPage_HasFormFieldAtPoint: *(lib_get(&lib, "FPDFPage_HasFormFieldAtPoint")?),
            fn_FPDFPage_FormFieldZOrderAtPoint: *(lib_get(
                &lib,
                "FPDFPage_FormFieldZOrderAtPoint",
            )?),
            fn_FPDFPage_CreateAnnot: *(lib_get(&lib, "FPDFPage_CreateAnnot")?),
            fn_FPDFPage_GetAnnotCount: *(lib_get(&lib, "FPDFPage_GetAnnotCount")?),
            fn_FPDFPage_GetAnnot: *(lib_get(&lib, "FPDFPage_GetAnnot")?),
            fn_FPDFPage_GetAnnotIndex: *(lib_get(&lib, "FPDFPage_GetAnnotIndex")?),
            fn_FPDFPage_CloseAnnot: *(lib_get(&lib, "FPDFPage_CloseAnnot")?),
            fn_FPDFPage_RemoveAnnot: *(lib_get(&lib, "FPDFPage_RemoveAnnot")?),
            fn_FPDFPage_New: *(lib_get(&lib, "FPDFPage_New")?),
            fn_FPDFPage_Delete: *(lib_get(&lib, "FPDFPage_Delete")?),
            fn_FPDFPage_GetRotation: *(lib_get(&lib, "FPDFPage_GetRotation")?),
            fn_FPDFPage_SetRotation: *(lib_get(&lib, "FPDFPage_SetRotation")?),
            fn_FPDFPage_InsertObject: *(lib_get(&lib, "FPDFPage_InsertObject")?),
            fn_FPDFPage_RemoveObject: *(lib_get(&lib, "FPDFPage_RemoveObject")?),
            fn_FPDFPage_CountObjects: *(lib_get(&lib, "FPDFPage_CountObjects")?),
            fn_FPDFPage_GetObject: *(lib_get(&lib, "FPDFPage_GetObject")?),
            fn_FPDFPage_HasTransparency: *(lib_get(&lib, "FPDFPage_HasTransparency")?),
            fn_FPDFPage_GenerateContent: *(lib_get(&lib, "FPDFPage_GenerateContent")?),
            fn_FPDFPage_TransformAnnots: *(lib_get(&lib, "FPDFPage_TransformAnnots")?),
            fn_FPDFPage_Flatten: *(lib_get(&lib, "FPDFPage_Flatten")?),
            fn_FPDF_NewXObjectFromPage: *(lib_get(&lib, "FPDF_NewXObjectFromPage")?),
            fn_FPDF_CloseXObject: *(lib_get(&lib, "FPDF_CloseXObject")?),
            fn_FPDFPage_GetDecodedThumbnailData: *(lib_get(
                &lib,
                "FPDFPage_GetDecodedThumbnailData",
            )?),
            fn_FPDFPage_GetRawThumbnailData: *(lib_get(&lib, "FPDFPage_GetRawThumbnailData")?),
            fn_FPDFPage_GetThumbnailAsBitmap: *(lib_get(&lib, "FPDFPage_GetThumbnailAsBitmap")?),
            fn_FPDFPage_SetMediaBox: *(lib_get(&lib, "FPDFPage_SetMediaBox")?),
            fn_FPDFPage_SetCropBox: *(lib_get(&lib, "FPDFPage_SetCropBox")?),
            fn_FPDFPage_SetBleedBox: *(lib_get(&lib, "FPDFPage_SetBleedBox")?),
            fn_FPDFPage_SetTrimBox: *(lib_get(&lib, "FPDFPage_SetTrimBox")?),
            fn_FPDFPage_SetArtBox: *(lib_get(&lib, "FPDFPage_SetArtBox")?),
            fn_FPDFPage_GetMediaBox: *(lib_get(&lib, "FPDFPage_GetMediaBox")?),
            fn_FPDFPage_GetCropBox: *(lib_get(&lib, "FPDFPage_GetCropBox")?),
            fn_FPDFPage_GetBleedBox: *(lib_get(&lib, "FPDFPage_GetBleedBox")?),
            fn_FPDFPage_GetTrimBox: *(lib_get(&lib, "FPDFPage_GetTrimBox")?),
            fn_FPDFPage_GetArtBox: *(lib_get(&lib, "FPDFPage_GetArtBox")?),
            fn_FPDFPage_TransFormWithClip: *(lib_get(&lib, "FPDFPage_TransFormWithClip")?),
            fn_FPDF_CreateClipPath: *(lib_get(&lib, "FPDF_CreateClipPath")?),
            fn_FPDF_DestroyClipPath: *(lib_get(&lib, "FPDF_DestroyClipPath")?),
            fn_FPDFPage_InsertClipPath: *(lib_get(&lib, "FPDFPage_InsertClipPath")?),
            lib,
        })
    }
}

impl Pdfium {
    /// C documentation for FPDF_InitLibraryWithConfig:
    ///
    /// ```text
    /// Function: FPDF_InitLibraryWithConfig
    ///          Initialize the PDFium library and allocate global resources for it.
    /// Parameters:
    ///          config - configuration information as above.
    /// Return value:
    ///          None.
    /// Comments:
    ///          You have to call this function before you can call any PDF
    ///          processing functions.
    /// ```
    pub fn FPDF_InitLibraryWithConfig(&self, config: &PdfiumLibraryConfig) {
        let config: FPDF_LIBRARY_CONFIG = config.into();
        unsafe { (self.fn_FPDF_InitLibraryWithConfig)(&config) }
    }

    /// C documentation for FPDF_InitLibrary:
    ///
    /// ```text
    /// Function: FPDF_InitLibrary
    ///          Initialize the PDFium library (alternative form).
    /// Parameters:
    ///          None
    /// Return value:
    ///          None.
    /// Comments:
    ///          Convenience function to call FPDF_InitLibraryWithConfig() with a
    ///          default configuration for backwards compatibility purposes. New
    ///          code should call FPDF_InitLibraryWithConfig() instead. This will
    ///          be deprecated in the future.
    /// ```
    pub fn FPDF_InitLibrary(&self) {
        unsafe { (self.fn_FPDF_InitLibrary)() }
    }

    /// C documentation for FPDF_DestroyLibrary:
    ///
    /// ```text
    /// Function: FPDF_DestroyLibrary
    ///          Release global resources allocated to the PDFium library by
    ///          FPDF_InitLibrary() or FPDF_InitLibraryWithConfig().
    /// Parameters:
    ///          None.
    /// Return value:
    ///          None.
    /// Comments:
    ///          After this function is called, you must not call any PDF
    ///          processing functions.
    ///
    ///          Calling this function does not automatically close other
    ///          objects. It is recommended to close other objects before
    ///          closing the library with this function.
    /// ```
    pub fn FPDF_DestroyLibrary(&self) {
        unsafe { (self.fn_FPDF_DestroyLibrary)() }
    }

    /// C documentation for FPDF_LoadCustomDocument:
    ///
    /// ```text
    /// Function: FPDF_LoadCustomDocument
    ///          Load PDF document from a custom access descriptor.
    /// Parameters:
    ///          pFileAccess -   A structure for accessing the file.
    ///          password    -   Optional password for decrypting the PDF file.
    /// Return value:
    ///          A handle to the loaded document, or NULL on failure.
    /// Comments:
    ///          The application must keep the file resources |pFileAccess| points to
    ///          valid until the returned FPDF_DOCUMENT is closed. |pFileAccess|
    ///          itself does not need to outlive the FPDF_DOCUMENT.
    ///
    ///          The loaded document can be closed with FPDF_CloseDocument().
    ///
    ///          See the comments for FPDF_LoadDocument() regarding the encoding for
    ///          |password|.
    /// Notes:
    ///          If PDFium is built with the XFA module, the application should call
    ///          FPDF_LoadXFA() function after the PDF document loaded to support XFA
    ///          fields defined in the fpdfformfill.h file.
    /// ```
    pub fn FPDF_LoadCustomDocument(
        &self,
        pFileAccess: &mut Box<crate::PdfiumReader>,
        password: &CString,
    ) -> FPDF_DOCUMENT {
        unsafe { (self.fn_FPDF_LoadCustomDocument)(pFileAccess.as_mut().into(), password.as_ptr()) }
    }

    /// C documentation for FPDF_GetLastError:
    ///
    /// ```text
    /// Function: FPDF_GetLastError
    ///          Get last error code when a function fails.
    /// Parameters:
    ///          None.
    /// Return value:
    ///          A 32-bit integer indicating error code as defined above.
    /// Comments:
    ///          If the previous SDK call succeeded, the return value of this
    ///          function is not defined. This function only works in conjunction
    ///          with APIs that mention FPDF_GetLastError() in their documentation.
    /// ```
    pub fn FPDF_GetLastError(&self) -> ::std::os::raw::c_ulong {
        unsafe { (self.fn_FPDF_GetLastError)() }
    }

    /// C documentation for FPDF_GetPageCount:
    ///
    /// ```text
    /// Function: FPDF_GetPageCount
    ///          Get total number of pages in the document.
    /// Parameters:
    ///          document    -   Handle to document. Returned by FPDF_LoadDocument.
    /// Return value:
    ///          Total number of pages in the document.
    /// ```
    pub fn FPDF_GetPageCount(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDF_GetPageCount)(document.into()) }
    }

    /// C documentation for FPDF_LoadPage:
    ///
    /// ```text
    /// Function: FPDF_LoadPage
    ///          Load a page inside the document.
    /// Parameters:
    ///          document    -   Handle to document. Returned by FPDF_LoadDocument
    ///          page_index  -   Index number of the page. 0 for the first page.
    /// Return value:
    ///          A handle to the loaded page, or NULL if page load fails.
    /// Comments:
    ///          The loaded page can be rendered to devices using FPDF_RenderPage.
    ///          The loaded page can be closed using FPDF_ClosePage.
    /// ```
    pub fn FPDF_LoadPage(
        &self,
        document: &PdfiumDocument,
        page_index: i32,
    ) -> PdfiumResult<PdfiumPage> {
        PdfiumPage::new_from_handle(unsafe { (self.fn_FPDF_LoadPage)(document.into(), page_index) })
    }

    /// C documentation for FPDF_RenderPageBitmapWithMatrix:
    ///
    /// ```text
    /// Function: FPDF_RenderPageBitmapWithMatrix
    ///          Render contents of a page to a device independent bitmap.
    /// Parameters:
    ///          bitmap      -   Handle to the device independent bitmap (as the
    ///                          output buffer). The bitmap handle can be created
    ///                          by FPDFBitmap_Create or retrieved by
    ///                          FPDFImageObj_GetBitmap.
    ///          page        -   Handle to the page. Returned by FPDF_LoadPage.
    ///          matrix      -   The transform matrix, which must be invertible.
    ///                          See PDF Reference 1.7, 4.2.2 Common Transformations.
    ///          clipping    -   The rect to clip to in device coords.
    ///          flags       -   0 for normal display, or combination of the Page
    ///                          Rendering flags defined above. With the FPDF_ANNOT
    ///                          flag, it renders all annotations that do not require
    ///                          user-interaction, which are all annotations except
    ///                          widget and popup annotations.
    /// Return value:
    ///          None. Note that behavior is undefined if det of |matrix| is 0.
    /// ```
    pub fn FPDF_RenderPageBitmapWithMatrix(
        &self,
        bitmap: &PdfiumBitmap,
        page: &PdfiumPage,
        matrix: &FS_MATRIX,
        clipping: &FS_RECTF,
        flags: i32,
    ) {
        unsafe {
            (self.fn_FPDF_RenderPageBitmapWithMatrix)(
                bitmap.into(),
                page.into(),
                matrix,
                clipping,
                flags,
            )
        }
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
    pub fn FPDF_ClosePage(&self, page: &PdfiumPage) {
        unsafe { (self.fn_FPDF_ClosePage)(page.into()) }
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
    pub fn FPDF_CloseDocument(&self, document: &PdfiumDocument) {
        unsafe { (self.fn_FPDF_CloseDocument)(document.into()) }
    }

    /// C documentation for FPDFBitmap_Create:
    ///
    /// ```text
    /// Function: FPDFBitmap_Create
    ///          Create a device independent bitmap (FXDIB).
    /// Parameters:
    ///          width       -   The number of pixels in width for the bitmap.
    ///                          Must be greater than 0.
    ///          height      -   The number of pixels in height for the bitmap.
    ///                          Must be greater than 0.
    ///          alpha       -   A flag indicating whether the alpha channel is used.
    ///                          Non-zero for using alpha, zero for not using.
    /// Return value:
    ///          The created bitmap handle, or NULL if a parameter error or out of
    ///          memory.
    /// Comments:
    ///          The bitmap always uses 4 bytes per pixel. The first byte is always
    ///          double word aligned.
    ///
    ///          The byte order is BGRx (the last byte unused if no alpha channel) or
    ///          BGRA.
    ///
    ///          The pixels in a horizontal line are stored side by side, with the
    ///          left most pixel stored first (with lower memory address).
    ///          Each line uses width * 4 bytes.
    ///
    ///          Lines are stored one after another, with the top most line stored
    ///          first. There is no gap between adjacent lines.
    ///
    ///          This function allocates enough memory for holding all pixels in the
    ///          bitmap, but it doesn't initialize the buffer. Applications can use
    ///          FPDFBitmap_FillRect() to fill the bitmap using any color. If the OS
    ///          allows it, this function can allocate up to 4 GB of memory.
    /// ```
    pub fn FPDFBitmap_Create(
        &self,
        width: i32,
        height: i32,
        alpha: i32,
    ) -> PdfiumResult<PdfiumBitmap> {
        PdfiumBitmap::new_from_handle(unsafe { (self.fn_FPDFBitmap_Create)(width, height, alpha) })
    }

    /// C documentation for FPDFBitmap_CreateEx:
    ///
    /// ```text
    /// Function: FPDFBitmap_CreateEx
    ///          Create a device independent bitmap (FXDIB)
    /// Parameters:
    ///          width       -   The number of pixels in width for the bitmap.
    ///                          Must be greater than 0.
    ///          height      -   The number of pixels in height for the bitmap.
    ///                          Must be greater than 0.
    ///          format      -   A number indicating for bitmap format, as defined
    ///                          above.
    ///          first_scan  -   A pointer to the first byte of the first line if
    ///                          using an external buffer. If this parameter is NULL,
    ///                          then a new buffer will be created.
    ///          stride      -   Number of bytes for each scan line. The value must
    ///                          be 0 or greater. When the value is 0,
    ///                          FPDFBitmap_CreateEx() will automatically calculate
    ///                          the appropriate value using |width| and |format|.
    ///                          When using an external buffer, it is recommended for
    ///                          the caller to pass in the value.
    ///                          When not using an external buffer, it is recommended
    ///                          for the caller to pass in 0.
    /// Return value:
    ///          The bitmap handle, or NULL if parameter error or out of memory.
    /// Comments:
    ///          Similar to FPDFBitmap_Create function, but allows for more formats
    ///          and an external buffer is supported. The bitmap created by this
    ///          function can be used in any place that a FPDF_BITMAP handle is
    ///          required.
    ///
    ///          If an external buffer is used, then the caller should destroy the
    ///          buffer. FPDFBitmap_Destroy() will not destroy the buffer.
    ///
    ///          It is recommended to use FPDFBitmap_GetStride() to get the stride
    ///          value.
    /// ```
    pub fn FPDFBitmap_CreateEx(
        &self,
        width: i32,
        height: i32,
        format: i32,
        first_scan: Option<&mut [u8]>,
        stride: i32,
    ) -> PdfiumResult<PdfiumBitmap> {
        PdfiumBitmap::new_from_handle(unsafe {
            (self.fn_FPDFBitmap_CreateEx)(
                width,
                height,
                format,
                to_void_ptr_mut(first_scan),
                stride,
            )
        })
    }

    /// C documentation for FPDFBitmap_GetFormat:
    ///
    /// ```text
    /// Function: FPDFBitmap_GetFormat
    ///          Get the format of the bitmap.
    /// Parameters:
    ///          bitmap      -   Handle to the bitmap. Returned by FPDFBitmap_Create
    ///                          or FPDFImageObj_GetBitmap.
    /// Return value:
    ///          The format of the bitmap.
    /// Comments:
    ///          Only formats supported by FPDFBitmap_CreateEx are supported by this
    ///          function; see the list of such formats above.
    /// ```
    pub fn FPDFBitmap_GetFormat(&self, bitmap: &PdfiumBitmap) -> i32 {
        unsafe { (self.fn_FPDFBitmap_GetFormat)(bitmap.into()) }
    }

    /// C documentation for FPDFBitmap_FillRect:
    ///
    /// ```text
    /// Function: FPDFBitmap_FillRect
    ///          Fill a rectangle in a bitmap.
    /// Parameters:
    ///          bitmap      -   The handle to the bitmap. Returned by
    ///                          FPDFBitmap_Create.
    ///          left        -   The left position. Starting from 0 at the
    ///                          left-most pixel.
    ///          top         -   The top position. Starting from 0 at the
    ///                          top-most line.
    ///          width       -   Width in pixels to be filled.
    ///          height      -   Height in pixels to be filled.
    ///          color       -   A 32-bit value specifing the color, in 8888 ARGB
    ///                          format.
    /// Return value:
    ///          Returns whether the operation succeeded or not.
    /// Comments:
    ///          This function sets the color and (optionally) alpha value in the
    ///          specified region of the bitmap.
    ///
    ///          NOTE: If the alpha channel is used, this function does NOT
    ///          composite the background with the source color, instead the
    ///          background will be replaced by the source color and the alpha.
    ///
    ///          If the alpha channel is not used, the alpha parameter is ignored.
    /// ```
    pub fn FPDFBitmap_FillRect(
        &self,
        bitmap: &PdfiumBitmap,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        color: FPDF_DWORD,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFBitmap_FillRect)(bitmap.into(), left, top, width, height, color)
        })
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
    pub fn FPDFBitmap_GetBuffer(&self, bitmap: &PdfiumBitmap) -> *mut ::std::os::raw::c_void {
        unsafe { (self.fn_FPDFBitmap_GetBuffer)(bitmap.into()) }
    }

    /// C documentation for FPDFBitmap_GetWidth:
    ///
    /// ```text
    /// Function: FPDFBitmap_GetWidth
    ///          Get width of a bitmap.
    /// Parameters:
    ///          bitmap      -   Handle to the bitmap. Returned by FPDFBitmap_Create
    ///                          or FPDFImageObj_GetBitmap.
    /// Return value:
    ///          The width of the bitmap in pixels.
    /// ```
    pub fn FPDFBitmap_GetWidth(&self, bitmap: &PdfiumBitmap) -> i32 {
        unsafe { (self.fn_FPDFBitmap_GetWidth)(bitmap.into()) }
    }

    /// C documentation for FPDFBitmap_GetHeight:
    ///
    /// ```text
    /// Function: FPDFBitmap_GetHeight
    ///          Get height of a bitmap.
    /// Parameters:
    ///          bitmap      -   Handle to the bitmap. Returned by FPDFBitmap_Create
    ///                          or FPDFImageObj_GetBitmap.
    /// Return value:
    ///          The height of the bitmap in pixels.
    /// ```
    pub fn FPDFBitmap_GetHeight(&self, bitmap: &PdfiumBitmap) -> i32 {
        unsafe { (self.fn_FPDFBitmap_GetHeight)(bitmap.into()) }
    }

    /// C documentation for FPDFBitmap_GetStride:
    ///
    /// ```text
    /// Function: FPDFBitmap_GetStride
    ///          Get number of bytes for each line in the bitmap buffer.
    /// Parameters:
    ///          bitmap      -   Handle to the bitmap. Returned by FPDFBitmap_Create
    ///                          or FPDFImageObj_GetBitmap.
    /// Return value:
    ///          The number of bytes for each line in the bitmap buffer.
    /// Comments:
    ///          The stride may be more than width * number of bytes per pixel.
    /// ```
    pub fn FPDFBitmap_GetStride(&self, bitmap: &PdfiumBitmap) -> i32 {
        unsafe { (self.fn_FPDFBitmap_GetStride)(bitmap.into()) }
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
    pub fn FPDFBitmap_Destroy(&self, bitmap: &PdfiumBitmap) {
        unsafe { (self.fn_FPDFBitmap_Destroy)(bitmap.into()) }
    }

    /// C documentation for FPDFDOC_InitFormFillEnvironment:
    ///
    /// ```text
    /// Function: FPDFDOC_InitFormFillEnvironment
    ///       Initialize form fill environment.
    /// Parameters:
    ///       document        -   Handle to document from FPDF_LoadDocument().
    ///       formInfo        -   Pointer to a FPDF_FORMFILLINFO structure.
    /// Return Value:
    ///       Handle to the form fill module, or NULL on failure.
    /// Comments:
    ///       This function should be called before any form fill operation.
    ///       The FPDF_FORMFILLINFO passed in via |formInfo| must remain valid until
    ///       the returned FPDF_FORMHANDLE is closed.
    /// ```
    pub fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: &PdfiumDocument,
        formInfo: &mut PdfiumFormFillInfo,
    ) -> PdfiumResult<PdfiumForm> {
        let mut formInfo: FPDF_FORMFILLINFO = formInfo.into();
        PdfiumForm::new_from_handle(unsafe {
            (self.fn_FPDFDOC_InitFormFillEnvironment)(document.into(), &mut formInfo)
        })
    }

    /// C documentation for FPDFPage_HasFormFieldAtPoint:
    ///
    /// ```text
    /// Function: FPDFPage_HasFormFieldAtPoint
    ///     Get the form field type by point.
    /// Parameters:
    ///     hHandle     -   Handle to the form fill module. Returned by
    ///                     FPDFDOC_InitFormFillEnvironment().
    ///     page        -   Handle to the page. Returned by FPDF_LoadPage().
    ///     page_x      -   X position in PDF "user space".
    ///     page_y      -   Y position in PDF "user space".
    /// Return Value:
    ///     Return the type of the form field; -1 indicates no field.
    ///     See field types above.
    /// ```
    pub fn FPDFPage_HasFormFieldAtPoint(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        page_x: f64,
        page_y: f64,
    ) -> i32 {
        unsafe {
            (self.fn_FPDFPage_HasFormFieldAtPoint)(hHandle.into(), page.into(), page_x, page_y)
        }
    }

    /// C documentation for FPDFPage_FormFieldZOrderAtPoint:
    ///
    /// ```text
    /// Function: FPDFPage_FormFieldZOrderAtPoint
    ///     Get the form field z-order by point.
    /// Parameters:
    ///     hHandle     -   Handle to the form fill module. Returned by
    ///                     FPDFDOC_InitFormFillEnvironment().
    ///     page        -   Handle to the page. Returned by FPDF_LoadPage().
    ///     page_x      -   X position in PDF "user space".
    ///     page_y      -   Y position in PDF "user space".
    /// Return Value:
    ///     Return the z-order of the form field; -1 indicates no field.
    ///     Higher numbers are closer to the front.
    /// ```
    pub fn FPDFPage_FormFieldZOrderAtPoint(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        page_x: f64,
        page_y: f64,
    ) -> i32 {
        unsafe {
            (self.fn_FPDFPage_FormFieldZOrderAtPoint)(hHandle.into(), page.into(), page_x, page_y)
        }
    }

    /// C documentation for FPDFPage_CreateAnnot:
    ///
    /// ```text
    /// Experimental API.
    /// Create an annotation in |page| of the subtype |subtype|. If the specified
    /// subtype is illegal or unsupported, then a new annotation will not be created.
    /// Must call FPDFPage_CloseAnnot() when the annotation returned by this
    /// function is no longer needed.
    ///
    ///   page      - handle to a page.
    ///   subtype   - the subtype of the new annotation.
    ///
    /// Returns a handle to the new annotation object, or NULL on failure.
    /// ```
    pub fn FPDFPage_CreateAnnot(
        &self,
        page: &PdfiumPage,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> PdfiumResult<PdfiumAnnotation> {
        PdfiumAnnotation::new_from_handle(unsafe {
            (self.fn_FPDFPage_CreateAnnot)(page.into(), subtype)
        })
    }

    /// C documentation for FPDFPage_GetAnnotCount:
    ///
    /// ```text
    /// Experimental API.
    /// Get the number of annotations in |page|.
    ///
    ///   page   - handle to a page.
    ///
    /// Returns the number of annotations in |page|.
    /// ```
    pub fn FPDFPage_GetAnnotCount(&self, page: &PdfiumPage) -> i32 {
        unsafe { (self.fn_FPDFPage_GetAnnotCount)(page.into()) }
    }

    /// C documentation for FPDFPage_GetAnnot:
    ///
    /// ```text
    /// Experimental API.
    /// Get annotation in |page| at |index|. Must call FPDFPage_CloseAnnot() when the
    /// annotation returned by this function is no longer needed.
    ///
    ///   page  - handle to a page.
    ///   index - the index of the annotation.
    ///
    /// Returns a handle to the annotation object, or NULL on failure.
    /// ```
    pub fn FPDFPage_GetAnnot(
        &self,
        page: &PdfiumPage,
        index: i32,
    ) -> PdfiumResult<PdfiumAnnotation> {
        PdfiumAnnotation::new_from_handle(unsafe {
            (self.fn_FPDFPage_GetAnnot)(page.into(), index)
        })
    }

    /// C documentation for FPDFPage_GetAnnotIndex:
    ///
    /// ```text
    /// Experimental API.
    /// Get the index of |annot| in |page|. This is the opposite of
    /// FPDFPage_GetAnnot().
    ///
    ///   page  - handle to the page that the annotation is on.
    ///   annot - handle to an annotation.
    ///
    /// Returns the index of |annot|, or -1 on failure.
    /// ```
    pub fn FPDFPage_GetAnnotIndex(&self, page: &PdfiumPage, annot: &PdfiumAnnotation) -> i32 {
        unsafe { (self.fn_FPDFPage_GetAnnotIndex)(page.into(), annot.into()) }
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
    pub fn FPDFPage_CloseAnnot(&self, annot: &PdfiumAnnotation) {
        unsafe { (self.fn_FPDFPage_CloseAnnot)(annot.into()) }
    }

    /// C documentation for FPDFPage_RemoveAnnot:
    ///
    /// ```text
    /// Experimental API.
    /// Remove the annotation in |page| at |index|.
    ///
    ///   page  - handle to a page.
    ///   index - the index of the annotation.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFPage_RemoveAnnot(&self, page: &PdfiumPage, index: i32) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_RemoveAnnot)(page.into(), index) })
    }

    /// C documentation for FPDFPage_New:
    ///
    /// ```text
    /// Create a new PDF page.
    ///
    ///   document   - handle to document.
    ///   page_index - suggested 0-based index of the page to create. If it is larger
    ///                than document's current last index(L), the created page index
    ///                is the next available index -- L+1.
    ///   width      - the page width in points.
    ///   height     - the page height in points.
    ///
    /// Returns the handle to the new page or NULL on failure.
    ///
    /// The page should be closed with FPDF_ClosePage() when finished as
    /// with any other page in the document.
    /// ```
    pub fn FPDFPage_New(
        &self,
        document: &PdfiumDocument,
        page_index: i32,
        width: f64,
        height: f64,
    ) -> PdfiumResult<PdfiumPage> {
        PdfiumPage::new_from_handle(unsafe {
            (self.fn_FPDFPage_New)(document.into(), page_index, width, height)
        })
    }

    /// C documentation for FPDFPage_Delete:
    ///
    /// ```text
    /// Delete the page at |page_index|.
    ///
    ///   document   - handle to document.
    ///   page_index - the index of the page to delete.
    /// ```
    pub fn FPDFPage_Delete(&self, document: &PdfiumDocument, page_index: i32) {
        unsafe { (self.fn_FPDFPage_Delete)(document.into(), page_index) }
    }

    /// C documentation for FPDFPage_GetRotation:
    ///
    /// ```text
    /// Get the rotation of |page|.
    ///
    ///   page - handle to a page
    ///
    /// Returns one of the following indicating the page rotation:
    ///   0 - No rotation.
    ///   1 - Rotated 90 degrees clockwise.
    ///   2 - Rotated 180 degrees clockwise.
    ///   3 - Rotated 270 degrees clockwise.
    /// ```
    pub fn FPDFPage_GetRotation(&self, page: &PdfiumPage) -> i32 {
        unsafe { (self.fn_FPDFPage_GetRotation)(page.into()) }
    }

    /// C documentation for FPDFPage_SetRotation:
    ///
    /// ```text
    /// Set rotation for |page|.
    ///
    ///   page   - handle to a page.
    ///   rotate - the rotation value, one of:
    ///              0 - No rotation.
    ///              1 - Rotated 90 degrees clockwise.
    ///              2 - Rotated 180 degrees clockwise.
    ///              3 - Rotated 270 degrees clockwise.
    /// ```
    pub fn FPDFPage_SetRotation(&self, page: &PdfiumPage, rotate: i32) {
        unsafe { (self.fn_FPDFPage_SetRotation)(page.into(), rotate) }
    }

    /// C documentation for FPDFPage_InsertObject:
    ///
    /// ```text
    /// Insert |page_object| into |page|.
    ///
    ///   page        - handle to a page
    ///   page_object - handle to a page object. The |page_object| will be
    ///                 automatically freed.
    /// ```
    pub fn FPDFPage_InsertObject(&self, page: &PdfiumPage, page_object: &PdfiumPageObject) {
        unsafe { (self.fn_FPDFPage_InsertObject)(page.into(), page_object.into()) }
    }

    /// C documentation for FPDFPage_RemoveObject:
    ///
    /// ```text
    /// Experimental API.
    /// Remove |page_object| from |page|.
    ///
    ///   page        - handle to a page
    ///   page_object - handle to a page object to be removed.
    ///
    /// Returns TRUE on success.
    ///
    /// Ownership is transferred to the caller. Call FPDFPageObj_Destroy() to free
    /// it.
    /// Note that when removing a |page_object| of type FPDF_PAGEOBJ_TEXT, all
    /// FPDF_TEXTPAGE handles for |page| are no longer valid.
    /// ```
    pub fn FPDFPage_RemoveObject(
        &self,
        page: &PdfiumPage,
        page_object: &PdfiumPageObject,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_RemoveObject)(page.into(), page_object.into()) })
    }

    /// C documentation for FPDFPage_CountObjects:
    ///
    /// ```text
    /// Get number of page objects inside |page|.
    ///
    ///   page - handle to a page.
    ///
    /// Returns the number of objects in |page|.
    /// ```
    pub fn FPDFPage_CountObjects(&self, page: &PdfiumPage) -> i32 {
        unsafe { (self.fn_FPDFPage_CountObjects)(page.into()) }
    }

    /// C documentation for FPDFPage_GetObject:
    ///
    /// ```text
    /// Get object in |page| at |index|.
    ///
    ///   page  - handle to a page.
    ///   index - the index of a page object.
    ///
    /// Returns the handle to the page object, or NULL on failed.
    /// ```
    pub fn FPDFPage_GetObject(
        &self,
        page: &PdfiumPage,
        index: i32,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDFPage_GetObject)(page.into(), index)
        })
    }

    /// C documentation for FPDFPage_HasTransparency:
    ///
    /// ```text
    /// Checks if |page| contains transparency.
    ///
    ///   page - handle to a page.
    ///
    /// Returns TRUE if |page| contains transparency.
    /// ```
    pub fn FPDFPage_HasTransparency(&self, page: &PdfiumPage) -> i32 {
        unsafe { (self.fn_FPDFPage_HasTransparency)(page.into()) }
    }

    /// C documentation for FPDFPage_GenerateContent:
    ///
    /// ```text
    /// Generate the content of |page|.
    ///
    ///   page - handle to a page.
    ///
    /// Returns TRUE on success.
    ///
    /// Before you save the page to a file, or reload the page, you must call
    /// |FPDFPage_GenerateContent| or any changes to |page| will be lost.
    /// ```
    pub fn FPDFPage_GenerateContent(&self, page: &PdfiumPage) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_GenerateContent)(page.into()) })
    }

    /// C documentation for FPDFPage_TransformAnnots:
    ///
    /// ```text
    /// Transform all annotations in |page|.
    ///
    ///   page - handle to a page.
    ///   a    - matrix value.
    ///   b    - matrix value.
    ///   c    - matrix value.
    ///   d    - matrix value.
    ///   e    - matrix value.
    ///   f    - matrix value.
    ///
    /// The matrix is composed as:
    ///   |a c e|
    ///   |b d f|
    /// and can be used to scale, rotate, shear and translate the |page| annotations.
    /// ```
    pub fn FPDFPage_TransformAnnots(
        &self,
        page: &PdfiumPage,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) {
        unsafe { (self.fn_FPDFPage_TransformAnnots)(page.into(), a, b, c, d, e, f) }
    }

    /// C documentation for FPDFPage_Flatten:
    ///
    /// ```text
    /// Flatten annotations and form fields into the page contents.
    ///
    ///   page  - handle to the page.
    ///   nFlag - One of the |FLAT_*| values denoting the page usage.
    ///
    /// Returns one of the |FLATTEN_*| values.
    ///
    /// Currently, all failures return |FLATTEN_FAIL| with no indication of the
    /// cause.
    /// ```
    pub fn FPDFPage_Flatten(&self, page: &PdfiumPage, nFlag: i32) -> i32 {
        unsafe { (self.fn_FPDFPage_Flatten)(page.into(), nFlag) }
    }

    /// C documentation for FPDF_NewXObjectFromPage:
    ///
    /// ```text
    /// Experimental API.
    /// Create a template to generate form xobjects from |src_doc|'s page at
    /// |src_page_index|, for use in |dest_doc|.
    ///
    /// Returns a handle on success, or NULL on failure. Caller owns the newly
    /// created object.
    /// ```
    pub fn FPDF_NewXObjectFromPage(
        &self,
        dest_doc: &PdfiumDocument,
        src_doc: &PdfiumDocument,
        src_page_index: i32,
    ) -> PdfiumResult<PdfiumXObject> {
        PdfiumXObject::new_from_handle(unsafe {
            (self.fn_FPDF_NewXObjectFromPage)(dest_doc.into(), src_doc.into(), src_page_index)
        })
    }

    /// C documentation for FPDF_CloseXObject:
    ///
    /// ```text
    /// Experimental API.
    /// Close an FPDF_XOBJECT handle created by FPDF_NewXObjectFromPage().
    /// FPDF_PAGEOBJECTs created from the FPDF_XOBJECT handle are not affected.
    /// ```
    pub fn FPDF_CloseXObject(&self, xobject: &PdfiumXObject) {
        unsafe { (self.fn_FPDF_CloseXObject)(xobject.into()) }
    }

    /// C documentation for FPDFPage_GetDecodedThumbnailData:
    ///
    /// ```text
    /// Experimental API.
    /// Gets the decoded data from the thumbnail of |page| if it exists.
    /// This only modifies |buffer| if |buflen| less than or equal to the
    /// size of the decoded data. Returns the size of the decoded
    /// data or 0 if thumbnail DNE. Optional, pass null to just retrieve
    /// the size of the buffer needed.
    ///
    ///   page    - handle to a page.
    ///   buffer  - buffer for holding the decoded image data.
    ///   buflen  - length of the buffer in bytes.
    /// ```
    pub fn FPDFPage_GetDecodedThumbnailData(
        &self,
        page: &PdfiumPage,
        buffer: Option<&mut [u8]>,
        buflen: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        unsafe {
            (self.fn_FPDFPage_GetDecodedThumbnailData)(page.into(), to_void_ptr_mut(buffer), buflen)
        }
    }

    /// C documentation for FPDFPage_GetRawThumbnailData:
    ///
    /// ```text
    /// Experimental API.
    /// Gets the raw data from the thumbnail of |page| if it exists.
    /// This only modifies |buffer| if |buflen| is less than or equal to
    /// the size of the raw data. Returns the size of the raw data or 0
    /// if thumbnail DNE. Optional, pass null to just retrieve the size
    /// of the buffer needed.
    ///
    ///   page    - handle to a page.
    ///   buffer  - buffer for holding the raw image data.
    ///   buflen  - length of the buffer in bytes.
    /// ```
    pub fn FPDFPage_GetRawThumbnailData(
        &self,
        page: &PdfiumPage,
        buffer: Option<&mut [u8]>,
        buflen: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        unsafe {
            (self.fn_FPDFPage_GetRawThumbnailData)(page.into(), to_void_ptr_mut(buffer), buflen)
        }
    }

    /// C documentation for FPDFPage_GetThumbnailAsBitmap:
    ///
    /// ```text
    /// Experimental API.
    /// Returns the thumbnail of |page| as a FPDF_BITMAP. Returns a nullptr
    /// if unable to access the thumbnail's stream.
    ///
    ///   page - handle to a page.
    /// ```
    pub fn FPDFPage_GetThumbnailAsBitmap(&self, page: &PdfiumPage) -> PdfiumResult<PdfiumBitmap> {
        PdfiumBitmap::new_from_handle(unsafe {
            (self.fn_FPDFPage_GetThumbnailAsBitmap)(page.into())
        })
    }

    /// C documentation for FPDFPage_SetMediaBox:
    ///
    /// ```text
    /// Set "MediaBox" entry to the page dictionary.
    ///
    /// page   - Handle to a page.
    /// left   - The left of the rectangle.
    /// bottom - The bottom of the rectangle.
    /// right  - The right of the rectangle.
    /// top    - The top of the rectangle.
    /// ```
    pub fn FPDFPage_SetMediaBox(
        &self,
        page: &PdfiumPage,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) {
        unsafe { (self.fn_FPDFPage_SetMediaBox)(page.into(), left, bottom, right, top) }
    }

    /// C documentation for FPDFPage_SetCropBox:
    ///
    /// ```text
    /// Set "CropBox" entry to the page dictionary.
    ///
    /// page   - Handle to a page.
    /// left   - The left of the rectangle.
    /// bottom - The bottom of the rectangle.
    /// right  - The right of the rectangle.
    /// top    - The top of the rectangle.
    /// ```
    pub fn FPDFPage_SetCropBox(
        &self,
        page: &PdfiumPage,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) {
        unsafe { (self.fn_FPDFPage_SetCropBox)(page.into(), left, bottom, right, top) }
    }

    /// C documentation for FPDFPage_SetBleedBox:
    ///
    /// ```text
    /// Set "BleedBox" entry to the page dictionary.
    ///
    /// page   - Handle to a page.
    /// left   - The left of the rectangle.
    /// bottom - The bottom of the rectangle.
    /// right  - The right of the rectangle.
    /// top    - The top of the rectangle.
    /// ```
    pub fn FPDFPage_SetBleedBox(
        &self,
        page: &PdfiumPage,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) {
        unsafe { (self.fn_FPDFPage_SetBleedBox)(page.into(), left, bottom, right, top) }
    }

    /// C documentation for FPDFPage_SetTrimBox:
    ///
    /// ```text
    /// Set "TrimBox" entry to the page dictionary.
    ///
    /// page   - Handle to a page.
    /// left   - The left of the rectangle.
    /// bottom - The bottom of the rectangle.
    /// right  - The right of the rectangle.
    /// top    - The top of the rectangle.
    /// ```
    pub fn FPDFPage_SetTrimBox(
        &self,
        page: &PdfiumPage,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) {
        unsafe { (self.fn_FPDFPage_SetTrimBox)(page.into(), left, bottom, right, top) }
    }

    /// C documentation for FPDFPage_SetArtBox:
    ///
    /// ```text
    /// Set "ArtBox" entry to the page dictionary.
    ///
    /// page   - Handle to a page.
    /// left   - The left of the rectangle.
    /// bottom - The bottom of the rectangle.
    /// right  - The right of the rectangle.
    /// top    - The top of the rectangle.
    /// ```
    pub fn FPDFPage_SetArtBox(
        &self,
        page: &PdfiumPage,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) {
        unsafe { (self.fn_FPDFPage_SetArtBox)(page.into(), left, bottom, right, top) }
    }

    /// C documentation for FPDFPage_GetMediaBox:
    ///
    /// ```text
    /// Get "MediaBox" entry from the page dictionary.
    ///
    /// page   - Handle to a page.
    /// left   - Pointer to a float value receiving the left of the rectangle.
    /// bottom - Pointer to a float value receiving the bottom of the rectangle.
    /// right  - Pointer to a float value receiving the right of the rectangle.
    /// top    - Pointer to a float value receiving the top of the rectangle.
    ///
    /// On success, return true and write to the out parameters. Otherwise return
    /// false and leave the out parameters unmodified.
    /// ```
    pub fn FPDFPage_GetMediaBox(
        &self,
        page: &PdfiumPage,
        left: &mut f32,
        bottom: &mut f32,
        right: &mut f32,
        top: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_GetMediaBox)(page.into(), left, bottom, right, top) })
    }

    /// C documentation for FPDFPage_GetCropBox:
    ///
    /// ```text
    /// Get "CropBox" entry from the page dictionary.
    ///
    /// page   - Handle to a page.
    /// left   - Pointer to a float value receiving the left of the rectangle.
    /// bottom - Pointer to a float value receiving the bottom of the rectangle.
    /// right  - Pointer to a float value receiving the right of the rectangle.
    /// top    - Pointer to a float value receiving the top of the rectangle.
    ///
    /// On success, return true and write to the out parameters. Otherwise return
    /// false and leave the out parameters unmodified.
    /// ```
    pub fn FPDFPage_GetCropBox(
        &self,
        page: &PdfiumPage,
        left: &mut f32,
        bottom: &mut f32,
        right: &mut f32,
        top: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_GetCropBox)(page.into(), left, bottom, right, top) })
    }

    /// C documentation for FPDFPage_GetBleedBox:
    ///
    /// ```text
    /// Get "BleedBox" entry from the page dictionary.
    ///
    /// page   - Handle to a page.
    /// left   - Pointer to a float value receiving the left of the rectangle.
    /// bottom - Pointer to a float value receiving the bottom of the rectangle.
    /// right  - Pointer to a float value receiving the right of the rectangle.
    /// top    - Pointer to a float value receiving the top of the rectangle.
    ///
    /// On success, return true and write to the out parameters. Otherwise return
    /// false and leave the out parameters unmodified.
    /// ```
    pub fn FPDFPage_GetBleedBox(
        &self,
        page: &PdfiumPage,
        left: &mut f32,
        bottom: &mut f32,
        right: &mut f32,
        top: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_GetBleedBox)(page.into(), left, bottom, right, top) })
    }

    /// C documentation for FPDFPage_GetTrimBox:
    ///
    /// ```text
    /// Get "TrimBox" entry from the page dictionary.
    ///
    /// page   - Handle to a page.
    /// left   - Pointer to a float value receiving the left of the rectangle.
    /// bottom - Pointer to a float value receiving the bottom of the rectangle.
    /// right  - Pointer to a float value receiving the right of the rectangle.
    /// top    - Pointer to a float value receiving the top of the rectangle.
    ///
    /// On success, return true and write to the out parameters. Otherwise return
    /// false and leave the out parameters unmodified.
    /// ```
    pub fn FPDFPage_GetTrimBox(
        &self,
        page: &PdfiumPage,
        left: &mut f32,
        bottom: &mut f32,
        right: &mut f32,
        top: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_GetTrimBox)(page.into(), left, bottom, right, top) })
    }

    /// C documentation for FPDFPage_GetArtBox:
    ///
    /// ```text
    /// Get "ArtBox" entry from the page dictionary.
    ///
    /// page   - Handle to a page.
    /// left   - Pointer to a float value receiving the left of the rectangle.
    /// bottom - Pointer to a float value receiving the bottom of the rectangle.
    /// right  - Pointer to a float value receiving the right of the rectangle.
    /// top    - Pointer to a float value receiving the top of the rectangle.
    ///
    /// On success, return true and write to the out parameters. Otherwise return
    /// false and leave the out parameters unmodified.
    /// ```
    pub fn FPDFPage_GetArtBox(
        &self,
        page: &PdfiumPage,
        left: &mut f32,
        bottom: &mut f32,
        right: &mut f32,
        top: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_GetArtBox)(page.into(), left, bottom, right, top) })
    }

    /// C documentation for FPDFPage_TransFormWithClip:
    ///
    /// ```text
    /// Apply transforms to |page|.
    ///
    /// If |matrix| is provided it will be applied to transform the page.
    /// If |clipRect| is provided it will be used to clip the resulting page.
    /// If neither |matrix| or |clipRect| are provided this method returns |false|.
    /// Returns |true| if transforms are applied.
    ///
    /// This function will transform the whole page, and would take effect to all the
    /// objects in the page.
    ///
    /// page        - Page handle.
    /// matrix      - Transform matrix.
    /// clipRect    - Clipping rectangle.
    /// ```
    pub fn FPDFPage_TransFormWithClip(
        &self,
        page: &PdfiumPage,
        matrix: &FS_MATRIX,
        clipRect: &FS_RECTF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_TransFormWithClip)(page.into(), matrix, clipRect) })
    }

    /// C documentation for FPDF_CreateClipPath:
    ///
    /// ```text
    /// Create a new clip path, with a rectangle inserted.
    ///
    /// Caller takes ownership of the returned FPDF_CLIPPATH. It should be freed with
    /// FPDF_DestroyClipPath().
    ///
    /// left   - The left of the clip box.
    /// bottom - The bottom of the clip box.
    /// right  - The right of the clip box.
    /// top    - The top of the clip box.
    /// ```
    pub fn FPDF_CreateClipPath(
        &self,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) -> PdfiumResult<PdfiumClipPath> {
        PdfiumClipPath::new_from_handle(unsafe {
            (self.fn_FPDF_CreateClipPath)(left, bottom, right, top)
        })
    }

    /// C documentation for FPDF_DestroyClipPath:
    ///
    /// ```text
    /// Destroy the clip path.
    ///
    /// clipPath - A handle to the clip path. It will be invalid after this call.
    /// ```
    pub fn FPDF_DestroyClipPath(&self, clipPath: &PdfiumClipPath) {
        unsafe { (self.fn_FPDF_DestroyClipPath)(clipPath.into()) }
    }

    /// C documentation for FPDFPage_InsertClipPath:
    ///
    /// ```text
    /// Clip the page content, the page content that outside the clipping region
    /// become invisible.
    ///
    /// A clip path will be inserted before the page content stream or content array.
    /// In this way, the page content will be clipped by this clip path.
    ///
    /// page        - A page handle.
    /// clipPath    - A handle to the clip path. (Does not take ownership.)
    /// ```
    pub fn FPDFPage_InsertClipPath(&self, page: &PdfiumPage, clipPath: &PdfiumClipPath) {
        unsafe { (self.fn_FPDFPage_InsertClipPath)(page.into(), clipPath.into()) }
    }
}

fn to_void_ptr(data: Option<&[u8]>) -> *const c_void {
    match data {
        Some(slice) => slice.as_ptr() as *const c_void,
        None => std::ptr::null(),
    }
}

fn to_void_ptr_mut(data: Option<&mut [u8]>) -> *mut c_void {
    match data {
        Some(slice) => slice.as_ptr() as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

fn to_result(b: FPDF_BOOL) -> PdfiumResult<()> {
    if b == 0 {
        Err(PdfiumError::InvokationFailed)
    } else {
        Ok(())
    }
}
