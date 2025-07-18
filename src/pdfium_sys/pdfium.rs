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

#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::CString;
use std::os::raw::{c_int, c_ulong, c_void};

use super::lib_get;
use crate::{
    PdfiumBitmap, PdfiumDocument, PdfiumError, PdfiumMatrix, PdfiumPage, PdfiumRect, PdfiumResult,
    pdfium_types::*,
};
use libloading::Library;

/// # Low level Rust bindings to the PDFium C library
///
/// These functions aim to fully follow the original API, while also providing a safe interface by replacing
/// all pointer based parameters with safe Rust replacements. The same applies to the function results
/// whenever possible.
#[allow(non_snake_case)]
pub struct PdfiumBindings {
    fn_FPDF_InitLibrary: unsafe extern "C" fn(),
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
    fn_FPDFPage_GetMediaBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    lib: Library,
}

impl PdfiumBindings {
    pub fn new(lib: Library) -> Result<Self, PdfiumError> {
        Ok(Self {
            fn_FPDF_InitLibrary: *(lib_get(&lib, "FPDF_InitLibrary")?),
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
            fn_FPDFBitmap_CreateEx: *(lib_get(&lib, "FPDFBitmap_CreateEx")?),
            fn_FPDFBitmap_GetFormat: *(lib_get(&lib, "FPDFBitmap_GetFormat")?),
            fn_FPDFBitmap_FillRect: *(lib_get(&lib, "FPDFBitmap_FillRect")?),
            fn_FPDFBitmap_GetBuffer: *(lib_get(&lib, "FPDFBitmap_GetBuffer")?),
            fn_FPDFBitmap_GetWidth: *(lib_get(&lib, "FPDFBitmap_GetWidth")?),
            fn_FPDFBitmap_GetHeight: *(lib_get(&lib, "FPDFBitmap_GetHeight")?),
            fn_FPDFBitmap_GetStride: *(lib_get(&lib, "FPDFBitmap_GetStride")?),
            fn_FPDFBitmap_Destroy: *(lib_get(&lib, "FPDFBitmap_Destroy")?),
            fn_FPDFPage_GetMediaBox: *(lib_get(&lib, "FPDFPage_GetMediaBox")?),
            lib,
        })
    }
}

impl PdfiumBindings {
    /// # **FPDF_InitLibrary** *(original C documentation)*
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

    /// # **FPDF_LoadCustomDocument** *(original C documentation)*
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

    /// # **FPDF_GetLastError** *(original C documentation)*
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

    /// # **FPDF_GetPageCount** *(original C documentation)*
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

    /// # **FPDF_LoadPage** *(original C documentation)*
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

    /// # **FPDF_RenderPageBitmapWithMatrix** *(original C documentation)*
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
        matrix: &PdfiumMatrix,
        clipping: &PdfiumRect,
        flags: i32,
    ) {
        let matrix: FS_MATRIX = matrix.into();
        let clipping: FS_RECTF = clipping.into();
        unsafe {
            (self.fn_FPDF_RenderPageBitmapWithMatrix)(
                bitmap.into(),
                page.into(),
                &matrix,
                &clipping,
                flags,
            )
        }
    }

    /// # **FPDF_ClosePage** *(original C documentation)*
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

    /// # **FPDF_CloseDocument** *(original C documentation)*
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

    /// # **FPDFBitmap_CreateEx** *(original C documentation)*
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

    /// # **FPDFBitmap_GetFormat** *(original C documentation)*
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

    /// # **FPDFBitmap_FillRect** *(original C documentation)*
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
    ) -> FPDF_BOOL {
        unsafe { (self.fn_FPDFBitmap_FillRect)(bitmap.into(), left, top, width, height, color) }
    }

    /// # **FPDFBitmap_GetBuffer** *(original C documentation)*
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

    /// # **FPDFBitmap_GetWidth** *(original C documentation)*
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

    /// # **FPDFBitmap_GetHeight** *(original C documentation)*
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

    /// # **FPDFBitmap_GetStride** *(original C documentation)*
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

    /// # **FPDFBitmap_Destroy** *(original C documentation)*
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

    /// # **FPDFPage_GetMediaBox** *(original C documentation)*
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
    ) -> FPDF_BOOL {
        unsafe { (self.fn_FPDFPage_GetMediaBox)(page.into(), left, bottom, right, top) }
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
