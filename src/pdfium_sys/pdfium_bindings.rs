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

#![allow(clippy::too_many_arguments)]

use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};

use super::pdfium_types::*;
use libloading::Library;

/// Safe Rust wrapper around the complete PDFium C API
#[allow(non_snake_case)]
pub struct Pdfium {
    pub(crate) fn_FORM_CanRedo:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL,
    pub(crate) fn_FORM_CanUndo:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL,
    pub(crate) fn_FORM_DoDocumentAAction:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, aaType: c_int),
    pub(crate) fn_FORM_DoDocumentJSAction: unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE),
    pub(crate) fn_FORM_DoDocumentOpenAction: unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE),
    pub(crate) fn_FORM_DoPageAAction:
        unsafe extern "C" fn(page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE, aaType: c_int),
    pub(crate) fn_FORM_ForceToKillFocus:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE) -> FPDF_BOOL,
    pub(crate) fn_FORM_GetFocusedAnnot: unsafe extern "C" fn(
        handle: FPDF_FORMHANDLE,
        page_index: *mut c_int,
        annot: *mut FPDF_ANNOTATION,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_GetFocusedText: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FORM_GetSelectedText: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FORM_IsIndexSelected:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, index: c_int) -> FPDF_BOOL,
    pub(crate) fn_FORM_OnAfterLoadPage:
        unsafe extern "C" fn(page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE),
    pub(crate) fn_FORM_OnBeforeClosePage:
        unsafe extern "C" fn(page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE),
    pub(crate) fn_FORM_OnChar: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        nChar: c_int,
        modifier: c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_OnFocus: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_OnKeyDown: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        nKeyCode: c_int,
        modifier: c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_OnKeyUp: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        nKeyCode: c_int,
        modifier: c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_OnLButtonDoubleClick: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_OnLButtonDown: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_OnLButtonUp: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_OnMouseMove: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_OnMouseWheel: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: c_int,
        page_coord: *const FS_POINTF,
        delta_x: c_int,
        delta_y: c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_OnRButtonDown: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_OnRButtonUp: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        modifier: c_int,
        page_x: f64,
        page_y: f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_Redo:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL,
    pub(crate) fn_FORM_ReplaceAndKeepSelection:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, wsText: FPDF_WIDESTRING),
    pub(crate) fn_FORM_ReplaceSelection:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE, wsText: FPDF_WIDESTRING),
    pub(crate) fn_FORM_SelectAllText:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL,
    pub(crate) fn_FORM_SetFocusedAnnot:
        unsafe extern "C" fn(handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL,
    pub(crate) fn_FORM_SetIndexSelected: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        index: c_int,
        selected: FPDF_BOOL,
    ) -> FPDF_BOOL,
    pub(crate) fn_FORM_Undo:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, page: FPDF_PAGE) -> FPDF_BOOL,
    pub(crate) fn_FPDFAction_GetDest:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST,
    pub(crate) fn_FPDFAction_GetFilePath:
        unsafe extern "C" fn(action: FPDF_ACTION, buffer: *mut c_void, buflen: c_ulong) -> c_ulong,
    pub(crate) fn_FPDFAction_GetType: unsafe extern "C" fn(action: FPDF_ACTION) -> c_ulong,
    pub(crate) fn_FPDFAction_GetURIPath: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAnnot_AddFileAttachment:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, name: FPDF_WIDESTRING) -> FPDF_ATTACHMENT,
    pub(crate) fn_FPDFAnnot_AddInkStroke: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        points: *const FS_POINTF,
        point_count: usize,
    ) -> c_int,
    pub(crate) fn_FPDFAnnot_AppendAttachmentPoints: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_AppendObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_CountAttachmentPoints:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> usize,
    pub(crate) fn_FPDFAnnot_GetAP: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAnnot_GetAttachmentPoints: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        quad_index: usize,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_GetBorder: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        horizontal_radius: *mut f32,
        vertical_radius: *mut f32,
        border_width: *mut f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_GetColor: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        type_: FPDFANNOT_COLORTYPE,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_GetFileAttachment:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_ATTACHMENT,
    pub(crate) fn_FPDFAnnot_GetFlags: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_int,
    pub(crate) fn_FPDFAnnot_GetFocusableSubtypes: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
        count: usize,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_GetFocusableSubtypesCount:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE) -> c_int,
    pub(crate) fn_FPDFAnnot_GetFontColor: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_GetFontSize: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        value: *mut f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_GetFormAdditionalActionJavaScript: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        event: c_int,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAnnot_GetFormControlCount:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    pub(crate) fn_FPDFAnnot_GetFormControlIndex:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    pub(crate) fn_FPDFAnnot_GetFormFieldAlternateName: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAnnot_GetFormFieldAtPoint: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        point: *const FS_POINTF,
    ) -> FPDF_ANNOTATION,
    pub(crate) fn_FPDFAnnot_GetFormFieldExportValue: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAnnot_GetFormFieldFlags:
        unsafe extern "C" fn(handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    pub(crate) fn_FPDFAnnot_GetFormFieldName: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAnnot_GetFormFieldType:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    pub(crate) fn_FPDFAnnot_GetFormFieldValue: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAnnot_GetInkListCount:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_ulong,
    pub(crate) fn_FPDFAnnot_GetInkListPath: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        path_index: c_ulong,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAnnot_GetLine: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        start: *mut FS_POINTF,
        end: *mut FS_POINTF,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_GetLink: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_LINK,
    pub(crate) fn_FPDFAnnot_GetLinkedAnnot:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_ANNOTATION,
    pub(crate) fn_FPDFAnnot_GetNumberValue: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        value: *mut f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_GetObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, index: c_int) -> FPDF_PAGEOBJECT,
    pub(crate) fn_FPDFAnnot_GetObjectCount: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_int,
    pub(crate) fn_FPDFAnnot_GetOptionCount:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    pub(crate) fn_FPDFAnnot_GetOptionLabel: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAnnot_GetRect:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_GetStringValue: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAnnot_GetSubtype:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE,
    pub(crate) fn_FPDFAnnot_GetValueType:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
    pub(crate) fn_FPDFAnnot_GetVertices: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAnnot_HasAttachmentPoints:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_HasKey:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_IsChecked:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_IsObjectSupportedSubtype:
        unsafe extern "C" fn(subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_IsOptionSelected: unsafe extern "C" fn(
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_IsSupportedSubtype:
        unsafe extern "C" fn(subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_RemoveInkList:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_RemoveObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, index: c_int) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_SetAP: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_SetAttachmentPoints: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        quad_index: usize,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_SetBorder: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        horizontal_radius: f32,
        vertical_radius: f32,
        border_width: f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_SetColor: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        type_: FPDFANNOT_COLORTYPE,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_SetFlags:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, flags: c_int) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_SetFocusableSubtypes: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        subtypes: *const FPDF_ANNOTATION_SUBTYPE,
        count: usize,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_SetFontColor: unsafe extern "C" fn(
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        R: c_uint,
        G: c_uint,
        B: c_uint,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_SetFormFieldFlags: unsafe extern "C" fn(
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        flags: c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_SetRect:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_SetStringValue: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_SetURI:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, uri: *const c_char) -> FPDF_BOOL,
    pub(crate) fn_FPDFAnnot_UpdateObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    pub(crate) fn_FPDFAttachment_GetFile: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAttachment_GetName: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAttachment_GetStringValue: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        key: FPDF_BYTESTRING,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAttachment_GetSubtype: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFAttachment_GetValueType:
        unsafe extern "C" fn(attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
    pub(crate) fn_FPDFAttachment_HasKey:
        unsafe extern "C" fn(attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING) -> FPDF_BOOL,
    pub(crate) fn_FPDFAttachment_SetFile: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        document: FPDF_DOCUMENT,
        contents: *const c_void,
        len: c_ulong,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAttachment_SetStringValue: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        key: FPDF_BYTESTRING,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFAvail_Create: unsafe extern "C" fn(
        file_avail: *mut FX_FILEAVAIL,
        file: *mut FPDF_FILEACCESS,
    ) -> FPDF_AVAIL,
    pub(crate) fn_FPDFAvail_Destroy: unsafe extern "C" fn(avail: FPDF_AVAIL),
    pub(crate) fn_FPDFAvail_GetDocument:
        unsafe extern "C" fn(avail: FPDF_AVAIL, password: FPDF_BYTESTRING) -> FPDF_DOCUMENT,
    pub(crate) fn_FPDFAvail_GetFirstPageNum: unsafe extern "C" fn(doc: FPDF_DOCUMENT) -> c_int,
    pub(crate) fn_FPDFAvail_IsDocAvail:
        unsafe extern "C" fn(avail: FPDF_AVAIL, hints: *mut FX_DOWNLOADHINTS) -> c_int,
    pub(crate) fn_FPDFAvail_IsFormAvail:
        unsafe extern "C" fn(avail: FPDF_AVAIL, hints: *mut FX_DOWNLOADHINTS) -> c_int,
    pub(crate) fn_FPDFAvail_IsLinearized: unsafe extern "C" fn(avail: FPDF_AVAIL) -> c_int,
    pub(crate) fn_FPDFAvail_IsPageAvail: unsafe extern "C" fn(
        avail: FPDF_AVAIL,
        page_index: c_int,
        hints: *mut FX_DOWNLOADHINTS,
    ) -> c_int,
    pub(crate) fn_FPDFBitmap_Create:
        unsafe extern "C" fn(width: c_int, height: c_int, alpha: c_int) -> FPDF_BITMAP,
    pub(crate) fn_FPDFBitmap_CreateEx: unsafe extern "C" fn(
        width: c_int,
        height: c_int,
        format: c_int,
        first_scan: *mut c_void,
        stride: c_int,
    ) -> FPDF_BITMAP,
    pub(crate) fn_FPDFBitmap_Destroy: unsafe extern "C" fn(bitmap: FPDF_BITMAP),
    pub(crate) fn_FPDFBitmap_FillRect: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        color: FPDF_DWORD,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFBitmap_GetBuffer: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> *mut c_void,
    pub(crate) fn_FPDFBitmap_GetFormat: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    pub(crate) fn_FPDFBitmap_GetHeight: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    pub(crate) fn_FPDFBitmap_GetStride: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    pub(crate) fn_FPDFBitmap_GetWidth: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    pub(crate) fn_FPDFBookmark_Find:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK,
    pub(crate) fn_FPDFBookmark_GetAction:
        unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> FPDF_ACTION,
    pub(crate) fn_FPDFBookmark_GetCount: unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> c_int,
    pub(crate) fn_FPDFBookmark_GetDest:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST,
    pub(crate) fn_FPDFBookmark_GetFirstChild:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK,
    pub(crate) fn_FPDFBookmark_GetNextSibling:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK,
    pub(crate) fn_FPDFBookmark_GetTitle: unsafe extern "C" fn(
        bookmark: FPDF_BOOKMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFCatalog_IsTagged: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL,
    pub(crate) fn_FPDFCatalog_SetLanguage:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, language: FPDF_BYTESTRING) -> FPDF_BOOL,
    pub(crate) fn_FPDFClipPath_CountPathSegments:
        unsafe extern "C" fn(clip_path: FPDF_CLIPPATH, path_index: c_int) -> c_int,
    pub(crate) fn_FPDFClipPath_CountPaths: unsafe extern "C" fn(clip_path: FPDF_CLIPPATH) -> c_int,
    pub(crate) fn_FPDFClipPath_GetPathSegment: unsafe extern "C" fn(
        clip_path: FPDF_CLIPPATH,
        path_index: c_int,
        segment_index: c_int,
    ) -> FPDF_PATHSEGMENT,
    pub(crate) fn_FPDFDOC_ExitFormFillEnvironment: unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE),
    pub(crate) fn_FPDFDOC_InitFormFillEnvironment: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        formInfo: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE,
    pub(crate) fn_FPDFDest_GetDestPageIndex:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int,
    pub(crate) fn_FPDFDest_GetLocationInPage: unsafe extern "C" fn(
        dest: FPDF_DEST,
        hasXVal: *mut FPDF_BOOL,
        hasYVal: *mut FPDF_BOOL,
        hasZoomVal: *mut FPDF_BOOL,
        x: *mut FS_FLOAT,
        y: *mut FS_FLOAT,
        zoom: *mut FS_FLOAT,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFDest_GetView: unsafe extern "C" fn(
        dest: FPDF_DEST,
        pNumParams: *mut c_ulong,
        pParams: *mut FS_FLOAT,
    ) -> c_ulong,
    pub(crate) fn_FPDFDoc_AddAttachment:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, name: FPDF_WIDESTRING) -> FPDF_ATTACHMENT,
    pub(crate) fn_FPDFDoc_CloseJavaScriptAction:
        unsafe extern "C" fn(javascript: FPDF_JAVASCRIPT_ACTION),
    pub(crate) fn_FPDFDoc_DeleteAttachment:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_BOOL,
    pub(crate) fn_FPDFDoc_GetAttachment:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_ATTACHMENT,
    pub(crate) fn_FPDFDoc_GetAttachmentCount:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    pub(crate) fn_FPDFDoc_GetJavaScriptAction:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_JAVASCRIPT_ACTION,
    pub(crate) fn_FPDFDoc_GetJavaScriptActionCount:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    pub(crate) fn_FPDFDoc_GetPageMode: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    pub(crate) fn_FPDFFont_Close: unsafe extern "C" fn(font: FPDF_FONT),
    pub(crate) fn_FPDFFont_GetAscent:
        unsafe extern "C" fn(font: FPDF_FONT, font_size: f32, ascent: *mut f32) -> FPDF_BOOL,
    pub(crate) fn_FPDFFont_GetBaseFontName:
        unsafe extern "C" fn(font: FPDF_FONT, buffer: *mut c_char, length: usize) -> usize,
    pub(crate) fn_FPDFFont_GetDescent:
        unsafe extern "C" fn(font: FPDF_FONT, font_size: f32, descent: *mut f32) -> FPDF_BOOL,
    pub(crate) fn_FPDFFont_GetFamilyName:
        unsafe extern "C" fn(font: FPDF_FONT, buffer: *mut c_char, length: usize) -> usize,
    pub(crate) fn_FPDFFont_GetFlags: unsafe extern "C" fn(font: FPDF_FONT) -> c_int,
    pub(crate) fn_FPDFFont_GetFontData: unsafe extern "C" fn(
        font: FPDF_FONT,
        buffer: *mut u8,
        buflen: usize,
        out_buflen: *mut usize,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFFont_GetGlyphPath:
        unsafe extern "C" fn(font: FPDF_FONT, glyph: u32, font_size: f32) -> FPDF_GLYPHPATH,
    pub(crate) fn_FPDFFont_GetGlyphWidth: unsafe extern "C" fn(
        font: FPDF_FONT,
        glyph: u32,
        font_size: f32,
        width: *mut f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFFont_GetIsEmbedded: unsafe extern "C" fn(font: FPDF_FONT) -> c_int,
    pub(crate) fn_FPDFFont_GetItalicAngle:
        unsafe extern "C" fn(font: FPDF_FONT, angle: *mut c_int) -> FPDF_BOOL,
    pub(crate) fn_FPDFFont_GetWeight: unsafe extern "C" fn(font: FPDF_FONT) -> c_int,
    pub(crate) fn_FPDFFormObj_CountObjects:
        unsafe extern "C" fn(form_object: FPDF_PAGEOBJECT) -> c_int,
    pub(crate) fn_FPDFFormObj_GetObject:
        unsafe extern "C" fn(form_object: FPDF_PAGEOBJECT, index: c_ulong) -> FPDF_PAGEOBJECT,
    pub(crate) fn_FPDFFormObj_RemoveObject: unsafe extern "C" fn(
        form_object: FPDF_PAGEOBJECT,
        page_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFGlyphPath_CountGlyphSegments:
        unsafe extern "C" fn(glyphpath: FPDF_GLYPHPATH) -> c_int,
    pub(crate) fn_FPDFGlyphPath_GetGlyphPathSegment:
        unsafe extern "C" fn(glyphpath: FPDF_GLYPHPATH, index: c_int) -> FPDF_PATHSEGMENT,
    pub(crate) fn_FPDFImageObj_GetBitmap:
        unsafe extern "C" fn(image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP,
    pub(crate) fn_FPDFImageObj_GetIccProfileDataDecoded: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        page: FPDF_PAGE,
        buffer: *mut u8,
        buflen: usize,
        out_buflen: *mut usize,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFImageObj_GetImageDataDecoded: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFImageObj_GetImageDataRaw: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFImageObj_GetImageFilter: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFImageObj_GetImageFilterCount:
        unsafe extern "C" fn(image_object: FPDF_PAGEOBJECT) -> c_int,
    pub(crate) fn_FPDFImageObj_GetImageMetadata: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        page: FPDF_PAGE,
        metadata: *mut FPDF_IMAGEOBJ_METADATA,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFImageObj_GetImagePixelSize: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        width: *mut c_uint,
        height: *mut c_uint,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFImageObj_GetRenderedBitmap: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        image_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BITMAP,
    pub(crate) fn_FPDFImageObj_SetMatrix: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFJavaScriptAction_GetName: unsafe extern "C" fn(
        javascript: FPDF_JAVASCRIPT_ACTION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFJavaScriptAction_GetScript: unsafe extern "C" fn(
        javascript: FPDF_JAVASCRIPT_ACTION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFLink_CloseWebLinks: unsafe extern "C" fn(link_page: FPDF_PAGELINK),
    pub(crate) fn_FPDFLink_CountQuadPoints: unsafe extern "C" fn(link_annot: FPDF_LINK) -> c_int,
    pub(crate) fn_FPDFLink_CountRects:
        unsafe extern "C" fn(link_page: FPDF_PAGELINK, link_index: c_int) -> c_int,
    pub(crate) fn_FPDFLink_CountWebLinks: unsafe extern "C" fn(link_page: FPDF_PAGELINK) -> c_int,
    pub(crate) fn_FPDFLink_GetAction: unsafe extern "C" fn(link: FPDF_LINK) -> FPDF_ACTION,
    pub(crate) fn_FPDFLink_GetAnnot:
        unsafe extern "C" fn(page: FPDF_PAGE, link_annot: FPDF_LINK) -> FPDF_ANNOTATION,
    pub(crate) fn_FPDFLink_GetAnnotRect:
        unsafe extern "C" fn(link_annot: FPDF_LINK, rect: *mut FS_RECTF) -> FPDF_BOOL,
    pub(crate) fn_FPDFLink_GetDest:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, link: FPDF_LINK) -> FPDF_DEST,
    pub(crate) fn_FPDFLink_GetLinkAtPoint:
        unsafe extern "C" fn(page: FPDF_PAGE, x: f64, y: f64) -> FPDF_LINK,
    pub(crate) fn_FPDFLink_GetLinkZOrderAtPoint:
        unsafe extern "C" fn(page: FPDF_PAGE, x: f64, y: f64) -> c_int,
    pub(crate) fn_FPDFLink_GetQuadPoints: unsafe extern "C" fn(
        link_annot: FPDF_LINK,
        quad_index: c_int,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFLink_GetRect: unsafe extern "C" fn(
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        rect_index: c_int,
        left: *mut f64,
        top: *mut f64,
        right: *mut f64,
        bottom: *mut f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFLink_GetTextRange: unsafe extern "C" fn(
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        start_char_index: *mut c_int,
        char_count: *mut c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFLink_GetURL: unsafe extern "C" fn(
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        buffer: *mut c_ushort,
        buflen: c_int,
    ) -> c_int,
    pub(crate) fn_FPDFLink_LoadWebLinks:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE) -> FPDF_PAGELINK,
    pub(crate) fn_FPDFPageObjMark_CountParams:
        unsafe extern "C" fn(mark: FPDF_PAGEOBJECTMARK) -> c_int,
    pub(crate) fn_FPDFPageObjMark_GetName: unsafe extern "C" fn(
        mark: FPDF_PAGEOBJECTMARK,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObjMark_GetParamBlobValue: unsafe extern "C" fn(
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        buffer: *mut c_uchar,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObjMark_GetParamIntValue: unsafe extern "C" fn(
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        out_value: *mut c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObjMark_GetParamKey: unsafe extern "C" fn(
        mark: FPDF_PAGEOBJECTMARK,
        index: c_ulong,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObjMark_GetParamStringValue: unsafe extern "C" fn(
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObjMark_GetParamValueType:
        unsafe extern "C" fn(mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
    pub(crate) fn_FPDFPageObjMark_RemoveParam: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObjMark_SetBlobParam: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: *const c_uchar,
        value_len: c_ulong,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObjMark_SetIntParam: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObjMark_SetStringParam: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: FPDF_BYTESTRING,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_AddMark: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        name: FPDF_BYTESTRING,
    ) -> FPDF_PAGEOBJECTMARK,
    pub(crate) fn_FPDFPageObj_CountMarks:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int,
    pub(crate) fn_FPDFPageObj_CreateNewPath:
        unsafe extern "C" fn(x: f32, y: f32) -> FPDF_PAGEOBJECT,
    pub(crate) fn_FPDFPageObj_CreateNewRect:
        unsafe extern "C" fn(x: f32, y: f32, w: f32, h: f32) -> FPDF_PAGEOBJECT,
    pub(crate) fn_FPDFPageObj_CreateTextObj: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        font: FPDF_FONT,
        font_size: f32,
    ) -> FPDF_PAGEOBJECT,
    pub(crate) fn_FPDFPageObj_Destroy: unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT),
    pub(crate) fn_FPDFPageObj_GetBounds: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_GetClipPath:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> FPDF_CLIPPATH,
    pub(crate) fn_FPDFPageObj_GetDashArray: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        dash_array: *mut f32,
        dash_count: usize,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_GetDashCount:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int,
    pub(crate) fn_FPDFPageObj_GetDashPhase:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, phase: *mut f32) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_GetFillColor: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_GetIsActive:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, active: *mut FPDF_BOOL) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_GetLineCap:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int,
    pub(crate) fn_FPDFPageObj_GetLineJoin:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int,
    pub(crate) fn_FPDFPageObj_GetMark:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, index: c_ulong) -> FPDF_PAGEOBJECTMARK,
    pub(crate) fn_FPDFPageObj_GetMarkedContentID:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int,
    pub(crate) fn_FPDFPageObj_GetMatrix:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, matrix: *mut FS_MATRIX) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_GetRotatedBounds: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_GetStrokeColor: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_GetStrokeWidth:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, width: *mut f32) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_GetType: unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int,
    pub(crate) fn_FPDFPageObj_HasTransparency:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_NewImageObj:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT,
    pub(crate) fn_FPDFPageObj_NewTextObj: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        font: FPDF_BYTESTRING,
        font_size: f32,
    ) -> FPDF_PAGEOBJECT,
    pub(crate) fn_FPDFPageObj_RemoveMark:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, mark: FPDF_PAGEOBJECTMARK) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_SetBlendMode:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, blend_mode: FPDF_BYTESTRING),
    pub(crate) fn_FPDFPageObj_SetDashArray: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        dash_array: *const f32,
        dash_count: usize,
        phase: f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_SetDashPhase:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, phase: f32) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_SetFillColor: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_SetIsActive:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, active: FPDF_BOOL) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_SetLineCap:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, line_cap: c_int) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_SetLineJoin:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, line_join: c_int) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_SetMatrix:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_SetStrokeColor: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_SetStrokeWidth:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, width: f32) -> FPDF_BOOL,
    pub(crate) fn_FPDFPageObj_Transform: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ),
    pub(crate) fn_FPDFPageObj_TransformClipPath: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ),
    pub(crate) fn_FPDFPageObj_TransformF:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_CloseAnnot: unsafe extern "C" fn(annot: FPDF_ANNOTATION),
    pub(crate) fn_FPDFPage_CountObjects: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    pub(crate) fn_FPDFPage_CreateAnnot:
        unsafe extern "C" fn(page: FPDF_PAGE, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_ANNOTATION,
    pub(crate) fn_FPDFPage_Delete: unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: c_int),
    pub(crate) fn_FPDFPage_Flatten: unsafe extern "C" fn(page: FPDF_PAGE, nFlag: c_int) -> c_int,
    pub(crate) fn_FPDFPage_FormFieldZOrderAtPoint: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        page_x: f64,
        page_y: f64,
    ) -> c_int,
    pub(crate) fn_FPDFPage_GenerateContent: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_GetAnnot:
        unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_ANNOTATION,
    pub(crate) fn_FPDFPage_GetAnnotCount: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    pub(crate) fn_FPDFPage_GetAnnotIndex:
        unsafe extern "C" fn(page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> c_int,
    pub(crate) fn_FPDFPage_GetArtBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_GetBleedBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_GetCropBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_GetDecodedThumbnailData:
        unsafe extern "C" fn(page: FPDF_PAGE, buffer: *mut c_void, buflen: c_ulong) -> c_ulong,
    pub(crate) fn_FPDFPage_GetMediaBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_GetObject:
        unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT,
    pub(crate) fn_FPDFPage_GetRawThumbnailData:
        unsafe extern "C" fn(page: FPDF_PAGE, buffer: *mut c_void, buflen: c_ulong) -> c_ulong,
    pub(crate) fn_FPDFPage_GetRotation: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    pub(crate) fn_FPDFPage_GetThumbnailAsBitmap:
        unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BITMAP,
    pub(crate) fn_FPDFPage_GetTrimBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut f32,
        bottom: *mut f32,
        right: *mut f32,
        top: *mut f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_HasFormFieldAtPoint: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        page_x: f64,
        page_y: f64,
    ) -> c_int,
    pub(crate) fn_FPDFPage_HasTransparency: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_InsertClipPath:
        unsafe extern "C" fn(page: FPDF_PAGE, clipPath: FPDF_CLIPPATH),
    pub(crate) fn_FPDFPage_InsertObject:
        unsafe extern "C" fn(page: FPDF_PAGE, page_object: FPDF_PAGEOBJECT),
    pub(crate) fn_FPDFPage_InsertObjectAtIndex: unsafe extern "C" fn(
        page: FPDF_PAGE,
        page_object: FPDF_PAGEOBJECT,
        index: usize,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_New: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        width: f64,
        height: f64,
    ) -> FPDF_PAGE,
    pub(crate) fn_FPDFPage_RemoveAnnot:
        unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_RemoveObject:
        unsafe extern "C" fn(page: FPDF_PAGE, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_SetArtBox:
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
    pub(crate) fn_FPDFPage_SetBleedBox:
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
    pub(crate) fn_FPDFPage_SetCropBox:
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
    pub(crate) fn_FPDFPage_SetMediaBox:
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
    pub(crate) fn_FPDFPage_SetRotation: unsafe extern "C" fn(page: FPDF_PAGE, rotate: c_int),
    pub(crate) fn_FPDFPage_SetTrimBox:
        unsafe extern "C" fn(page: FPDF_PAGE, left: f32, bottom: f32, right: f32, top: f32),
    pub(crate) fn_FPDFPage_TransFormWithClip: unsafe extern "C" fn(
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipRect: *const FS_RECTF,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPage_TransformAnnots:
        unsafe extern "C" fn(page: FPDF_PAGE, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64),
    pub(crate) fn_FPDFPathSegment_GetClose:
        unsafe extern "C" fn(segment: FPDF_PATHSEGMENT) -> FPDF_BOOL,
    pub(crate) fn_FPDFPathSegment_GetPoint:
        unsafe extern "C" fn(segment: FPDF_PATHSEGMENT, x: *mut f32, y: *mut f32) -> FPDF_BOOL,
    pub(crate) fn_FPDFPathSegment_GetType: unsafe extern "C" fn(segment: FPDF_PATHSEGMENT) -> c_int,
    pub(crate) fn_FPDFPath_BezierTo: unsafe extern "C" fn(
        path: FPDF_PAGEOBJECT,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPath_Close: unsafe extern "C" fn(path: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    pub(crate) fn_FPDFPath_CountSegments: unsafe extern "C" fn(path: FPDF_PAGEOBJECT) -> c_int,
    pub(crate) fn_FPDFPath_GetDrawMode: unsafe extern "C" fn(
        path: FPDF_PAGEOBJECT,
        fillmode: *mut c_int,
        stroke: *mut FPDF_BOOL,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFPath_GetPathSegment:
        unsafe extern "C" fn(path: FPDF_PAGEOBJECT, index: c_int) -> FPDF_PATHSEGMENT,
    pub(crate) fn_FPDFPath_LineTo:
        unsafe extern "C" fn(path: FPDF_PAGEOBJECT, x: f32, y: f32) -> FPDF_BOOL,
    pub(crate) fn_FPDFPath_MoveTo:
        unsafe extern "C" fn(path: FPDF_PAGEOBJECT, x: f32, y: f32) -> FPDF_BOOL,
    pub(crate) fn_FPDFPath_SetDrawMode: unsafe extern "C" fn(
        path: FPDF_PAGEOBJECT,
        fillmode: c_int,
        stroke: FPDF_BOOL,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFSignatureObj_GetByteRange: unsafe extern "C" fn(
        signature: FPDF_SIGNATURE,
        buffer: *mut c_int,
        length: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFSignatureObj_GetContents: unsafe extern "C" fn(
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFSignatureObj_GetDocMDPPermission:
        unsafe extern "C" fn(signature: FPDF_SIGNATURE) -> c_uint,
    pub(crate) fn_FPDFSignatureObj_GetReason: unsafe extern "C" fn(
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFSignatureObj_GetSubFilter: unsafe extern "C" fn(
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFSignatureObj_GetTime: unsafe extern "C" fn(
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFTextObj_GetFont: unsafe extern "C" fn(text: FPDF_PAGEOBJECT) -> FPDF_FONT,
    pub(crate) fn_FPDFTextObj_GetFontSize:
        unsafe extern "C" fn(text: FPDF_PAGEOBJECT, size: *mut f32) -> FPDF_BOOL,
    pub(crate) fn_FPDFTextObj_GetRenderedBitmap: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        text_object: FPDF_PAGEOBJECT,
        scale: f32,
    ) -> FPDF_BITMAP,
    pub(crate) fn_FPDFTextObj_GetText: unsafe extern "C" fn(
        text_object: FPDF_PAGEOBJECT,
        text_page: FPDF_TEXTPAGE,
        buffer: *mut FPDF_WCHAR,
        length: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDFTextObj_GetTextRenderMode:
        unsafe extern "C" fn(text: FPDF_PAGEOBJECT) -> FPDF_TEXT_RENDERMODE,
    pub(crate) fn_FPDFTextObj_SetTextRenderMode:
        unsafe extern "C" fn(text: FPDF_PAGEOBJECT, render_mode: FPDF_TEXT_RENDERMODE) -> FPDF_BOOL,
    pub(crate) fn_FPDFText_ClosePage: unsafe extern "C" fn(text_page: FPDF_TEXTPAGE),
    pub(crate) fn_FPDFText_CountChars: unsafe extern "C" fn(text_page: FPDF_TEXTPAGE) -> c_int,
    pub(crate) fn_FPDFText_CountRects:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, start_index: c_int, count: c_int) -> c_int,
    pub(crate) fn_FPDFText_FindClose: unsafe extern "C" fn(handle: FPDF_SCHHANDLE),
    pub(crate) fn_FPDFText_FindNext: unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> FPDF_BOOL,
    pub(crate) fn_FPDFText_FindPrev: unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> FPDF_BOOL,
    pub(crate) fn_FPDFText_FindStart: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        findwhat: FPDF_WIDESTRING,
        flags: c_ulong,
        start_index: c_int,
    ) -> FPDF_SCHHANDLE,
    pub(crate) fn_FPDFText_GetBoundedText: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
        buffer: *mut c_ushort,
        buflen: c_int,
    ) -> c_int,
    pub(crate) fn_FPDFText_GetCharAngle:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> f32,
    pub(crate) fn_FPDFText_GetCharBox: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        left: *mut f64,
        right: *mut f64,
        bottom: *mut f64,
        top: *mut f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFText_GetCharIndexAtPos: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        x: f64,
        y: f64,
        xTolerance: f64,
        yTolerance: f64,
    ) -> c_int,
    pub(crate) fn_FPDFText_GetCharIndexFromTextIndex:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, nTextIndex: c_int) -> c_int,
    pub(crate) fn_FPDFText_GetCharOrigin: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        x: *mut f64,
        y: *mut f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFText_GetFillColor: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFText_GetFontInfo: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
        flags: *mut c_int,
    ) -> c_ulong,
    pub(crate) fn_FPDFText_GetFontSize:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> f64,
    pub(crate) fn_FPDFText_GetFontWeight:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_int,
    pub(crate) fn_FPDFText_GetLooseCharBox: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        rect: *mut FS_RECTF,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFText_GetMatrix: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFText_GetRect: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        rect_index: c_int,
        left: *mut f64,
        top: *mut f64,
        right: *mut f64,
        bottom: *mut f64,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFText_GetSchCount: unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> c_int,
    pub(crate) fn_FPDFText_GetSchResultIndex: unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> c_int,
    pub(crate) fn_FPDFText_GetStrokeColor: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFText_GetText: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        start_index: c_int,
        count: c_int,
        result: *mut c_ushort,
    ) -> c_int,
    pub(crate) fn_FPDFText_GetTextIndexFromCharIndex:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, nCharIndex: c_int) -> c_int,
    pub(crate) fn_FPDFText_GetTextObject:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> FPDF_PAGEOBJECT,
    pub(crate) fn_FPDFText_GetUnicode:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_uint,
    pub(crate) fn_FPDFText_HasUnicodeMapError:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_int,
    pub(crate) fn_FPDFText_IsGenerated:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_int,
    pub(crate) fn_FPDFText_IsHyphen:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_int,
    pub(crate) fn_FPDFText_LoadCidType2Font: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        font_data: *const u8,
        font_data_size: u32,
        to_unicode_cmap: FPDF_BYTESTRING,
        cid_to_gid_map_data: *const u8,
        cid_to_gid_map_data_size: u32,
    ) -> FPDF_FONT,
    pub(crate) fn_FPDFText_LoadFont: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        data: *const u8,
        size: u32,
        font_type: c_int,
        cid: FPDF_BOOL,
    ) -> FPDF_FONT,
    pub(crate) fn_FPDFText_LoadPage: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_TEXTPAGE,
    pub(crate) fn_FPDFText_LoadStandardFont:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, font: FPDF_BYTESTRING) -> FPDF_FONT,
    pub(crate) fn_FPDFText_SetCharcodes: unsafe extern "C" fn(
        text_object: FPDF_PAGEOBJECT,
        charcodes: *const u32,
        count: usize,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDFText_SetText:
        unsafe extern "C" fn(text_object: FPDF_PAGEOBJECT, text: FPDF_WIDESTRING) -> FPDF_BOOL,
    pub(crate) fn_FPDF_AddInstalledFont:
        unsafe extern "C" fn(mapper: *mut c_void, face: *const c_char, charset: c_int),
    pub(crate) fn_FPDF_CloseDocument: unsafe extern "C" fn(document: FPDF_DOCUMENT),
    pub(crate) fn_FPDF_ClosePage: unsafe extern "C" fn(page: FPDF_PAGE),
    pub(crate) fn_FPDF_CloseXObject: unsafe extern "C" fn(xobject: FPDF_XOBJECT),
    pub(crate) fn_FPDF_CopyViewerPreferences:
        unsafe extern "C" fn(dest_doc: FPDF_DOCUMENT, src_doc: FPDF_DOCUMENT) -> FPDF_BOOL,
    pub(crate) fn_FPDF_CountNamedDests: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_DWORD,
    pub(crate) fn_FPDF_CreateClipPath:
        unsafe extern "C" fn(left: f32, bottom: f32, right: f32, top: f32) -> FPDF_CLIPPATH,
    pub(crate) fn_FPDF_CreateNewDocument: unsafe extern "C" fn() -> FPDF_DOCUMENT,
    pub(crate) fn_FPDF_DestroyClipPath: unsafe extern "C" fn(clipPath: FPDF_CLIPPATH),
    pub(crate) fn_FPDF_DestroyLibrary: unsafe extern "C" fn(),
    pub(crate) fn_FPDF_DeviceToPage: unsafe extern "C" fn(
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
    pub(crate) fn_FPDF_DocumentHasValidCrossReferenceTable:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL,
    pub(crate) fn_FPDF_FFLDraw: unsafe extern "C" fn(
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
    pub(crate) fn_FPDF_GetDefaultTTFMap: unsafe extern "C" fn() -> *const FPDF_CharsetFontMap,
    pub(crate) fn_FPDF_GetDefaultTTFMapCount: unsafe extern "C" fn() -> usize,
    pub(crate) fn_FPDF_GetDefaultTTFMapEntry:
        unsafe extern "C" fn(index: usize) -> *const FPDF_CharsetFontMap,
    pub(crate) fn_FPDF_GetDocPermissions: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_ulong,
    pub(crate) fn_FPDF_GetDocUserPermissions:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_ulong,
    pub(crate) fn_FPDF_GetFileIdentifier: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        id_type: FPDF_FILEIDTYPE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_GetFileVersion:
        unsafe extern "C" fn(doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL,
    pub(crate) fn_FPDF_GetFormType: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    pub(crate) fn_FPDF_GetLastError: unsafe extern "C" fn() -> c_ulong,
    pub(crate) fn_FPDF_GetMetaText: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        tag: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_GetNamedDest: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        index: c_int,
        buffer: *mut c_void,
        buflen: *mut c_long,
    ) -> FPDF_DEST,
    pub(crate) fn_FPDF_GetNamedDestByName:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, name: FPDF_BYTESTRING) -> FPDF_DEST,
    pub(crate) fn_FPDF_GetPageAAction:
        unsafe extern "C" fn(page: FPDF_PAGE, aa_type: c_int) -> FPDF_ACTION,
    pub(crate) fn_FPDF_GetPageBoundingBox:
        unsafe extern "C" fn(page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL,
    pub(crate) fn_FPDF_GetPageCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    pub(crate) fn_FPDF_GetPageHeight: unsafe extern "C" fn(page: FPDF_PAGE) -> f64,
    pub(crate) fn_FPDF_GetPageHeightF: unsafe extern "C" fn(page: FPDF_PAGE) -> f32,
    pub(crate) fn_FPDF_GetPageLabel: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_GetPageSizeByIndex: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        width: *mut f64,
        height: *mut f64,
    ) -> c_int,
    pub(crate) fn_FPDF_GetPageSizeByIndexF: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        size: *mut FS_SIZEF,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_GetPageWidth: unsafe extern "C" fn(page: FPDF_PAGE) -> f64,
    pub(crate) fn_FPDF_GetPageWidthF: unsafe extern "C" fn(page: FPDF_PAGE) -> f32,
    pub(crate) fn_FPDF_GetSecurityHandlerRevision:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    pub(crate) fn_FPDF_GetSignatureCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    pub(crate) fn_FPDF_GetSignatureObject:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_SIGNATURE,
    pub(crate) fn_FPDF_GetTrailerEnds: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        buffer: *mut c_uint,
        length: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_GetXFAPacketContent: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_GetXFAPacketCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    pub(crate) fn_FPDF_GetXFAPacketName: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_ImportNPagesToOne: unsafe extern "C" fn(
        src_doc: FPDF_DOCUMENT,
        output_width: f32,
        output_height: f32,
        num_pages_on_x_axis: usize,
        num_pages_on_y_axis: usize,
    ) -> FPDF_DOCUMENT,
    pub(crate) fn_FPDF_ImportPages: unsafe extern "C" fn(
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        pagerange: FPDF_BYTESTRING,
        index: c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_ImportPagesByIndex: unsafe extern "C" fn(
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        page_indices: *const c_int,
        length: c_ulong,
        index: c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_InitLibrary: unsafe extern "C" fn(),
    pub(crate) fn_FPDF_InitLibraryWithConfig:
        unsafe extern "C" fn(config: *const FPDF_LIBRARY_CONFIG),
    pub(crate) fn_FPDF_LoadCustomDocument: unsafe extern "C" fn(
        pFileAccess: *mut FPDF_FILEACCESS,
        password: FPDF_BYTESTRING,
    ) -> FPDF_DOCUMENT,
    pub(crate) fn_FPDF_LoadPage:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE,
    pub(crate) fn_FPDF_LoadXFA: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL,
    pub(crate) fn_FPDF_MovePages: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_indices: *const c_int,
        page_indices_len: c_ulong,
        dest_page_index: c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_NewFormObjectFromXObject:
        unsafe extern "C" fn(xobject: FPDF_XOBJECT) -> FPDF_PAGEOBJECT,
    pub(crate) fn_FPDF_NewXObjectFromPage: unsafe extern "C" fn(
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        src_page_index: c_int,
    ) -> FPDF_XOBJECT,
    pub(crate) fn_FPDF_PageToDevice: unsafe extern "C" fn(
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
    pub(crate) fn_FPDF_RemoveFormFieldHighlight: unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE),
    pub(crate) fn_FPDF_RenderPageBitmap: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    ),
    pub(crate) fn_FPDF_RenderPageBitmapWithColorScheme_Start: unsafe extern "C" fn(
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
    pub(crate) fn_FPDF_RenderPageBitmapWithMatrix: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipping: *const FS_RECTF,
        flags: c_int,
    ),
    pub(crate) fn_FPDF_RenderPageBitmap_Start: unsafe extern "C" fn(
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
    pub(crate) fn_FPDF_RenderPage_Close: unsafe extern "C" fn(page: FPDF_PAGE),
    pub(crate) fn_FPDF_RenderPage_Continue:
        unsafe extern "C" fn(page: FPDF_PAGE, pause: *mut IFSDK_PAUSE) -> c_int,
    pub(crate) fn_FPDF_SaveAsCopy: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_SaveWithVersion: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
        fileVersion: c_int,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_SetFormFieldHighlightAlpha:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, alpha: c_uchar),
    pub(crate) fn_FPDF_SetFormFieldHighlightColor:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, fieldType: c_int, color: c_ulong),
    pub(crate) fn_FPDF_SetSandBoxPolicy:
        unsafe extern "C" fn(policy: FPDF_DWORD, enable: FPDF_BOOL),
    pub(crate) fn_FPDF_StructElement_Attr_CountChildren:
        unsafe extern "C" fn(value: FPDF_STRUCTELEMENT_ATTR_VALUE) -> c_int,
    pub(crate) fn_FPDF_StructElement_Attr_GetBlobValue: unsafe extern "C" fn(
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_StructElement_Attr_GetBooleanValue: unsafe extern "C" fn(
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        out_value: *mut FPDF_BOOL,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_StructElement_Attr_GetChildAtIndex:
        unsafe extern "C" fn(
            value: FPDF_STRUCTELEMENT_ATTR_VALUE,
            index: c_int,
        ) -> FPDF_STRUCTELEMENT_ATTR_VALUE,
    pub(crate) fn_FPDF_StructElement_Attr_GetCount:
        unsafe extern "C" fn(struct_attribute: FPDF_STRUCTELEMENT_ATTR) -> c_int,
    pub(crate) fn_FPDF_StructElement_Attr_GetName: unsafe extern "C" fn(
        struct_attribute: FPDF_STRUCTELEMENT_ATTR,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_StructElement_Attr_GetNumberValue: unsafe extern "C" fn(
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        out_value: *mut f32,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_StructElement_Attr_GetStringValue: unsafe extern "C" fn(
        value: FPDF_STRUCTELEMENT_ATTR_VALUE,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    pub(crate) fn_FPDF_StructElement_Attr_GetType:
        unsafe extern "C" fn(value: FPDF_STRUCTELEMENT_ATTR_VALUE) -> FPDF_OBJECT_TYPE,
    pub(crate) fn_FPDF_StructElement_Attr_GetValue:
        unsafe extern "C" fn(
            struct_attribute: FPDF_STRUCTELEMENT_ATTR,
            name: FPDF_BYTESTRING,
        ) -> FPDF_STRUCTELEMENT_ATTR_VALUE,
    pub(crate) fn_FPDF_StructElement_CountChildren:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int,
    pub(crate) fn_FPDF_StructElement_GetActualText: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_StructElement_GetAltText: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_StructElement_GetAttributeAtIndex:
        unsafe extern "C" fn(
            struct_element: FPDF_STRUCTELEMENT,
            index: c_int,
        ) -> FPDF_STRUCTELEMENT_ATTR,
    pub(crate) fn_FPDF_StructElement_GetAttributeCount:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int,
    pub(crate) fn_FPDF_StructElement_GetChildAtIndex: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        index: c_int,
    )
        -> FPDF_STRUCTELEMENT,
    pub(crate) fn_FPDF_StructElement_GetChildMarkedContentID:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT, index: c_int) -> c_int,
    pub(crate) fn_FPDF_StructElement_GetID: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_StructElement_GetLang: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_StructElement_GetMarkedContentID:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int,
    pub(crate) fn_FPDF_StructElement_GetMarkedContentIdAtIndex:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT, index: c_int) -> c_int,
    pub(crate) fn_FPDF_StructElement_GetMarkedContentIdCount:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int,
    pub(crate) fn_FPDF_StructElement_GetObjType: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_StructElement_GetParent:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> FPDF_STRUCTELEMENT,
    pub(crate) fn_FPDF_StructElement_GetStringAttribute: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        attr_name: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_StructElement_GetTitle: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_StructElement_GetType: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_StructTree_Close: unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE),
    pub(crate) fn_FPDF_StructTree_CountChildren:
        unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE) -> c_int,
    pub(crate) fn_FPDF_StructTree_GetChildAtIndex:
        unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE, index: c_int) -> FPDF_STRUCTELEMENT,
    pub(crate) fn_FPDF_StructTree_GetForPage:
        unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_STRUCTTREE,
    pub(crate) fn_FPDF_VIEWERREF_GetDuplex:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_DUPLEXTYPE,
    pub(crate) fn_FPDF_VIEWERREF_GetName: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        key: FPDF_BYTESTRING,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong,
    pub(crate) fn_FPDF_VIEWERREF_GetNumCopies:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    pub(crate) fn_FPDF_VIEWERREF_GetPrintPageRange:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_PAGERANGE,
    pub(crate) fn_FPDF_VIEWERREF_GetPrintPageRangeCount:
        unsafe extern "C" fn(pagerange: FPDF_PAGERANGE) -> usize,
    pub(crate) fn_FPDF_VIEWERREF_GetPrintPageRangeElement:
        unsafe extern "C" fn(pagerange: FPDF_PAGERANGE, index: usize) -> c_int,
    pub(crate) fn_FPDF_VIEWERREF_GetPrintScaling:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL,
    pub(crate) fn_FSDK_SetUnSpObjProcessHandler:
        unsafe extern "C" fn(unsp_info: *mut UNSUPPORT_INFO) -> FPDF_BOOL,
    pub(crate) _lib: Library,
}
