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

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

//! Access to the C language types of PDFium

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_action_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_annotation_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_attachment_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_avail_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_bitmap_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_bookmark_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_clippath_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_dest_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_document_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_font_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_form_handle_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_glyphpath_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_javascript_action_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_link_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_page_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_pagelink_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_pageobject_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_pageobjectmark_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_pagerange_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_pathsegment_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_schhandle_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_signature_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_structelement_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_structelement_attr_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_structelement_attr_value_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_structtree_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_textpage_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_widget_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpdf_xobject_t__ {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_BSTR_ {
    pub str_: *mut ::std::os::raw::c_char,
    pub len: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FS_MATRIX_ {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FS_RECTF_ {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FS_SIZEF_ {
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FS_POINTF_ {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FS_QUADPOINTSF {
    pub x1: FS_FLOAT,
    pub y1: FS_FLOAT,
    pub x2: FS_FLOAT,
    pub y2: FS_FLOAT,
    pub x3: FS_FLOAT,
    pub y3: FS_FLOAT,
    pub x4: FS_FLOAT,
    pub y4: FS_FLOAT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_LIBRARY_CONFIG_ {
    pub version: ::std::os::raw::c_int,
    pub m_pUserFontPaths: *mut *const ::std::os::raw::c_char,
    pub m_pIsolate: *mut ::std::os::raw::c_void,
    pub m_v8EmbedderSlot: ::std::os::raw::c_uint,
    pub m_pPlatform: *mut ::std::os::raw::c_void,
    pub m_RendererType: FPDF_RENDERER_TYPE,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_FILEACCESS {
    pub m_FileLen: ::std::os::raw::c_ulong,
    pub m_GetBlock: ::std::option::Option<
        unsafe extern "C" fn(
            param: *mut ::std::os::raw::c_void,
            position: ::std::os::raw::c_ulong,
            pBuf: *mut ::std::os::raw::c_uchar,
            size: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_int,
    >,
    pub m_Param: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_FILEHANDLER_ {
    pub clientData: *mut ::std::os::raw::c_void,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(clientData: *mut ::std::os::raw::c_void)>,
    pub GetSize: ::std::option::Option<
        unsafe extern "C" fn(clientData: *mut ::std::os::raw::c_void) -> FPDF_DWORD,
    >,
    pub ReadBlock: ::std::option::Option<
        unsafe extern "C" fn(
            clientData: *mut ::std::os::raw::c_void,
            offset: FPDF_DWORD,
            buffer: *mut ::std::os::raw::c_void,
            size: FPDF_DWORD,
        ) -> FPDF_RESULT,
    >,
    pub WriteBlock: ::std::option::Option<
        unsafe extern "C" fn(
            clientData: *mut ::std::os::raw::c_void,
            offset: FPDF_DWORD,
            buffer: *const ::std::os::raw::c_void,
            size: FPDF_DWORD,
        ) -> FPDF_RESULT,
    >,
    pub Flush: ::std::option::Option<
        unsafe extern "C" fn(clientData: *mut ::std::os::raw::c_void) -> FPDF_RESULT,
    >,
    pub Truncate: ::std::option::Option<
        unsafe extern "C" fn(
            clientData: *mut ::std::os::raw::c_void,
            size: FPDF_DWORD,
        ) -> FPDF_RESULT,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_COLORSCHEME_ {
    pub path_fill_color: FPDF_DWORD,
    pub path_stroke_color: FPDF_DWORD,
    pub text_fill_color: FPDF_DWORD,
    pub text_stroke_color: FPDF_DWORD,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IPDF_JsPlatform {
    pub version: ::std::os::raw::c_int,
    pub app_alert: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            Msg: FPDF_WIDESTRING,
            Title: FPDF_WIDESTRING,
            Type: ::std::os::raw::c_int,
            Icon: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub app_beep: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _IPDF_JsPlatform, nType: ::std::os::raw::c_int),
    >,
    pub app_response: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            Question: FPDF_WIDESTRING,
            Title: FPDF_WIDESTRING,
            Default: FPDF_WIDESTRING,
            cLabel: FPDF_WIDESTRING,
            bPassword: FPDF_BOOL,
            response: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub Doc_getFilePath: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            filePath: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub Doc_mail: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            mailData: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
            bUI: FPDF_BOOL,
            To: FPDF_WIDESTRING,
            Subject: FPDF_WIDESTRING,
            CC: FPDF_WIDESTRING,
            BCC: FPDF_WIDESTRING,
            Msg: FPDF_WIDESTRING,
        ),
    >,
    pub Doc_print: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            bUI: FPDF_BOOL,
            nStart: ::std::os::raw::c_int,
            nEnd: ::std::os::raw::c_int,
            bSilent: FPDF_BOOL,
            bShrinkToFit: FPDF_BOOL,
            bPrintAsImage: FPDF_BOOL,
            bReverse: FPDF_BOOL,
            bAnnotations: FPDF_BOOL,
        ),
    >,
    pub Doc_submitForm: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            formData: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
            URL: FPDF_WIDESTRING,
        ),
    >,
    pub Doc_gotoPage: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _IPDF_JsPlatform, nPageNum: ::std::os::raw::c_int),
    >,
    pub Field_browse: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            filePath: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub m_pFormfillinfo: *mut ::std::os::raw::c_void,
    pub m_isolate: *mut ::std::os::raw::c_void,
    pub m_v8EmbedderSlot: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FPDF_SYSTEMTIME {
    pub wYear: ::std::os::raw::c_ushort,
    pub wMonth: ::std::os::raw::c_ushort,
    pub wDayOfWeek: ::std::os::raw::c_ushort,
    pub wDay: ::std::os::raw::c_ushort,
    pub wHour: ::std::os::raw::c_ushort,
    pub wMinute: ::std::os::raw::c_ushort,
    pub wSecond: ::std::os::raw::c_ushort,
    pub wMilliseconds: ::std::os::raw::c_ushort,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FPDF_FORMFILLINFO {
    pub version: ::std::os::raw::c_int,
    pub Release: ::std::option::Option<unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO)>,
    pub FFI_Invalidate: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
            left: f64,
            top: f64,
            right: f64,
            bottom: f64,
        ),
    >,
    pub FFI_OutputSelectedRect: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
            left: f64,
            top: f64,
            right: f64,
            bottom: f64,
        ),
    >,
    pub FFI_SetCursor: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, nCursorType: ::std::os::raw::c_int),
    >,
    pub FFI_SetTimer: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            uElapse: ::std::os::raw::c_int,
            lpTimerFunc: TimerCallback,
        ) -> ::std::os::raw::c_int,
    >,
    pub FFI_KillTimer: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, nTimerID: ::std::os::raw::c_int),
    >,
    pub FFI_GetLocalTime: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO) -> FPDF_SYSTEMTIME,
    >,
    pub FFI_OnChange: ::std::option::Option<unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO)>,
    pub FFI_GetPage: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
            nPageIndex: ::std::os::raw::c_int,
        ) -> FPDF_PAGE,
    >,
    pub FFI_GetCurrentPage: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, document: FPDF_DOCUMENT) -> FPDF_PAGE,
    >,
    pub FFI_GetRotation: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
        ) -> ::std::os::raw::c_int,
    >,
    pub FFI_ExecuteNamedAction: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, namedAction: FPDF_BYTESTRING),
    >,
    pub FFI_SetTextFieldFocus: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            value: FPDF_WIDESTRING,
            valueLen: FPDF_DWORD,
            is_focus: FPDF_BOOL,
        ),
    >,
    pub FFI_DoURIAction: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, bsURI: FPDF_BYTESTRING),
    >,
    pub FFI_DoGoToAction: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            nPageIndex: ::std::os::raw::c_int,
            zoomMode: ::std::os::raw::c_int,
            fPosArray: *mut f32,
            sizeofArray: ::std::os::raw::c_int,
        ),
    >,
    pub m_pJsPlatform: *mut IPDF_JSPLATFORM,
    pub xfa_disabled: FPDF_BOOL,
    pub FFI_DisplayCaret: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
            bVisible: FPDF_BOOL,
            left: f64,
            top: f64,
            right: f64,
            bottom: f64,
        ),
    >,
    pub FFI_GetCurrentPageIndex: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
        ) -> ::std::os::raw::c_int,
    >,
    pub FFI_SetCurrentPage: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
            iCurPage: ::std::os::raw::c_int,
        ),
    >,
    pub FFI_GotoURL: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
            wsURL: FPDF_WIDESTRING,
        ),
    >,
    pub FFI_GetPageViewRect: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
            left: *mut f64,
            top: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
        ),
    >,
    pub FFI_PageEvent: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page_count: ::std::os::raw::c_int,
            event_type: FPDF_DWORD,
        ),
    >,
    pub FFI_PopupMenu: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
            hWidget: FPDF_WIDGET,
            menuFlag: ::std::os::raw::c_int,
            x: f32,
            y: f32,
        ) -> FPDF_BOOL,
    >,
    pub FFI_OpenFile: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            fileFlag: ::std::os::raw::c_int,
            wsURL: FPDF_WIDESTRING,
            mode: *const ::std::os::raw::c_char,
        ) -> *mut FPDF_FILEHANDLER,
    >,
    pub FFI_EmailTo: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            fileHandler: *mut FPDF_FILEHANDLER,
            pTo: FPDF_WIDESTRING,
            pSubject: FPDF_WIDESTRING,
            pCC: FPDF_WIDESTRING,
            pBcc: FPDF_WIDESTRING,
            pMsg: FPDF_WIDESTRING,
        ),
    >,
    pub FFI_UploadTo: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            fileHandler: *mut FPDF_FILEHANDLER,
            fileFlag: ::std::os::raw::c_int,
            uploadTo: FPDF_WIDESTRING,
        ),
    >,
    pub FFI_GetPlatform: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            platform: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub FFI_GetLanguage: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            language: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub FFI_DownloadFromURL: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            URL: FPDF_WIDESTRING,
        ) -> *mut FPDF_FILEHANDLER,
    >,
    pub FFI_PostRequestURL: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            wsURL: FPDF_WIDESTRING,
            wsData: FPDF_WIDESTRING,
            wsContentType: FPDF_WIDESTRING,
            wsEncode: FPDF_WIDESTRING,
            wsHeader: FPDF_WIDESTRING,
            response: *mut FPDF_BSTR,
        ) -> FPDF_BOOL,
    >,
    pub FFI_PutRequestURL: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            wsURL: FPDF_WIDESTRING,
            wsData: FPDF_WIDESTRING,
            wsEncode: FPDF_WIDESTRING,
        ) -> FPDF_BOOL,
    >,
    pub FFI_OnFocusChange: ::std::option::Option<
        unsafe extern "C" fn(
            param: *mut _FPDF_FORMFILLINFO,
            annot: FPDF_ANNOTATION,
            page_index: ::std::os::raw::c_int,
        ),
    >,
    pub FFI_DoURIActionWithKeyboardModifier: ::std::option::Option<
        unsafe extern "C" fn(
            param: *mut _FPDF_FORMFILLINFO,
            uri: FPDF_BYTESTRING,
            modifiers: ::std::os::raw::c_int,
        ),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FX_FILEAVAIL {
    pub version: ::std::os::raw::c_int,
    pub IsDataAvail: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FX_FILEAVAIL, offset: usize, size: usize) -> FPDF_BOOL,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FX_DOWNLOADHINTS {
    pub version: ::std::os::raw::c_int,
    pub AddSegment: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FX_DOWNLOADHINTS, offset: usize, size: usize),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_IMAGEOBJ_METADATA {
    pub width: ::std::os::raw::c_uint,
    pub height: ::std::os::raw::c_uint,
    pub horizontal_dpi: f32,
    pub vertical_dpi: f32,
    pub bits_per_pixel: ::std::os::raw::c_uint,
    pub colorspace: ::std::os::raw::c_int,
    pub marked_content_id: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    pub tm_sec: ::std::os::raw::c_int,
    pub tm_min: ::std::os::raw::c_int,
    pub tm_hour: ::std::os::raw::c_int,
    pub tm_mday: ::std::os::raw::c_int,
    pub tm_mon: ::std::os::raw::c_int,
    pub tm_year: ::std::os::raw::c_int,
    pub tm_wday: ::std::os::raw::c_int,
    pub tm_yday: ::std::os::raw::c_int,
    pub tm_isdst: ::std::os::raw::c_int,
    pub tm_gmtoff: ::std::os::raw::c_long,
    pub tm_zone: *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigevent {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _UNSUPPORT_INFO {
    pub version: ::std::os::raw::c_int,
    pub FSDK_UnSupport_Handler: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _UNSUPPORT_INFO, nType: ::std::os::raw::c_int),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IFSDK_PAUSE {
    pub version: ::std::os::raw::c_int,
    pub NeedToPauseNow:
        ::std::option::Option<unsafe extern "C" fn(pThis: *mut _IFSDK_PAUSE) -> FPDF_BOOL>,
    pub user: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_FILEWRITE_ {
    pub version: ::std::os::raw::c_int,
    pub WriteBlock: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut FPDF_FILEWRITE_,
            pData: *const ::std::os::raw::c_void,
            size: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_int,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FPDF_SYSFONTINFO {
    pub version: ::std::os::raw::c_int,
    pub Release: ::std::option::Option<unsafe extern "C" fn(pThis: *mut _FPDF_SYSFONTINFO)>,
    pub EnumFonts: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_SYSFONTINFO, pMapper: *mut ::std::os::raw::c_void),
    >,
    pub MapFont: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            weight: ::std::os::raw::c_int,
            bItalic: FPDF_BOOL,
            charset: ::std::os::raw::c_int,
            pitch_family: ::std::os::raw::c_int,
            face: *const ::std::os::raw::c_char,
            bExact: *mut FPDF_BOOL,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetFont: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            face: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetFontData: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            hFont: *mut ::std::os::raw::c_void,
            table: ::std::os::raw::c_uint,
            buffer: *mut ::std::os::raw::c_uchar,
            buf_size: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_ulong,
    >,
    pub GetFaceName: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            hFont: *mut ::std::os::raw::c_void,
            buffer: *mut ::std::os::raw::c_char,
            buf_size: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_ulong,
    >,
    pub GetFontCharset: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            hFont: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub DeleteFont: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_SYSFONTINFO, hFont: *mut ::std::os::raw::c_void),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_CharsetFontMap_ {
    pub charset: ::std::os::raw::c_int,
    pub fontname: *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}

pub type wchar_t = ::std::os::raw::c_int;

pub type FPDF_TEXT_RENDERMODE = ::std::os::raw::c_int;

pub type FPDF_ACTION = *mut fpdf_action_t__;

pub type FPDF_ANNOTATION = *mut fpdf_annotation_t__;

pub type FPDF_ATTACHMENT = *mut fpdf_attachment_t__;

pub type FPDF_AVAIL = *mut fpdf_avail_t__;

pub type FPDF_BITMAP = *mut fpdf_bitmap_t__;

pub type FPDF_BOOKMARK = *mut fpdf_bookmark_t__;

pub type FPDF_CLIPPATH = *mut fpdf_clippath_t__;

pub type FPDF_DEST = *mut fpdf_dest_t__;

pub type FPDF_DOCUMENT = *mut fpdf_document_t__;

pub type FPDF_FONT = *mut fpdf_font_t__;

pub type FPDF_FORMHANDLE = *mut fpdf_form_handle_t__;

pub type FPDF_GLYPHPATH = *const fpdf_glyphpath_t__;

pub type FPDF_JAVASCRIPT_ACTION = *mut fpdf_javascript_action_t;

pub type FPDF_LINK = *mut fpdf_link_t__;

pub type FPDF_PAGE = *mut fpdf_page_t__;

pub type FPDF_PAGELINK = *mut fpdf_pagelink_t__;

pub type FPDF_PAGEOBJECT = *mut fpdf_pageobject_t__;

pub type FPDF_PAGEOBJECTMARK = *mut fpdf_pageobjectmark_t__;

pub type FPDF_PAGERANGE = *const fpdf_pagerange_t__;

pub type FPDF_PATHSEGMENT = *const fpdf_pathsegment_t;

pub type FPDF_SCHHANDLE = *mut fpdf_schhandle_t__;

pub type FPDF_SIGNATURE = *const fpdf_signature_t__;

pub type FPDF_SKIA_CANVAS = *mut ::std::os::raw::c_void;

pub type FPDF_STRUCTELEMENT = *mut fpdf_structelement_t__;

pub type FPDF_STRUCTELEMENT_ATTR = *const fpdf_structelement_attr_t__;

pub type FPDF_STRUCTELEMENT_ATTR_VALUE = *const fpdf_structelement_attr_value_t__;

pub type FPDF_STRUCTTREE = *mut fpdf_structtree_t__;

pub type FPDF_TEXTPAGE = *mut fpdf_textpage_t__;

pub type FPDF_WIDGET = *mut fpdf_widget_t__;

pub type FPDF_XOBJECT = *mut fpdf_xobject_t__;

pub type FPDF_BOOL = ::std::os::raw::c_int;

pub type FPDF_RESULT = ::std::os::raw::c_int;

pub type FPDF_DWORD = ::std::os::raw::c_ulong;

pub type FS_FLOAT = f32;

pub type _FPDF_DUPLEXTYPE_ = ::std::os::raw::c_uint;

pub type FPDF_WCHAR = ::std::os::raw::c_ushort;

pub type FPDF_BYTESTRING = *const ::std::os::raw::c_char;

pub type FPDF_WIDESTRING = *const FPDF_WCHAR;

pub type FPDF_BSTR = FPDF_BSTR_;

pub type FPDF_STRING = *const ::std::os::raw::c_char;

pub type FS_MATRIX = _FS_MATRIX_;

pub type FS_LPRECTF = *mut _FS_RECTF_;

pub type FS_RECTF = _FS_RECTF_;

pub type FS_LPCRECTF = *const FS_RECTF;

pub type FS_LPSIZEF = *mut FS_SIZEF_;

pub type FS_SIZEF = FS_SIZEF_;

pub type FS_LPCSIZEF = *const FS_SIZEF;

pub type FS_LPPOINTF = *mut FS_POINTF_;

pub type FS_POINTF = FS_POINTF_;

pub type FS_LPCPOINTF = *const FS_POINTF;

pub type FS_QUADPOINTSF = _FS_QUADPOINTSF;

pub type FPDF_ANNOTATION_SUBTYPE = ::std::os::raw::c_int;

pub type FPDF_ANNOT_APPEARANCEMODE = ::std::os::raw::c_int;

pub type FPDF_OBJECT_TYPE = ::std::os::raw::c_int;

pub type FPDF_RENDERER_TYPE = ::std::os::raw::c_uint;

pub type FPDF_LIBRARY_CONFIG = FPDF_LIBRARY_CONFIG_;

pub type FPDF_FILEHANDLER = FPDF_FILEHANDLER_;

pub type FPDF_COLORSCHEME = FPDF_COLORSCHEME_;

pub type IPDF_JSPLATFORM = _IPDF_JsPlatform;

pub type TimerCallback =
    ::std::option::Option<unsafe extern "C" fn(idEvent: ::std::os::raw::c_int)>;

pub type FPDF_SYSTEMTIME = _FPDF_SYSTEMTIME;

pub type FPDF_FORMFILLINFO = _FPDF_FORMFILLINFO;

pub type FPDFANNOT_COLORTYPE = ::std::os::raw::c_uint;

pub type FX_FILEAVAIL = _FX_FILEAVAIL;

pub type FX_DOWNLOADHINTS = _FX_DOWNLOADHINTS;

pub type FPDF_FILEIDTYPE = ::std::os::raw::c_uint;

pub type __u_char = ::std::os::raw::c_uchar;

pub type __u_short = ::std::os::raw::c_ushort;

pub type __u_int = ::std::os::raw::c_uint;

pub type __u_long = ::std::os::raw::c_ulong;

pub type __int8_t = ::std::os::raw::c_schar;

pub type __uint8_t = ::std::os::raw::c_uchar;

pub type __int16_t = ::std::os::raw::c_short;

pub type __uint16_t = ::std::os::raw::c_ushort;

pub type __int32_t = ::std::os::raw::c_int;

pub type __uint32_t = ::std::os::raw::c_uint;

pub type __int64_t = ::std::os::raw::c_long;

pub type __uint64_t = ::std::os::raw::c_ulong;

pub type __int_least8_t = __int8_t;

pub type __uint_least8_t = __uint8_t;

pub type __int_least16_t = __int16_t;

pub type __uint_least16_t = __uint16_t;

pub type __int_least32_t = __int32_t;

pub type __uint_least32_t = __uint32_t;

pub type __int_least64_t = __int64_t;

pub type __uint_least64_t = __uint64_t;

pub type __quad_t = ::std::os::raw::c_long;

pub type __u_quad_t = ::std::os::raw::c_ulong;

pub type __intmax_t = ::std::os::raw::c_long;

pub type __uintmax_t = ::std::os::raw::c_ulong;

pub type __dev_t = ::std::os::raw::c_ulong;

pub type __uid_t = ::std::os::raw::c_uint;

pub type __gid_t = ::std::os::raw::c_uint;

pub type __ino_t = ::std::os::raw::c_ulong;

pub type __ino64_t = ::std::os::raw::c_ulong;

pub type __mode_t = ::std::os::raw::c_uint;

pub type __nlink_t = ::std::os::raw::c_ulong;

pub type __off_t = ::std::os::raw::c_long;

pub type __off64_t = ::std::os::raw::c_long;

pub type __pid_t = ::std::os::raw::c_int;

pub type __clock_t = ::std::os::raw::c_long;

pub type __rlim_t = ::std::os::raw::c_ulong;

pub type __rlim64_t = ::std::os::raw::c_ulong;

pub type __id_t = ::std::os::raw::c_uint;

pub type __time_t = ::std::os::raw::c_long;

pub type __useconds_t = ::std::os::raw::c_uint;

pub type __suseconds_t = ::std::os::raw::c_long;

pub type __suseconds64_t = ::std::os::raw::c_long;

pub type __daddr_t = ::std::os::raw::c_int;

pub type __key_t = ::std::os::raw::c_int;

pub type __clockid_t = ::std::os::raw::c_int;

pub type __timer_t = *mut ::std::os::raw::c_void;

pub type __blksize_t = ::std::os::raw::c_long;

pub type __blkcnt_t = ::std::os::raw::c_long;

pub type __blkcnt64_t = ::std::os::raw::c_long;

pub type __fsblkcnt_t = ::std::os::raw::c_ulong;

pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;

pub type __fsfilcnt_t = ::std::os::raw::c_ulong;

pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;

pub type __fsword_t = ::std::os::raw::c_long;

pub type __ssize_t = ::std::os::raw::c_long;

pub type __syscall_slong_t = ::std::os::raw::c_long;

pub type __syscall_ulong_t = ::std::os::raw::c_ulong;

pub type __loff_t = __off64_t;

pub type __caddr_t = *mut ::std::os::raw::c_char;

pub type __intptr_t = ::std::os::raw::c_long;

pub type __socklen_t = ::std::os::raw::c_uint;

pub type __sig_atomic_t = ::std::os::raw::c_int;

pub type int_least8_t = __int_least8_t;

pub type int_least16_t = __int_least16_t;

pub type int_least32_t = __int_least32_t;

pub type int_least64_t = __int_least64_t;

pub type uint_least8_t = __uint_least8_t;

pub type uint_least16_t = __uint_least16_t;

pub type uint_least32_t = __uint_least32_t;

pub type uint_least64_t = __uint_least64_t;

pub type int_fast8_t = ::std::os::raw::c_schar;

pub type int_fast16_t = ::std::os::raw::c_long;

pub type int_fast32_t = ::std::os::raw::c_long;

pub type int_fast64_t = ::std::os::raw::c_long;

pub type uint_fast8_t = ::std::os::raw::c_uchar;

pub type uint_fast16_t = ::std::os::raw::c_ulong;

pub type uint_fast32_t = ::std::os::raw::c_ulong;

pub type uint_fast64_t = ::std::os::raw::c_ulong;

pub type intmax_t = __intmax_t;

pub type uintmax_t = __uintmax_t;

pub type clock_t = __clock_t;

pub type time_t = __time_t;

pub type clockid_t = __clockid_t;

pub type timer_t = __timer_t;

pub type pid_t = __pid_t;

pub type __locale_t = *mut __locale_struct;

pub type locale_t = __locale_t;

pub type UNSUPPORT_INFO = _UNSUPPORT_INFO;

pub type FWL_EVENTFLAG = ::std::os::raw::c_uint;

pub type FWL_VKEYCODE = ::std::os::raw::c_uint;

pub type IFSDK_PAUSE = _IFSDK_PAUSE;

pub type FPDF_FILEWRITE = FPDF_FILEWRITE_;

pub type FPDF_SYSFONTINFO = _FPDF_SYSFONTINFO;

pub type FPDF_CharsetFontMap = FPDF_CharsetFontMap_;
