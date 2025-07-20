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
    PdfiumAction, PdfiumAnnotation, PdfiumAttachment, PdfiumAvailability, PdfiumBitmap,
    PdfiumBookmark, PdfiumClipPath, PdfiumDestination, PdfiumDocument, PdfiumError, PdfiumForm,
    PdfiumJavascriptAction, PdfiumPage, PdfiumPageObject, PdfiumPageRange, PdfiumResult,
    PdfiumStructElement, PdfiumStructElementAttr, PdfiumStructElementAttrValue, PdfiumStructTree,
    PdfiumSystemFontInfo, PdfiumXObject, pdfium_types::*,
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
    fn_FPDF_SetSandBoxPolicy: unsafe extern "C" fn(policy: FPDF_DWORD, enable: FPDF_BOOL),
    fn_FPDF_LoadCustomDocument: unsafe extern "C" fn(
        pFileAccess: *mut FPDF_FILEACCESS,
        password: FPDF_BYTESTRING,
    ) -> FPDF_DOCUMENT,
    fn_FPDF_GetFileVersion:
        unsafe extern "C" fn(doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL,
    fn_FPDF_GetLastError: unsafe extern "C" fn() -> c_ulong,
    fn_FPDF_DocumentHasValidCrossReferenceTable:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL,
    fn_FPDF_GetTrailerEnds: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        buffer: *mut c_uint,
        length: c_ulong,
    ) -> c_ulong,
    fn_FPDF_GetDocPermissions: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_ulong,
    fn_FPDF_GetDocUserPermissions: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_ulong,
    fn_FPDF_GetSecurityHandlerRevision: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    fn_FPDF_GetPageCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    fn_FPDF_LoadPage: unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE,
    fn_FPDF_GetPageWidthF: unsafe extern "C" fn(page: FPDF_PAGE) -> f32,
    fn_FPDF_GetPageWidth: unsafe extern "C" fn(page: FPDF_PAGE) -> f64,
    fn_FPDF_GetPageHeightF: unsafe extern "C" fn(page: FPDF_PAGE) -> f32,
    fn_FPDF_GetPageHeight: unsafe extern "C" fn(page: FPDF_PAGE) -> f64,
    fn_FPDF_GetPageBoundingBox:
        unsafe extern "C" fn(page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL,
    fn_FPDF_GetPageSizeByIndexF: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        size: *mut FS_SIZEF,
    ) -> FPDF_BOOL,
    fn_FPDF_GetPageSizeByIndex: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        width: *mut f64,
        height: *mut f64,
    ) -> c_int,
    fn_FPDF_RenderPageBitmap: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    ),
    fn_FPDF_RenderPageBitmapWithMatrix: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipping: *const FS_RECTF,
        flags: c_int,
    ),
    fn_FPDF_ClosePage: unsafe extern "C" fn(page: FPDF_PAGE),
    fn_FPDF_CloseDocument: unsafe extern "C" fn(document: FPDF_DOCUMENT),
    fn_FPDF_DeviceToPage: unsafe extern "C" fn(
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        device_x: c_int,
        device_y: c_int,
        page_x: *mut f64,
        page_y: *mut f64,
    ) -> FPDF_BOOL,
    fn_FPDF_PageToDevice: unsafe extern "C" fn(
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        page_x: f64,
        page_y: f64,
        device_x: *mut c_int,
        device_y: *mut c_int,
    ) -> FPDF_BOOL,
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
    fn_FPDF_VIEWERREF_GetPrintScaling: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL,
    fn_FPDF_VIEWERREF_GetNumCopies: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    fn_FPDF_VIEWERREF_GetPrintPageRange:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_PAGERANGE,
    fn_FPDF_VIEWERREF_GetPrintPageRangeCount:
        unsafe extern "C" fn(pagerange: FPDF_PAGERANGE) -> usize,
    fn_FPDF_VIEWERREF_GetPrintPageRangeElement:
        unsafe extern "C" fn(pagerange: FPDF_PAGERANGE, index: usize) -> c_int,
    fn_FPDF_VIEWERREF_GetDuplex: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_DUPLEXTYPE,
    fn_FPDF_VIEWERREF_GetName: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        key: FPDF_BYTESTRING,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong,
    fn_FPDF_CountNamedDests: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_DWORD,
    fn_FPDF_GetNamedDestByName:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, name: FPDF_BYTESTRING) -> FPDF_DEST,
    fn_FPDF_GetNamedDest: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        index: c_int,
        buffer: *mut c_void,
        buflen: *mut c_long,
    ) -> FPDF_DEST,
    fn_FPDF_GetXFAPacketCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    fn_FPDF_GetXFAPacketName: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_GetXFAPacketContent: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    fn_FPDFDOC_InitFormFillEnvironment: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        formInfo: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE,
    fn_FPDFDOC_ExitFormFillEnvironment: unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE),
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
    fn_FPDF_SetFormFieldHighlightColor:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, fieldType: c_int, color: c_ulong),
    fn_FPDF_SetFormFieldHighlightAlpha:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, alpha: c_uchar),
    fn_FPDF_RemoveFormFieldHighlight: unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE),
    fn_FPDF_FFLDraw: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    ),
    fn_FPDF_GetFormType: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    fn_FPDF_LoadXFA: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL,
    fn_FPDFAnnot_IsSupportedSubtype:
        unsafe extern "C" fn(subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL,
    fn_FPDFPage_CreateAnnot:
        unsafe extern "C" fn(page: FPDF_PAGE, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_ANNOTATION,
    fn_FPDFPage_GetAnnotCount: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    fn_FPDFPage_GetAnnot: unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_ANNOTATION,
    fn_FPDFPage_GetAnnotIndex:
        unsafe extern "C" fn(page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> c_int,
    fn_FPDFPage_CloseAnnot: unsafe extern "C" fn(annot: FPDF_ANNOTATION),
    fn_FPDFPage_RemoveAnnot: unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_BOOL,
    fn_FPDFAnnot_GetSubtype:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE,
    fn_FPDFAnnot_IsObjectSupportedSubtype:
        unsafe extern "C" fn(subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL,
    fn_FPDFAnnot_UpdateObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    fn_FPDFAnnot_AddInkStroke: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        points: *const FS_POINTF,
        point_count: usize,
    ) -> c_int,
    fn_FPDFAnnot_RemoveInkList: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_BOOL,
    fn_FPDFAnnot_AppendObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    fn_FPDFAnnot_GetObjectCount: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_int,
    fn_FPDFAnnot_GetObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, index: c_int) -> FPDF_PAGEOBJECT,
    fn_FPDFAnnot_RemoveObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, index: c_int) -> FPDF_BOOL,
    fn_FPDFAnnot_SetColor: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        type_: FPDFANNOT_COLORTYPE,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_GetColor: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        type_: FPDFANNOT_COLORTYPE,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_HasAttachmentPoints: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_BOOL,
    fn_FPDFAnnot_SetAttachmentPoints: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        quad_index: usize,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_AppendAttachmentPoints: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_CountAttachmentPoints: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> usize,
    fn_FPDFAnnot_GetAttachmentPoints: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        quad_index: usize,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_SetRect:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> FPDF_BOOL,
    fn_FPDFAnnot_GetRect:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> FPDF_BOOL,
    fn_FPDFAnnot_GetVertices: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong,
    fn_FPDFAnnot_GetInkListCount: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_ulong,
    fn_FPDFAnnot_GetInkListPath: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        path_index: c_ulong,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong,
    fn_FPDFAnnot_GetLine: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        start: *mut FS_POINTF,
        end: *mut FS_POINTF,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_SetBorder: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        horizontal_radius: f32,
        vertical_radius: f32,
        border_width: f32,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_GetBorder: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        horizontal_radius: *mut f32,
        vertical_radius: *mut f32,
        border_width: *mut f32,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_GetFormAdditionalActionJavaScript: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        event: c_int,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFAnnot_HasKey:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_BOOL,
    fn_FPDFAnnot_GetValueType:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
    fn_FPDFAnnot_SetStringValue: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_GetStringValue: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFAnnot_GetNumberValue: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        value: *mut f32,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_SetAP: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_GetAP: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFAnnot_GetLinkedAnnot:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_ANNOTATION,
    fn_FPDFAnnot_GetFlags: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_int,
    fn_FPDFAnnot_SetFlags: unsafe extern "C" fn(annot: FPDF_ANNOTATION, flags: c_int) -> FPDF_BOOL,
    fn_FPDFAnnot_GetFormFieldFlags:
        unsafe extern "C" fn(handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    fn_FPDFAnnot_SetFormFieldFlags: unsafe extern "C" fn(
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        flags: c_int,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_GetFormFieldAtPoint: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        point: *const FS_POINTF,
    ) -> FPDF_ANNOTATION,
    fn_FPDFAnnot_GetFormFieldName: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFAnnot_GetFormFieldAlternateName: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFAnnot_GetFormFieldType:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    fn_FPDFAnnot_GetFormFieldValue: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFAnnot_GetOptionCount:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    fn_FPDFAnnot_GetOptionLabel: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFAnnot_IsOptionSelected: unsafe extern "C" fn(
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_GetFontSize: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        value: *mut f32,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_SetFontColor: unsafe extern "C" fn(
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        R: c_uint,
        G: c_uint,
        B: c_uint,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_GetFontColor: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_IsChecked:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL,
    fn_FPDFAnnot_SetFocusableSubtypes: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        subtypes: *const FPDF_ANNOTATION_SUBTYPE,
        count: usize,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_GetFocusableSubtypesCount: unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE) -> c_int,
    fn_FPDFAnnot_GetFocusableSubtypes: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
        count: usize,
    ) -> FPDF_BOOL,
    fn_FPDFAnnot_GetLink: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_LINK,
    fn_FPDFAnnot_GetFormControlCount:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    fn_FPDFAnnot_GetFormControlIndex:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    fn_FPDFAnnot_GetFormFieldExportValue: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFAnnot_SetURI:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, uri: *const c_char) -> FPDF_BOOL,
    fn_FPDFAnnot_GetFileAttachment: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_ATTACHMENT,
    fn_FPDFAnnot_AddFileAttachment:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, name: FPDF_WIDESTRING) -> FPDF_ATTACHMENT,
    fn_FPDFDoc_GetAttachmentCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    fn_FPDFDoc_AddAttachment:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, name: FPDF_WIDESTRING) -> FPDF_ATTACHMENT,
    fn_FPDFDoc_GetAttachment:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_ATTACHMENT,
    fn_FPDFDoc_DeleteAttachment:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_BOOL,
    fn_FPDFAttachment_GetName: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFAttachment_HasKey:
        unsafe extern "C" fn(attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING) -> FPDF_BOOL,
    fn_FPDFAttachment_GetValueType:
        unsafe extern "C" fn(attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
    fn_FPDFAttachment_SetStringValue: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        key: FPDF_BYTESTRING,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL,
    fn_FPDFAttachment_GetStringValue: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        key: FPDF_BYTESTRING,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFAttachment_SetFile: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        document: FPDF_DOCUMENT,
        contents: *const c_void,
        len: c_ulong,
    ) -> FPDF_BOOL,
    fn_FPDFAttachment_GetFile: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    fn_FPDFAttachment_GetSubtype: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFCatalog_IsTagged: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL,
    fn_FPDFCatalog_SetLanguage:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, language: FPDF_BYTESTRING) -> FPDF_BOOL,
    fn_FPDFAvail_Create: unsafe extern "C" fn(
        file_avail: *mut FX_FILEAVAIL,
        file: *mut FPDF_FILEACCESS,
    ) -> FPDF_AVAIL,
    fn_FPDFAvail_Destroy: unsafe extern "C" fn(avail: FPDF_AVAIL),
    fn_FPDFAvail_IsDocAvail:
        unsafe extern "C" fn(avail: FPDF_AVAIL, hints: *mut FX_DOWNLOADHINTS) -> c_int,
    fn_FPDFAvail_GetDocument:
        unsafe extern "C" fn(avail: FPDF_AVAIL, password: FPDF_BYTESTRING) -> FPDF_DOCUMENT,
    fn_FPDFAvail_GetFirstPageNum: unsafe extern "C" fn(doc: FPDF_DOCUMENT) -> c_int,
    fn_FPDFAvail_IsPageAvail: unsafe extern "C" fn(
        avail: FPDF_AVAIL,
        page_index: c_int,
        hints: *mut FX_DOWNLOADHINTS,
    ) -> c_int,
    fn_FPDFAvail_IsFormAvail:
        unsafe extern "C" fn(avail: FPDF_AVAIL, hints: *mut FX_DOWNLOADHINTS) -> c_int,
    fn_FPDFAvail_IsLinearized: unsafe extern "C" fn(avail: FPDF_AVAIL) -> c_int,
    fn_FPDFBookmark_GetFirstChild:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK,
    fn_FPDFBookmark_GetNextSibling:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK,
    fn_FPDFBookmark_GetTitle: unsafe extern "C" fn(
        bookmark: FPDF_BOOKMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFBookmark_GetCount: unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> c_int,
    fn_FPDFBookmark_Find:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK,
    fn_FPDFBookmark_GetDest:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST,
    fn_FPDFBookmark_GetAction: unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> FPDF_ACTION,
    fn_FPDFAction_GetType: unsafe extern "C" fn(action: FPDF_ACTION) -> c_ulong,
    fn_FPDFAction_GetDest:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST,
    fn_FPDFAction_GetFilePath:
        unsafe extern "C" fn(action: FPDF_ACTION, buffer: *mut c_void, buflen: c_ulong) -> c_ulong,
    fn_FPDFAction_GetURIPath: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDFDest_GetDestPageIndex:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int,
    fn_FPDFDest_GetView: unsafe extern "C" fn(
        dest: FPDF_DEST,
        pNumParams: *mut c_ulong,
        pParams: *mut FS_FLOAT,
    ) -> c_ulong,
    fn_FPDFDest_GetLocationInPage: unsafe extern "C" fn(
        dest: FPDF_DEST,
        hasXVal: *mut FPDF_BOOL,
        hasYVal: *mut FPDF_BOOL,
        hasZoomVal: *mut FPDF_BOOL,
        x: *mut FS_FLOAT,
        y: *mut FS_FLOAT,
        zoom: *mut FS_FLOAT,
    ) -> FPDF_BOOL,
    fn_FPDF_GetPageAAction: unsafe extern "C" fn(page: FPDF_PAGE, aa_type: c_int) -> FPDF_ACTION,
    fn_FPDF_GetFileIdentifier: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        id_type: FPDF_FILEIDTYPE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_GetMetaText: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        tag: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_GetPageLabel: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_CreateNewDocument: unsafe extern "C" fn() -> FPDF_DOCUMENT,
    fn_FPDFPage_New: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        width: f64,
        height: f64,
    ) -> FPDF_PAGE,
    fn_FPDFPage_Delete: unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: c_int),
    fn_FPDF_MovePages: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_indices: *const c_int,
        page_indices_len: c_ulong,
        dest_page_index: c_int,
    ) -> FPDF_BOOL,
    fn_FPDFPage_GetRotation: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    fn_FPDFPage_SetRotation: unsafe extern "C" fn(page: FPDF_PAGE, rotate: c_int),
    fn_FPDFPage_InsertObject: unsafe extern "C" fn(page: FPDF_PAGE, page_object: FPDF_PAGEOBJECT),
    fn_FPDFPage_InsertObjectAtIndex: unsafe extern "C" fn(
        page: FPDF_PAGE,
        page_object: FPDF_PAGEOBJECT,
        index: usize,
    ) -> FPDF_BOOL,
    fn_FPDFPage_RemoveObject:
        unsafe extern "C" fn(page: FPDF_PAGE, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    fn_FPDFPage_CountObjects: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    fn_FPDFPage_GetObject: unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT,
    fn_FPDFPage_HasTransparency: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL,
    fn_FPDFPage_GenerateContent: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL,
    fn_FPDFPage_TransformAnnots:
        unsafe extern "C" fn(page: FPDF_PAGE, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64),
    fn_FPDFDoc_GetPageMode: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    fn_FPDFPage_Flatten: unsafe extern "C" fn(page: FPDF_PAGE, nFlag: c_int) -> c_int,
    fn_FPDFDoc_GetJavaScriptActionCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    fn_FPDFDoc_GetJavaScriptAction:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_JAVASCRIPT_ACTION,
    fn_FPDFDoc_CloseJavaScriptAction: unsafe extern "C" fn(javascript: FPDF_JAVASCRIPT_ACTION),
    fn_FPDF_ImportPagesByIndex: unsafe extern "C" fn(
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        page_indices: *const c_int,
        length: c_ulong,
        index: c_int,
    ) -> FPDF_BOOL,
    fn_FPDF_ImportPages: unsafe extern "C" fn(
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        pagerange: FPDF_BYTESTRING,
        index: c_int,
    ) -> FPDF_BOOL,
    fn_FPDF_ImportNPagesToOne: unsafe extern "C" fn(
        src_doc: FPDF_DOCUMENT,
        output_width: f32,
        output_height: f32,
        num_pages_on_x_axis: usize,
        num_pages_on_y_axis: usize,
    ) -> FPDF_DOCUMENT,
    fn_FPDF_NewXObjectFromPage: unsafe extern "C" fn(
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        src_page_index: c_int,
    ) -> FPDF_XOBJECT,
    fn_FPDF_CloseXObject: unsafe extern "C" fn(xobject: FPDF_XOBJECT),
    fn_FPDF_NewFormObjectFromXObject:
        unsafe extern "C" fn(xobject: FPDF_XOBJECT) -> FPDF_PAGEOBJECT,
    fn_FPDF_CopyViewerPreferences:
        unsafe extern "C" fn(dest_doc: FPDF_DOCUMENT, src_doc: FPDF_DOCUMENT) -> FPDF_BOOL,
    fn_FPDF_RenderPageBitmapWithColorScheme_Start: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
        color_scheme: *const FPDF_COLORSCHEME,
        pause: *mut IFSDK_PAUSE,
    ) -> c_int,
    fn_FPDF_RenderPageBitmap_Start: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
        pause: *mut IFSDK_PAUSE,
    ) -> c_int,
    fn_FPDF_RenderPage_Continue:
        unsafe extern "C" fn(page: FPDF_PAGE, pause: *mut IFSDK_PAUSE) -> c_int,
    fn_FPDF_RenderPage_Close: unsafe extern "C" fn(page: FPDF_PAGE),
    fn_FPDF_SaveAsCopy: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
    ) -> FPDF_BOOL,
    fn_FPDF_SaveWithVersion: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
        fileVersion: c_int,
    ) -> FPDF_BOOL,
    fn_FPDF_GetSignatureCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    fn_FPDF_GetSignatureObject:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_SIGNATURE,
    fn_FPDF_StructTree_GetForPage: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_STRUCTTREE,
    fn_FPDF_StructTree_Close: unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE),
    fn_FPDF_StructTree_CountChildren: unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE) -> c_int,
    fn_FPDF_StructTree_GetChildAtIndex:
        unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE, index: c_int) -> FPDF_STRUCTELEMENT,
    fn_FPDF_StructElement_GetAltText: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_StructElement_GetActualText: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_StructElement_GetID: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_StructElement_GetLang: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_StructElement_GetStringAttribute: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        attr_name: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_StructElement_GetMarkedContentID:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int,
    fn_FPDF_StructElement_GetType: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_StructElement_GetObjType: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_StructElement_GetTitle: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    fn_FPDF_StructElement_CountChildren:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int,
    fn_FPDF_StructElement_GetChildAtIndex: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        index: c_int,
    ) -> FPDF_STRUCTELEMENT,
    fn_FPDF_StructElement_GetChildMarkedContentID:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT, index: c_int) -> c_int,
    fn_FPDF_StructElement_GetParent:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> FPDF_STRUCTELEMENT,
    fn_FPDF_StructElement_GetAttributeCount:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int,
    fn_FPDF_StructElement_GetAttributeAtIndex: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        index: c_int,
    ) -> FPDF_STRUCTELEMENT_ATTR,
    fn_FPDF_StructElement_Attr_GetCount:
        unsafe extern "C" fn(struct_attribute: FPDF_STRUCTELEMENT_ATTR) -> c_int,
    fn_FPDF_StructElement_Attr_GetName: unsafe extern "C" fn(
        struct_attribute: FPDF_STRUCTELEMENT_ATTR,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    fn_FPDF_StructElement_Attr_GetValue: unsafe extern "C" fn(
        struct_attribute: FPDF_STRUCTELEMENT_ATTR,
        name: FPDF_BYTESTRING,
    ) -> FPDF_STRUCTELEMENT_ATTR_VALUE,
    fn_FPDF_StructElement_Attr_GetType:
        unsafe extern "C" fn(value: FPDF_STRUCTELEMENT_ATTR_VALUE) -> FPDF_OBJECT_TYPE,
    fn_FPDF_StructElement_Attr_GetBooleanValue: unsafe extern "C" fn(
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        out_value: *mut FPDF_BOOL,
    ) -> FPDF_BOOL,
    fn_FPDF_StructElement_Attr_GetNumberValue: unsafe extern "C" fn(
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        out_value: *mut f32,
    ) -> FPDF_BOOL,
    fn_FPDF_StructElement_Attr_GetStringValue: unsafe extern "C" fn(
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    fn_FPDF_StructElement_Attr_GetBlobValue: unsafe extern "C" fn(
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    fn_FPDF_StructElement_Attr_CountChildren:
        unsafe extern "C" fn(value: FPDF_STRUCTELEMENT_ATTR_VALUE) -> c_int,
    fn_FPDF_StructElement_Attr_GetChildAtIndex:
        unsafe extern "C" fn(
            value: FPDF_STRUCTELEMENT_ATTR_VALUE,
            index: c_int,
        ) -> FPDF_STRUCTELEMENT_ATTR_VALUE,
    fn_FPDF_StructElement_GetMarkedContentIdCount:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int,
    fn_FPDF_StructElement_GetMarkedContentIdAtIndex:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT, index: c_int) -> c_int,
    fn_FPDF_GetDefaultTTFMap: unsafe extern "C" fn() -> *const FPDF_CharsetFontMap,
    fn_FPDF_GetDefaultTTFMapCount: unsafe extern "C" fn() -> usize,
    fn_FPDF_GetDefaultTTFMapEntry: unsafe extern "C" fn(index: usize) -> *const FPDF_CharsetFontMap,
    fn_FPDF_AddInstalledFont:
        unsafe extern "C" fn(mapper: *mut c_void, face: *const c_char, charset: c_int),
    fn_FPDF_SetSystemFontInfo: unsafe extern "C" fn(font_info: *mut FPDF_SYSFONTINFO),
    fn_FPDF_GetDefaultSystemFontInfo: unsafe extern "C" fn() -> *mut FPDF_SYSFONTINFO,
    fn_FPDF_FreeDefaultSystemFontInfo: unsafe extern "C" fn(font_info: *mut FPDF_SYSFONTINFO),
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
    fn_FPDFClipPath_CountPaths: unsafe extern "C" fn(clip_path: FPDF_CLIPPATH) -> c_int,
    fn_FPDFClipPath_CountPathSegments:
        unsafe extern "C" fn(clip_path: FPDF_CLIPPATH, path_index: c_int) -> c_int,
    fn_FPDFClipPath_GetPathSegment: unsafe extern "C" fn(
        clip_path: FPDF_CLIPPATH,
        path_index: c_int,
        segment_index: c_int,
    ) -> FPDF_PATHSEGMENT,
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
            fn_FPDF_SetSandBoxPolicy: *(lib_get(&lib, "FPDF_SetSandBoxPolicy")?),
            fn_FPDF_LoadCustomDocument: *(lib_get(&lib, "FPDF_LoadCustomDocument")?),
            fn_FPDF_GetFileVersion: *(lib_get(&lib, "FPDF_GetFileVersion")?),
            fn_FPDF_GetLastError: *(lib_get(&lib, "FPDF_GetLastError")?),
            fn_FPDF_DocumentHasValidCrossReferenceTable: *(lib_get(
                &lib,
                "FPDF_DocumentHasValidCrossReferenceTable",
            )?),
            fn_FPDF_GetTrailerEnds: *(lib_get(&lib, "FPDF_GetTrailerEnds")?),
            fn_FPDF_GetDocPermissions: *(lib_get(&lib, "FPDF_GetDocPermissions")?),
            fn_FPDF_GetDocUserPermissions: *(lib_get(&lib, "FPDF_GetDocUserPermissions")?),
            fn_FPDF_GetSecurityHandlerRevision: *(lib_get(
                &lib,
                "FPDF_GetSecurityHandlerRevision",
            )?),
            fn_FPDF_GetPageCount: *(lib_get(&lib, "FPDF_GetPageCount")?),
            fn_FPDF_LoadPage: *(lib_get(&lib, "FPDF_LoadPage")?),
            fn_FPDF_GetPageWidthF: *(lib_get(&lib, "FPDF_GetPageWidthF")?),
            fn_FPDF_GetPageWidth: *(lib_get(&lib, "FPDF_GetPageWidth")?),
            fn_FPDF_GetPageHeightF: *(lib_get(&lib, "FPDF_GetPageHeightF")?),
            fn_FPDF_GetPageHeight: *(lib_get(&lib, "FPDF_GetPageHeight")?),
            fn_FPDF_GetPageBoundingBox: *(lib_get(&lib, "FPDF_GetPageBoundingBox")?),
            fn_FPDF_GetPageSizeByIndexF: *(lib_get(&lib, "FPDF_GetPageSizeByIndexF")?),
            fn_FPDF_GetPageSizeByIndex: *(lib_get(&lib, "FPDF_GetPageSizeByIndex")?),
            fn_FPDF_RenderPageBitmap: *(lib_get(&lib, "FPDF_RenderPageBitmap")?),
            fn_FPDF_RenderPageBitmapWithMatrix: *(lib_get(
                &lib,
                "FPDF_RenderPageBitmapWithMatrix",
            )?),
            fn_FPDF_ClosePage: *(lib_get(&lib, "FPDF_ClosePage")?),
            fn_FPDF_CloseDocument: *(lib_get(&lib, "FPDF_CloseDocument")?),
            fn_FPDF_DeviceToPage: *(lib_get(&lib, "FPDF_DeviceToPage")?),
            fn_FPDF_PageToDevice: *(lib_get(&lib, "FPDF_PageToDevice")?),
            fn_FPDFBitmap_Create: *(lib_get(&lib, "FPDFBitmap_Create")?),
            fn_FPDFBitmap_CreateEx: *(lib_get(&lib, "FPDFBitmap_CreateEx")?),
            fn_FPDFBitmap_GetFormat: *(lib_get(&lib, "FPDFBitmap_GetFormat")?),
            fn_FPDFBitmap_FillRect: *(lib_get(&lib, "FPDFBitmap_FillRect")?),
            fn_FPDFBitmap_GetBuffer: *(lib_get(&lib, "FPDFBitmap_GetBuffer")?),
            fn_FPDFBitmap_GetWidth: *(lib_get(&lib, "FPDFBitmap_GetWidth")?),
            fn_FPDFBitmap_GetHeight: *(lib_get(&lib, "FPDFBitmap_GetHeight")?),
            fn_FPDFBitmap_GetStride: *(lib_get(&lib, "FPDFBitmap_GetStride")?),
            fn_FPDFBitmap_Destroy: *(lib_get(&lib, "FPDFBitmap_Destroy")?),
            fn_FPDF_VIEWERREF_GetPrintScaling: *(lib_get(&lib, "FPDF_VIEWERREF_GetPrintScaling")?),
            fn_FPDF_VIEWERREF_GetNumCopies: *(lib_get(&lib, "FPDF_VIEWERREF_GetNumCopies")?),
            fn_FPDF_VIEWERREF_GetPrintPageRange: *(lib_get(
                &lib,
                "FPDF_VIEWERREF_GetPrintPageRange",
            )?),
            fn_FPDF_VIEWERREF_GetPrintPageRangeCount: *(lib_get(
                &lib,
                "FPDF_VIEWERREF_GetPrintPageRangeCount",
            )?),
            fn_FPDF_VIEWERREF_GetPrintPageRangeElement: *(lib_get(
                &lib,
                "FPDF_VIEWERREF_GetPrintPageRangeElement",
            )?),
            fn_FPDF_VIEWERREF_GetDuplex: *(lib_get(&lib, "FPDF_VIEWERREF_GetDuplex")?),
            fn_FPDF_VIEWERREF_GetName: *(lib_get(&lib, "FPDF_VIEWERREF_GetName")?),
            fn_FPDF_CountNamedDests: *(lib_get(&lib, "FPDF_CountNamedDests")?),
            fn_FPDF_GetNamedDestByName: *(lib_get(&lib, "FPDF_GetNamedDestByName")?),
            fn_FPDF_GetNamedDest: *(lib_get(&lib, "FPDF_GetNamedDest")?),
            fn_FPDF_GetXFAPacketCount: *(lib_get(&lib, "FPDF_GetXFAPacketCount")?),
            fn_FPDF_GetXFAPacketName: *(lib_get(&lib, "FPDF_GetXFAPacketName")?),
            fn_FPDF_GetXFAPacketContent: *(lib_get(&lib, "FPDF_GetXFAPacketContent")?),
            fn_FPDFDOC_InitFormFillEnvironment: *(lib_get(
                &lib,
                "FPDFDOC_InitFormFillEnvironment",
            )?),
            fn_FPDFDOC_ExitFormFillEnvironment: *(lib_get(
                &lib,
                "FPDFDOC_ExitFormFillEnvironment",
            )?),
            fn_FPDFPage_HasFormFieldAtPoint: *(lib_get(&lib, "FPDFPage_HasFormFieldAtPoint")?),
            fn_FPDFPage_FormFieldZOrderAtPoint: *(lib_get(
                &lib,
                "FPDFPage_FormFieldZOrderAtPoint",
            )?),
            fn_FPDF_SetFormFieldHighlightColor: *(lib_get(
                &lib,
                "FPDF_SetFormFieldHighlightColor",
            )?),
            fn_FPDF_SetFormFieldHighlightAlpha: *(lib_get(
                &lib,
                "FPDF_SetFormFieldHighlightAlpha",
            )?),
            fn_FPDF_RemoveFormFieldHighlight: *(lib_get(&lib, "FPDF_RemoveFormFieldHighlight")?),
            fn_FPDF_FFLDraw: *(lib_get(&lib, "FPDF_FFLDraw")?),
            fn_FPDF_GetFormType: *(lib_get(&lib, "FPDF_GetFormType")?),
            fn_FPDF_LoadXFA: *(lib_get(&lib, "FPDF_LoadXFA")?),
            fn_FPDFAnnot_IsSupportedSubtype: *(lib_get(&lib, "FPDFAnnot_IsSupportedSubtype")?),
            fn_FPDFPage_CreateAnnot: *(lib_get(&lib, "FPDFPage_CreateAnnot")?),
            fn_FPDFPage_GetAnnotCount: *(lib_get(&lib, "FPDFPage_GetAnnotCount")?),
            fn_FPDFPage_GetAnnot: *(lib_get(&lib, "FPDFPage_GetAnnot")?),
            fn_FPDFPage_GetAnnotIndex: *(lib_get(&lib, "FPDFPage_GetAnnotIndex")?),
            fn_FPDFPage_CloseAnnot: *(lib_get(&lib, "FPDFPage_CloseAnnot")?),
            fn_FPDFPage_RemoveAnnot: *(lib_get(&lib, "FPDFPage_RemoveAnnot")?),
            fn_FPDFAnnot_GetSubtype: *(lib_get(&lib, "FPDFAnnot_GetSubtype")?),
            fn_FPDFAnnot_IsObjectSupportedSubtype: *(lib_get(
                &lib,
                "FPDFAnnot_IsObjectSupportedSubtype",
            )?),
            fn_FPDFAnnot_UpdateObject: *(lib_get(&lib, "FPDFAnnot_UpdateObject")?),
            fn_FPDFAnnot_AddInkStroke: *(lib_get(&lib, "FPDFAnnot_AddInkStroke")?),
            fn_FPDFAnnot_RemoveInkList: *(lib_get(&lib, "FPDFAnnot_RemoveInkList")?),
            fn_FPDFAnnot_AppendObject: *(lib_get(&lib, "FPDFAnnot_AppendObject")?),
            fn_FPDFAnnot_GetObjectCount: *(lib_get(&lib, "FPDFAnnot_GetObjectCount")?),
            fn_FPDFAnnot_GetObject: *(lib_get(&lib, "FPDFAnnot_GetObject")?),
            fn_FPDFAnnot_RemoveObject: *(lib_get(&lib, "FPDFAnnot_RemoveObject")?),
            fn_FPDFAnnot_SetColor: *(lib_get(&lib, "FPDFAnnot_SetColor")?),
            fn_FPDFAnnot_GetColor: *(lib_get(&lib, "FPDFAnnot_GetColor")?),
            fn_FPDFAnnot_HasAttachmentPoints: *(lib_get(&lib, "FPDFAnnot_HasAttachmentPoints")?),
            fn_FPDFAnnot_SetAttachmentPoints: *(lib_get(&lib, "FPDFAnnot_SetAttachmentPoints")?),
            fn_FPDFAnnot_AppendAttachmentPoints: *(lib_get(
                &lib,
                "FPDFAnnot_AppendAttachmentPoints",
            )?),
            fn_FPDFAnnot_CountAttachmentPoints: *(lib_get(
                &lib,
                "FPDFAnnot_CountAttachmentPoints",
            )?),
            fn_FPDFAnnot_GetAttachmentPoints: *(lib_get(&lib, "FPDFAnnot_GetAttachmentPoints")?),
            fn_FPDFAnnot_SetRect: *(lib_get(&lib, "FPDFAnnot_SetRect")?),
            fn_FPDFAnnot_GetRect: *(lib_get(&lib, "FPDFAnnot_GetRect")?),
            fn_FPDFAnnot_GetVertices: *(lib_get(&lib, "FPDFAnnot_GetVertices")?),
            fn_FPDFAnnot_GetInkListCount: *(lib_get(&lib, "FPDFAnnot_GetInkListCount")?),
            fn_FPDFAnnot_GetInkListPath: *(lib_get(&lib, "FPDFAnnot_GetInkListPath")?),
            fn_FPDFAnnot_GetLine: *(lib_get(&lib, "FPDFAnnot_GetLine")?),
            fn_FPDFAnnot_SetBorder: *(lib_get(&lib, "FPDFAnnot_SetBorder")?),
            fn_FPDFAnnot_GetBorder: *(lib_get(&lib, "FPDFAnnot_GetBorder")?),
            fn_FPDFAnnot_GetFormAdditionalActionJavaScript: *(lib_get(
                &lib,
                "FPDFAnnot_GetFormAdditionalActionJavaScript",
            )?),
            fn_FPDFAnnot_HasKey: *(lib_get(&lib, "FPDFAnnot_HasKey")?),
            fn_FPDFAnnot_GetValueType: *(lib_get(&lib, "FPDFAnnot_GetValueType")?),
            fn_FPDFAnnot_SetStringValue: *(lib_get(&lib, "FPDFAnnot_SetStringValue")?),
            fn_FPDFAnnot_GetStringValue: *(lib_get(&lib, "FPDFAnnot_GetStringValue")?),
            fn_FPDFAnnot_GetNumberValue: *(lib_get(&lib, "FPDFAnnot_GetNumberValue")?),
            fn_FPDFAnnot_SetAP: *(lib_get(&lib, "FPDFAnnot_SetAP")?),
            fn_FPDFAnnot_GetAP: *(lib_get(&lib, "FPDFAnnot_GetAP")?),
            fn_FPDFAnnot_GetLinkedAnnot: *(lib_get(&lib, "FPDFAnnot_GetLinkedAnnot")?),
            fn_FPDFAnnot_GetFlags: *(lib_get(&lib, "FPDFAnnot_GetFlags")?),
            fn_FPDFAnnot_SetFlags: *(lib_get(&lib, "FPDFAnnot_SetFlags")?),
            fn_FPDFAnnot_GetFormFieldFlags: *(lib_get(&lib, "FPDFAnnot_GetFormFieldFlags")?),
            fn_FPDFAnnot_SetFormFieldFlags: *(lib_get(&lib, "FPDFAnnot_SetFormFieldFlags")?),
            fn_FPDFAnnot_GetFormFieldAtPoint: *(lib_get(&lib, "FPDFAnnot_GetFormFieldAtPoint")?),
            fn_FPDFAnnot_GetFormFieldName: *(lib_get(&lib, "FPDFAnnot_GetFormFieldName")?),
            fn_FPDFAnnot_GetFormFieldAlternateName: *(lib_get(
                &lib,
                "FPDFAnnot_GetFormFieldAlternateName",
            )?),
            fn_FPDFAnnot_GetFormFieldType: *(lib_get(&lib, "FPDFAnnot_GetFormFieldType")?),
            fn_FPDFAnnot_GetFormFieldValue: *(lib_get(&lib, "FPDFAnnot_GetFormFieldValue")?),
            fn_FPDFAnnot_GetOptionCount: *(lib_get(&lib, "FPDFAnnot_GetOptionCount")?),
            fn_FPDFAnnot_GetOptionLabel: *(lib_get(&lib, "FPDFAnnot_GetOptionLabel")?),
            fn_FPDFAnnot_IsOptionSelected: *(lib_get(&lib, "FPDFAnnot_IsOptionSelected")?),
            fn_FPDFAnnot_GetFontSize: *(lib_get(&lib, "FPDFAnnot_GetFontSize")?),
            fn_FPDFAnnot_SetFontColor: *(lib_get(&lib, "FPDFAnnot_SetFontColor")?),
            fn_FPDFAnnot_GetFontColor: *(lib_get(&lib, "FPDFAnnot_GetFontColor")?),
            fn_FPDFAnnot_IsChecked: *(lib_get(&lib, "FPDFAnnot_IsChecked")?),
            fn_FPDFAnnot_SetFocusableSubtypes: *(lib_get(&lib, "FPDFAnnot_SetFocusableSubtypes")?),
            fn_FPDFAnnot_GetFocusableSubtypesCount: *(lib_get(
                &lib,
                "FPDFAnnot_GetFocusableSubtypesCount",
            )?),
            fn_FPDFAnnot_GetFocusableSubtypes: *(lib_get(&lib, "FPDFAnnot_GetFocusableSubtypes")?),
            fn_FPDFAnnot_GetLink: *(lib_get(&lib, "FPDFAnnot_GetLink")?),
            fn_FPDFAnnot_GetFormControlCount: *(lib_get(&lib, "FPDFAnnot_GetFormControlCount")?),
            fn_FPDFAnnot_GetFormControlIndex: *(lib_get(&lib, "FPDFAnnot_GetFormControlIndex")?),
            fn_FPDFAnnot_GetFormFieldExportValue: *(lib_get(
                &lib,
                "FPDFAnnot_GetFormFieldExportValue",
            )?),
            fn_FPDFAnnot_SetURI: *(lib_get(&lib, "FPDFAnnot_SetURI")?),
            fn_FPDFAnnot_GetFileAttachment: *(lib_get(&lib, "FPDFAnnot_GetFileAttachment")?),
            fn_FPDFAnnot_AddFileAttachment: *(lib_get(&lib, "FPDFAnnot_AddFileAttachment")?),
            fn_FPDFDoc_GetAttachmentCount: *(lib_get(&lib, "FPDFDoc_GetAttachmentCount")?),
            fn_FPDFDoc_AddAttachment: *(lib_get(&lib, "FPDFDoc_AddAttachment")?),
            fn_FPDFDoc_GetAttachment: *(lib_get(&lib, "FPDFDoc_GetAttachment")?),
            fn_FPDFDoc_DeleteAttachment: *(lib_get(&lib, "FPDFDoc_DeleteAttachment")?),
            fn_FPDFAttachment_GetName: *(lib_get(&lib, "FPDFAttachment_GetName")?),
            fn_FPDFAttachment_HasKey: *(lib_get(&lib, "FPDFAttachment_HasKey")?),
            fn_FPDFAttachment_GetValueType: *(lib_get(&lib, "FPDFAttachment_GetValueType")?),
            fn_FPDFAttachment_SetStringValue: *(lib_get(&lib, "FPDFAttachment_SetStringValue")?),
            fn_FPDFAttachment_GetStringValue: *(lib_get(&lib, "FPDFAttachment_GetStringValue")?),
            fn_FPDFAttachment_SetFile: *(lib_get(&lib, "FPDFAttachment_SetFile")?),
            fn_FPDFAttachment_GetFile: *(lib_get(&lib, "FPDFAttachment_GetFile")?),
            fn_FPDFAttachment_GetSubtype: *(lib_get(&lib, "FPDFAttachment_GetSubtype")?),
            fn_FPDFCatalog_IsTagged: *(lib_get(&lib, "FPDFCatalog_IsTagged")?),
            fn_FPDFCatalog_SetLanguage: *(lib_get(&lib, "FPDFCatalog_SetLanguage")?),
            fn_FPDFAvail_Create: *(lib_get(&lib, "FPDFAvail_Create")?),
            fn_FPDFAvail_Destroy: *(lib_get(&lib, "FPDFAvail_Destroy")?),
            fn_FPDFAvail_IsDocAvail: *(lib_get(&lib, "FPDFAvail_IsDocAvail")?),
            fn_FPDFAvail_GetDocument: *(lib_get(&lib, "FPDFAvail_GetDocument")?),
            fn_FPDFAvail_GetFirstPageNum: *(lib_get(&lib, "FPDFAvail_GetFirstPageNum")?),
            fn_FPDFAvail_IsPageAvail: *(lib_get(&lib, "FPDFAvail_IsPageAvail")?),
            fn_FPDFAvail_IsFormAvail: *(lib_get(&lib, "FPDFAvail_IsFormAvail")?),
            fn_FPDFAvail_IsLinearized: *(lib_get(&lib, "FPDFAvail_IsLinearized")?),
            fn_FPDFBookmark_GetFirstChild: *(lib_get(&lib, "FPDFBookmark_GetFirstChild")?),
            fn_FPDFBookmark_GetNextSibling: *(lib_get(&lib, "FPDFBookmark_GetNextSibling")?),
            fn_FPDFBookmark_GetTitle: *(lib_get(&lib, "FPDFBookmark_GetTitle")?),
            fn_FPDFBookmark_GetCount: *(lib_get(&lib, "FPDFBookmark_GetCount")?),
            fn_FPDFBookmark_Find: *(lib_get(&lib, "FPDFBookmark_Find")?),
            fn_FPDFBookmark_GetDest: *(lib_get(&lib, "FPDFBookmark_GetDest")?),
            fn_FPDFBookmark_GetAction: *(lib_get(&lib, "FPDFBookmark_GetAction")?),
            fn_FPDFAction_GetType: *(lib_get(&lib, "FPDFAction_GetType")?),
            fn_FPDFAction_GetDest: *(lib_get(&lib, "FPDFAction_GetDest")?),
            fn_FPDFAction_GetFilePath: *(lib_get(&lib, "FPDFAction_GetFilePath")?),
            fn_FPDFAction_GetURIPath: *(lib_get(&lib, "FPDFAction_GetURIPath")?),
            fn_FPDFDest_GetDestPageIndex: *(lib_get(&lib, "FPDFDest_GetDestPageIndex")?),
            fn_FPDFDest_GetView: *(lib_get(&lib, "FPDFDest_GetView")?),
            fn_FPDFDest_GetLocationInPage: *(lib_get(&lib, "FPDFDest_GetLocationInPage")?),
            fn_FPDF_GetPageAAction: *(lib_get(&lib, "FPDF_GetPageAAction")?),
            fn_FPDF_GetFileIdentifier: *(lib_get(&lib, "FPDF_GetFileIdentifier")?),
            fn_FPDF_GetMetaText: *(lib_get(&lib, "FPDF_GetMetaText")?),
            fn_FPDF_GetPageLabel: *(lib_get(&lib, "FPDF_GetPageLabel")?),
            fn_FPDF_CreateNewDocument: *(lib_get(&lib, "FPDF_CreateNewDocument")?),
            fn_FPDFPage_New: *(lib_get(&lib, "FPDFPage_New")?),
            fn_FPDFPage_Delete: *(lib_get(&lib, "FPDFPage_Delete")?),
            fn_FPDF_MovePages: *(lib_get(&lib, "FPDF_MovePages")?),
            fn_FPDFPage_GetRotation: *(lib_get(&lib, "FPDFPage_GetRotation")?),
            fn_FPDFPage_SetRotation: *(lib_get(&lib, "FPDFPage_SetRotation")?),
            fn_FPDFPage_InsertObject: *(lib_get(&lib, "FPDFPage_InsertObject")?),
            fn_FPDFPage_InsertObjectAtIndex: *(lib_get(&lib, "FPDFPage_InsertObjectAtIndex")?),
            fn_FPDFPage_RemoveObject: *(lib_get(&lib, "FPDFPage_RemoveObject")?),
            fn_FPDFPage_CountObjects: *(lib_get(&lib, "FPDFPage_CountObjects")?),
            fn_FPDFPage_GetObject: *(lib_get(&lib, "FPDFPage_GetObject")?),
            fn_FPDFPage_HasTransparency: *(lib_get(&lib, "FPDFPage_HasTransparency")?),
            fn_FPDFPage_GenerateContent: *(lib_get(&lib, "FPDFPage_GenerateContent")?),
            fn_FPDFPage_TransformAnnots: *(lib_get(&lib, "FPDFPage_TransformAnnots")?),
            fn_FPDFDoc_GetPageMode: *(lib_get(&lib, "FPDFDoc_GetPageMode")?),
            fn_FPDFPage_Flatten: *(lib_get(&lib, "FPDFPage_Flatten")?),
            fn_FPDFDoc_GetJavaScriptActionCount: *(lib_get(
                &lib,
                "FPDFDoc_GetJavaScriptActionCount",
            )?),
            fn_FPDFDoc_GetJavaScriptAction: *(lib_get(&lib, "FPDFDoc_GetJavaScriptAction")?),
            fn_FPDFDoc_CloseJavaScriptAction: *(lib_get(&lib, "FPDFDoc_CloseJavaScriptAction")?),
            fn_FPDF_ImportPagesByIndex: *(lib_get(&lib, "FPDF_ImportPagesByIndex")?),
            fn_FPDF_ImportPages: *(lib_get(&lib, "FPDF_ImportPages")?),
            fn_FPDF_ImportNPagesToOne: *(lib_get(&lib, "FPDF_ImportNPagesToOne")?),
            fn_FPDF_NewXObjectFromPage: *(lib_get(&lib, "FPDF_NewXObjectFromPage")?),
            fn_FPDF_CloseXObject: *(lib_get(&lib, "FPDF_CloseXObject")?),
            fn_FPDF_NewFormObjectFromXObject: *(lib_get(&lib, "FPDF_NewFormObjectFromXObject")?),
            fn_FPDF_CopyViewerPreferences: *(lib_get(&lib, "FPDF_CopyViewerPreferences")?),
            fn_FPDF_RenderPageBitmapWithColorScheme_Start: *(lib_get(
                &lib,
                "FPDF_RenderPageBitmapWithColorScheme_Start",
            )?),
            fn_FPDF_RenderPageBitmap_Start: *(lib_get(&lib, "FPDF_RenderPageBitmap_Start")?),
            fn_FPDF_RenderPage_Continue: *(lib_get(&lib, "FPDF_RenderPage_Continue")?),
            fn_FPDF_RenderPage_Close: *(lib_get(&lib, "FPDF_RenderPage_Close")?),
            fn_FPDF_SaveAsCopy: *(lib_get(&lib, "FPDF_SaveAsCopy")?),
            fn_FPDF_SaveWithVersion: *(lib_get(&lib, "FPDF_SaveWithVersion")?),
            fn_FPDF_GetSignatureCount: *(lib_get(&lib, "FPDF_GetSignatureCount")?),
            fn_FPDF_GetSignatureObject: *(lib_get(&lib, "FPDF_GetSignatureObject")?),
            fn_FPDF_StructTree_GetForPage: *(lib_get(&lib, "FPDF_StructTree_GetForPage")?),
            fn_FPDF_StructTree_Close: *(lib_get(&lib, "FPDF_StructTree_Close")?),
            fn_FPDF_StructTree_CountChildren: *(lib_get(&lib, "FPDF_StructTree_CountChildren")?),
            fn_FPDF_StructTree_GetChildAtIndex: *(lib_get(
                &lib,
                "FPDF_StructTree_GetChildAtIndex",
            )?),
            fn_FPDF_StructElement_GetAltText: *(lib_get(&lib, "FPDF_StructElement_GetAltText")?),
            fn_FPDF_StructElement_GetActualText: *(lib_get(
                &lib,
                "FPDF_StructElement_GetActualText",
            )?),
            fn_FPDF_StructElement_GetID: *(lib_get(&lib, "FPDF_StructElement_GetID")?),
            fn_FPDF_StructElement_GetLang: *(lib_get(&lib, "FPDF_StructElement_GetLang")?),
            fn_FPDF_StructElement_GetStringAttribute: *(lib_get(
                &lib,
                "FPDF_StructElement_GetStringAttribute",
            )?),
            fn_FPDF_StructElement_GetMarkedContentID: *(lib_get(
                &lib,
                "FPDF_StructElement_GetMarkedContentID",
            )?),
            fn_FPDF_StructElement_GetType: *(lib_get(&lib, "FPDF_StructElement_GetType")?),
            fn_FPDF_StructElement_GetObjType: *(lib_get(&lib, "FPDF_StructElement_GetObjType")?),
            fn_FPDF_StructElement_GetTitle: *(lib_get(&lib, "FPDF_StructElement_GetTitle")?),
            fn_FPDF_StructElement_CountChildren: *(lib_get(
                &lib,
                "FPDF_StructElement_CountChildren",
            )?),
            fn_FPDF_StructElement_GetChildAtIndex: *(lib_get(
                &lib,
                "FPDF_StructElement_GetChildAtIndex",
            )?),
            fn_FPDF_StructElement_GetChildMarkedContentID: *(lib_get(
                &lib,
                "FPDF_StructElement_GetChildMarkedContentID",
            )?),
            fn_FPDF_StructElement_GetParent: *(lib_get(&lib, "FPDF_StructElement_GetParent")?),
            fn_FPDF_StructElement_GetAttributeCount: *(lib_get(
                &lib,
                "FPDF_StructElement_GetAttributeCount",
            )?),
            fn_FPDF_StructElement_GetAttributeAtIndex: *(lib_get(
                &lib,
                "FPDF_StructElement_GetAttributeAtIndex",
            )?),
            fn_FPDF_StructElement_Attr_GetCount: *(lib_get(
                &lib,
                "FPDF_StructElement_Attr_GetCount",
            )?),
            fn_FPDF_StructElement_Attr_GetName: *(lib_get(
                &lib,
                "FPDF_StructElement_Attr_GetName",
            )?),
            fn_FPDF_StructElement_Attr_GetValue: *(lib_get(
                &lib,
                "FPDF_StructElement_Attr_GetValue",
            )?),
            fn_FPDF_StructElement_Attr_GetType: *(lib_get(
                &lib,
                "FPDF_StructElement_Attr_GetType",
            )?),
            fn_FPDF_StructElement_Attr_GetBooleanValue: *(lib_get(
                &lib,
                "FPDF_StructElement_Attr_GetBooleanValue",
            )?),
            fn_FPDF_StructElement_Attr_GetNumberValue: *(lib_get(
                &lib,
                "FPDF_StructElement_Attr_GetNumberValue",
            )?),
            fn_FPDF_StructElement_Attr_GetStringValue: *(lib_get(
                &lib,
                "FPDF_StructElement_Attr_GetStringValue",
            )?),
            fn_FPDF_StructElement_Attr_GetBlobValue: *(lib_get(
                &lib,
                "FPDF_StructElement_Attr_GetBlobValue",
            )?),
            fn_FPDF_StructElement_Attr_CountChildren: *(lib_get(
                &lib,
                "FPDF_StructElement_Attr_CountChildren",
            )?),
            fn_FPDF_StructElement_Attr_GetChildAtIndex: *(lib_get(
                &lib,
                "FPDF_StructElement_Attr_GetChildAtIndex",
            )?),
            fn_FPDF_StructElement_GetMarkedContentIdCount: *(lib_get(
                &lib,
                "FPDF_StructElement_GetMarkedContentIdCount",
            )?),
            fn_FPDF_StructElement_GetMarkedContentIdAtIndex: *(lib_get(
                &lib,
                "FPDF_StructElement_GetMarkedContentIdAtIndex",
            )?),
            fn_FPDF_GetDefaultTTFMap: *(lib_get(&lib, "FPDF_GetDefaultTTFMap")?),
            fn_FPDF_GetDefaultTTFMapCount: *(lib_get(&lib, "FPDF_GetDefaultTTFMapCount")?),
            fn_FPDF_GetDefaultTTFMapEntry: *(lib_get(&lib, "FPDF_GetDefaultTTFMapEntry")?),
            fn_FPDF_AddInstalledFont: *(lib_get(&lib, "FPDF_AddInstalledFont")?),
            fn_FPDF_SetSystemFontInfo: *(lib_get(&lib, "FPDF_SetSystemFontInfo")?),
            fn_FPDF_GetDefaultSystemFontInfo: *(lib_get(&lib, "FPDF_GetDefaultSystemFontInfo")?),
            fn_FPDF_FreeDefaultSystemFontInfo: *(lib_get(&lib, "FPDF_FreeDefaultSystemFontInfo")?),
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
            fn_FPDFClipPath_CountPaths: *(lib_get(&lib, "FPDFClipPath_CountPaths")?),
            fn_FPDFClipPath_CountPathSegments: *(lib_get(&lib, "FPDFClipPath_CountPathSegments")?),
            fn_FPDFClipPath_GetPathSegment: *(lib_get(&lib, "FPDFClipPath_GetPathSegment")?),
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
    pub fn FPDF_InitLibraryWithConfig(&self, config: &FPDF_LIBRARY_CONFIG) {
        unsafe { (self.fn_FPDF_InitLibraryWithConfig)(config) }
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

    /// C documentation for FPDF_SetSandBoxPolicy:
    ///
    /// ```text
    /// Function: FPDF_SetSandBoxPolicy
    ///          Set the policy for the sandbox environment.
    /// Parameters:
    ///          policy -   The specified policy for setting, for example:
    ///                     FPDF_POLICY_MACHINETIME_ACCESS.
    ///          enable -   True to enable, false to disable the policy.
    /// Return value:
    ///          None.
    /// ```
    pub fn FPDF_SetSandBoxPolicy(&self, policy: FPDF_DWORD, enable: i32) {
        unsafe { (self.fn_FPDF_SetSandBoxPolicy)(policy, enable) }
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

    /// C documentation for FPDF_GetFileVersion:
    ///
    /// ```text
    /// Function: FPDF_GetFileVersion
    ///          Get the file version of the given PDF document.
    /// Parameters:
    ///          doc         -   Handle to a document.
    ///          fileVersion -   The PDF file version. File version: 14 for 1.4, 15
    ///                          for 1.5, ...
    /// Return value:
    ///          True if succeeds, false otherwise.
    /// Comments:
    ///          If the document was created by FPDF_CreateNewDocument,
    ///          then this function will always fail.
    /// ```
    pub fn FPDF_GetFileVersion(
        &self,
        doc: &PdfiumDocument,
        fileVersion: &mut i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_GetFileVersion)(doc.into(), fileVersion) })
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
    pub fn FPDF_GetLastError(&self) -> c_ulong {
        unsafe { (self.fn_FPDF_GetLastError)() }
    }

    /// C documentation for FPDF_DocumentHasValidCrossReferenceTable:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_DocumentHasValidCrossReferenceTable
    ///          Whether the document's cross reference table is valid or not.
    /// Parameters:
    ///          document    -   Handle to a document. Returned by FPDF_LoadDocument.
    /// Return value:
    ///          True if the PDF parser did not encounter problems parsing the cross
    ///          reference table. False if the parser could not parse the cross
    ///          reference table and the table had to be rebuild from other data
    ///          within the document.
    /// Comments:
    ///          The return value can change over time as the PDF parser evolves.
    /// ```
    pub fn FPDF_DocumentHasValidCrossReferenceTable(
        &self,
        document: &PdfiumDocument,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_DocumentHasValidCrossReferenceTable)(document.into()) })
    }

    /// C documentation for FPDF_GetTrailerEnds:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_GetTrailerEnds
    ///          Get the byte offsets of trailer ends.
    /// Parameters:
    ///          document    -   Handle to document. Returned by FPDF_LoadDocument().
    ///          buffer      -   The address of a buffer that receives the
    ///                          byte offsets.
    ///          length      -   The size, in ints, of |buffer|.
    /// Return value:
    ///          Returns the number of ints in the buffer on success, 0 on error.
    ///
    /// |buffer| is an array of integers that describes the exact byte offsets of the
    /// trailer ends in the document. If |length| is less than the returned length,
    /// or |document| or |buffer| is NULL, |buffer| will not be modified.
    /// ```
    pub fn FPDF_GetTrailerEnds(
        &self,
        document: &PdfiumDocument,
        buffer: &mut u32,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDF_GetTrailerEnds)(document.into(), buffer, length) }
    }

    /// C documentation for FPDF_GetDocPermissions:
    ///
    /// ```text
    /// Function: FPDF_GetDocPermissions
    ///          Get file permission flags of the document.
    /// Parameters:
    ///          document    -   Handle to a document. Returned by FPDF_LoadDocument.
    /// Return value:
    ///          A 32-bit integer indicating permission flags. Please refer to the
    ///          PDF Reference for detailed descriptions. If the document is not
    ///          protected or was unlocked by the owner, 0xffffffff will be returned.
    /// ```
    pub fn FPDF_GetDocPermissions(&self, document: &PdfiumDocument) -> c_ulong {
        unsafe { (self.fn_FPDF_GetDocPermissions)(document.into()) }
    }

    /// C documentation for FPDF_GetDocUserPermissions:
    ///
    /// ```text
    /// Function: FPDF_GetDocUserPermissions
    ///          Get user file permission flags of the document.
    /// Parameters:
    ///          document    -   Handle to a document. Returned by FPDF_LoadDocument.
    /// Return value:
    ///          A 32-bit integer indicating permission flags. Please refer to the
    ///          PDF Reference for detailed descriptions. If the document is not
    ///          protected, 0xffffffff will be returned. Always returns user
    ///          permissions, even if the document was unlocked by the owner.
    /// ```
    pub fn FPDF_GetDocUserPermissions(&self, document: &PdfiumDocument) -> c_ulong {
        unsafe { (self.fn_FPDF_GetDocUserPermissions)(document.into()) }
    }

    /// C documentation for FPDF_GetSecurityHandlerRevision:
    ///
    /// ```text
    /// Function: FPDF_GetSecurityHandlerRevision
    ///          Get the revision for the security handler.
    /// Parameters:
    ///          document    -   Handle to a document. Returned by FPDF_LoadDocument.
    /// Return value:
    ///          The security handler revision number. Please refer to the PDF
    ///          Reference for a detailed description. If the document is not
    ///          protected, -1 will be returned.
    /// ```
    pub fn FPDF_GetSecurityHandlerRevision(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDF_GetSecurityHandlerRevision)(document.into()) }
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

    /// C documentation for FPDF_GetPageWidthF:
    ///
    /// ```text
    /// Experimental API
    /// Function: FPDF_GetPageWidthF
    ///          Get page width.
    /// Parameters:
    ///          page        -   Handle to the page. Returned by FPDF_LoadPage().
    /// Return value:
    ///          Page width (excluding non-displayable area) measured in points.
    ///          One point is 1/72 inch (around 0.3528 mm).
    /// Comments:
    ///          Changing the rotation of |page| affects the return value.
    /// ```
    pub fn FPDF_GetPageWidthF(&self, page: &PdfiumPage) -> f32 {
        unsafe { (self.fn_FPDF_GetPageWidthF)(page.into()) }
    }

    /// C documentation for FPDF_GetPageWidth:
    ///
    /// ```text
    /// Function: FPDF_GetPageWidth
    ///          Get page width.
    /// Parameters:
    ///          page        -   Handle to the page. Returned by FPDF_LoadPage.
    /// Return value:
    ///          Page width (excluding non-displayable area) measured in points.
    ///          One point is 1/72 inch (around 0.3528 mm).
    /// Note:
    ///          Prefer FPDF_GetPageWidthF() above. This will be deprecated in the
    ///          future.
    /// Comments:
    ///          Changing the rotation of |page| affects the return value.
    /// ```
    pub fn FPDF_GetPageWidth(&self, page: &PdfiumPage) -> f64 {
        unsafe { (self.fn_FPDF_GetPageWidth)(page.into()) }
    }

    /// C documentation for FPDF_GetPageHeightF:
    ///
    /// ```text
    /// Experimental API
    /// Function: FPDF_GetPageHeightF
    ///          Get page height.
    /// Parameters:
    ///          page        -   Handle to the page. Returned by FPDF_LoadPage().
    /// Return value:
    ///          Page height (excluding non-displayable area) measured in points.
    ///          One point is 1/72 inch (around 0.3528 mm)
    /// Comments:
    ///          Changing the rotation of |page| affects the return value.
    /// ```
    pub fn FPDF_GetPageHeightF(&self, page: &PdfiumPage) -> f32 {
        unsafe { (self.fn_FPDF_GetPageHeightF)(page.into()) }
    }

    /// C documentation for FPDF_GetPageHeight:
    ///
    /// ```text
    /// Function: FPDF_GetPageHeight
    ///          Get page height.
    /// Parameters:
    ///          page        -   Handle to the page. Returned by FPDF_LoadPage.
    /// Return value:
    ///          Page height (excluding non-displayable area) measured in points.
    ///          One point is 1/72 inch (around 0.3528 mm)
    /// Note:
    ///          Prefer FPDF_GetPageHeightF() above. This will be deprecated in the
    ///          future.
    /// Comments:
    ///          Changing the rotation of |page| affects the return value.
    /// ```
    pub fn FPDF_GetPageHeight(&self, page: &PdfiumPage) -> f64 {
        unsafe { (self.fn_FPDF_GetPageHeight)(page.into()) }
    }

    /// C documentation for FPDF_GetPageBoundingBox:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_GetPageBoundingBox
    ///          Get the bounding box of the page. This is the intersection between
    ///          its media box and its crop box.
    /// Parameters:
    ///          page        -   Handle to the page. Returned by FPDF_LoadPage.
    ///          rect        -   Pointer to a rect to receive the page bounding box.
    ///                          On an error, |rect| won't be filled.
    /// Return value:
    ///          True for success.
    /// ```
    pub fn FPDF_GetPageBoundingBox(
        &self,
        page: &PdfiumPage,
        rect: &mut FS_RECTF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_GetPageBoundingBox)(page.into(), rect) })
    }

    /// C documentation for FPDF_GetPageSizeByIndexF:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_GetPageSizeByIndexF
    ///          Get the size of the page at the given index.
    /// Parameters:
    ///          document    -   Handle to document. Returned by FPDF_LoadDocument().
    ///          page_index  -   Page index, zero for the first page.
    ///          size        -   Pointer to a FS_SIZEF to receive the page size.
    ///                          (in points).
    /// Return value:
    ///          Non-zero for success. 0 for error (document or page not found).
    /// ```
    pub fn FPDF_GetPageSizeByIndexF(
        &self,
        document: &PdfiumDocument,
        page_index: i32,
        size: &mut FS_SIZEF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_GetPageSizeByIndexF)(document.into(), page_index, size) })
    }

    /// C documentation for FPDF_GetPageSizeByIndex:
    ///
    /// ```text
    /// Function: FPDF_GetPageSizeByIndex
    ///          Get the size of the page at the given index.
    /// Parameters:
    ///          document    -   Handle to document. Returned by FPDF_LoadDocument.
    ///          page_index  -   Page index, zero for the first page.
    ///          width       -   Pointer to a double to receive the page width
    ///                          (in points).
    ///          height      -   Pointer to a double to receive the page height
    ///                          (in points).
    /// Return value:
    ///          Non-zero for success. 0 for error (document or page not found).
    /// Note:
    ///          Prefer FPDF_GetPageSizeByIndexF() above. This will be deprecated in
    ///          the future.
    /// ```
    pub fn FPDF_GetPageSizeByIndex(
        &self,
        document: &PdfiumDocument,
        page_index: i32,
        width: &mut f64,
        height: &mut f64,
    ) -> i32 {
        unsafe { (self.fn_FPDF_GetPageSizeByIndex)(document.into(), page_index, width, height) }
    }

    /// C documentation for FPDF_RenderPageBitmap:
    ///
    /// ```text
    /// Function: FPDF_RenderPageBitmap
    ///          Render contents of a page to a device independent bitmap.
    /// Parameters:
    ///          bitmap      -   Handle to the device independent bitmap (as the
    ///                          output buffer). The bitmap handle can be created
    ///                          by FPDFBitmap_Create or retrieved from an image
    ///                          object by FPDFImageObj_GetBitmap.
    ///          page        -   Handle to the page. Returned by FPDF_LoadPage
    ///          start_x     -   Left pixel position of the display area in
    ///                          bitmap coordinates.
    ///          start_y     -   Top pixel position of the display area in bitmap
    ///                          coordinates.
    ///          size_x      -   Horizontal size (in pixels) for displaying the page.
    ///          size_y      -   Vertical size (in pixels) for displaying the page.
    ///          rotate      -   Page orientation:
    ///                            0 (normal)
    ///                            1 (rotated 90 degrees clockwise)
    ///                            2 (rotated 180 degrees)
    ///                            3 (rotated 90 degrees counter-clockwise)
    ///          flags       -   0 for normal display, or combination of the Page
    ///                          Rendering flags defined above. With the FPDF_ANNOT
    ///                          flag, it renders all annotations that do not require
    ///                          user-interaction, which are all annotations except
    ///                          widget and popup annotations.
    /// Return value:
    ///          None.
    /// ```
    pub fn FPDF_RenderPageBitmap(
        &self,
        bitmap: &PdfiumBitmap,
        page: &PdfiumPage,
        start_x: i32,
        start_y: i32,
        size_x: i32,
        size_y: i32,
        rotate: i32,
        flags: i32,
    ) {
        unsafe {
            (self.fn_FPDF_RenderPageBitmap)(
                bitmap.into(),
                page.into(),
                start_x,
                start_y,
                size_x,
                size_y,
                rotate,
                flags,
            )
        }
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

    /// C documentation for FPDF_DeviceToPage:
    ///
    /// ```text
    /// Function: FPDF_DeviceToPage
    ///          Convert the screen coordinates of a point to page coordinates.
    /// Parameters:
    ///          page        -   Handle to the page. Returned by FPDF_LoadPage.
    ///          start_x     -   Left pixel position of the display area in
    ///                          device coordinates.
    ///          start_y     -   Top pixel position of the display area in device
    ///                          coordinates.
    ///          size_x      -   Horizontal size (in pixels) for displaying the page.
    ///          size_y      -   Vertical size (in pixels) for displaying the page.
    ///          rotate      -   Page orientation:
    ///                            0 (normal)
    ///                            1 (rotated 90 degrees clockwise)
    ///                            2 (rotated 180 degrees)
    ///                            3 (rotated 90 degrees counter-clockwise)
    ///          device_x    -   X value in device coordinates to be converted.
    ///          device_y    -   Y value in device coordinates to be converted.
    ///          page_x      -   A pointer to a double receiving the converted X
    ///                          value in page coordinates.
    ///          page_y      -   A pointer to a double receiving the converted Y
    ///                          value in page coordinates.
    /// Return value:
    ///          Returns true if the conversion succeeds, and |page_x| and |page_y|
    ///          successfully receives the converted coordinates.
    /// Comments:
    ///          The page coordinate system has its origin at the left-bottom corner
    ///          of the page, with the X-axis on the bottom going to the right, and
    ///          the Y-axis on the left side going up.
    ///
    ///          NOTE: this coordinate system can be altered when you zoom, scroll,
    ///          or rotate a page, however, a point on the page should always have
    ///          the same coordinate values in the page coordinate system.
    ///
    ///          The device coordinate system is device dependent. For screen device,
    ///          its origin is at the left-top corner of the window. However this
    ///          origin can be altered by the Windows coordinate transformation
    ///          utilities.
    ///
    ///          You must make sure the start_x, start_y, size_x, size_y
    ///          and rotate parameters have exactly same values as you used in
    ///          the FPDF_RenderPage() function call.
    /// ```
    pub fn FPDF_DeviceToPage(
        &self,
        page: &PdfiumPage,
        start_x: i32,
        start_y: i32,
        size_x: i32,
        size_y: i32,
        rotate: i32,
        device_x: i32,
        device_y: i32,
        page_x: &mut f64,
        page_y: &mut f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_DeviceToPage)(
                page.into(),
                start_x,
                start_y,
                size_x,
                size_y,
                rotate,
                device_x,
                device_y,
                page_x,
                page_y,
            )
        })
    }

    /// C documentation for FPDF_PageToDevice:
    ///
    /// ```text
    /// Function: FPDF_PageToDevice
    ///          Convert the page coordinates of a point to screen coordinates.
    /// Parameters:
    ///          page        -   Handle to the page. Returned by FPDF_LoadPage.
    ///          start_x     -   Left pixel position of the display area in
    ///                          device coordinates.
    ///          start_y     -   Top pixel position of the display area in device
    ///                          coordinates.
    ///          size_x      -   Horizontal size (in pixels) for displaying the page.
    ///          size_y      -   Vertical size (in pixels) for displaying the page.
    ///          rotate      -   Page orientation:
    ///                            0 (normal)
    ///                            1 (rotated 90 degrees clockwise)
    ///                            2 (rotated 180 degrees)
    ///                            3 (rotated 90 degrees counter-clockwise)
    ///          page_x      -   X value in page coordinates.
    ///          page_y      -   Y value in page coordinate.
    ///          device_x    -   A pointer to an integer receiving the result X
    ///                          value in device coordinates.
    ///          device_y    -   A pointer to an integer receiving the result Y
    ///                          value in device coordinates.
    /// Return value:
    ///          Returns true if the conversion succeeds, and |device_x| and
    ///          |device_y| successfully receives the converted coordinates.
    /// Comments:
    ///          See comments for FPDF_DeviceToPage().
    /// ```
    pub fn FPDF_PageToDevice(
        &self,
        page: &PdfiumPage,
        start_x: i32,
        start_y: i32,
        size_x: i32,
        size_y: i32,
        rotate: i32,
        page_x: f64,
        page_y: f64,
        device_x: &mut i32,
        device_y: &mut i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_PageToDevice)(
                page.into(),
                start_x,
                start_y,
                size_x,
                size_y,
                rotate,
                page_x,
                page_y,
                device_x,
                device_y,
            )
        })
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

    /// C documentation for FPDF_VIEWERREF_GetPrintScaling:
    ///
    /// ```text
    /// Function: FPDF_VIEWERREF_GetPrintScaling
    ///          Whether the PDF document prefers to be scaled or not.
    /// Parameters:
    ///          document    -   Handle to the loaded document.
    /// Return value:
    ///          None.
    /// ```
    pub fn FPDF_VIEWERREF_GetPrintScaling(&self, document: &PdfiumDocument) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_VIEWERREF_GetPrintScaling)(document.into()) })
    }

    /// C documentation for FPDF_VIEWERREF_GetNumCopies:
    ///
    /// ```text
    /// Function: FPDF_VIEWERREF_GetNumCopies
    ///          Returns the number of copies to be printed.
    /// Parameters:
    ///          document    -   Handle to the loaded document.
    /// Return value:
    ///          The number of copies to be printed.
    /// ```
    pub fn FPDF_VIEWERREF_GetNumCopies(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDF_VIEWERREF_GetNumCopies)(document.into()) }
    }

    /// C documentation for FPDF_VIEWERREF_GetPrintPageRange:
    ///
    /// ```text
    /// Function: FPDF_VIEWERREF_GetPrintPageRange
    ///          Page numbers to initialize print dialog box when file is printed.
    /// Parameters:
    ///          document    -   Handle to the loaded document.
    /// Return value:
    ///          The print page range to be used for printing.
    /// ```
    pub fn FPDF_VIEWERREF_GetPrintPageRange(
        &self,
        document: &PdfiumDocument,
    ) -> PdfiumResult<PdfiumPageRange> {
        PdfiumPageRange::new_from_handle(unsafe {
            (self.fn_FPDF_VIEWERREF_GetPrintPageRange)(document.into())
        })
    }

    /// C documentation for FPDF_VIEWERREF_GetPrintPageRangeCount:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_VIEWERREF_GetPrintPageRangeCount
    ///          Returns the number of elements in a FPDF_PAGERANGE.
    /// Parameters:
    ///          pagerange   -   Handle to the page range.
    /// Return value:
    ///          The number of elements in the page range. Returns 0 on error.
    /// ```
    pub fn FPDF_VIEWERREF_GetPrintPageRangeCount(&self, pagerange: &PdfiumPageRange) -> usize {
        unsafe { (self.fn_FPDF_VIEWERREF_GetPrintPageRangeCount)(pagerange.into()) }
    }

    /// C documentation for FPDF_VIEWERREF_GetPrintPageRangeElement:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_VIEWERREF_GetPrintPageRangeElement
    ///          Returns an element from a FPDF_PAGERANGE.
    /// Parameters:
    ///          pagerange   -   Handle to the page range.
    ///          index       -   Index of the element.
    /// Return value:
    ///          The value of the element in the page range at a given index.
    ///          Returns -1 on error.
    /// ```
    pub fn FPDF_VIEWERREF_GetPrintPageRangeElement(
        &self,
        pagerange: &PdfiumPageRange,
        index: usize,
    ) -> i32 {
        unsafe { (self.fn_FPDF_VIEWERREF_GetPrintPageRangeElement)(pagerange.into(), index) }
    }

    /// C documentation for FPDF_VIEWERREF_GetDuplex:
    ///
    /// ```text
    /// Function: FPDF_VIEWERREF_GetDuplex
    ///          Returns the paper handling option to be used when printing from
    ///          the print dialog.
    /// Parameters:
    ///          document    -   Handle to the loaded document.
    /// Return value:
    ///          The paper handling option to be used when printing.
    /// ```
    pub fn FPDF_VIEWERREF_GetDuplex(&self, document: &PdfiumDocument) -> FPDF_DUPLEXTYPE {
        unsafe { (self.fn_FPDF_VIEWERREF_GetDuplex)(document.into()) }
    }

    /// C documentation for FPDF_VIEWERREF_GetName:
    ///
    /// ```text
    /// Function: FPDF_VIEWERREF_GetName
    ///          Gets the contents for a viewer ref, with a given key. The value must
    ///          be of type "name".
    /// Parameters:
    ///          document    -   Handle to the loaded document.
    ///          key         -   Name of the key in the viewer pref dictionary,
    ///                          encoded in UTF-8.
    ///          buffer      -   Caller-allocate buffer to receive the key, or NULL
    ///                      -   to query the required length.
    ///          length      -   Length of the buffer.
    /// Return value:
    ///          The number of bytes in the contents, including the NULL terminator.
    ///          Thus if the return value is 0, then that indicates an error, such
    ///          as when |document| is invalid. If |length| is less than the required
    ///          length, or |buffer| is NULL, |buffer| will not be modified.
    /// ```
    pub fn FPDF_VIEWERREF_GetName(
        &self,
        document: &PdfiumDocument,
        key: &CString,
        buffer: Option<&mut [i8]>,
        length: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_VIEWERREF_GetName)(
                document.into(),
                key.as_ptr(),
                to_char_ptr_mut(buffer),
                length,
            )
        }
    }

    /// C documentation for FPDF_CountNamedDests:
    ///
    /// ```text
    /// Function: FPDF_CountNamedDests
    ///          Get the count of named destinations in the PDF document.
    /// Parameters:
    ///          document    -   Handle to a document
    /// Return value:
    ///          The count of named destinations.
    /// ```
    pub fn FPDF_CountNamedDests(&self, document: &PdfiumDocument) -> FPDF_DWORD {
        unsafe { (self.fn_FPDF_CountNamedDests)(document.into()) }
    }

    /// C documentation for FPDF_GetNamedDestByName:
    ///
    /// ```text
    /// Function: FPDF_GetNamedDestByName
    ///          Get a the destination handle for the given name.
    /// Parameters:
    ///          document    -   Handle to the loaded document.
    ///          name        -   The name of a destination.
    /// Return value:
    ///          The handle to the destination.
    /// ```
    pub fn FPDF_GetNamedDestByName(
        &self,
        document: &PdfiumDocument,
        name: &CString,
    ) -> PdfiumResult<PdfiumDestination> {
        PdfiumDestination::new_from_handle(unsafe {
            (self.fn_FPDF_GetNamedDestByName)(document.into(), name.as_ptr())
        })
    }

    /// C documentation for FPDF_GetNamedDest:
    ///
    /// ```text
    /// Function: FPDF_GetNamedDest
    ///          Get the named destination by index.
    /// Parameters:
    ///          document        -   Handle to a document
    ///          index           -   The index of a named destination.
    ///          buffer          -   The buffer to store the destination name,
    ///                              used as wchar_t*.
    ///          buflen [in/out] -   Size of the buffer in bytes on input,
    ///                              length of the result in bytes on output
    ///                              or -1 if the buffer is too small.
    /// Return value:
    ///          The destination handle for a given index, or NULL if there is no
    ///          named destination corresponding to |index|.
    /// Comments:
    ///          Call this function twice to get the name of the named destination:
    ///            1) First time pass in |buffer| as NULL and get buflen.
    ///            2) Second time pass in allocated |buffer| and buflen to retrieve
    ///               |buffer|, which should be used as wchar_t*.
    ///
    ///         If buflen is not sufficiently large, it will be set to -1 upon
    ///         return.
    /// ```
    pub fn FPDF_GetNamedDest(
        &self,
        document: &PdfiumDocument,
        index: i32,
        buffer: Option<&mut [u8]>,
        buflen: &mut c_long,
    ) -> PdfiumResult<PdfiumDestination> {
        PdfiumDestination::new_from_handle(unsafe {
            (self.fn_FPDF_GetNamedDest)(document.into(), index, to_void_ptr_mut(buffer), buflen)
        })
    }

    /// C documentation for FPDF_GetXFAPacketCount:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_GetXFAPacketCount
    ///          Get the number of valid packets in the XFA entry.
    /// Parameters:
    ///          document - Handle to the document.
    /// Return value:
    ///          The number of valid packets, or -1 on error.
    /// ```
    pub fn FPDF_GetXFAPacketCount(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDF_GetXFAPacketCount)(document.into()) }
    }

    /// C documentation for FPDF_GetXFAPacketName:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_GetXFAPacketName
    ///          Get the name of a packet in the XFA array.
    /// Parameters:
    ///          document - Handle to the document.
    ///          index    - Index number of the packet. 0 for the first packet.
    ///          buffer   - Buffer for holding the name of the XFA packet.
    ///          buflen   - Length of |buffer| in bytes.
    /// Return value:
    ///          The length of the packet name in bytes, or 0 on error.
    ///
    /// |document| must be valid and |index| must be in the range [0, N), where N is
    /// the value returned by FPDF_GetXFAPacketCount().
    /// |buffer| is only modified if it is non-NULL and |buflen| is greater than or
    /// equal to the length of the packet name. The packet name includes a
    /// terminating NUL character. |buffer| is unmodified on error.
    /// ```
    pub fn FPDF_GetXFAPacketName(
        &self,
        document: &PdfiumDocument,
        index: i32,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_GetXFAPacketName)(document.into(), index, to_void_ptr_mut(buffer), buflen)
        }
    }

    /// C documentation for FPDF_GetXFAPacketContent:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_GetXFAPacketContent
    ///          Get the content of a packet in the XFA array.
    /// Parameters:
    ///          document   - Handle to the document.
    ///          index      - Index number of the packet. 0 for the first packet.
    ///          buffer     - Buffer for holding the content of the XFA packet.
    ///          buflen     - Length of |buffer| in bytes.
    ///          out_buflen - Pointer to the variable that will receive the minimum
    ///                       buffer size needed to contain the content of the XFA
    ///                       packet.
    /// Return value:
    ///          Whether the operation succeeded or not.
    ///
    /// |document| must be valid and |index| must be in the range [0, N), where N is
    /// the value returned by FPDF_GetXFAPacketCount(). |out_buflen| must not be
    /// NULL. When the aforementioned arguments are valid, the operation succeeds,
    /// and |out_buflen| receives the content size. |buffer| is only modified if
    /// |buffer| is non-null and long enough to contain the content. Callers must
    /// check both the return value and the input |buflen| is no less than the
    /// returned |out_buflen| before using the data in |buffer|.
    /// ```
    pub fn FPDF_GetXFAPacketContent(
        &self,
        document: &PdfiumDocument,
        index: i32,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
        out_buflen: &mut c_ulong,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_GetXFAPacketContent)(
                document.into(),
                index,
                to_void_ptr_mut(buffer),
                buflen,
                out_buflen,
            )
        })
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
        formInfo: &mut FPDF_FORMFILLINFO,
    ) -> PdfiumResult<PdfiumForm> {
        PdfiumForm::new_from_handle(unsafe {
            (self.fn_FPDFDOC_InitFormFillEnvironment)(document.into(), formInfo)
        })
    }

    /// C documentation for FPDFDOC_ExitFormFillEnvironment:
    ///
    /// ```text
    /// Function: FPDFDOC_ExitFormFillEnvironment
    ///       Take ownership of |hHandle| and exit form fill environment.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    /// Return Value:
    ///       None.
    /// Comments:
    ///       This function is a no-op when |hHandle| is null.
    /// ```
    pub fn FPDFDOC_ExitFormFillEnvironment(&self, hHandle: &PdfiumForm) {
        unsafe { (self.fn_FPDFDOC_ExitFormFillEnvironment)(hHandle.into()) }
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

    /// C documentation for FPDF_SetFormFieldHighlightColor:
    ///
    /// ```text
    /// Function: FPDF_SetFormFieldHighlightColor
    ///       Set the highlight color of the specified (or all) form fields
    ///       in the document.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       doc         -   Handle to the document, as returned by
    ///                       FPDF_LoadDocument().
    ///       fieldType   -   A 32-bit integer indicating the type of a form
    ///                       field (defined above).
    ///       color       -   The highlight color of the form field. Constructed by
    ///                       0xxxrrggbb.
    /// Return Value:
    ///       None.
    /// Comments:
    ///       When the parameter fieldType is set to FPDF_FORMFIELD_UNKNOWN, the
    ///       highlight color will be applied to all the form fields in the
    ///       document.
    ///       Please refresh the client window to show the highlight immediately
    ///       if necessary.
    /// ```
    pub fn FPDF_SetFormFieldHighlightColor(
        &self,
        hHandle: &PdfiumForm,
        fieldType: i32,
        color: c_ulong,
    ) {
        unsafe { (self.fn_FPDF_SetFormFieldHighlightColor)(hHandle.into(), fieldType, color) }
    }

    /// C documentation for FPDF_SetFormFieldHighlightAlpha:
    ///
    /// ```text
    /// Function: FPDF_SetFormFieldHighlightAlpha
    ///       Set the transparency of the form field highlight color in the
    ///       document.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       doc         -   Handle to the document, as returaned by
    ///                       FPDF_LoadDocument().
    ///       alpha       -   The transparency of the form field highlight color,
    ///                       between 0-255.
    /// Return Value:
    ///       None.
    /// ```
    pub fn FPDF_SetFormFieldHighlightAlpha(
        &self,
        hHandle: &PdfiumForm,
        alpha: ::std::os::raw::c_uchar,
    ) {
        unsafe { (self.fn_FPDF_SetFormFieldHighlightAlpha)(hHandle.into(), alpha) }
    }

    /// C documentation for FPDF_RemoveFormFieldHighlight:
    ///
    /// ```text
    /// Function: FPDF_RemoveFormFieldHighlight
    ///       Remove the form field highlight color in the document.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    /// Return Value:
    ///       None.
    /// Comments:
    ///       Please refresh the client window to remove the highlight immediately
    ///       if necessary.
    /// ```
    pub fn FPDF_RemoveFormFieldHighlight(&self, hHandle: &PdfiumForm) {
        unsafe { (self.fn_FPDF_RemoveFormFieldHighlight)(hHandle.into()) }
    }

    /// C documentation for FPDF_FFLDraw:
    ///
    /// ```text
    /// Function: FPDF_FFLDraw
    ///       Render FormFields and popup window on a page to a device independent
    ///       bitmap.
    /// Parameters:
    ///       hHandle      -   Handle to the form fill module, as returned by
    ///                        FPDFDOC_InitFormFillEnvironment().
    ///       bitmap       -   Handle to the device independent bitmap (as the
    ///                        output buffer). Bitmap handles can be created by
    ///                        FPDFBitmap_Create().
    ///       page         -   Handle to the page, as returned by FPDF_LoadPage().
    ///       start_x      -   Left pixel position of the display area in the
    ///                        device coordinates.
    ///       start_y      -   Top pixel position of the display area in the device
    ///                        coordinates.
    ///       size_x       -   Horizontal size (in pixels) for displaying the page.
    ///       size_y       -   Vertical size (in pixels) for displaying the page.
    ///       rotate       -   Page orientation: 0 (normal), 1 (rotated 90 degrees
    ///                        clockwise), 2 (rotated 180 degrees), 3 (rotated 90
    ///                        degrees counter-clockwise).
    ///       flags        -   0 for normal display, or combination of flags
    ///                        defined above.
    /// Return Value:
    ///       None.
    /// Comments:
    ///       This function is designed to render annotations that are
    ///       user-interactive, which are widget annotations (for FormFields) and
    ///       popup annotations.
    ///       With the FPDF_ANNOT flag, this function will render a popup annotation
    ///       when users mouse-hover on a non-widget annotation. Regardless of
    ///       FPDF_ANNOT flag, this function will always render widget annotations
    ///       for FormFields.
    ///       In order to implement the FormFill functions, implementation should
    ///       call this function after rendering functions, such as
    ///       FPDF_RenderPageBitmap() or FPDF_RenderPageBitmap_Start(), have
    ///       finished rendering the page contents.
    /// ```
    pub fn FPDF_FFLDraw(
        &self,
        hHandle: &PdfiumForm,
        bitmap: &PdfiumBitmap,
        page: &PdfiumPage,
        start_x: i32,
        start_y: i32,
        size_x: i32,
        size_y: i32,
        rotate: i32,
        flags: i32,
    ) {
        unsafe {
            (self.fn_FPDF_FFLDraw)(
                hHandle.into(),
                bitmap.into(),
                page.into(),
                start_x,
                start_y,
                size_x,
                size_y,
                rotate,
                flags,
            )
        }
    }

    /// C documentation for FPDF_GetFormType:
    ///
    /// ```text
    /// Experimental API
    /// Function: FPDF_GetFormType
    ///           Returns the type of form contained in the PDF document.
    /// Parameters:
    ///           document - Handle to document.
    /// Return Value:
    ///           Integer value representing one of the FORMTYPE_ values.
    /// Comments:
    ///           If |document| is NULL, then the return value is FORMTYPE_NONE.
    /// ```
    pub fn FPDF_GetFormType(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDF_GetFormType)(document.into()) }
    }

    /// C documentation for FPDF_LoadXFA:
    ///
    /// ```text
    /// Function: FPDF_LoadXFA
    ///          If the document consists of XFA fields, call this method to
    ///          attempt to load XFA fields.
    /// Parameters:
    ///          document     -   Handle to document from FPDF_LoadDocument().
    /// Return Value:
    ///          TRUE upon success, otherwise FALSE. If XFA support is not built
    ///          into PDFium, performs no action and always returns FALSE.
    /// ```
    pub fn FPDF_LoadXFA(&self, document: &PdfiumDocument) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_LoadXFA)(document.into()) })
    }

    /// C documentation for FPDFAnnot_IsSupportedSubtype:
    ///
    /// ```text
    /// Experimental API.
    /// Check if an annotation subtype is currently supported for creation.
    /// Currently supported subtypes:
    ///    - circle
    ///    - fileattachment
    ///    - freetext
    ///    - highlight
    ///    - ink
    ///    - link
    ///    - popup
    ///    - square,
    ///    - squiggly
    ///    - stamp
    ///    - strikeout
    ///    - text
    ///    - underline
    ///
    ///   subtype   - the subtype to be checked.
    ///
    /// Returns true if this subtype supported.
    /// ```
    pub fn FPDFAnnot_IsSupportedSubtype(
        &self,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_IsSupportedSubtype)(subtype) })
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

    /// C documentation for FPDFAnnot_GetSubtype:
    ///
    /// ```text
    /// Experimental API.
    /// Get the subtype of an annotation.
    ///
    ///   annot  - handle to an annotation.
    ///
    /// Returns the annotation subtype.
    /// ```
    pub fn FPDFAnnot_GetSubtype(&self, annot: &PdfiumAnnotation) -> FPDF_ANNOTATION_SUBTYPE {
        unsafe { (self.fn_FPDFAnnot_GetSubtype)(annot.into()) }
    }

    /// C documentation for FPDFAnnot_IsObjectSupportedSubtype:
    ///
    /// ```text
    /// Experimental API.
    /// Check if an annotation subtype is currently supported for object extraction,
    /// update, and removal.
    /// Currently supported subtypes: ink and stamp.
    ///
    ///   subtype   - the subtype to be checked.
    ///
    /// Returns true if this subtype supported.
    /// ```
    pub fn FPDFAnnot_IsObjectSupportedSubtype(
        &self,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_IsObjectSupportedSubtype)(subtype) })
    }

    /// C documentation for FPDFAnnot_UpdateObject:
    ///
    /// ```text
    /// Experimental API.
    /// Update |obj| in |annot|. |obj| must be in |annot| already and must have
    /// been retrieved by FPDFAnnot_GetObject(). Currently, only ink and stamp
    /// annotations are supported by this API. Also note that only path, image, and
    /// text objects have APIs for modification; see FPDFPath_*(), FPDFText_*(), and
    /// FPDFImageObj_*().
    ///
    ///   annot  - handle to an annotation.
    ///   obj    - handle to the object that |annot| needs to update.
    ///
    /// Return true if successful.
    /// ```
    pub fn FPDFAnnot_UpdateObject(
        &self,
        annot: &PdfiumAnnotation,
        obj: &PdfiumPageObject,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_UpdateObject)(annot.into(), obj.into()) })
    }

    /// C documentation for FPDFAnnot_AddInkStroke:
    ///
    /// ```text
    /// Experimental API.
    /// Add a new InkStroke, represented by an array of points, to the InkList of
    /// |annot|. The API creates an InkList if one doesn't already exist in |annot|.
    /// This API works only for ink annotations. Please refer to ISO 32000-1:2008
    /// spec, section 12.5.6.13.
    ///
    ///   annot       - handle to an annotation.
    ///   points      - pointer to a FS_POINTF array representing input points.
    ///   point_count - number of elements in |points| array. This should not exceed
    ///                 the maximum value that can be represented by an int32_t).
    ///
    /// Returns the 0-based index at which the new InkStroke is added in the InkList
    /// of the |annot|. Returns -1 on failure.
    /// ```
    pub fn FPDFAnnot_AddInkStroke(
        &self,
        annot: &PdfiumAnnotation,
        points: &FS_POINTF,
        point_count: usize,
    ) -> i32 {
        unsafe { (self.fn_FPDFAnnot_AddInkStroke)(annot.into(), points, point_count) }
    }

    /// C documentation for FPDFAnnot_RemoveInkList:
    ///
    /// ```text
    /// Experimental API.
    /// Removes an InkList in |annot|.
    /// This API works only for ink annotations.
    ///
    ///   annot  - handle to an annotation.
    ///
    /// Return true on successful removal of /InkList entry from context of the
    /// non-null ink |annot|. Returns false on failure.
    /// ```
    pub fn FPDFAnnot_RemoveInkList(&self, annot: &PdfiumAnnotation) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_RemoveInkList)(annot.into()) })
    }

    /// C documentation for FPDFAnnot_AppendObject:
    ///
    /// ```text
    /// Experimental API.
    /// Add |obj| to |annot|. |obj| must have been created by
    /// FPDFPageObj_CreateNew{Path|Rect}() or FPDFPageObj_New{Text|Image}Obj(), and
    /// will be owned by |annot|. Note that an |obj| cannot belong to more than one
    /// |annot|. Currently, only ink and stamp annotations are supported by this API.
    /// Also note that only path, image, and text objects have APIs for creation.
    ///
    ///   annot  - handle to an annotation.
    ///   obj    - handle to the object that is to be added to |annot|.
    ///
    /// Return true if successful.
    /// ```
    pub fn FPDFAnnot_AppendObject(
        &self,
        annot: &PdfiumAnnotation,
        obj: &PdfiumPageObject,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_AppendObject)(annot.into(), obj.into()) })
    }

    /// C documentation for FPDFAnnot_GetObjectCount:
    ///
    /// ```text
    /// Experimental API.
    /// Get the total number of objects in |annot|, including path objects, text
    /// objects, external objects, image objects, and shading objects.
    ///
    ///   annot  - handle to an annotation.
    ///
    /// Returns the number of objects in |annot|.
    /// ```
    pub fn FPDFAnnot_GetObjectCount(&self, annot: &PdfiumAnnotation) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetObjectCount)(annot.into()) }
    }

    /// C documentation for FPDFAnnot_GetObject:
    ///
    /// ```text
    /// Experimental API.
    /// Get the object in |annot| at |index|.
    ///
    ///   annot  - handle to an annotation.
    ///   index  - the index of the object.
    ///
    /// Return a handle to the object, or NULL on failure.
    /// ```
    pub fn FPDFAnnot_GetObject(
        &self,
        annot: &PdfiumAnnotation,
        index: i32,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDFAnnot_GetObject)(annot.into(), index)
        })
    }

    /// C documentation for FPDFAnnot_RemoveObject:
    ///
    /// ```text
    /// Experimental API.
    /// Remove the object in |annot| at |index|.
    ///
    ///   annot  - handle to an annotation.
    ///   index  - the index of the object to be removed.
    ///
    /// Return true if successful.
    /// ```
    pub fn FPDFAnnot_RemoveObject(&self, annot: &PdfiumAnnotation, index: i32) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_RemoveObject)(annot.into(), index) })
    }

    /// C documentation for FPDFAnnot_SetColor:
    ///
    /// ```text
    /// Experimental API.
    /// Set the color of an annotation. Fails when called on annotations with
    /// appearance streams already defined; instead use
    /// FPDFPageObj_Set{Stroke|Fill}Color().
    ///
    ///   annot    - handle to an annotation.
    ///   type     - type of the color to be set.
    ///   R, G, B  - buffer to hold the RGB value of the color. Ranges from 0 to 255.
    ///   A        - buffer to hold the opacity. Ranges from 0 to 255.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_SetColor(
        &self,
        annot: &PdfiumAnnotation,
        type_: FPDFANNOT_COLORTYPE,
        R: u32,
        G: u32,
        B: u32,
        A: u32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_SetColor)(annot.into(), type_, R, G, B, A) })
    }

    /// C documentation for FPDFAnnot_GetColor:
    ///
    /// ```text
    /// Experimental API.
    /// Get the color of an annotation. If no color is specified, default to yellow
    /// for highlight annotation, black for all else. Fails when called on
    /// annotations with appearance streams already defined; instead use
    /// FPDFPageObj_Get{Stroke|Fill}Color().
    ///
    ///   annot    - handle to an annotation.
    ///   type     - type of the color requested.
    ///   R, G, B  - buffer to hold the RGB value of the color. Ranges from 0 to 255.
    ///   A        - buffer to hold the opacity. Ranges from 0 to 255.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_GetColor(
        &self,
        annot: &PdfiumAnnotation,
        type_: FPDFANNOT_COLORTYPE,
        R: &mut u32,
        G: &mut u32,
        B: &mut u32,
        A: &mut u32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_GetColor)(annot.into(), type_, R, G, B, A) })
    }

    /// C documentation for FPDFAnnot_HasAttachmentPoints:
    ///
    /// ```text
    /// Experimental API.
    /// Check if the annotation is of a type that has attachment points
    /// (i.e. quadpoints). Quadpoints are the vertices of the rectangle that
    /// encompasses the texts affected by the annotation. They provide the
    /// coordinates in the page where the annotation is attached. Only text markup
    /// annotations (i.e. highlight, strikeout, squiggly, and underline) and link
    /// annotations have quadpoints.
    ///
    ///   annot  - handle to an annotation.
    ///
    /// Returns true if the annotation is of a type that has quadpoints, false
    /// otherwise.
    /// ```
    pub fn FPDFAnnot_HasAttachmentPoints(&self, annot: &PdfiumAnnotation) -> i32 {
        unsafe { (self.fn_FPDFAnnot_HasAttachmentPoints)(annot.into()) }
    }

    /// C documentation for FPDFAnnot_SetAttachmentPoints:
    ///
    /// ```text
    /// Experimental API.
    /// Replace the attachment points (i.e. quadpoints) set of an annotation at
    /// |quad_index|. This index needs to be within the result of
    /// FPDFAnnot_CountAttachmentPoints().
    /// If the annotation's appearance stream is defined and this annotation is of a
    /// type with quadpoints, then update the bounding box too if the new quadpoints
    /// define a bigger one.
    ///
    ///   annot       - handle to an annotation.
    ///   quad_index  - index of the set of quadpoints.
    ///   quad_points - the quadpoints to be set.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_SetAttachmentPoints(
        &self,
        annot: &PdfiumAnnotation,
        quad_index: usize,
        quad_points: &FS_QUADPOINTSF,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFAnnot_SetAttachmentPoints)(annot.into(), quad_index, quad_points)
        })
    }

    /// C documentation for FPDFAnnot_AppendAttachmentPoints:
    ///
    /// ```text
    /// Experimental API.
    /// Append to the list of attachment points (i.e. quadpoints) of an annotation.
    /// If the annotation's appearance stream is defined and this annotation is of a
    /// type with quadpoints, then update the bounding box too if the new quadpoints
    /// define a bigger one.
    ///
    ///   annot       - handle to an annotation.
    ///   quad_points - the quadpoints to be set.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_AppendAttachmentPoints(
        &self,
        annot: &PdfiumAnnotation,
        quad_points: &FS_QUADPOINTSF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_AppendAttachmentPoints)(annot.into(), quad_points) })
    }

    /// C documentation for FPDFAnnot_CountAttachmentPoints:
    ///
    /// ```text
    /// Experimental API.
    /// Get the number of sets of quadpoints of an annotation.
    ///
    ///   annot  - handle to an annotation.
    ///
    /// Returns the number of sets of quadpoints, or 0 on failure.
    /// ```
    pub fn FPDFAnnot_CountAttachmentPoints(&self, annot: &PdfiumAnnotation) -> usize {
        unsafe { (self.fn_FPDFAnnot_CountAttachmentPoints)(annot.into()) }
    }

    /// C documentation for FPDFAnnot_GetAttachmentPoints:
    ///
    /// ```text
    /// Experimental API.
    /// Get the attachment points (i.e. quadpoints) of an annotation.
    ///
    ///   annot       - handle to an annotation.
    ///   quad_index  - index of the set of quadpoints.
    ///   quad_points - receives the quadpoints; must not be NULL.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_GetAttachmentPoints(
        &self,
        annot: &PdfiumAnnotation,
        quad_index: usize,
        quad_points: &mut FS_QUADPOINTSF,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFAnnot_GetAttachmentPoints)(annot.into(), quad_index, quad_points)
        })
    }

    /// C documentation for FPDFAnnot_SetRect:
    ///
    /// ```text
    /// Experimental API.
    /// Set the annotation rectangle defining the location of the annotation. If the
    /// annotation's appearance stream is defined and this annotation is of a type
    /// without quadpoints, then update the bounding box too if the new rectangle
    /// defines a bigger one.
    ///
    ///   annot  - handle to an annotation.
    ///   rect   - the annotation rectangle to be set.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_SetRect(&self, annot: &PdfiumAnnotation, rect: &FS_RECTF) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_SetRect)(annot.into(), rect) })
    }

    /// C documentation for FPDFAnnot_GetRect:
    ///
    /// ```text
    /// Experimental API.
    /// Get the annotation rectangle defining the location of the annotation.
    ///
    ///   annot  - handle to an annotation.
    ///   rect   - receives the rectangle; must not be NULL.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_GetRect(
        &self,
        annot: &PdfiumAnnotation,
        rect: &mut FS_RECTF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_GetRect)(annot.into(), rect) })
    }

    /// C documentation for FPDFAnnot_GetVertices:
    ///
    /// ```text
    /// Experimental API.
    /// Get the vertices of a polygon or polyline annotation. |buffer| is an array of
    /// points of the annotation. If |length| is less than the returned length, or
    /// |annot| or |buffer| is NULL, |buffer| will not be modified.
    ///
    ///   annot  - handle to an annotation, as returned by e.g. FPDFPage_GetAnnot()
    ///   buffer - buffer for holding the points.
    ///   length - length of the buffer in points.
    ///
    /// Returns the number of points if the annotation is of type polygon or
    /// polyline, 0 otherwise.
    /// ```
    pub fn FPDFAnnot_GetVertices(
        &self,
        annot: &PdfiumAnnotation,
        buffer: &mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFAnnot_GetVertices)(annot.into(), buffer, length) }
    }

    /// C documentation for FPDFAnnot_GetInkListCount:
    ///
    /// ```text
    /// Experimental API.
    /// Get the number of paths in the ink list of an ink annotation.
    ///
    ///   annot  - handle to an annotation, as returned by e.g. FPDFPage_GetAnnot()
    ///
    /// Returns the number of paths in the ink list if the annotation is of type ink,
    /// 0 otherwise.
    /// ```
    pub fn FPDFAnnot_GetInkListCount(&self, annot: &PdfiumAnnotation) -> c_ulong {
        unsafe { (self.fn_FPDFAnnot_GetInkListCount)(annot.into()) }
    }

    /// C documentation for FPDFAnnot_GetInkListPath:
    ///
    /// ```text
    /// Experimental API.
    /// Get a path in the ink list of an ink annotation. |buffer| is an array of
    /// points of the path. If |length| is less than the returned length, or |annot|
    /// or |buffer| is NULL, |buffer| will not be modified.
    ///
    ///   annot  - handle to an annotation, as returned by e.g. FPDFPage_GetAnnot()
    ///   path_index - index of the path
    ///   buffer - buffer for holding the points.
    ///   length - length of the buffer in points.
    ///
    /// Returns the number of points of the path if the annotation is of type ink, 0
    /// otherwise.
    /// ```
    pub fn FPDFAnnot_GetInkListPath(
        &self,
        annot: &PdfiumAnnotation,
        path_index: c_ulong,
        buffer: &mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFAnnot_GetInkListPath)(annot.into(), path_index, buffer, length) }
    }

    /// C documentation for FPDFAnnot_GetLine:
    ///
    /// ```text
    /// Experimental API.
    /// Get the starting and ending coordinates of a line annotation.
    ///
    ///   annot  - handle to an annotation, as returned by e.g. FPDFPage_GetAnnot()
    ///   start - starting point
    ///   end - ending point
    ///
    /// Returns true if the annotation is of type line, |start| and |end| are not
    /// NULL, false otherwise.
    /// ```
    pub fn FPDFAnnot_GetLine(
        &self,
        annot: &PdfiumAnnotation,
        start: &mut FS_POINTF,
        end: &mut FS_POINTF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_GetLine)(annot.into(), start, end) })
    }

    /// C documentation for FPDFAnnot_SetBorder:
    ///
    /// ```text
    /// Experimental API.
    /// Set the characteristics of the annotation's border (rounded rectangle).
    ///
    ///   annot              - handle to an annotation
    ///   horizontal_radius  - horizontal corner radius, in default user space units
    ///   vertical_radius    - vertical corner radius, in default user space units
    ///   border_width       - border width, in default user space units
    ///
    /// Returns true if setting the border for |annot| succeeds, false otherwise.
    ///
    /// If |annot| contains an appearance stream that overrides the border values,
    /// then the appearance stream will be removed on success.
    /// ```
    pub fn FPDFAnnot_SetBorder(
        &self,
        annot: &PdfiumAnnotation,
        horizontal_radius: f32,
        vertical_radius: f32,
        border_width: f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFAnnot_SetBorder)(
                annot.into(),
                horizontal_radius,
                vertical_radius,
                border_width,
            )
        })
    }

    /// C documentation for FPDFAnnot_GetBorder:
    ///
    /// ```text
    /// Experimental API.
    /// Get the characteristics of the annotation's border (rounded rectangle).
    ///
    ///   annot              - handle to an annotation
    ///   horizontal_radius  - horizontal corner radius, in default user space units
    ///   vertical_radius    - vertical corner radius, in default user space units
    ///   border_width       - border width, in default user space units
    ///
    /// Returns true if |horizontal_radius|, |vertical_radius| and |border_width| are
    /// not NULL, false otherwise.
    /// ```
    pub fn FPDFAnnot_GetBorder(
        &self,
        annot: &PdfiumAnnotation,
        horizontal_radius: &mut f32,
        vertical_radius: &mut f32,
        border_width: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFAnnot_GetBorder)(
                annot.into(),
                horizontal_radius,
                vertical_radius,
                border_width,
            )
        })
    }

    /// C documentation for FPDFAnnot_GetFormAdditionalActionJavaScript:
    ///
    /// ```text
    /// Experimental API.
    /// Get the JavaScript of an event of the annotation's additional actions.
    /// |buffer| is only modified if |buflen| is large enough to hold the whole
    /// JavaScript string. If |buflen| is smaller, the total size of the JavaScript
    /// is still returned, but nothing is copied.  If there is no JavaScript for
    /// |event| in |annot|, an empty string is written to |buf| and 2 is returned,
    /// denoting the size of the null terminator in the buffer.  On other errors,
    /// nothing is written to |buffer| and 0 is returned.
    ///
    ///    hHandle     -   handle to the form fill module, returned by
    ///                    FPDFDOC_InitFormFillEnvironment().
    ///    annot       -   handle to an interactive form annotation.
    ///    event       -   event type, one of the FPDF_ANNOT_AACTION_* values.
    ///    buffer      -   buffer for holding the value string, encoded in UTF-16LE.
    ///    buflen      -   length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes, including the 2-byte
    /// null terminator.
    /// ```
    pub fn FPDFAnnot_GetFormAdditionalActionJavaScript(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        event: i32,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFAnnot_GetFormAdditionalActionJavaScript)(
                hHandle.into(),
                annot.into(),
                event,
                buffer.as_mut_ptr(),
                buflen,
            )
        }
    }

    /// C documentation for FPDFAnnot_HasKey:
    ///
    /// ```text
    /// Experimental API.
    /// Check if |annot|'s dictionary has |key| as a key.
    ///
    ///   annot  - handle to an annotation.
    ///   key    - the key to look for, encoded in UTF-8.
    ///
    /// Returns true if |key| exists.
    /// ```
    pub fn FPDFAnnot_HasKey(&self, annot: &PdfiumAnnotation, key: &CString) -> i32 {
        unsafe { (self.fn_FPDFAnnot_HasKey)(annot.into(), key.as_ptr()) }
    }

    /// C documentation for FPDFAnnot_GetValueType:
    ///
    /// ```text
    /// Experimental API.
    /// Get the type of the value corresponding to |key| in |annot|'s dictionary.
    ///
    ///   annot  - handle to an annotation.
    ///   key    - the key to look for, encoded in UTF-8.
    ///
    /// Returns the type of the dictionary value.
    /// ```
    pub fn FPDFAnnot_GetValueType(
        &self,
        annot: &PdfiumAnnotation,
        key: &CString,
    ) -> FPDF_OBJECT_TYPE {
        unsafe { (self.fn_FPDFAnnot_GetValueType)(annot.into(), key.as_ptr()) }
    }

    /// C documentation for FPDFAnnot_SetStringValue:
    ///
    /// ```text
    /// Experimental API.
    /// Set the string value corresponding to |key| in |annot|'s dictionary,
    /// overwriting the existing value if any. The value type would be
    /// FPDF_OBJECT_STRING after this function call succeeds.
    ///
    ///   annot  - handle to an annotation.
    ///   key    - the key to the dictionary entry to be set, encoded in UTF-8.
    ///   value  - the string value to be set, encoded in UTF-16LE.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_SetStringValue(
        &self,
        annot: &PdfiumAnnotation,
        key: &CString,
        value: &str,
    ) -> PdfiumResult<()> {
        let value = str_to_utf16le_vec(value);
        to_result(unsafe {
            (self.fn_FPDFAnnot_SetStringValue)(annot.into(), key.as_ptr(), value.as_ptr())
        })
    }

    /// C documentation for FPDFAnnot_GetStringValue:
    ///
    /// ```text
    /// Experimental API.
    /// Get the string value corresponding to |key| in |annot|'s dictionary. |buffer|
    /// is only modified if |buflen| is longer than the length of contents. Note that
    /// if |key| does not exist in the dictionary or if |key|'s corresponding value
    /// in the dictionary is not a string (i.e. the value is not of type
    /// FPDF_OBJECT_STRING or FPDF_OBJECT_NAME), then an empty string would be copied
    /// to |buffer| and the return value would be 2. On other errors, nothing would
    /// be added to |buffer| and the return value would be 0.
    ///
    ///   annot  - handle to an annotation.
    ///   key    - the key to the requested dictionary entry, encoded in UTF-8.
    ///   buffer - buffer for holding the value string, encoded in UTF-16LE.
    ///   buflen - length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    /// ```
    pub fn FPDFAnnot_GetStringValue(
        &self,
        annot: &PdfiumAnnotation,
        key: &CString,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFAnnot_GetStringValue)(
                annot.into(),
                key.as_ptr(),
                buffer.as_mut_ptr(),
                buflen,
            )
        }
    }

    /// C documentation for FPDFAnnot_GetNumberValue:
    ///
    /// ```text
    /// Experimental API.
    /// Get the float value corresponding to |key| in |annot|'s dictionary. Writes
    /// value to |value| and returns True if |key| exists in the dictionary and
    /// |key|'s corresponding value is a number (FPDF_OBJECT_NUMBER), False
    /// otherwise.
    ///
    ///   annot  - handle to an annotation.
    ///   key    - the key to the requested dictionary entry, encoded in UTF-8.
    ///   value  - receives the value, must not be NULL.
    ///
    /// Returns True if value found, False otherwise.
    /// ```
    pub fn FPDFAnnot_GetNumberValue(
        &self,
        annot: &PdfiumAnnotation,
        key: &CString,
        value: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_GetNumberValue)(annot.into(), key.as_ptr(), value) })
    }

    /// C documentation for FPDFAnnot_SetAP:
    ///
    /// ```text
    /// Experimental API.
    /// Set the AP (appearance string) in |annot|'s dictionary for a given
    /// |appearanceMode|.
    ///
    ///   annot          - handle to an annotation.
    ///   appearanceMode - the appearance mode (normal, rollover or down) for which
    ///                    to get the AP.
    ///   value          - the string value to be set, encoded in UTF-16LE. If
    ///                    nullptr is passed, the AP is cleared for that mode. If the
    ///                    mode is Normal, APs for all modes are cleared.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_SetAP(
        &self,
        annot: &PdfiumAnnotation,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: &str,
    ) -> PdfiumResult<()> {
        let value = str_to_utf16le_vec(value);
        to_result(unsafe {
            (self.fn_FPDFAnnot_SetAP)(annot.into(), appearanceMode, value.as_ptr())
        })
    }

    /// C documentation for FPDFAnnot_GetAP:
    ///
    /// ```text
    /// Experimental API.
    /// Get the AP (appearance string) from |annot|'s dictionary for a given
    /// |appearanceMode|.
    /// |buffer| is only modified if |buflen| is large enough to hold the whole AP
    /// string. If |buflen| is smaller, the total size of the AP is still returned,
    /// but nothing is copied.
    /// If there is no appearance stream for |annot| in |appearanceMode|, an empty
    /// string is written to |buf| and 2 is returned.
    /// On other errors, nothing is written to |buffer| and 0 is returned.
    ///
    ///   annot          - handle to an annotation.
    ///   appearanceMode - the appearance mode (normal, rollover or down) for which
    ///                    to get the AP.
    ///   buffer         - buffer for holding the value string, encoded in UTF-16LE.
    ///   buflen         - length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    /// ```
    pub fn FPDFAnnot_GetAP(
        &self,
        annot: &PdfiumAnnotation,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFAnnot_GetAP)(annot.into(), appearanceMode, buffer.as_mut_ptr(), buflen)
        }
    }

    /// C documentation for FPDFAnnot_GetLinkedAnnot:
    ///
    /// ```text
    /// Experimental API.
    /// Get the annotation corresponding to |key| in |annot|'s dictionary. Common
    /// keys for linking annotations include "IRT" and "Popup". Must call
    /// FPDFPage_CloseAnnot() when the annotation returned by this function is no
    /// longer needed.
    ///
    ///   annot  - handle to an annotation.
    ///   key    - the key to the requested dictionary entry, encoded in UTF-8.
    ///
    /// Returns a handle to the linked annotation object, or NULL on failure.
    /// ```
    pub fn FPDFAnnot_GetLinkedAnnot(
        &self,
        annot: &PdfiumAnnotation,
        key: &CString,
    ) -> PdfiumResult<PdfiumAnnotation> {
        PdfiumAnnotation::new_from_handle(unsafe {
            (self.fn_FPDFAnnot_GetLinkedAnnot)(annot.into(), key.as_ptr())
        })
    }

    /// C documentation for FPDFAnnot_GetFlags:
    ///
    /// ```text
    /// Experimental API.
    /// Get the annotation flags of |annot|.
    ///
    ///   annot    - handle to an annotation.
    ///
    /// Returns the annotation flags.
    /// ```
    pub fn FPDFAnnot_GetFlags(&self, annot: &PdfiumAnnotation) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetFlags)(annot.into()) }
    }

    /// C documentation for FPDFAnnot_SetFlags:
    ///
    /// ```text
    /// Experimental API.
    /// Set the |annot|'s flags to be of the value |flags|.
    ///
    ///   annot      - handle to an annotation.
    ///   flags      - the flag values to be set.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_SetFlags(&self, annot: &PdfiumAnnotation, flags: i32) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_SetFlags)(annot.into(), flags) })
    }

    /// C documentation for FPDFAnnot_GetFormFieldFlags:
    ///
    /// ```text
    /// Experimental API.
    /// Get the annotation flags of |annot|.
    ///
    ///    hHandle     -   handle to the form fill module, returned by
    ///                    FPDFDOC_InitFormFillEnvironment().
    ///    annot       -   handle to an interactive form annotation.
    ///
    /// Returns the annotation flags specific to interactive forms.
    /// ```
    pub fn FPDFAnnot_GetFormFieldFlags(
        &self,
        handle: &PdfiumForm,
        annot: &PdfiumAnnotation,
    ) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetFormFieldFlags)(handle.into(), annot.into()) }
    }

    /// C documentation for FPDFAnnot_SetFormFieldFlags:
    ///
    /// ```text
    /// Experimental API.
    /// Sets the form field flags for an interactive form annotation.
    ///
    ///   handle       -   the handle to the form fill module, returned by
    ///                    FPDFDOC_InitFormFillEnvironment().
    ///   annot        -   handle to an interactive form annotation.
    ///   flags        -   the form field flags to be set.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_SetFormFieldFlags(
        &self,
        handle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        flags: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFAnnot_SetFormFieldFlags)(handle.into(), annot.into(), flags)
        })
    }

    /// C documentation for FPDFAnnot_GetFormFieldAtPoint:
    ///
    /// ```text
    /// Experimental API.
    /// Retrieves an interactive form annotation whose rectangle contains a given
    /// point on a page. Must call FPDFPage_CloseAnnot() when the annotation returned
    /// is no longer needed.
    ///
    ///
    ///    hHandle     -   handle to the form fill module, returned by
    ///                    FPDFDOC_InitFormFillEnvironment().
    ///    page        -   handle to the page, returned by FPDF_LoadPage function.
    ///    point       -   position in PDF "user space".
    ///
    /// Returns the interactive form annotation whose rectangle contains the given
    /// coordinates on the page. If there is no such annotation, return NULL.
    /// ```
    pub fn FPDFAnnot_GetFormFieldAtPoint(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        point: &FS_POINTF,
    ) -> PdfiumResult<PdfiumAnnotation> {
        PdfiumAnnotation::new_from_handle(unsafe {
            (self.fn_FPDFAnnot_GetFormFieldAtPoint)(hHandle.into(), page.into(), point)
        })
    }

    /// C documentation for FPDFAnnot_GetFormFieldName:
    ///
    /// ```text
    /// Experimental API.
    /// Gets the name of |annot|, which is an interactive form annotation.
    /// |buffer| is only modified if |buflen| is longer than the length of contents.
    /// In case of error, nothing will be added to |buffer| and the return value will
    /// be 0. Note that return value of empty string is 2 for "\\0\\0".
    ///
    ///    hHandle     -   handle to the form fill module, returned by
    ///                    FPDFDOC_InitFormFillEnvironment().
    ///    annot       -   handle to an interactive form annotation.
    ///    buffer      -   buffer for holding the name string, encoded in UTF-16LE.
    ///    buflen      -   length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    /// ```
    pub fn FPDFAnnot_GetFormFieldName(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFAnnot_GetFormFieldName)(
                hHandle.into(),
                annot.into(),
                buffer.as_mut_ptr(),
                buflen,
            )
        }
    }

    /// C documentation for FPDFAnnot_GetFormFieldAlternateName:
    ///
    /// ```text
    /// Experimental API.
    /// Gets the alternate name of |annot|, which is an interactive form annotation.
    /// |buffer| is only modified if |buflen| is longer than the length of contents.
    /// In case of error, nothing will be added to |buffer| and the return value will
    /// be 0. Note that return value of empty string is 2 for "\\0\\0".
    ///
    ///    hHandle     -   handle to the form fill module, returned by
    ///                    FPDFDOC_InitFormFillEnvironment().
    ///    annot       -   handle to an interactive form annotation.
    ///    buffer      -   buffer for holding the alternate name string, encoded in
    ///                    UTF-16LE.
    ///    buflen      -   length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    /// ```
    pub fn FPDFAnnot_GetFormFieldAlternateName(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFAnnot_GetFormFieldAlternateName)(
                hHandle.into(),
                annot.into(),
                buffer.as_mut_ptr(),
                buflen,
            )
        }
    }

    /// C documentation for FPDFAnnot_GetFormFieldType:
    ///
    /// ```text
    /// Experimental API.
    /// Gets the form field type of |annot|, which is an interactive form annotation.
    ///
    ///    hHandle     -   handle to the form fill module, returned by
    ///                    FPDFDOC_InitFormFillEnvironment().
    ///    annot       -   handle to an interactive form annotation.
    ///
    /// Returns the type of the form field (one of the FPDF_FORMFIELD_* values) on
    /// success. Returns -1 on error.
    /// See field types in fpdf_formfill.h.
    /// ```
    pub fn FPDFAnnot_GetFormFieldType(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
    ) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetFormFieldType)(hHandle.into(), annot.into()) }
    }

    /// C documentation for FPDFAnnot_GetFormFieldValue:
    ///
    /// ```text
    /// Experimental API.
    /// Gets the value of |annot|, which is an interactive form annotation.
    /// |buffer| is only modified if |buflen| is longer than the length of contents.
    /// In case of error, nothing will be added to |buffer| and the return value will
    /// be 0. Note that return value of empty string is 2 for "\\0\\0".
    ///
    ///    hHandle     -   handle to the form fill module, returned by
    ///                    FPDFDOC_InitFormFillEnvironment().
    ///    annot       -   handle to an interactive form annotation.
    ///    buffer      -   buffer for holding the value string, encoded in UTF-16LE.
    ///    buflen      -   length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    /// ```
    pub fn FPDFAnnot_GetFormFieldValue(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFAnnot_GetFormFieldValue)(
                hHandle.into(),
                annot.into(),
                buffer.as_mut_ptr(),
                buflen,
            )
        }
    }

    /// C documentation for FPDFAnnot_GetOptionCount:
    ///
    /// ```text
    /// Experimental API.
    /// Get the number of options in the |annot|'s "Opt" dictionary. Intended for
    /// use with listbox and combobox widget annotations.
    ///
    ///   hHandle - handle to the form fill module, returned by
    ///             FPDFDOC_InitFormFillEnvironment.
    ///   annot   - handle to an annotation.
    ///
    /// Returns the number of options in "Opt" dictionary on success. Return value
    /// will be -1 if annotation does not have an "Opt" dictionary or other error.
    /// ```
    pub fn FPDFAnnot_GetOptionCount(&self, hHandle: &PdfiumForm, annot: &PdfiumAnnotation) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetOptionCount)(hHandle.into(), annot.into()) }
    }

    /// C documentation for FPDFAnnot_GetOptionLabel:
    ///
    /// ```text
    /// Experimental API.
    /// Get the string value for the label of the option at |index| in |annot|'s
    /// "Opt" dictionary. Intended for use with listbox and combobox widget
    /// annotations. |buffer| is only modified if |buflen| is longer than the length
    /// of contents. If index is out of range or in case of other error, nothing
    /// will be added to |buffer| and the return value will be 0. Note that
    /// return value of empty string is 2 for "\\0\\0".
    ///
    ///   hHandle - handle to the form fill module, returned by
    ///             FPDFDOC_InitFormFillEnvironment.
    ///   annot   - handle to an annotation.
    ///   index   - numeric index of the option in the "Opt" array
    ///   buffer  - buffer for holding the value string, encoded in UTF-16LE.
    ///   buflen  - length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    /// If |annot| does not have an "Opt" array, |index| is out of range or if any
    /// other error occurs, returns 0.
    /// ```
    pub fn FPDFAnnot_GetOptionLabel(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        index: i32,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFAnnot_GetOptionLabel)(
                hHandle.into(),
                annot.into(),
                index,
                buffer.as_mut_ptr(),
                buflen,
            )
        }
    }

    /// C documentation for FPDFAnnot_IsOptionSelected:
    ///
    /// ```text
    /// Experimental API.
    /// Determine whether or not the option at |index| in |annot|'s "Opt" dictionary
    /// is selected. Intended for use with listbox and combobox widget annotations.
    ///
    ///   handle  - handle to the form fill module, returned by
    ///             FPDFDOC_InitFormFillEnvironment.
    ///   annot   - handle to an annotation.
    ///   index   - numeric index of the option in the "Opt" array.
    ///
    /// Returns true if the option at |index| in |annot|'s "Opt" dictionary is
    /// selected, false otherwise.
    /// ```
    pub fn FPDFAnnot_IsOptionSelected(
        &self,
        handle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        index: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFAnnot_IsOptionSelected)(handle.into(), annot.into(), index)
        })
    }

    /// C documentation for FPDFAnnot_GetFontSize:
    ///
    /// ```text
    /// Experimental API.
    /// Get the float value of the font size for an |annot| with variable text.
    /// If 0, the font is to be auto-sized: its size is computed as a function of
    /// the height of the annotation rectangle.
    ///
    ///   hHandle - handle to the form fill module, returned by
    ///             FPDFDOC_InitFormFillEnvironment.
    ///   annot   - handle to an annotation.
    ///   value   - Required. Float which will be set to font size on success.
    ///
    /// Returns true if the font size was set in |value|, false on error or if
    /// |value| not provided.
    /// ```
    pub fn FPDFAnnot_GetFontSize(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        value: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_GetFontSize)(hHandle.into(), annot.into(), value) })
    }

    /// C documentation for FPDFAnnot_SetFontColor:
    ///
    /// ```text
    /// Experimental API.
    /// Set the text color of an annotation.
    ///
    ///   handle   - handle to the form fill module, returned by
    ///              FPDFDOC_InitFormFillEnvironment.
    ///   annot    - handle to an annotation.
    ///   R        - the red component for the text color.
    ///   G        - the green component for the text color.
    ///   B        - the blue component for the text color.
    ///
    /// Returns true if successful.
    ///
    /// Currently supported subtypes: freetext.
    /// The range for the color components is 0 to 255.
    /// ```
    pub fn FPDFAnnot_SetFontColor(
        &self,
        handle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        R: u32,
        G: u32,
        B: u32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_SetFontColor)(handle.into(), annot.into(), R, G, B) })
    }

    /// C documentation for FPDFAnnot_GetFontColor:
    ///
    /// ```text
    /// Experimental API.
    /// Get the RGB value of the font color for an |annot| with variable text.
    ///
    ///   hHandle  - handle to the form fill module, returned by
    ///              FPDFDOC_InitFormFillEnvironment.
    ///   annot    - handle to an annotation.
    ///   R, G, B  - buffer to hold the RGB value of the color. Ranges from 0 to 255.
    ///
    /// Returns true if the font color was set, false on error or if the font
    /// color was not provided.
    /// ```
    pub fn FPDFAnnot_GetFontColor(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        R: &mut u32,
        G: &mut u32,
        B: &mut u32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFAnnot_GetFontColor)(hHandle.into(), annot.into(), R, G, B)
        })
    }

    /// C documentation for FPDFAnnot_IsChecked:
    ///
    /// ```text
    /// Experimental API.
    /// Determine if |annot| is a form widget that is checked. Intended for use with
    /// checkbox and radio button widgets.
    ///
    ///   hHandle - handle to the form fill module, returned by
    ///             FPDFDOC_InitFormFillEnvironment.
    ///   annot   - handle to an annotation.
    ///
    /// Returns true if |annot| is a form widget and is checked, false otherwise.
    /// ```
    pub fn FPDFAnnot_IsChecked(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_IsChecked)(hHandle.into(), annot.into()) })
    }

    /// C documentation for FPDFAnnot_SetFocusableSubtypes:
    ///
    /// ```text
    /// Experimental API.
    /// Set the list of focusable annotation subtypes. Annotations of subtype
    /// FPDF_ANNOT_WIDGET are by default focusable. New subtypes set using this API
    /// will override the existing subtypes.
    ///
    ///   hHandle  - handle to the form fill module, returned by
    ///              FPDFDOC_InitFormFillEnvironment.
    ///   subtypes - list of annotation subtype which can be tabbed over.
    ///   count    - total number of annotation subtype in list.
    /// Returns true if list of annotation subtype is set successfully, false
    /// otherwise.
    /// ```
    pub fn FPDFAnnot_SetFocusableSubtypes(
        &self,
        hHandle: &PdfiumForm,
        subtypes: &FPDF_ANNOTATION_SUBTYPE,
        count: usize,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFAnnot_SetFocusableSubtypes)(hHandle.into(), subtypes, count)
        })
    }

    /// C documentation for FPDFAnnot_GetFocusableSubtypesCount:
    ///
    /// ```text
    /// Experimental API.
    /// Get the count of focusable annotation subtypes as set by host
    /// for a |hHandle|.
    ///
    ///   hHandle  - handle to the form fill module, returned by
    ///              FPDFDOC_InitFormFillEnvironment.
    /// Returns the count of focusable annotation subtypes or -1 on error.
    /// Note : Annotations of type FPDF_ANNOT_WIDGET are by default focusable.
    /// ```
    pub fn FPDFAnnot_GetFocusableSubtypesCount(&self, hHandle: &PdfiumForm) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetFocusableSubtypesCount)(hHandle.into()) }
    }

    /// C documentation for FPDFAnnot_GetFocusableSubtypes:
    ///
    /// ```text
    /// Experimental API.
    /// Get the list of focusable annotation subtype as set by host.
    ///
    ///   hHandle  - handle to the form fill module, returned by
    ///              FPDFDOC_InitFormFillEnvironment.
    ///   subtypes - receives the list of annotation subtype which can be tabbed
    ///              over. Caller must have allocated |subtypes| more than or
    ///              equal to the count obtained from
    ///              FPDFAnnot_GetFocusableSubtypesCount() API.
    ///   count    - size of |subtypes|.
    /// Returns true on success and set list of annotation subtype to |subtypes|,
    /// false otherwise.
    /// Note : Annotations of type FPDF_ANNOT_WIDGET are by default focusable.
    /// ```
    pub fn FPDFAnnot_GetFocusableSubtypes(
        &self,
        hHandle: &PdfiumForm,
        subtypes: &mut FPDF_ANNOTATION_SUBTYPE,
        count: usize,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFAnnot_GetFocusableSubtypes)(hHandle.into(), subtypes, count)
        })
    }

    /// C documentation for FPDFAnnot_GetLink:
    ///
    /// ```text
    /// Experimental API.
    /// Gets FPDF_LINK object for |annot|. Intended to use for link annotations.
    ///
    ///   annot   - handle to an annotation.
    ///
    /// Returns FPDF_LINK from the FPDF_ANNOTATION and NULL on failure,
    /// if the input annot is NULL or input annot's subtype is not link.
    /// ```
    pub fn FPDFAnnot_GetLink(&self, annot: &PdfiumAnnotation) -> FPDF_LINK {
        unsafe { (self.fn_FPDFAnnot_GetLink)(annot.into()) }
    }

    /// C documentation for FPDFAnnot_GetFormControlCount:
    ///
    /// ```text
    /// Experimental API.
    /// Gets the count of annotations in the |annot|'s control group.
    /// A group of interactive form annotations is collectively called a form
    /// control group. Here, |annot|, an interactive form annotation, should be
    /// either a radio button or a checkbox.
    ///
    ///   hHandle - handle to the form fill module, returned by
    ///             FPDFDOC_InitFormFillEnvironment.
    ///   annot   - handle to an annotation.
    ///
    /// Returns number of controls in its control group or -1 on error.
    /// ```
    pub fn FPDFAnnot_GetFormControlCount(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
    ) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetFormControlCount)(hHandle.into(), annot.into()) }
    }

    /// C documentation for FPDFAnnot_GetFormControlIndex:
    ///
    /// ```text
    /// Experimental API.
    /// Gets the index of |annot| in |annot|'s control group.
    /// A group of interactive form annotations is collectively called a form
    /// control group. Here, |annot|, an interactive form annotation, should be
    /// either a radio button or a checkbox.
    ///
    ///   hHandle - handle to the form fill module, returned by
    ///             FPDFDOC_InitFormFillEnvironment.
    ///   annot   - handle to an annotation.
    ///
    /// Returns index of a given |annot| in its control group or -1 on error.
    /// ```
    pub fn FPDFAnnot_GetFormControlIndex(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
    ) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetFormControlIndex)(hHandle.into(), annot.into()) }
    }

    /// C documentation for FPDFAnnot_GetFormFieldExportValue:
    ///
    /// ```text
    /// Experimental API.
    /// Gets the export value of |annot| which is an interactive form annotation.
    /// Intended for use with radio button and checkbox widget annotations.
    /// |buffer| is only modified if |buflen| is longer than the length of contents.
    /// In case of error, nothing will be added to |buffer| and the return value
    /// will be 0. Note that return value of empty string is 2 for "\\0\\0".
    ///
    ///    hHandle     -   handle to the form fill module, returned by
    ///                    FPDFDOC_InitFormFillEnvironment().
    ///    annot       -   handle to an interactive form annotation.
    ///    buffer      -   buffer for holding the value string, encoded in UTF-16LE.
    ///    buflen      -   length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    /// ```
    pub fn FPDFAnnot_GetFormFieldExportValue(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFAnnot_GetFormFieldExportValue)(
                hHandle.into(),
                annot.into(),
                buffer.as_mut_ptr(),
                buflen,
            )
        }
    }

    /// C documentation for FPDFAnnot_SetURI:
    ///
    /// ```text
    /// Experimental API.
    /// Add a URI action to |annot|, overwriting the existing action, if any.
    ///
    ///   annot  - handle to a link annotation.
    ///   uri    - the URI to be set, encoded in 7-bit ASCII.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAnnot_SetURI(
        &self,
        annot: &PdfiumAnnotation,
        uri: Option<&[i8]>,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_SetURI)(annot.into(), to_char_ptr(uri)) })
    }

    /// C documentation for FPDFAnnot_GetFileAttachment:
    ///
    /// ```text
    /// Experimental API.
    /// Get the attachment from |annot|.
    ///
    ///   annot - handle to a file annotation.
    ///
    /// Returns the handle to the attachment object, or NULL on failure.
    /// ```
    pub fn FPDFAnnot_GetFileAttachment(
        &self,
        annot: &PdfiumAnnotation,
    ) -> PdfiumResult<PdfiumAttachment> {
        PdfiumAttachment::new_from_handle(unsafe {
            (self.fn_FPDFAnnot_GetFileAttachment)(annot.into())
        })
    }

    /// C documentation for FPDFAnnot_AddFileAttachment:
    ///
    /// ```text
    /// Experimental API.
    /// Add an embedded file with |name| to |annot|.
    ///
    ///   annot    - handle to a file annotation.
    ///   name     - name of the new attachment.
    ///
    /// Returns a handle to the new attachment object, or NULL on failure.
    /// ```
    pub fn FPDFAnnot_AddFileAttachment(
        &self,
        annot: &PdfiumAnnotation,
        name: &str,
    ) -> PdfiumResult<PdfiumAttachment> {
        let name = str_to_utf16le_vec(name);
        PdfiumAttachment::new_from_handle(unsafe {
            (self.fn_FPDFAnnot_AddFileAttachment)(annot.into(), name.as_ptr())
        })
    }

    /// C documentation for FPDFDoc_GetAttachmentCount:
    ///
    /// ```text
    /// Experimental API.
    /// Get the number of embedded files in |document|.
    ///
    ///   document - handle to a document.
    ///
    /// Returns the number of embedded files in |document|.
    /// ```
    pub fn FPDFDoc_GetAttachmentCount(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDFDoc_GetAttachmentCount)(document.into()) }
    }

    /// C documentation for FPDFDoc_AddAttachment:
    ///
    /// ```text
    /// Experimental API.
    /// Add an embedded file with |name| in |document|. If |name| is empty, or if
    /// |name| is the name of a existing embedded file in |document|, or if
    /// |document|'s embedded file name tree is too deep (i.e. |document| has too
    /// many embedded files already), then a new attachment will not be added.
    ///
    ///   document - handle to a document.
    ///   name     - name of the new attachment.
    ///
    /// Returns a handle to the new attachment object, or NULL on failure.
    /// ```
    pub fn FPDFDoc_AddAttachment(
        &self,
        document: &PdfiumDocument,
        name: &str,
    ) -> PdfiumResult<PdfiumAttachment> {
        let name = str_to_utf16le_vec(name);
        PdfiumAttachment::new_from_handle(unsafe {
            (self.fn_FPDFDoc_AddAttachment)(document.into(), name.as_ptr())
        })
    }

    /// C documentation for FPDFDoc_GetAttachment:
    ///
    /// ```text
    /// Experimental API.
    /// Get the embedded attachment at |index| in |document|. Note that the returned
    /// attachment handle is only valid while |document| is open.
    ///
    ///   document - handle to a document.
    ///   index    - the index of the requested embedded file.
    ///
    /// Returns the handle to the attachment object, or NULL on failure.
    /// ```
    pub fn FPDFDoc_GetAttachment(
        &self,
        document: &PdfiumDocument,
        index: i32,
    ) -> PdfiumResult<PdfiumAttachment> {
        PdfiumAttachment::new_from_handle(unsafe {
            (self.fn_FPDFDoc_GetAttachment)(document.into(), index)
        })
    }

    /// C documentation for FPDFDoc_DeleteAttachment:
    ///
    /// ```text
    /// Experimental API.
    /// Delete the embedded attachment at |index| in |document|. Note that this does
    /// not remove the attachment data from the PDF file; it simply removes the
    /// file's entry in the embedded files name tree so that it does not appear in
    /// the attachment list. This behavior may change in the future.
    ///
    ///   document - handle to a document.
    ///   index    - the index of the embedded file to be deleted.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFDoc_DeleteAttachment(
        &self,
        document: &PdfiumDocument,
        index: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFDoc_DeleteAttachment)(document.into(), index) })
    }

    /// C documentation for FPDFAttachment_GetName:
    ///
    /// ```text
    /// Experimental API.
    /// Get the name of the |attachment| file. |buffer| is only modified if |buflen|
    /// is longer than the length of the file name. On errors, |buffer| is unmodified
    /// and the returned length is 0.
    ///
    ///   attachment - handle to an attachment.
    ///   buffer     - buffer for holding the file name, encoded in UTF-16LE.
    ///   buflen     - length of the buffer in bytes.
    ///
    /// Returns the length of the file name in bytes.
    /// ```
    pub fn FPDFAttachment_GetName(
        &self,
        attachment: &PdfiumAttachment,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFAttachment_GetName)(attachment.into(), buffer.as_mut_ptr(), buflen) }
    }

    /// C documentation for FPDFAttachment_HasKey:
    ///
    /// ```text
    /// Experimental API.
    /// Check if the params dictionary of |attachment| has |key| as a key.
    ///
    ///   attachment - handle to an attachment.
    ///   key        - the key to look for, encoded in UTF-8.
    ///
    /// Returns true if |key| exists.
    /// ```
    pub fn FPDFAttachment_HasKey(&self, attachment: &PdfiumAttachment, key: &CString) -> i32 {
        unsafe { (self.fn_FPDFAttachment_HasKey)(attachment.into(), key.as_ptr()) }
    }

    /// C documentation for FPDFAttachment_GetValueType:
    ///
    /// ```text
    /// Experimental API.
    /// Get the type of the value corresponding to |key| in the params dictionary of
    /// the embedded |attachment|.
    ///
    ///   attachment - handle to an attachment.
    ///   key        - the key to look for, encoded in UTF-8.
    ///
    /// Returns the type of the dictionary value.
    /// ```
    pub fn FPDFAttachment_GetValueType(
        &self,
        attachment: &PdfiumAttachment,
        key: &CString,
    ) -> FPDF_OBJECT_TYPE {
        unsafe { (self.fn_FPDFAttachment_GetValueType)(attachment.into(), key.as_ptr()) }
    }

    /// C documentation for FPDFAttachment_SetStringValue:
    ///
    /// ```text
    /// Experimental API.
    /// Set the string value corresponding to |key| in the params dictionary of the
    /// embedded file |attachment|, overwriting the existing value if any. The value
    /// type should be FPDF_OBJECT_STRING after this function call succeeds.
    ///
    ///   attachment - handle to an attachment.
    ///   key        - the key to the dictionary entry, encoded in UTF-8.
    ///   value      - the string value to be set, encoded in UTF-16LE.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAttachment_SetStringValue(
        &self,
        attachment: &PdfiumAttachment,
        key: &CString,
        value: &str,
    ) -> PdfiumResult<()> {
        let value = str_to_utf16le_vec(value);
        to_result(unsafe {
            (self.fn_FPDFAttachment_SetStringValue)(attachment.into(), key.as_ptr(), value.as_ptr())
        })
    }

    /// C documentation for FPDFAttachment_GetStringValue:
    ///
    /// ```text
    /// Experimental API.
    /// Get the string value corresponding to |key| in the params dictionary of the
    /// embedded file |attachment|. |buffer| is only modified if |buflen| is longer
    /// than the length of the string value. Note that if |key| does not exist in the
    /// dictionary or if |key|'s corresponding value in the dictionary is not a
    /// string (i.e. the value is not of type FPDF_OBJECT_STRING or
    /// FPDF_OBJECT_NAME), then an empty string would be copied to |buffer| and the
    /// return value would be 2. On other errors, nothing would be added to |buffer|
    /// and the return value would be 0.
    ///
    ///   attachment - handle to an attachment.
    ///   key        - the key to the requested string value, encoded in UTF-8.
    ///   buffer     - buffer for holding the string value encoded in UTF-16LE.
    ///   buflen     - length of the buffer in bytes.
    ///
    /// Returns the length of the dictionary value string in bytes.
    /// ```
    pub fn FPDFAttachment_GetStringValue(
        &self,
        attachment: &PdfiumAttachment,
        key: &CString,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFAttachment_GetStringValue)(
                attachment.into(),
                key.as_ptr(),
                buffer.as_mut_ptr(),
                buflen,
            )
        }
    }

    /// C documentation for FPDFAttachment_SetFile:
    ///
    /// ```text
    /// Experimental API.
    /// Set the file data of |attachment|, overwriting the existing file data if any.
    /// The creation date and checksum will be updated, while all other dictionary
    /// entries will be deleted. Note that only contents with |len| smaller than
    /// INT_MAX is supported.
    ///
    ///   attachment - handle to an attachment.
    ///   contents   - buffer holding the file data to write to |attachment|.
    ///   len        - length of file data in bytes.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFAttachment_SetFile(
        &self,
        attachment: &PdfiumAttachment,
        document: &PdfiumDocument,
        contents: Option<&[u8]>,
        len: c_ulong,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFAttachment_SetFile)(
                attachment.into(),
                document.into(),
                to_void_ptr(contents),
                len,
            )
        })
    }

    /// C documentation for FPDFAttachment_GetFile:
    ///
    /// ```text
    /// Experimental API.
    /// Get the file data of |attachment|.
    /// When the attachment file data is readable, true is returned, and |out_buflen|
    /// is updated to indicate the file data size. |buffer| is only modified if
    /// |buflen| is non-null and long enough to contain the entire file data. Callers
    /// must check both the return value and the input |buflen| is no less than the
    /// returned |out_buflen| before using the data.
    ///
    /// Otherwise, when the attachment file data is unreadable or when |out_buflen|
    /// is null, false is returned and |buffer| and |out_buflen| remain unmodified.
    ///
    ///   attachment - handle to an attachment.
    ///   buffer     - buffer for holding the file data from |attachment|.
    ///   buflen     - length of the buffer in bytes.
    ///   out_buflen - pointer to the variable that will receive the minimum buffer
    ///                size to contain the file data of |attachment|.
    ///
    /// Returns true on success, false otherwise.
    /// ```
    pub fn FPDFAttachment_GetFile(
        &self,
        attachment: &PdfiumAttachment,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
        out_buflen: &mut c_ulong,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFAttachment_GetFile)(
                attachment.into(),
                to_void_ptr_mut(buffer),
                buflen,
                out_buflen,
            )
        })
    }

    /// C documentation for FPDFAttachment_GetSubtype:
    ///
    /// ```text
    /// Experimental API.
    /// Get the MIME type (Subtype) of the embedded file |attachment|. |buffer| is
    /// only modified if |buflen| is longer than the length of the MIME type string.
    /// If the Subtype is not found or if there is no file stream, an empty string
    /// would be copied to |buffer| and the return value would be 2. On other errors,
    /// nothing would be added to |buffer| and the return value would be 0.
    ///
    ///   attachment - handle to an attachment.
    ///   buffer     - buffer for holding the MIME type string encoded in UTF-16LE.
    ///   buflen     - length of the buffer in bytes.
    ///
    /// Returns the length of the MIME type string in bytes.
    /// ```
    pub fn FPDFAttachment_GetSubtype(
        &self,
        attachment: &PdfiumAttachment,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFAttachment_GetSubtype)(attachment.into(), buffer.as_mut_ptr(), buflen)
        }
    }

    /// C documentation for FPDFCatalog_IsTagged:
    ///
    /// ```text
    /// Experimental API.
    ///
    /// Determine if |document| represents a tagged PDF.
    ///
    /// For the definition of tagged PDF, See (see 10.7 "Tagged PDF" in PDF
    /// Reference 1.7).
    ///
    ///   document - handle to a document.
    ///
    /// Returns |true| iff |document| is a tagged PDF.
    /// ```
    pub fn FPDFCatalog_IsTagged(&self, document: &PdfiumDocument) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFCatalog_IsTagged)(document.into()) })
    }

    /// C documentation for FPDFCatalog_SetLanguage:
    ///
    /// ```text
    /// Experimental API.
    /// Sets the language of |document| to |language|.
    ///
    /// document - handle to a document.
    /// language - the language to set to.
    ///
    /// Returns TRUE on success.
    /// ```
    pub fn FPDFCatalog_SetLanguage(
        &self,
        document: &PdfiumDocument,
        language: &CString,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFCatalog_SetLanguage)(document.into(), language.as_ptr()) })
    }

    /// C documentation for FPDFAvail_Create:
    ///
    /// ```text
    /// Create a document availability provider.
    ///
    ///   file_avail - pointer to file availability interface.
    ///   file       - pointer to a file access interface.
    ///
    /// Returns a handle to the document availability provider, or NULL on error.
    ///
    /// FPDFAvail_Destroy() must be called when done with the availability provider.
    /// ```
    pub fn FPDFAvail_Create(
        &self,
        file_avail: &mut FX_FILEAVAIL,
        file: &mut Box<crate::PdfiumReader>,
    ) -> PdfiumResult<PdfiumAvailability> {
        PdfiumAvailability::new_from_handle(unsafe {
            (self.fn_FPDFAvail_Create)(file_avail, file.as_mut().into())
        })
    }

    /// C documentation for FPDFAvail_Destroy:
    ///
    /// ```text
    /// Destroy the |avail| document availability provider.
    ///
    ///   avail - handle to document availability provider to be destroyed.
    /// ```
    pub fn FPDFAvail_Destroy(&self, avail: &PdfiumAvailability) {
        unsafe { (self.fn_FPDFAvail_Destroy)(avail.into()) }
    }

    /// C documentation for FPDFAvail_IsDocAvail:
    ///
    /// ```text
    /// Checks if the document is ready for loading, if not, gets download hints.
    ///
    ///   avail - handle to document availability provider.
    ///   hints - pointer to a download hints interface.
    ///
    /// Returns one of:
    ///   PDF_DATA_ERROR: A common error is returned. Data availability unknown.
    ///   PDF_DATA_NOTAVAIL: Data not yet available.
    ///   PDF_DATA_AVAIL: Data available.
    ///
    /// Applications should call this function whenever new data arrives, and process
    /// all the generated download hints, if any, until the function returns
    /// |PDF_DATA_ERROR| or |PDF_DATA_AVAIL|.
    /// if hints is nullptr, the function just check current document availability.
    ///
    /// Once all data is available, call FPDFAvail_GetDocument() to get a document
    /// handle.
    /// ```
    pub fn FPDFAvail_IsDocAvail(
        &self,
        avail: &PdfiumAvailability,
        hints: &mut FX_DOWNLOADHINTS,
    ) -> i32 {
        unsafe { (self.fn_FPDFAvail_IsDocAvail)(avail.into(), hints) }
    }

    /// C documentation for FPDFAvail_GetDocument:
    ///
    /// ```text
    /// Get document from the availability provider.
    ///
    ///   avail    - handle to document availability provider.
    ///   password - password for decrypting the PDF file. Optional.
    ///
    /// Returns a handle to the document.
    ///
    /// When FPDFAvail_IsDocAvail() returns TRUE, call FPDFAvail_GetDocument() to
    /// retrieve the document handle.
    /// See the comments for FPDF_LoadDocument() regarding the encoding for
    /// |password|.
    /// ```
    pub fn FPDFAvail_GetDocument(
        &self,
        avail: &PdfiumAvailability,
        password: &CString,
    ) -> FPDF_DOCUMENT {
        unsafe { (self.fn_FPDFAvail_GetDocument)(avail.into(), password.as_ptr()) }
    }

    /// C documentation for FPDFAvail_GetFirstPageNum:
    ///
    /// ```text
    /// Get the page number for the first available page in a linearized PDF.
    ///
    ///   doc - document handle.
    ///
    /// Returns the zero-based index for the first available page.
    ///
    /// For most linearized PDFs, the first available page will be the first page,
    /// however, some PDFs might make another page the first available page.
    /// For non-linearized PDFs, this function will always return zero.
    /// ```
    pub fn FPDFAvail_GetFirstPageNum(&self, doc: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDFAvail_GetFirstPageNum)(doc.into()) }
    }

    /// C documentation for FPDFAvail_IsPageAvail:
    ///
    /// ```text
    /// Check if |page_index| is ready for loading, if not, get the
    /// |FX_DOWNLOADHINTS|.
    ///
    ///   avail      - handle to document availability provider.
    ///   page_index - index number of the page. Zero for the first page.
    ///   hints      - pointer to a download hints interface. Populated if
    ///                |page_index| is not available.
    ///
    /// Returns one of:
    ///   PDF_DATA_ERROR: A common error is returned. Data availability unknown.
    ///   PDF_DATA_NOTAVAIL: Data not yet available.
    ///   PDF_DATA_AVAIL: Data available.
    ///
    /// This function can be called only after FPDFAvail_GetDocument() is called.
    /// Applications should call this function whenever new data arrives and process
    /// all the generated download |hints|, if any, until this function returns
    /// |PDF_DATA_ERROR| or |PDF_DATA_AVAIL|. Applications can then perform page
    /// loading.
    /// if hints is nullptr, the function just check current availability of
    /// specified page.
    /// ```
    pub fn FPDFAvail_IsPageAvail(
        &self,
        avail: &PdfiumAvailability,
        page_index: i32,
        hints: &mut FX_DOWNLOADHINTS,
    ) -> i32 {
        unsafe { (self.fn_FPDFAvail_IsPageAvail)(avail.into(), page_index, hints) }
    }

    /// C documentation for FPDFAvail_IsFormAvail:
    ///
    /// ```text
    /// Check if form data is ready for initialization, if not, get the
    /// |FX_DOWNLOADHINTS|.
    ///
    ///   avail - handle to document availability provider.
    ///   hints - pointer to a download hints interface. Populated if form is not
    ///           ready for initialization.
    ///
    /// Returns one of:
    ///   PDF_FORM_ERROR: A common eror, in general incorrect parameters.
    ///   PDF_FORM_NOTAVAIL: Data not available.
    ///   PDF_FORM_AVAIL: Data available.
    ///   PDF_FORM_NOTEXIST: No form data.
    ///
    /// This function can be called only after FPDFAvail_GetDocument() is called.
    /// The application should call this function whenever new data arrives and
    /// process all the generated download |hints|, if any, until the function
    /// |PDF_FORM_ERROR|, |PDF_FORM_AVAIL| or |PDF_FORM_NOTEXIST|.
    /// if hints is nullptr, the function just check current form availability.
    ///
    /// Applications can then perform page loading. It is recommend to call
    /// FPDFDOC_InitFormFillEnvironment() when |PDF_FORM_AVAIL| is returned.
    /// ```
    pub fn FPDFAvail_IsFormAvail(
        &self,
        avail: &PdfiumAvailability,
        hints: &mut FX_DOWNLOADHINTS,
    ) -> i32 {
        unsafe { (self.fn_FPDFAvail_IsFormAvail)(avail.into(), hints) }
    }

    /// C documentation for FPDFAvail_IsLinearized:
    ///
    /// ```text
    /// Check whether a document is a linearized PDF.
    ///
    ///   avail - handle to document availability provider.
    ///
    /// Returns one of:
    ///   PDF_LINEARIZED
    ///   PDF_NOT_LINEARIZED
    ///   PDF_LINEARIZATION_UNKNOWN
    ///
    /// FPDFAvail_IsLinearized() will return |PDF_LINEARIZED| or |PDF_NOT_LINEARIZED|
    /// when we have 1k  of data. If the files size less than 1k, it returns
    /// |PDF_LINEARIZATION_UNKNOWN| as there is insufficient information to determine
    /// if the PDF is linearlized.
    /// ```
    pub fn FPDFAvail_IsLinearized(&self, avail: &PdfiumAvailability) -> i32 {
        unsafe { (self.fn_FPDFAvail_IsLinearized)(avail.into()) }
    }

    /// C documentation for FPDFBookmark_GetFirstChild:
    ///
    /// ```text
    /// Get the first child of |bookmark|, or the first top-level bookmark item.
    ///
    ///   document - handle to the document.
    ///   bookmark - handle to the current bookmark. Pass NULL for the first top
    ///              level item.
    ///
    /// Returns a handle to the first child of |bookmark| or the first top-level
    /// bookmark item. NULL if no child or top-level bookmark found.
    /// Note that another name for the bookmarks is the document outline, as
    /// described in ISO 32000-1:2008, section 12.3.3.
    /// ```
    pub fn FPDFBookmark_GetFirstChild(
        &self,
        document: &PdfiumDocument,
        bookmark: &PdfiumBookmark,
    ) -> PdfiumResult<PdfiumBookmark> {
        PdfiumBookmark::new_from_handle(unsafe {
            (self.fn_FPDFBookmark_GetFirstChild)(document.into(), bookmark.into())
        })
    }

    /// C documentation for FPDFBookmark_GetNextSibling:
    ///
    /// ```text
    /// Get the next sibling of |bookmark|.
    ///
    ///   document - handle to the document.
    ///   bookmark - handle to the current bookmark.
    ///
    /// Returns a handle to the next sibling of |bookmark|, or NULL if this is the
    /// last bookmark at this level.
    ///
    /// Note that the caller is responsible for handling circular bookmark
    /// references, as may arise from malformed documents.
    /// ```
    pub fn FPDFBookmark_GetNextSibling(
        &self,
        document: &PdfiumDocument,
        bookmark: &PdfiumBookmark,
    ) -> PdfiumResult<PdfiumBookmark> {
        PdfiumBookmark::new_from_handle(unsafe {
            (self.fn_FPDFBookmark_GetNextSibling)(document.into(), bookmark.into())
        })
    }

    /// C documentation for FPDFBookmark_GetTitle:
    ///
    /// ```text
    /// Get the title of |bookmark|.
    ///
    ///   bookmark - handle to the bookmark.
    ///   buffer   - buffer for the title. May be NULL.
    ///   buflen   - the length of the buffer in bytes. May be 0.
    ///
    /// Returns the number of bytes in the title, including the terminating NUL
    /// character. The number of bytes is returned regardless of the |buffer| and
    /// |buflen| parameters.
    ///
    /// Regardless of the platform, the |buffer| is always in UTF-16LE encoding. The
    /// string is terminated by a UTF16 NUL character. If |buflen| is less than the
    /// required length, or |buffer| is NULL, |buffer| will not be modified.
    /// ```
    pub fn FPDFBookmark_GetTitle(
        &self,
        bookmark: &PdfiumBookmark,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFBookmark_GetTitle)(bookmark.into(), to_void_ptr_mut(buffer), buflen) }
    }

    /// C documentation for FPDFBookmark_GetCount:
    ///
    /// ```text
    /// Experimental API.
    /// Get the number of chlidren of |bookmark|.
    ///
    ///   bookmark - handle to the bookmark.
    ///
    /// Returns a signed integer that represents the number of sub-items the given
    /// bookmark has. If the value is positive, child items shall be shown by default
    /// (open state). If the value is negative, child items shall be hidden by
    /// default (closed state). Please refer to PDF 32000-1:2008, Table 153.
    /// Returns 0 if the bookmark has no children or is invalid.
    /// ```
    pub fn FPDFBookmark_GetCount(&self, bookmark: &PdfiumBookmark) -> i32 {
        unsafe { (self.fn_FPDFBookmark_GetCount)(bookmark.into()) }
    }

    /// C documentation for FPDFBookmark_Find:
    ///
    /// ```text
    /// Find the bookmark with |title| in |document|.
    ///
    ///   document - handle to the document.
    ///   title    - the UTF-16LE encoded Unicode title for which to search.
    ///
    /// Returns the handle to the bookmark, or NULL if |title| can't be found.
    ///
    /// FPDFBookmark_Find() will always return the first bookmark found even if
    /// multiple bookmarks have the same |title|.
    /// ```
    pub fn FPDFBookmark_Find(
        &self,
        document: &PdfiumDocument,
        title: &str,
    ) -> PdfiumResult<PdfiumBookmark> {
        let title = str_to_utf16le_vec(title);
        PdfiumBookmark::new_from_handle(unsafe {
            (self.fn_FPDFBookmark_Find)(document.into(), title.as_ptr())
        })
    }

    /// C documentation for FPDFBookmark_GetDest:
    ///
    /// ```text
    /// Get the destination associated with |bookmark|.
    ///
    ///   document - handle to the document.
    ///   bookmark - handle to the bookmark.
    ///
    /// Returns the handle to the destination data, or NULL if no destination is
    /// associated with |bookmark|.
    /// ```
    pub fn FPDFBookmark_GetDest(
        &self,
        document: &PdfiumDocument,
        bookmark: &PdfiumBookmark,
    ) -> PdfiumResult<PdfiumDestination> {
        PdfiumDestination::new_from_handle(unsafe {
            (self.fn_FPDFBookmark_GetDest)(document.into(), bookmark.into())
        })
    }

    /// C documentation for FPDFBookmark_GetAction:
    ///
    /// ```text
    /// Get the action associated with |bookmark|.
    ///
    ///   bookmark - handle to the bookmark.
    ///
    /// Returns the handle to the action data, or NULL if no action is associated
    /// with |bookmark|.
    /// If this function returns a valid handle, it is valid as long as |bookmark| is
    /// valid.
    /// If this function returns NULL, FPDFBookmark_GetDest() should be called to get
    /// the |bookmark| destination data.
    /// ```
    pub fn FPDFBookmark_GetAction(&self, bookmark: &PdfiumBookmark) -> PdfiumResult<PdfiumAction> {
        PdfiumAction::new_from_handle(unsafe { (self.fn_FPDFBookmark_GetAction)(bookmark.into()) })
    }

    /// C documentation for FPDFAction_GetType:
    ///
    /// ```text
    /// Get the type of |action|.
    ///
    ///   action - handle to the action.
    ///
    /// Returns one of:
    ///   PDFACTION_UNSUPPORTED
    ///   PDFACTION_GOTO
    ///   PDFACTION_REMOTEGOTO
    ///   PDFACTION_URI
    ///   PDFACTION_LAUNCH
    /// ```
    pub fn FPDFAction_GetType(&self, action: &PdfiumAction) -> c_ulong {
        unsafe { (self.fn_FPDFAction_GetType)(action.into()) }
    }

    /// C documentation for FPDFAction_GetDest:
    ///
    /// ```text
    /// Get the destination of |action|.
    ///
    ///   document - handle to the document.
    ///   action   - handle to the action. |action| must be a |PDFACTION_GOTO| or
    ///              |PDFACTION_REMOTEGOTO|.
    ///
    /// Returns a handle to the destination data, or NULL on error, typically
    /// because the arguments were bad or the action was of the wrong type.
    ///
    /// In the case of |PDFACTION_REMOTEGOTO|, you must first call
    /// FPDFAction_GetFilePath(), then load the document at that path, then pass
    /// the document handle from that document as |document| to FPDFAction_GetDest().
    /// ```
    pub fn FPDFAction_GetDest(
        &self,
        document: &PdfiumDocument,
        action: &PdfiumAction,
    ) -> PdfiumResult<PdfiumDestination> {
        PdfiumDestination::new_from_handle(unsafe {
            (self.fn_FPDFAction_GetDest)(document.into(), action.into())
        })
    }

    /// C documentation for FPDFAction_GetFilePath:
    ///
    /// ```text
    /// Get the file path of |action|.
    ///
    ///   action - handle to the action. |action| must be a |PDFACTION_LAUNCH| or
    ///            |PDFACTION_REMOTEGOTO|.
    ///   buffer - a buffer for output the path string. May be NULL.
    ///   buflen - the length of the buffer, in bytes. May be 0.
    ///
    /// Returns the number of bytes in the file path, including the trailing NUL
    /// character, or 0 on error, typically because the arguments were bad or the
    /// action was of the wrong type.
    ///
    /// Regardless of the platform, the |buffer| is always in UTF-8 encoding.
    /// If |buflen| is less than the returned length, or |buffer| is NULL, |buffer|
    /// will not be modified.
    /// ```
    pub fn FPDFAction_GetFilePath(
        &self,
        action: &PdfiumAction,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFAction_GetFilePath)(action.into(), to_void_ptr_mut(buffer), buflen) }
    }

    /// C documentation for FPDFAction_GetURIPath:
    ///
    /// ```text
    /// Get the URI path of |action|.
    ///
    ///   document - handle to the document.
    ///   action   - handle to the action. Must be a |PDFACTION_URI|.
    ///   buffer   - a buffer for the path string. May be NULL.
    ///   buflen   - the length of the buffer, in bytes. May be 0.
    ///
    /// Returns the number of bytes in the URI path, including the trailing NUL
    /// character, or 0 on error, typically because the arguments were bad or the
    /// action was of the wrong type.
    ///
    /// The |buffer| may contain badly encoded data. The caller should validate the
    /// output. e.g. Check to see if it is UTF-8.
    ///
    /// If |buflen| is less than the returned length, or |buffer| is NULL, |buffer|
    /// will not be modified.
    ///
    /// Historically, the documentation for this API claimed |buffer| is always
    /// encoded in 7-bit ASCII, but did not actually enforce it.
    /// https://pdfium.googlesource.com/pdfium.git/+/d609e84cee2e14a18333247485af91df48a40592
    /// added that enforcement, but that did not work well for real world PDFs that
    /// used UTF-8. As of this writing, this API reverted back to its original
    /// behavior prior to commit d609e84cee.
    /// ```
    pub fn FPDFAction_GetURIPath(
        &self,
        document: &PdfiumDocument,
        action: &PdfiumAction,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFAction_GetURIPath)(
                document.into(),
                action.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDFDest_GetDestPageIndex:
    ///
    /// ```text
    /// Get the page index of |dest|.
    ///
    ///   document - handle to the document.
    ///   dest     - handle to the destination.
    ///
    /// Returns the 0-based page index containing |dest|. Returns -1 on error.
    /// ```
    pub fn FPDFDest_GetDestPageIndex(
        &self,
        document: &PdfiumDocument,
        dest: &PdfiumDestination,
    ) -> i32 {
        unsafe { (self.fn_FPDFDest_GetDestPageIndex)(document.into(), dest.into()) }
    }

    /// C documentation for FPDFDest_GetView:
    ///
    /// ```text
    /// Experimental API.
    /// Get the view (fit type) specified by |dest|.
    ///
    ///   dest         - handle to the destination.
    ///   pNumParams   - receives the number of view parameters, which is at most 4.
    ///   pParams      - buffer to write the view parameters. Must be at least 4
    ///                  FS_FLOATs long.
    /// Returns one of the PDFDEST_VIEW_* constants, PDFDEST_VIEW_UNKNOWN_MODE if
    /// |dest| does not specify a view.
    /// ```
    pub fn FPDFDest_GetView(
        &self,
        dest: &PdfiumDestination,
        pNumParams: &mut c_ulong,
        pParams: &mut f32,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFDest_GetView)(dest.into(), pNumParams, pParams) }
    }

    /// C documentation for FPDFDest_GetLocationInPage:
    ///
    /// ```text
    /// Get the (x, y, zoom) location of |dest| in the destination page, if the
    /// destination is in [page /XYZ x y zoom] syntax.
    ///
    ///   dest       - handle to the destination.
    ///   hasXVal    - out parameter; true if the x value is not null
    ///   hasYVal    - out parameter; true if the y value is not null
    ///   hasZoomVal - out parameter; true if the zoom value is not null
    ///   x          - out parameter; the x coordinate, in page coordinates.
    ///   y          - out parameter; the y coordinate, in page coordinates.
    ///   zoom       - out parameter; the zoom value.
    /// Returns TRUE on successfully reading the /XYZ value.
    ///
    /// Note the [x, y, zoom] values are only set if the corresponding hasXVal,
    /// hasYVal or hasZoomVal flags are true.
    /// ```
    pub fn FPDFDest_GetLocationInPage(
        &self,
        dest: &PdfiumDestination,
        hasXVal: &mut FPDF_BOOL,
        hasYVal: &mut FPDF_BOOL,
        hasZoomVal: &mut FPDF_BOOL,
        x: &mut f32,
        y: &mut f32,
        zoom: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFDest_GetLocationInPage)(
                dest.into(),
                hasXVal,
                hasYVal,
                hasZoomVal,
                x,
                y,
                zoom,
            )
        })
    }

    /// C documentation for FPDF_GetPageAAction:
    ///
    /// ```text
    /// Experimental API
    /// Gets an additional-action from |page|.
    ///
    ///   page      - handle to the page, as returned by FPDF_LoadPage().
    ///   aa_type   - the type of the page object's addtional-action, defined
    ///               in public/fpdf_formfill.h
    ///
    ///   Returns the handle to the action data, or NULL if there is no
    ///   additional-action of type |aa_type|.
    ///   If this function returns a valid handle, it is valid as long as |page| is
    ///   valid.
    /// ```
    pub fn FPDF_GetPageAAction(
        &self,
        page: &PdfiumPage,
        aa_type: i32,
    ) -> PdfiumResult<PdfiumAction> {
        PdfiumAction::new_from_handle(unsafe {
            (self.fn_FPDF_GetPageAAction)(page.into(), aa_type)
        })
    }

    /// C documentation for FPDF_GetFileIdentifier:
    ///
    /// ```text
    /// Experimental API.
    /// Get the file identifer defined in the trailer of |document|.
    ///
    ///   document - handle to the document.
    ///   id_type  - the file identifier type to retrieve.
    ///   buffer   - a buffer for the file identifier. May be NULL.
    ///   buflen   - the length of the buffer, in bytes. May be 0.
    ///
    /// Returns the number of bytes in the file identifier, including the NUL
    /// terminator.
    ///
    /// The |buffer| is always a byte string. The |buffer| is followed by a NUL
    /// terminator.  If |buflen| is less than the returned length, or |buffer| is
    /// NULL, |buffer| will not be modified.
    /// ```
    pub fn FPDF_GetFileIdentifier(
        &self,
        document: &PdfiumDocument,
        id_type: FPDF_FILEIDTYPE,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_GetFileIdentifier)(
                document.into(),
                id_type,
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDF_GetMetaText:
    ///
    /// ```text
    /// Get meta-data |tag| content from |document|.
    ///
    ///   document - handle to the document.
    ///   tag      - the tag to retrieve. The tag can be one of:
    ///                Title, Author, Subject, Keywords, Creator, Producer,
    ///                CreationDate, or ModDate.
    ///              For detailed explanations of these tags and their respective
    ///              values, please refer to PDF Reference 1.6, section 10.2.1,
    ///              'Document Information Dictionary'.
    ///   buffer   - a buffer for the tag. May be NULL.
    ///   buflen   - the length of the buffer, in bytes. May be 0.
    ///
    /// Returns the number of bytes in the tag, including trailing zeros.
    ///
    /// The |buffer| is always encoded in UTF-16LE. The |buffer| is followed by two
    /// bytes of zeros indicating the end of the string.  If |buflen| is less than
    /// the returned length, or |buffer| is NULL, |buffer| will not be modified.
    ///
    /// For linearized files, FPDFAvail_IsFormAvail must be called before this, and
    /// it must have returned PDF_FORM_AVAIL or PDF_FORM_NOTEXIST. Before that, there
    /// is no guarantee the metadata has been loaded.
    /// ```
    pub fn FPDF_GetMetaText(
        &self,
        document: &PdfiumDocument,
        tag: &CString,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_GetMetaText)(
                document.into(),
                tag.as_ptr(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDF_GetPageLabel:
    ///
    /// ```text
    /// Get the page label for |page_index| from |document|.
    ///
    ///   document    - handle to the document.
    ///   page_index  - the 0-based index of the page.
    ///   buffer      - a buffer for the page label. May be NULL.
    ///   buflen      - the length of the buffer, in bytes. May be 0.
    ///
    /// Returns the number of bytes in the page label, including trailing zeros.
    ///
    /// The |buffer| is always encoded in UTF-16LE. The |buffer| is followed by two
    /// bytes of zeros indicating the end of the string.  If |buflen| is less than
    /// the returned length, or |buffer| is NULL, |buffer| will not be modified.
    /// ```
    pub fn FPDF_GetPageLabel(
        &self,
        document: &PdfiumDocument,
        page_index: i32,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_GetPageLabel)(
                document.into(),
                page_index,
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDF_CreateNewDocument:
    ///
    /// ```text
    /// Create a new PDF document.
    ///
    /// Returns a handle to a new document, or NULL on failure.
    /// ```
    pub fn FPDF_CreateNewDocument(&self) -> FPDF_DOCUMENT {
        unsafe { (self.fn_FPDF_CreateNewDocument)() }
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

    /// C documentation for FPDF_MovePages:
    ///
    /// ```text
    /// Experimental API.
    /// Move the given pages to a new index position.
    ///
    ///  page_indices     - the ordered list of pages to move. No duplicates allowed.
    ///  page_indices_len - the number of elements in |page_indices|
    ///  dest_page_index  - the new index position to which the pages in
    ///                     |page_indices| are moved.
    ///
    /// Returns TRUE on success. If it returns FALSE, the document may be left in an
    /// indeterminate state.
    ///
    /// Example: The PDF document starts out with pages [A, B, C, D], with indices
    /// [0, 1, 2, 3].
    ///
    /// >  Move(doc, [3, 2], 2, 1); // returns true
    /// >  // The document has pages [A, D, C, B].
    /// >
    /// >  Move(doc, [0, 4, 3], 3, 1); // returns false
    /// >  // Returned false because index 4 is out of range.
    /// >
    /// >  Move(doc, [0, 3, 1], 3, 2); // returns false
    /// >  // Returned false because index 2 is out of range for 3 page indices.
    /// >
    /// >  Move(doc, [2, 2], 2, 0); // returns false
    /// >  // Returned false because [2, 2] contains duplicates.
    /// ```
    pub fn FPDF_MovePages(
        &self,
        document: &PdfiumDocument,
        page_indices: &i32,
        page_indices_len: c_ulong,
        dest_page_index: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_MovePages)(
                document.into(),
                page_indices,
                page_indices_len,
                dest_page_index,
            )
        })
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

    /// C documentation for FPDFPage_InsertObjectAtIndex:
    ///
    /// ```text
    /// Insert |page_object| into |page| at the specified |index|.
    ///
    ///   page        - handle to a page
    ///   page_object - handle to a page object as previously obtained by
    ///                 FPDFPageObj_CreateNew{Path|Rect}() or
    ///                 FPDFPageObj_New{Text|Image}Obj(). Ownership of the object
    ///                 is transferred back to PDFium.
    ///   index       - the index position to insert the object at. If index equals
    ///                 the current object count, the object will be appended to the
    ///                 end. If index is greater than the object count, the function
    ///                 will fail and return false.
    ///
    /// Returns true if successful.
    /// ```
    pub fn FPDFPage_InsertObjectAtIndex(
        &self,
        page: &PdfiumPage,
        page_object: &PdfiumPageObject,
        index: usize,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPage_InsertObjectAtIndex)(page.into(), page_object.into(), index)
        })
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

    /// C documentation for FPDFDoc_GetPageMode:
    ///
    /// ```text
    /// Get the document's PageMode.
    ///
    ///   doc - Handle to document.
    ///
    /// Returns one of the |PAGEMODE_*| flags defined above.
    ///
    /// The page mode defines how the document should be initially displayed.
    /// ```
    pub fn FPDFDoc_GetPageMode(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDFDoc_GetPageMode)(document.into()) }
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

    /// C documentation for FPDFDoc_GetJavaScriptActionCount:
    ///
    /// ```text
    /// Experimental API.
    /// Get the number of JavaScript actions in |document|.
    ///
    ///   document - handle to a document.
    ///
    /// Returns the number of JavaScript actions in |document| or -1 on error.
    /// ```
    pub fn FPDFDoc_GetJavaScriptActionCount(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDFDoc_GetJavaScriptActionCount)(document.into()) }
    }

    /// C documentation for FPDFDoc_GetJavaScriptAction:
    ///
    /// ```text
    /// Experimental API.
    /// Get the JavaScript action at |index| in |document|.
    ///
    ///   document - handle to a document.
    ///   index    - the index of the requested JavaScript action.
    ///
    /// Returns the handle to the JavaScript action, or NULL on failure.
    /// Caller owns the returned handle and must close it with
    /// FPDFDoc_CloseJavaScriptAction().
    /// ```
    pub fn FPDFDoc_GetJavaScriptAction(
        &self,
        document: &PdfiumDocument,
        index: i32,
    ) -> PdfiumResult<PdfiumJavascriptAction> {
        PdfiumJavascriptAction::new_from_handle(unsafe {
            (self.fn_FPDFDoc_GetJavaScriptAction)(document.into(), index)
        })
    }

    /// C documentation for FPDFDoc_CloseJavaScriptAction:
    ///
    /// ```text
    ///   javascript - Handle to a JavaScript action.
    /// ```
    pub fn FPDFDoc_CloseJavaScriptAction(&self, javascript: &PdfiumJavascriptAction) {
        unsafe { (self.fn_FPDFDoc_CloseJavaScriptAction)(javascript.into()) }
    }

    /// C documentation for FPDF_ImportPagesByIndex:
    ///
    /// ```text
    /// Experimental API.
    /// Import pages to a FPDF_DOCUMENT.
    ///
    ///   dest_doc     - The destination document for the pages.
    ///   src_doc      - The document to be imported.
    ///   page_indices - An array of page indices to be imported. The first page is
    ///                  zero. If |page_indices| is NULL, all pages from |src_doc|
    ///                  are imported.
    ///   length       - The length of the |page_indices| array.
    ///   index        - The page index at which to insert the first imported page
    ///                  into |dest_doc|. The first page is zero.
    ///
    /// Returns TRUE on success. Returns FALSE if any pages in |page_indices| is
    /// invalid.
    /// ```
    pub fn FPDF_ImportPagesByIndex(
        &self,
        dest_doc: &PdfiumDocument,
        src_doc: &PdfiumDocument,
        page_indices: &i32,
        length: c_ulong,
        index: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_ImportPagesByIndex)(
                dest_doc.into(),
                src_doc.into(),
                page_indices,
                length,
                index,
            )
        })
    }

    /// C documentation for FPDF_ImportPages:
    ///
    /// ```text
    /// Import pages to a FPDF_DOCUMENT.
    ///
    ///   dest_doc  - The destination document for the pages.
    ///   src_doc   - The document to be imported.
    ///   pagerange - A page range string, Such as "1,3,5-7". The first page is one.
    ///               If |pagerange| is NULL, all pages from |src_doc| are imported.
    ///   index     - The page index at which to insert the first imported page into
    ///               |dest_doc|. The first page is zero.
    ///
    /// Returns TRUE on success. Returns FALSE if any pages in |pagerange| is
    /// invalid or if |pagerange| cannot be read.
    /// ```
    pub fn FPDF_ImportPages(
        &self,
        dest_doc: &PdfiumDocument,
        src_doc: &PdfiumDocument,
        pagerange: &CString,
        index: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_ImportPages)(dest_doc.into(), src_doc.into(), pagerange.as_ptr(), index)
        })
    }

    /// C documentation for FPDF_ImportNPagesToOne:
    ///
    /// ```text
    /// Experimental API.
    /// Create a new document from |src_doc|.  The pages of |src_doc| will be
    /// combined to provide |num_pages_on_x_axis x num_pages_on_y_axis| pages per
    /// |output_doc| page.
    ///
    ///   src_doc             - The document to be imported.
    ///   output_width        - The output page width in PDF "user space" units.
    ///   output_height       - The output page height in PDF "user space" units.
    ///   num_pages_on_x_axis - The number of pages on X Axis.
    ///   num_pages_on_y_axis - The number of pages on Y Axis.
    ///
    /// Return value:
    ///   A handle to the created document, or NULL on failure.
    ///
    /// Comments:
    ///   number of pages per page = num_pages_on_x_axis * num_pages_on_y_axis
    /// ```
    pub fn FPDF_ImportNPagesToOne(
        &self,
        src_doc: &PdfiumDocument,
        output_width: f32,
        output_height: f32,
        num_pages_on_x_axis: usize,
        num_pages_on_y_axis: usize,
    ) -> FPDF_DOCUMENT {
        unsafe {
            (self.fn_FPDF_ImportNPagesToOne)(
                src_doc.into(),
                output_width,
                output_height,
                num_pages_on_x_axis,
                num_pages_on_y_axis,
            )
        }
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

    /// C documentation for FPDF_NewFormObjectFromXObject:
    ///
    /// ```text
    /// Experimental API.
    /// Create a new form object from an FPDF_XOBJECT object.
    ///
    /// Returns a new form object on success, or NULL on failure. Caller owns the
    /// newly created object.
    /// ```
    pub fn FPDF_NewFormObjectFromXObject(
        &self,
        xobject: &PdfiumXObject,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDF_NewFormObjectFromXObject)(xobject.into())
        })
    }

    /// C documentation for FPDF_CopyViewerPreferences:
    ///
    /// ```text
    /// Copy the viewer preferences from |src_doc| into |dest_doc|.
    ///
    ///   dest_doc - Document to write the viewer preferences into.
    ///   src_doc  - Document to read the viewer preferences from.
    ///
    /// Returns TRUE on success.
    /// ```
    pub fn FPDF_CopyViewerPreferences(
        &self,
        dest_doc: &PdfiumDocument,
        src_doc: &PdfiumDocument,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_CopyViewerPreferences)(dest_doc.into(), src_doc.into()) })
    }

    /// C documentation for FPDF_RenderPageBitmapWithColorScheme_Start:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_RenderPageBitmapWithColorScheme_Start
    ///          Start to render page contents to a device independent bitmap
    ///          progressively with a specified color scheme for the content.
    /// Parameters:
    ///          bitmap       -   Handle to the device independent bitmap (as the
    ///                           output buffer). Bitmap handle can be created by
    ///                           FPDFBitmap_Create function.
    ///          page         -   Handle to the page as returned by FPDF_LoadPage
    ///                           function.
    ///          start_x      -   Left pixel position of the display area in the
    ///                           bitmap coordinate.
    ///          start_y      -   Top pixel position of the display area in the
    ///                           bitmap coordinate.
    ///          size_x       -   Horizontal size (in pixels) for displaying the
    ///                           page.
    ///          size_y       -   Vertical size (in pixels) for displaying the page.
    ///          rotate       -   Page orientation: 0 (normal), 1 (rotated 90
    ///                           degrees clockwise), 2 (rotated 180 degrees),
    ///                           3 (rotated 90 degrees counter-clockwise).
    ///          flags        -   0 for normal display, or combination of flags
    ///                           defined in fpdfview.h. With FPDF_ANNOT flag, it
    ///                           renders all annotations that does not require
    ///                           user-interaction, which are all annotations except
    ///                           widget and popup annotations.
    ///          color_scheme -   Color scheme to be used in rendering the |page|.
    ///                           If null, this function will work similar to
    ///                           FPDF_RenderPageBitmap_Start().
    ///          pause        -   The IFSDK_PAUSE interface. A callback mechanism
    ///                           allowing the page rendering process.
    /// Return value:
    ///          Rendering Status. See flags for progressive process status for the
    ///          details.
    /// ```
    pub fn FPDF_RenderPageBitmapWithColorScheme_Start(
        &self,
        bitmap: &PdfiumBitmap,
        page: &PdfiumPage,
        start_x: i32,
        start_y: i32,
        size_x: i32,
        size_y: i32,
        rotate: i32,
        flags: i32,
        color_scheme: &FPDF_COLORSCHEME,
        pause: &mut IFSDK_PAUSE,
    ) -> i32 {
        unsafe {
            (self.fn_FPDF_RenderPageBitmapWithColorScheme_Start)(
                bitmap.into(),
                page.into(),
                start_x,
                start_y,
                size_x,
                size_y,
                rotate,
                flags,
                color_scheme,
                pause,
            )
        }
    }

    /// C documentation for FPDF_RenderPageBitmap_Start:
    ///
    /// ```text
    /// Function: FPDF_RenderPageBitmap_Start
    ///          Start to render page contents to a device independent bitmap
    ///          progressively.
    /// Parameters:
    ///          bitmap      -   Handle to the device independent bitmap (as the
    ///                          output buffer). Bitmap handle can be created by
    ///                          FPDFBitmap_Create().
    ///          page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///          start_x     -   Left pixel position of the display area in the
    ///                          bitmap coordinates.
    ///          start_y     -   Top pixel position of the display area in the bitmap
    ///                          coordinates.
    ///          size_x      -   Horizontal size (in pixels) for displaying the page.
    ///          size_y      -   Vertical size (in pixels) for displaying the page.
    ///          rotate      -   Page orientation: 0 (normal), 1 (rotated 90 degrees
    ///                          clockwise), 2 (rotated 180 degrees), 3 (rotated 90
    ///                          degrees counter-clockwise).
    ///          flags       -   0 for normal display, or combination of flags
    ///                          defined in fpdfview.h. With FPDF_ANNOT flag, it
    ///                          renders all annotations that does not require
    ///                          user-interaction, which are all annotations except
    ///                          widget and popup annotations.
    ///          pause       -   The IFSDK_PAUSE interface.A callback mechanism
    ///                          allowing the page rendering process
    /// Return value:
    ///          Rendering Status. See flags for progressive process status for the
    ///          details.
    /// ```
    pub fn FPDF_RenderPageBitmap_Start(
        &self,
        bitmap: &PdfiumBitmap,
        page: &PdfiumPage,
        start_x: i32,
        start_y: i32,
        size_x: i32,
        size_y: i32,
        rotate: i32,
        flags: i32,
        pause: &mut IFSDK_PAUSE,
    ) -> i32 {
        unsafe {
            (self.fn_FPDF_RenderPageBitmap_Start)(
                bitmap.into(),
                page.into(),
                start_x,
                start_y,
                size_x,
                size_y,
                rotate,
                flags,
                pause,
            )
        }
    }

    /// C documentation for FPDF_RenderPage_Continue:
    ///
    /// ```text
    /// Function: FPDF_RenderPage_Continue
    ///          Continue rendering a PDF page.
    /// Parameters:
    ///          page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///          pause       -   The IFSDK_PAUSE interface (a callback mechanism
    ///                          allowing the page rendering process to be paused
    ///                          before it's finished). This can be NULL if you
    ///                          don't want to pause.
    /// Return value:
    ///          The rendering status. See flags for progressive process status for
    ///          the details.
    /// ```
    pub fn FPDF_RenderPage_Continue(&self, page: &PdfiumPage, pause: &mut IFSDK_PAUSE) -> i32 {
        unsafe { (self.fn_FPDF_RenderPage_Continue)(page.into(), pause) }
    }

    /// C documentation for FPDF_RenderPage_Close:
    ///
    /// ```text
    /// Function: FPDF_RenderPage_Close
    ///          Release the resource allocate during page rendering. Need to be
    ///          called after finishing rendering or
    ///          cancel the rendering.
    /// Parameters:
    ///          page        -   Handle to the page, as returned by FPDF_LoadPage().
    /// Return value:
    ///          None.
    /// ```
    pub fn FPDF_RenderPage_Close(&self, page: &PdfiumPage) {
        unsafe { (self.fn_FPDF_RenderPage_Close)(page.into()) }
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
    pub fn FPDF_SaveAsCopy(
        &self,
        document: &PdfiumDocument,
        pFileWrite: &mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_SaveAsCopy)(document.into(), pFileWrite, flags) })
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
    pub fn FPDF_SaveWithVersion(
        &self,
        document: &PdfiumDocument,
        pFileWrite: &mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
        fileVersion: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_SaveWithVersion)(document.into(), pFileWrite, flags, fileVersion)
        })
    }

    /// C documentation for FPDF_GetSignatureCount:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_GetSignatureCount
    ///          Get total number of signatures in the document.
    /// Parameters:
    ///          document    -   Handle to document. Returned by FPDF_LoadDocument().
    /// Return value:
    ///          Total number of signatures in the document on success, -1 on error.
    /// ```
    pub fn FPDF_GetSignatureCount(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDF_GetSignatureCount)(document.into()) }
    }

    /// C documentation for FPDF_GetSignatureObject:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_GetSignatureObject
    ///          Get the Nth signature of the document.
    /// Parameters:
    ///          document    -   Handle to document. Returned by FPDF_LoadDocument().
    ///          index       -   Index into the array of signatures of the document.
    /// Return value:
    ///          Returns the handle to the signature, or NULL on failure. The caller
    ///          does not take ownership of the returned FPDF_SIGNATURE. Instead, it
    ///          remains valid until FPDF_CloseDocument() is called for the document.
    /// ```
    pub fn FPDF_GetSignatureObject(&self, document: &PdfiumDocument, index: i32) -> FPDF_SIGNATURE {
        unsafe { (self.fn_FPDF_GetSignatureObject)(document.into(), index) }
    }

    /// C documentation for FPDF_StructTree_GetForPage:
    ///
    /// ```text
    /// Function: FPDF_StructTree_GetForPage
    ///          Get the structure tree for a page.
    /// Parameters:
    ///          page        -   Handle to the page, as returned by FPDF_LoadPage().
    /// Return value:
    ///          A handle to the structure tree or NULL on error. The caller owns the
    ///          returned handle and must use FPDF_StructTree_Close() to release it.
    ///          The handle should be released before |page| gets released.
    /// ```
    pub fn FPDF_StructTree_GetForPage(&self, page: &PdfiumPage) -> PdfiumResult<PdfiumStructTree> {
        PdfiumStructTree::new_from_handle(unsafe {
            (self.fn_FPDF_StructTree_GetForPage)(page.into())
        })
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
    pub fn FPDF_StructTree_Close(&self, struct_tree: &PdfiumStructTree) {
        unsafe { (self.fn_FPDF_StructTree_Close)(struct_tree.into()) }
    }

    /// C documentation for FPDF_StructTree_CountChildren:
    ///
    /// ```text
    /// Function: FPDF_StructTree_CountChildren
    ///          Count the number of children for the structure tree.
    /// Parameters:
    ///          struct_tree -   Handle to the structure tree, as returned by
    ///                          FPDF_StructTree_LoadPage().
    /// Return value:
    ///          The number of children, or -1 on error.
    /// ```
    pub fn FPDF_StructTree_CountChildren(&self, struct_tree: &PdfiumStructTree) -> i32 {
        unsafe { (self.fn_FPDF_StructTree_CountChildren)(struct_tree.into()) }
    }

    /// C documentation for FPDF_StructTree_GetChildAtIndex:
    ///
    /// ```text
    /// Function: FPDF_StructTree_GetChildAtIndex
    ///          Get a child in the structure tree.
    /// Parameters:
    ///          struct_tree -   Handle to the structure tree, as returned by
    ///                          FPDF_StructTree_LoadPage().
    ///          index       -   The index for the child, 0-based.
    /// Return value:
    ///          The child at the n-th index or NULL on error. The caller does not
    ///          own the handle. The handle remains valid as long as |struct_tree|
    ///          remains valid.
    /// Comments:
    ///          The |index| must be less than the FPDF_StructTree_CountChildren()
    ///          return value.
    /// ```
    pub fn FPDF_StructTree_GetChildAtIndex(
        &self,
        struct_tree: &PdfiumStructTree,
        index: i32,
    ) -> PdfiumResult<PdfiumStructElement> {
        PdfiumStructElement::new_from_handle(unsafe {
            (self.fn_FPDF_StructTree_GetChildAtIndex)(struct_tree.into(), index)
        })
    }

    /// C documentation for FPDF_StructElement_GetAltText:
    ///
    /// ```text
    /// Function: FPDF_StructElement_GetAltText
    ///          Get the alt text for a given element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    ///          buffer         -   A buffer for output the alt text. May be NULL.
    ///          buflen         -   The length of the buffer, in bytes. May be 0.
    /// Return value:
    ///          The number of bytes in the alt text, including the terminating NUL
    ///          character. The number of bytes is returned regardless of the
    ///          |buffer| and |buflen| parameters.
    /// Comments:
    ///          Regardless of the platform, the |buffer| is always in UTF-16LE
    ///          encoding. The string is terminated by a UTF16 NUL character. If
    ///          |buflen| is less than the required length, or |buffer| is NULL,
    ///          |buffer| will not be modified.
    /// ```
    pub fn FPDF_StructElement_GetAltText(
        &self,
        struct_element: &PdfiumStructElement,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_StructElement_GetAltText)(
                struct_element.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDF_StructElement_GetActualText:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_GetActualText
    ///          Get the actual text for a given element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    ///          buffer         -   A buffer for output the actual text. May be NULL.
    ///          buflen         -   The length of the buffer, in bytes. May be 0.
    /// Return value:
    ///          The number of bytes in the actual text, including the terminating
    ///          NUL character. The number of bytes is returned regardless of the
    ///          |buffer| and |buflen| parameters.
    /// Comments:
    ///          Regardless of the platform, the |buffer| is always in UTF-16LE
    ///          encoding. The string is terminated by a UTF16 NUL character. If
    ///          |buflen| is less than the required length, or |buffer| is NULL,
    ///          |buffer| will not be modified.
    /// ```
    pub fn FPDF_StructElement_GetActualText(
        &self,
        struct_element: &PdfiumStructElement,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_StructElement_GetActualText)(
                struct_element.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDF_StructElement_GetID:
    ///
    /// ```text
    /// Function: FPDF_StructElement_GetID
    ///          Get the ID for a given element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    ///          buffer         -   A buffer for output the ID string. May be NULL.
    ///          buflen         -   The length of the buffer, in bytes. May be 0.
    /// Return value:
    ///          The number of bytes in the ID string, including the terminating NUL
    ///          character. The number of bytes is returned regardless of the
    ///          |buffer| and |buflen| parameters.
    /// Comments:
    ///          Regardless of the platform, the |buffer| is always in UTF-16LE
    ///          encoding. The string is terminated by a UTF16 NUL character. If
    ///          |buflen| is less than the required length, or |buffer| is NULL,
    ///          |buffer| will not be modified.
    /// ```
    pub fn FPDF_StructElement_GetID(
        &self,
        struct_element: &PdfiumStructElement,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_StructElement_GetID)(
                struct_element.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDF_StructElement_GetLang:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_GetLang
    ///          Get the case-insensitive IETF BCP 47 language code for an element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    ///          buffer         -   A buffer for output the lang string. May be NULL.
    ///          buflen         -   The length of the buffer, in bytes. May be 0.
    /// Return value:
    ///          The number of bytes in the ID string, including the terminating NUL
    ///          character. The number of bytes is returned regardless of the
    ///          |buffer| and |buflen| parameters.
    /// Comments:
    ///          Regardless of the platform, the |buffer| is always in UTF-16LE
    ///          encoding. The string is terminated by a UTF16 NUL character. If
    ///          |buflen| is less than the required length, or |buffer| is NULL,
    ///          |buffer| will not be modified.
    /// ```
    pub fn FPDF_StructElement_GetLang(
        &self,
        struct_element: &PdfiumStructElement,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_StructElement_GetLang)(
                struct_element.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDF_StructElement_GetStringAttribute:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_GetStringAttribute
    ///          Get a struct element attribute of type "name" or "string".
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    ///          attr_name      -   The name of the attribute to retrieve.
    ///          buffer         -   A buffer for output. May be NULL.
    ///          buflen         -   The length of the buffer, in bytes. May be 0.
    /// Return value:
    ///          The number of bytes in the attribute value, including the
    ///          terminating NUL character. The number of bytes is returned
    ///          regardless of the |buffer| and |buflen| parameters.
    /// Comments:
    ///          Regardless of the platform, the |buffer| is always in UTF-16LE
    ///          encoding. The string is terminated by a UTF16 NUL character. If
    ///          |buflen| is less than the required length, or |buffer| is NULL,
    ///          |buffer| will not be modified.
    /// ```
    pub fn FPDF_StructElement_GetStringAttribute(
        &self,
        struct_element: &PdfiumStructElement,
        attr_name: &CString,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_StructElement_GetStringAttribute)(
                struct_element.into(),
                attr_name.as_ptr(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDF_StructElement_GetMarkedContentID:
    ///
    /// ```text
    /// Function: FPDF_StructElement_GetMarkedContentID
    ///          Get the marked content ID for a given element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    /// Return value:
    ///          The marked content ID of the element. If no ID exists, returns
    ///          -1.
    /// Comments:
    ///          FPDF_StructElement_GetMarkedContentIdAtIndex() may be able to
    ///          extract more marked content IDs out of |struct_element|. This API
    ///          may be deprecated in the future.
    /// ```
    pub fn FPDF_StructElement_GetMarkedContentID(
        &self,
        struct_element: &PdfiumStructElement,
    ) -> i32 {
        unsafe { (self.fn_FPDF_StructElement_GetMarkedContentID)(struct_element.into()) }
    }

    /// C documentation for FPDF_StructElement_GetType:
    ///
    /// ```text
    /// Function: FPDF_StructElement_GetType
    ///           Get the type (/S) for a given element.
    /// Parameters:
    ///           struct_element - Handle to the struct element.
    ///           buffer         - A buffer for output. May be NULL.
    ///           buflen         - The length of the buffer, in bytes. May be 0.
    /// Return value:
    ///           The number of bytes in the type, including the terminating NUL
    ///           character. The number of bytes is returned regardless of the
    ///           |buffer| and |buflen| parameters.
    /// Comments:
    ///           Regardless of the platform, the |buffer| is always in UTF-16LE
    ///           encoding. The string is terminated by a UTF16 NUL character. If
    ///           |buflen| is less than the required length, or |buffer| is NULL,
    ///           |buffer| will not be modified.
    /// ```
    pub fn FPDF_StructElement_GetType(
        &self,
        struct_element: &PdfiumStructElement,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_StructElement_GetType)(
                struct_element.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDF_StructElement_GetObjType:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_GetObjType
    ///           Get the object type (/Type) for a given element.
    /// Parameters:
    ///           struct_element - Handle to the struct element.
    ///           buffer         - A buffer for output. May be NULL.
    ///           buflen         - The length of the buffer, in bytes. May be 0.
    /// Return value:
    ///           The number of bytes in the object type, including the terminating
    ///           NUL character. The number of bytes is returned regardless of the
    ///           |buffer| and |buflen| parameters.
    /// Comments:
    ///           Regardless of the platform, the |buffer| is always in UTF-16LE
    ///           encoding. The string is terminated by a UTF16 NUL character. If
    ///           |buflen| is less than the required length, or |buffer| is NULL,
    ///           |buffer| will not be modified.
    /// ```
    pub fn FPDF_StructElement_GetObjType(
        &self,
        struct_element: &PdfiumStructElement,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_StructElement_GetObjType)(
                struct_element.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDF_StructElement_GetTitle:
    ///
    /// ```text
    /// Function: FPDF_StructElement_GetTitle
    ///           Get the title (/T) for a given element.
    /// Parameters:
    ///           struct_element - Handle to the struct element.
    ///           buffer         - A buffer for output. May be NULL.
    ///           buflen         - The length of the buffer, in bytes. May be 0.
    /// Return value:
    ///           The number of bytes in the title, including the terminating NUL
    ///           character. The number of bytes is returned regardless of the
    ///           |buffer| and |buflen| parameters.
    /// Comments:
    ///           Regardless of the platform, the |buffer| is always in UTF-16LE
    ///           encoding. The string is terminated by a UTF16 NUL character. If
    ///           |buflen| is less than the required length, or |buffer| is NULL,
    ///           |buffer| will not be modified.
    /// ```
    pub fn FPDF_StructElement_GetTitle(
        &self,
        struct_element: &PdfiumStructElement,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDF_StructElement_GetTitle)(
                struct_element.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDF_StructElement_CountChildren:
    ///
    /// ```text
    /// Function: FPDF_StructElement_CountChildren
    ///          Count the number of children for the structure element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    /// Return value:
    ///          The number of children, or -1 on error.
    /// ```
    pub fn FPDF_StructElement_CountChildren(&self, struct_element: &PdfiumStructElement) -> i32 {
        unsafe { (self.fn_FPDF_StructElement_CountChildren)(struct_element.into()) }
    }

    /// C documentation for FPDF_StructElement_GetChildAtIndex:
    ///
    /// ```text
    /// Function: FPDF_StructElement_GetChildAtIndex
    ///          Get a child in the structure element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    ///          index          -   The index for the child, 0-based.
    /// Return value:
    ///          The child at the n-th index or NULL on error.
    /// Comments:
    ///          If the child exists but is not an element, then this function will
    ///          return NULL. This will also return NULL for out of bounds indices.
    ///          The |index| must be less than the FPDF_StructElement_CountChildren()
    ///          return value.
    /// ```
    pub fn FPDF_StructElement_GetChildAtIndex(
        &self,
        struct_element: &PdfiumStructElement,
        index: i32,
    ) -> PdfiumResult<PdfiumStructElement> {
        PdfiumStructElement::new_from_handle(unsafe {
            (self.fn_FPDF_StructElement_GetChildAtIndex)(struct_element.into(), index)
        })
    }

    /// C documentation for FPDF_StructElement_GetChildMarkedContentID:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_GetChildMarkedContentID
    ///          Get the child's content id
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    ///          index          -   The index for the child, 0-based.
    /// Return value:
    ///          The marked content ID of the child. If no ID exists, returns -1.
    /// Comments:
    ///          If the child exists but is not a stream or object, then this
    ///          function will return -1. This will also return -1 for out of bounds
    ///          indices. Compared to FPDF_StructElement_GetMarkedContentIdAtIndex,
    ///          it is scoped to the current page.
    ///          The |index| must be less than the FPDF_StructElement_CountChildren()
    ///          return value.
    /// ```
    pub fn FPDF_StructElement_GetChildMarkedContentID(
        &self,
        struct_element: &PdfiumStructElement,
        index: i32,
    ) -> i32 {
        unsafe {
            (self.fn_FPDF_StructElement_GetChildMarkedContentID)(struct_element.into(), index)
        }
    }

    /// C documentation for FPDF_StructElement_GetParent:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_GetParent
    ///          Get the parent of the structure element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    /// Return value:
    ///          The parent structure element or NULL on error.
    /// Comments:
    ///          If structure element is StructTreeRoot, then this function will
    ///          return NULL.
    /// ```
    pub fn FPDF_StructElement_GetParent(
        &self,
        struct_element: &PdfiumStructElement,
    ) -> PdfiumResult<PdfiumStructElement> {
        PdfiumStructElement::new_from_handle(unsafe {
            (self.fn_FPDF_StructElement_GetParent)(struct_element.into())
        })
    }

    /// C documentation for FPDF_StructElement_GetAttributeCount:
    ///
    /// ```text
    /// Function: FPDF_StructElement_GetAttributeCount
    ///          Count the number of attributes for the structure element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    /// Return value:
    ///          The number of attributes, or -1 on error.
    /// ```
    pub fn FPDF_StructElement_GetAttributeCount(
        &self,
        struct_element: &PdfiumStructElement,
    ) -> i32 {
        unsafe { (self.fn_FPDF_StructElement_GetAttributeCount)(struct_element.into()) }
    }

    /// C documentation for FPDF_StructElement_GetAttributeAtIndex:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_GetAttributeAtIndex
    ///          Get an attribute object in the structure element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    ///          index          -   The index for the attribute object, 0-based.
    /// Return value:
    ///          The attribute object at the n-th index or NULL on error.
    /// Comments:
    ///          If the attribute object exists but is not a dict, then this
    ///          function will return NULL. This will also return NULL for out of
    ///          bounds indices. The caller does not own the handle. The handle
    ///          remains valid as long as |struct_element| remains valid.
    ///          The |index| must be less than the
    ///          FPDF_StructElement_GetAttributeCount() return value.
    /// ```
    pub fn FPDF_StructElement_GetAttributeAtIndex(
        &self,
        struct_element: &PdfiumStructElement,
        index: i32,
    ) -> PdfiumResult<PdfiumStructElementAttr> {
        PdfiumStructElementAttr::new_from_handle(unsafe {
            (self.fn_FPDF_StructElement_GetAttributeAtIndex)(struct_element.into(), index)
        })
    }

    /// C documentation for FPDF_StructElement_Attr_GetCount:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_Attr_GetCount
    ///          Count the number of attributes in a structure element attribute map.
    /// Parameters:
    ///          struct_attribute - Handle to the struct element attribute.
    /// Return value:
    ///          The number of attributes, or -1 on error.
    /// ```
    pub fn FPDF_StructElement_Attr_GetCount(
        &self,
        struct_attribute: &PdfiumStructElementAttr,
    ) -> i32 {
        unsafe { (self.fn_FPDF_StructElement_Attr_GetCount)(struct_attribute.into()) }
    }

    /// C documentation for FPDF_StructElement_Attr_GetName:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_Attr_GetName
    ///          Get the name of an attribute in a structure element attribute map.
    /// Parameters:
    ///          struct_attribute   - Handle to the struct element attribute.
    ///          index              - The index of attribute in the map.
    ///          buffer             - A buffer for output. May be NULL. This is only
    ///                               modified if |buflen| is longer than the length
    ///                               of the key. Optional, pass null to just
    ///                               retrieve the size of the buffer needed.
    ///          buflen             - The length of the buffer.
    ///          out_buflen         - A pointer to variable that will receive the
    ///                               minimum buffer size to contain the key. Not
    ///                               filled if FALSE is returned.
    /// Return value:
    ///          TRUE if the operation was successful, FALSE otherwise.
    /// ```
    pub fn FPDF_StructElement_Attr_GetName(
        &self,
        struct_attribute: &PdfiumStructElementAttr,
        index: i32,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
        out_buflen: &mut c_ulong,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_StructElement_Attr_GetName)(
                struct_attribute.into(),
                index,
                to_void_ptr_mut(buffer),
                buflen,
                out_buflen,
            )
        })
    }

    /// C documentation for FPDF_StructElement_Attr_GetValue:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_Attr_GetValue
    ///           Get a handle to a value for an attribute in a structure element
    ///           attribute map.
    /// Parameters:
    ///           struct_attribute   - Handle to the struct element attribute.
    ///           name               - The attribute name.
    /// Return value:
    ///           Returns a handle to the value associated with the input, if any.
    ///           Returns NULL on failure. The caller does not own the handle.
    ///           The handle remains valid as long as |struct_attribute| remains
    ///           valid.
    /// ```
    pub fn FPDF_StructElement_Attr_GetValue(
        &self,
        struct_attribute: &PdfiumStructElementAttr,
        name: &CString,
    ) -> PdfiumResult<PdfiumStructElementAttrValue> {
        PdfiumStructElementAttrValue::new_from_handle(unsafe {
            (self.fn_FPDF_StructElement_Attr_GetValue)(struct_attribute.into(), name.as_ptr())
        })
    }

    /// C documentation for FPDF_StructElement_Attr_GetType:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_Attr_GetType
    ///           Get the type of an attribute in a structure element attribute map.
    /// Parameters:
    ///           value - Handle to the value.
    /// Return value:
    ///           Returns the type of the value, or FPDF_OBJECT_UNKNOWN in case of
    ///           failure. Note that this will never return FPDF_OBJECT_REFERENCE, as
    ///           references are always dereferenced.
    /// ```
    pub fn FPDF_StructElement_Attr_GetType(
        &self,
        value: &PdfiumStructElementAttrValue,
    ) -> FPDF_OBJECT_TYPE {
        unsafe { (self.fn_FPDF_StructElement_Attr_GetType)(value.into()) }
    }

    /// C documentation for FPDF_StructElement_Attr_GetBooleanValue:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_Attr_GetBooleanValue
    ///           Get the value of a boolean attribute in an attribute map as
    ///           FPDF_BOOL. FPDF_StructElement_Attr_GetType() should have returned
    ///           FPDF_OBJECT_BOOLEAN for this property.
    /// Parameters:
    ///           value     - Handle to the value.
    ///           out_value - A pointer to variable that will receive the value. Not
    ///                       filled if false is returned.
    /// Return value:
    ///           Returns TRUE if the attribute maps to a boolean value, FALSE
    ///           otherwise.
    /// ```
    pub fn FPDF_StructElement_Attr_GetBooleanValue(
        &self,
        value: &PdfiumStructElementAttrValue,
        out_value: &mut FPDF_BOOL,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_StructElement_Attr_GetBooleanValue)(value.into(), out_value)
        })
    }

    /// C documentation for FPDF_StructElement_Attr_GetNumberValue:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_Attr_GetNumberValue
    ///           Get the value of a number attribute in an attribute map as float.
    ///           FPDF_StructElement_Attr_GetType() should have returned
    ///           FPDF_OBJECT_NUMBER for this property.
    /// Parameters:
    ///           value     - Handle to the value.
    ///           out_value - A pointer to variable that will receive the value. Not
    ///                       filled if false is returned.
    /// Return value:
    ///           Returns TRUE if the attribute maps to a number value, FALSE
    ///           otherwise.
    /// ```
    pub fn FPDF_StructElement_Attr_GetNumberValue(
        &self,
        value: &PdfiumStructElementAttrValue,
        out_value: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_StructElement_Attr_GetNumberValue)(value.into(), out_value)
        })
    }

    /// C documentation for FPDF_StructElement_Attr_GetStringValue:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_Attr_GetStringValue
    ///           Get the value of a string attribute in an attribute map as string.
    ///           FPDF_StructElement_Attr_GetType() should have returned
    ///           FPDF_OBJECT_STRING or FPDF_OBJECT_NAME for this property.
    /// Parameters:
    ///           value      - Handle to the value.
    ///           buffer     - A buffer for holding the returned key in UTF-16LE.
    ///                        This is only modified if |buflen| is longer than the
    ///                        length of the key. Optional, pass null to just
    ///                        retrieve the size of the buffer needed.
    ///           buflen     - The length of the buffer.
    ///           out_buflen - A pointer to variable that will receive the minimum
    ///                        buffer size to contain the key. Not filled if FALSE is
    ///                        returned.
    /// Return value:
    ///           Returns TRUE if the attribute maps to a string value, FALSE
    ///           otherwise.
    /// ```
    pub fn FPDF_StructElement_Attr_GetStringValue(
        &self,
        value: &PdfiumStructElementAttrValue,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
        out_buflen: &mut c_ulong,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_StructElement_Attr_GetStringValue)(
                value.into(),
                to_void_ptr_mut(buffer),
                buflen,
                out_buflen,
            )
        })
    }

    /// C documentation for FPDF_StructElement_Attr_GetBlobValue:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_Attr_GetBlobValue
    ///           Get the value of a blob attribute in an attribute map as string.
    /// Parameters:
    ///           value      - Handle to the value.
    ///           buffer     - A buffer for holding the returned value. This is only
    ///                        modified if |buflen| is at least as long as the length
    ///                        of the value. Optional, pass null to just retrieve the
    ///                        size of the buffer needed.
    ///           buflen     - The length of the buffer.
    ///           out_buflen - A pointer to variable that will receive the minimum
    ///                        buffer size to contain the key. Not filled if FALSE is
    ///                        returned.
    /// Return value:
    ///           Returns TRUE if the attribute maps to a string value, FALSE
    ///           otherwise.
    /// ```
    pub fn FPDF_StructElement_Attr_GetBlobValue(
        &self,
        value: &PdfiumStructElementAttrValue,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
        out_buflen: &mut c_ulong,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_StructElement_Attr_GetBlobValue)(
                value.into(),
                to_void_ptr_mut(buffer),
                buflen,
                out_buflen,
            )
        })
    }

    /// C documentation for FPDF_StructElement_Attr_CountChildren:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_Attr_CountChildren
    ///           Count the number of children values in an attribute.
    /// Parameters:
    ///           value - Handle to the value.
    /// Return value:
    ///           The number of children, or -1 on error.
    /// ```
    pub fn FPDF_StructElement_Attr_CountChildren(
        &self,
        value: &PdfiumStructElementAttrValue,
    ) -> i32 {
        unsafe { (self.fn_FPDF_StructElement_Attr_CountChildren)(value.into()) }
    }

    /// C documentation for FPDF_StructElement_Attr_GetChildAtIndex:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_Attr_GetChildAtIndex
    ///           Get a child from an attribute.
    /// Parameters:
    ///           value - Handle to the value.
    ///           index - The index for the child, 0-based.
    /// Return value:
    ///           The child at the n-th index or NULL on error.
    /// Comments:
    ///           The |index| must be less than the
    ///           FPDF_StructElement_Attr_CountChildren() return value.
    /// ```
    pub fn FPDF_StructElement_Attr_GetChildAtIndex(
        &self,
        value: &PdfiumStructElementAttrValue,
        index: i32,
    ) -> PdfiumResult<PdfiumStructElementAttrValue> {
        PdfiumStructElementAttrValue::new_from_handle(unsafe {
            (self.fn_FPDF_StructElement_Attr_GetChildAtIndex)(value.into(), index)
        })
    }

    /// C documentation for FPDF_StructElement_GetMarkedContentIdCount:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_GetMarkedContentIdCount
    ///          Get the count of marked content ids for a given element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    /// Return value:
    ///          The count of marked content ids or -1 if none exists.
    /// ```
    pub fn FPDF_StructElement_GetMarkedContentIdCount(
        &self,
        struct_element: &PdfiumStructElement,
    ) -> i32 {
        unsafe { (self.fn_FPDF_StructElement_GetMarkedContentIdCount)(struct_element.into()) }
    }

    /// C documentation for FPDF_StructElement_GetMarkedContentIdAtIndex:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDF_StructElement_GetMarkedContentIdAtIndex
    ///          Get the marked content id at a given index for a given element.
    /// Parameters:
    ///          struct_element -   Handle to the struct element.
    ///          index          -   The index of the marked content id, 0-based.
    /// Return value:
    ///          The marked content ID of the element. If no ID exists, returns
    ///          -1.
    /// Comments:
    ///          The |index| must be less than the
    ///          FPDF_StructElement_GetMarkedContentIdCount() return value.
    ///          This will likely supersede FPDF_StructElement_GetMarkedContentID().
    /// ```
    pub fn FPDF_StructElement_GetMarkedContentIdAtIndex(
        &self,
        struct_element: &PdfiumStructElement,
        index: i32,
    ) -> i32 {
        unsafe {
            (self.fn_FPDF_StructElement_GetMarkedContentIdAtIndex)(struct_element.into(), index)
        }
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
    pub fn FPDF_GetDefaultTTFMap(&self) -> *const FPDF_CharsetFontMap {
        unsafe { (self.fn_FPDF_GetDefaultTTFMap)() }
    }

    /// C documentation for FPDF_GetDefaultTTFMapCount:
    ///
    /// ```text
    /// Experimental API.
    ///
    /// Function: FPDF_GetDefaultTTFMapCount
    ///    Returns the number of entries in the default character set to TT Font name
    ///    map.
    /// Parameters:
    ///    None.
    /// Return Value:
    ///    The number of entries in the map.
    /// ```
    pub fn FPDF_GetDefaultTTFMapCount(&self) -> usize {
        unsafe { (self.fn_FPDF_GetDefaultTTFMapCount)() }
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
    pub fn FPDF_GetDefaultTTFMapEntry(&self, index: usize) -> *const FPDF_CharsetFontMap {
        unsafe { (self.fn_FPDF_GetDefaultTTFMapEntry)(index) }
    }

    /// C documentation for FPDF_AddInstalledFont:
    ///
    /// ```text
    /// Function: FPDF_AddInstalledFont
    ///          Add a system font to the list in PDFium.
    /// Comments:
    ///          This function is only called during the system font list building
    ///          process.
    /// Parameters:
    ///          mapper          -   Opaque pointer to Foxit font mapper
    ///          face            -   The font face name
    ///          charset         -   Font character set. See above defined constants.
    /// Return Value:
    ///          None.
    /// ```
    pub fn FPDF_AddInstalledFont(
        &self,
        mapper: Option<&mut [u8]>,
        face: Option<&[i8]>,
        charset: i32,
    ) {
        unsafe {
            (self.fn_FPDF_AddInstalledFont)(to_void_ptr_mut(mapper), to_char_ptr(face), charset)
        }
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
    pub fn FPDF_SetSystemFontInfo(&self, font_info: &mut PdfiumSystemFontInfo) {
        unsafe { (self.fn_FPDF_SetSystemFontInfo)(font_info.into()) }
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
    pub fn FPDF_GetDefaultSystemFontInfo(&self) -> PdfiumResult<PdfiumSystemFontInfo> {
        PdfiumSystemFontInfo::new_from_handle(unsafe { (self.fn_FPDF_GetDefaultSystemFontInfo)() })
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
    pub fn FPDF_FreeDefaultSystemFontInfo(&self, font_info: &mut PdfiumSystemFontInfo) {
        unsafe { (self.fn_FPDF_FreeDefaultSystemFontInfo)(font_info.into()) }
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
        buflen: c_ulong,
    ) -> c_ulong {
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
        buflen: c_ulong,
    ) -> c_ulong {
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

    /// C documentation for FPDFClipPath_CountPaths:
    ///
    /// ```text
    /// Experimental API.
    /// Get number of paths inside |clip_path|.
    ///
    ///   clip_path - handle to a clip_path.
    ///
    /// Returns the number of objects in |clip_path| or -1 on failure.
    /// ```
    pub fn FPDFClipPath_CountPaths(&self, clip_path: &PdfiumClipPath) -> i32 {
        unsafe { (self.fn_FPDFClipPath_CountPaths)(clip_path.into()) }
    }

    /// C documentation for FPDFClipPath_CountPathSegments:
    ///
    /// ```text
    /// Experimental API.
    /// Get number of segments inside one path of |clip_path|.
    ///
    ///   clip_path  - handle to a clip_path.
    ///   path_index - index into the array of paths of the clip path.
    ///
    /// Returns the number of segments or -1 on failure.
    /// ```
    pub fn FPDFClipPath_CountPathSegments(
        &self,
        clip_path: &PdfiumClipPath,
        path_index: i32,
    ) -> i32 {
        unsafe { (self.fn_FPDFClipPath_CountPathSegments)(clip_path.into(), path_index) }
    }

    /// C documentation for FPDFClipPath_GetPathSegment:
    ///
    /// ```text
    /// Experimental API.
    /// Get segment in one specific path of |clip_path| at index.
    ///
    ///   clip_path     - handle to a clip_path.
    ///   path_index    - the index of a path.
    ///   segment_index - the index of a segment.
    ///
    /// Returns the handle to the segment, or NULL on failure. The caller does not
    /// take ownership of the returned FPDF_PATHSEGMENT. Instead, it remains valid
    /// until FPDF_ClosePage() is called for the page containing |clip_path|.
    /// ```
    pub fn FPDFClipPath_GetPathSegment(
        &self,
        clip_path: &PdfiumClipPath,
        path_index: i32,
        segment_index: i32,
    ) -> FPDF_PATHSEGMENT {
        unsafe {
            (self.fn_FPDFClipPath_GetPathSegment)(clip_path.into(), path_index, segment_index)
        }
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

fn to_char_ptr(data: Option<&[i8]>) -> *const c_char {
    match data {
        Some(slice) => slice.as_ptr() as *const c_char,
        None => std::ptr::null(),
    }
}

fn to_char_ptr_mut(data: Option<&mut [i8]>) -> *mut c_char {
    match data {
        Some(slice) => slice.as_ptr() as *mut c_char,
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

fn str_to_utf16le_vec(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
