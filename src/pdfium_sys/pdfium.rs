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
    pub fn FPDF_InitLibrary(&self) {
        unsafe { (self.fn_FPDF_InitLibrary)() }
    }

    pub fn FPDF_LoadCustomDocument(
        &self,
        pFileAccess: &mut Box<crate::PdfiumReader>,
        password: &CString,
    ) -> FPDF_DOCUMENT {
        unsafe { (self.fn_FPDF_LoadCustomDocument)(pFileAccess.as_mut().into(), password.as_ptr()) }
    }

    pub fn FPDF_GetLastError(&self) -> ::std::os::raw::c_ulong {
        unsafe { (self.fn_FPDF_GetLastError)() }
    }

    pub fn FPDF_GetPageCount(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDF_GetPageCount)(document.into()) }
    }

    pub fn FPDF_LoadPage(
        &self,
        document: &PdfiumDocument,
        page_index: i32,
    ) -> PdfiumResult<PdfiumPage> {
        PdfiumPage::new_from_handle(unsafe { (self.fn_FPDF_LoadPage)(document.into(), page_index) })
    }

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

    pub fn FPDF_ClosePage(&self, page: &PdfiumPage) {
        unsafe { (self.fn_FPDF_ClosePage)(page.into()) }
    }

    pub fn FPDF_CloseDocument(&self, document: &PdfiumDocument) {
        unsafe { (self.fn_FPDF_CloseDocument)(document.into()) }
    }

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

    pub fn FPDFBitmap_GetFormat(&self, bitmap: &PdfiumBitmap) -> i32 {
        unsafe { (self.fn_FPDFBitmap_GetFormat)(bitmap.into()) }
    }

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

    pub fn FPDFBitmap_GetBuffer(&self, bitmap: &PdfiumBitmap) -> *mut ::std::os::raw::c_void {
        unsafe { (self.fn_FPDFBitmap_GetBuffer)(bitmap.into()) }
    }

    pub fn FPDFBitmap_GetWidth(&self, bitmap: &PdfiumBitmap) -> i32 {
        unsafe { (self.fn_FPDFBitmap_GetWidth)(bitmap.into()) }
    }

    pub fn FPDFBitmap_GetHeight(&self, bitmap: &PdfiumBitmap) -> i32 {
        unsafe { (self.fn_FPDFBitmap_GetHeight)(bitmap.into()) }
    }

    pub fn FPDFBitmap_GetStride(&self, bitmap: &PdfiumBitmap) -> i32 {
        unsafe { (self.fn_FPDFBitmap_GetStride)(bitmap.into()) }
    }

    pub fn FPDFBitmap_Destroy(&self, bitmap: &PdfiumBitmap) {
        unsafe { (self.fn_FPDFBitmap_Destroy)(bitmap.into()) }
    }

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
