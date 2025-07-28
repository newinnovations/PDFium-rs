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

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

//! Types (structures and typedefs) used by the PDFium C library
pub use crate::c_api::pdfium_handle::Handle;

/// Define 'max_align_t' to match the GCC definition.
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

/// Structure for persisting a string beyond the duration of a callback.
///
/// C documentation:
///
/// ```text
/// Structure for persisting a string beyond the duration of a callback.
/// Note: although represented as a char*, string may be interpreted as
/// a UTF-16LE formated string. Used only by XFA callbacks.
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_BSTR_ {
    /// String buffer, manipulate only with FPDF_BStr_* methods.
    pub str_: *mut ::std::os::raw::c_char,
    /// Length of the string, in bytes.
    pub len: ::std::os::raw::c_int,
}

/// Matrix for transformation
///
/// C documentation:
///
/// ```text
/// Matrix for transformation, in the form \[a b c d e f\], equivalent to:
/// | a  b  0 |
/// | c  d  0 |
/// | e  f  1 |
///
/// Translation is performed with \[1 0 0 1 tx ty\].
/// Scaling is performed with \[sx 0 0 sy 0 0\].
/// See PDF Reference 1.7, 4.2.2 Common Transformations for more.
/// ```
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

/// Rectangle area(float) in device or page coordinate system.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FS_RECTF_ {
    /// The x-coordinate of the left-top corner.
    pub left: f32,
    /// The y-coordinate of the left-top corner.
    pub top: f32,
    /// The x-coordinate of the right-bottom corner.
    pub right: f32,
    /// The y-coordinate of the right-bottom corner.
    pub bottom: f32,
}

/// Rectangle size. Coordinate system agnostic.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FS_SIZEF_ {
    pub width: f32,
    pub height: f32,
}

/// 2D Point. Coordinate system agnostic.
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

/// Process-wide options for initializing the library.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_LIBRARY_CONFIG_ {
    /// Version number of the interface
    ///
    /// C documentation:
    ///
    /// ```text
    /// Version number of the interface. Currently must be 2.
    /// Support for version 1 will be deprecated in the future.
    /// ```
    pub version: ::std::os::raw::c_int,
    /// Array of paths to scan in place of the defaults when using built-in
    ///
    /// C documentation:
    ///
    /// ```text
    /// Array of paths to scan in place of the defaults when using built-in
    /// FXGE font loading code. The array is terminated by a NULL pointer.
    /// The Array may be NULL itself to use the default paths. May be ignored
    /// entirely depending upon the platform.
    /// ```
    pub m_pUserFontPaths: *mut *const ::std::os::raw::c_char,
    /// Pointer to the v8::Isolate to use, or NULL to force PDFium to create one.
    pub m_pIsolate: *mut ::std::os::raw::c_void,
    /// The embedder data slot to use in the v8::Isolate to store PDFium's
    ///
    /// C documentation:
    ///
    /// ```text
    /// The embedder data slot to use in the v8::Isolate to store PDFium's
    /// per-isolate data. The value needs to be in the range
    /// \[0, |v8::Internals::kNumIsolateDataLots|). Note that 0 is fine for most
    /// embedders.
    /// ```
    pub m_v8EmbedderSlot: ::std::os::raw::c_uint,
    /// Pointer to the V8::Platform to use.
    pub m_pPlatform: *mut ::std::os::raw::c_void,
    /// Explicit specification of core renderer to use
    ///
    /// C documentation:
    ///
    /// ```text
    /// Explicit specification of core renderer to use. |m_RendererType| must be
    /// a valid value for |FPDF_LIBRARY_CONFIG| versions of this level or higher,
    /// or else the initialization will fail with an immediate crash.
    /// Note that use of a specified |FPDF_RENDERER_TYPE| value for which the
    /// corresponding render library is not included in the build will similarly
    /// fail with an immediate crash.
    /// ```
    pub m_RendererType: FPDF_RENDERER_TYPE,
}

/// Structure for custom file access.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_FILEACCESS {
    /// File length, in bytes.
    pub m_FileLen: ::std::os::raw::c_ulong,
    /// A function pointer for getting a block of data from a specific position.
    ///
    /// C documentation:
    ///
    /// ```text
    /// A function pointer for getting a block of data from a specific position.
    /// Position is specified by byte offset from the beginning of the file.
    /// The pointer to the buffer is never NULL and the size is never 0.
    /// The position and size will never go out of range of the file length.
    /// It may be possible for PDFium to call this function multiple times for
    /// the same position.
    /// Return value: should be non-zero if successful, zero for error.
    /// ```
    pub m_GetBlock: ::std::option::Option<
        unsafe extern "C" fn(
            param: *mut ::std::os::raw::c_void,
            position: ::std::os::raw::c_ulong,
            pBuf: *mut ::std::os::raw::c_uchar,
            size: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_int,
    >,
    /// A custom pointer for all implementation specific data
    ///
    /// C documentation:
    ///
    /// ```text
    /// A custom pointer for all implementation specific data.  This pointer will
    /// be used as the first parameter to the m_GetBlock callback.
    /// ```
    pub m_Param: *mut ::std::os::raw::c_void,
}

/// Structure for file reading or writing (I/O).
///
/// C documentation:
///
/// ```text
/// Structure for file reading or writing (I/O).
///
/// Note: This is a handler and should be implemented by callers,
/// and is only used from XFA.
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_FILEHANDLER_ {
    /// User-defined data.
    ///
    /// C documentation:
    ///
    /// ```text
    /// User-defined data.
    /// Note: Callers can use this field to track controls.
    /// ```
    pub clientData: *mut ::std::os::raw::c_void,
    /// Callback function to release the current file stream object.
    ///
    /// C documentation:
    ///
    /// ```text
    /// Callback function to release the current file stream object.
    ///
    /// Parameters:
    ///       clientData   -  Pointer to user-defined data.
    /// Returns:
    ///       None.
    /// ```
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(clientData: *mut ::std::os::raw::c_void)>,
    /// Callback function to retrieve the current file stream size.
    ///
    /// C documentation:
    ///
    /// ```text
    /// Callback function to retrieve the current file stream size.
    ///
    /// Parameters:
    ///       clientData   -  Pointer to user-defined data.
    /// Returns:
    ///       Size of file stream.
    /// ```
    pub GetSize: ::std::option::Option<
        unsafe extern "C" fn(clientData: *mut ::std::os::raw::c_void) -> FPDF_DWORD,
    >,
    /// Callback function to read data from the current file stream.
    ///
    /// C documentation:
    ///
    /// ```text
    /// Callback function to read data from the current file stream.
    ///
    /// Parameters:
    ///       clientData   -  Pointer to user-defined data.
    ///       offset       -  Offset position starts from the beginning of file
    ///                       stream. This parameter indicates reading position.
    ///       buffer       -  Memory buffer to store data which are read from
    ///                       file stream. This parameter should not be NULL.
    ///       size         -  Size of data which should be read from file stream,
    ///                       in bytes. The buffer indicated by |buffer| must be
    ///                       large enough to store specified data.
    /// Returns:
    ///       0 for success, other value for failure.
    /// ```
    pub ReadBlock: ::std::option::Option<
        unsafe extern "C" fn(
            clientData: *mut ::std::os::raw::c_void,
            offset: FPDF_DWORD,
            buffer: *mut ::std::os::raw::c_void,
            size: FPDF_DWORD,
        ) -> FPDF_RESULT,
    >,
    /// Callback function to write data into the current file stream.
    ///
    /// C documentation:
    ///
    /// ```text
    /// Callback function to write data into the current file stream.
    ///
    /// Parameters:
    ///       clientData   -  Pointer to user-defined data.
    ///       offset       -  Offset position starts from the beginning of file
    ///                       stream. This parameter indicates writing position.
    ///       buffer       -  Memory buffer contains data which is written into
    ///                       file stream. This parameter should not be NULL.
    ///       size         -  Size of data which should be written into file
    ///                       stream, in bytes.
    /// Returns:
    ///       0 for success, other value for failure.
    /// ```
    pub WriteBlock: ::std::option::Option<
        unsafe extern "C" fn(
            clientData: *mut ::std::os::raw::c_void,
            offset: FPDF_DWORD,
            buffer: *const ::std::os::raw::c_void,
            size: FPDF_DWORD,
        ) -> FPDF_RESULT,
    >,
    /// Callback function to flush all internal accessing buffers.
    ///
    /// C documentation:
    ///
    /// ```text
    /// Callback function to flush all internal accessing buffers.
    ///
    /// Parameters:
    ///       clientData   -  Pointer to user-defined data.
    /// Returns:
    ///       0 for success, other value for failure.
    /// ```
    pub Flush: ::std::option::Option<
        unsafe extern "C" fn(clientData: *mut ::std::os::raw::c_void) -> FPDF_RESULT,
    >,
    /// Callback function to change file size.
    ///
    /// C documentation:
    ///
    /// ```text
    /// Callback function to change file size.
    ///
    /// Description:
    ///       This function is called under writing mode usually. Implementer
    ///       can determine whether to realize it based on application requests.
    /// Parameters:
    ///       clientData   -  Pointer to user-defined data.
    ///       size         -  New size of file stream, in bytes.
    /// Returns:
    ///       0 for success, other value for failure.
    /// ```
    pub Truncate: ::std::option::Option<
        unsafe extern "C" fn(
            clientData: *mut ::std::os::raw::c_void,
            size: FPDF_DWORD,
        ) -> FPDF_RESULT,
    >,
}

/// Struct for color scheme.
///
/// C documentation:
///
/// ```text
/// Struct for color scheme.
/// Each should be a 32-bit value specifying the color, in 8888 ARGB format.
/// ```
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
    /// Version number of the interface. Currently must be 2.
    pub version: ::std::os::raw::c_int,
    /// Method: app_alert
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: app_alert
    ///       Pop up a dialog to show warning or hint.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    ///       Msg         -   A string containing the message to be displayed.
    ///       Title       -   The title of the dialog.
    ///       Type        -   The type of button group, one of the
    ///                       JSPLATFORM_ALERT_BUTTON_* values above.
    ///       nIcon       -   The type of the icon, one of the
    ///                       JSPLATFORM_ALERT_ICON_* above.
    /// Return Value:
    ///       Option selected by user in dialogue, one of the
    ///       JSPLATFORM_ALERT_RETURN_* values above.
    /// ```
    pub app_alert: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            Msg: FPDF_WIDESTRING,
            Title: FPDF_WIDESTRING,
            Type: ::std::os::raw::c_int,
            Icon: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    /// Method: app_beep
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: app_beep
    ///       Causes the system to play a sound.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself
    ///       nType       -   The sound type, see JSPLATFORM_BEEP_TYPE_*
    ///                       above.
    /// Return Value:
    ///       None
    /// ```
    pub app_beep: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _IPDF_JsPlatform, nType: ::std::os::raw::c_int),
    >,
    /// Method: app_response
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: app_response
    ///       Displays a dialog box containing a question and an entry field for
    ///       the user to reply to the question.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself
    ///       Question    -   The question to be posed to the user.
    ///       Title       -   The title of the dialog box.
    ///       Default     -   A default value for the answer to the question. If
    ///                       not specified, no default value is presented.
    ///       cLabel      -   A short string to appear in front of and on the
    ///                       same line as the edit text field.
    ///       bPassword   -   If true, indicates that the user's response should
    ///                       be shown as asterisks (*) or bullets (?) to mask
    ///                       the response, which might be sensitive information.
    ///       response    -   A string buffer allocated by PDFium, to receive the
    ///                       user's response.
    ///       length      -   The length of the buffer in bytes. Currently, it is
    ///                       always 2048.
    /// Return Value:
    ///       Number of bytes the complete user input would actually require, not
    ///       including trailing zeros, regardless of the value of the length
    ///       parameter or the presence of the response buffer.
    /// Comments:
    ///       No matter on what platform, the response buffer should be always
    ///       written using UTF-16LE encoding. If a response buffer is
    ///       present and the size of the user input exceeds the capacity of the
    ///       buffer as specified by the length parameter, only the
    ///       first "length" bytes of the user input are to be written to the
    ///       buffer.
    /// ```
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
    /// Method: Doc_getFilePath
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: Doc_getFilePath
    ///       Get the file path of the current document.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself
    ///       filePath    -   The string buffer to receive the file path. Can
    ///                       be NULL.
    ///       length      -   The length of the buffer, number of bytes. Can
    ///                       be 0.
    /// Return Value:
    ///       Number of bytes the filePath consumes, including trailing zeros.
    /// Comments:
    ///       The filePath should always be provided in the local encoding.
    ///       The return value always indicated number of bytes required for
    ///       the buffer, even when there is no buffer specified, or the buffer
    ///       size is less than required. In this case, the buffer will not
    ///       be modified.
    /// ```
    pub Doc_getFilePath: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            filePath: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    /// Method: Doc_mail
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: Doc_mail
    ///       Mails the data buffer as an attachment to all recipients, with or
    ///       without user interaction.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself
    ///       mailData    -   Pointer to the data buffer to be sent. Can be NULL.
    ///       length      -   The size,in bytes, of the buffer pointed by
    ///                       mailData parameter. Can be 0.
    ///       bUI         -   If true, the rest of the parameters are used in a
    ///                       compose-new-message window that is displayed to the
    ///                       user. If false, the cTo parameter is required and
    ///                       all others are optional.
    ///       To          -   A semicolon-delimited list of recipients for the
    ///                       message.
    ///       Subject     -   The subject of the message. The length limit is
    ///                       64 KB.
    ///       CC          -   A semicolon-delimited list of CC recipients for
    ///                       the message.
    ///       BCC         -   A semicolon-delimited list of BCC recipients for
    ///                       the message.
    ///       Msg         -   The content of the message. The length limit is
    ///                       64 KB.
    /// Return Value:
    ///       None.
    /// Comments:
    ///       If the parameter mailData is NULL or length is 0, the current
    ///       document will be mailed as an attachment to all recipients.
    /// ```
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
    /// Method: Doc_print
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: Doc_print
    ///       Prints all or a specific number of pages of the document.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis         -   Pointer to the interface structure itself.
    ///       bUI           -   If true, will cause a UI to be presented to the
    ///                         user to obtain printing information and confirm
    ///                         the action.
    ///       nStart        -   A 0-based index that defines the start of an
    ///                         inclusive range of pages.
    ///       nEnd          -   A 0-based index that defines the end of an
    ///                         inclusive page range.
    ///       bSilent       -   If true, suppresses the cancel dialog box while
    ///                         the document is printing. The default is false.
    ///       bShrinkToFit  -   If true, the page is shrunk (if necessary) to
    ///                         fit within the imageable area of the printed page.
    ///       bPrintAsImage -   If true, print pages as an image.
    ///       bReverse      -   If true, print from nEnd to nStart.
    ///       bAnnotations  -   If true (the default), annotations are
    ///                         printed.
    /// Return Value:
    ///       None.
    /// ```
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
    /// Method: Doc_submitForm
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: Doc_submitForm
    ///       Send the form data to a specified URL.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself
    ///       formData    -   Pointer to the data buffer to be sent.
    ///       length      -   The size,in bytes, of the buffer pointed by
    ///                       formData parameter.
    ///       URL         -   The URL to send to.
    /// Return Value:
    ///       None.
    /// ```
    pub Doc_submitForm: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            formData: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
            URL: FPDF_WIDESTRING,
        ),
    >,
    /// Method: Doc_gotoPage
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: Doc_gotoPage
    ///       Jump to a specified page.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself
    ///       nPageNum    -   The specified page number, zero for the first page.
    /// Return Value:
    ///       None.
    /// ```
    pub Doc_gotoPage: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _IPDF_JsPlatform, nPageNum: ::std::os::raw::c_int),
    >,
    /// Method: Field_browse
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: Field_browse
    ///       Show a file selection dialog, and return the selected file path.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    ///       filePath    -   Pointer to the data buffer to receive the file
    ///                       path. Can be NULL.
    ///       length      -   The length of the buffer, in bytes. Can be 0.
    /// Return Value:
    ///       Number of bytes the filePath consumes, including trailing zeros.
    /// Comments:
    ///       The filePath should always be provided in local encoding.
    /// ```
    pub Field_browse: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _IPDF_JsPlatform,
            filePath: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    /// Pointer for embedder-specific data
    ///
    /// C documentation:
    ///
    /// ```text
    /// Pointer for embedder-specific data. Unused by PDFium, and despite
    /// its name, can be any data the embedder desires, though traditionally
    /// a FPDF_FORMFILLINFO interface.
    /// ```
    pub m_pFormfillinfo: *mut ::std::os::raw::c_void,
    /// Unused in v3, retain for compatibility.
    pub m_isolate: *mut ::std::os::raw::c_void,
    /// Unused in v3, retain for compatibility.
    pub m_v8EmbedderSlot: ::std::os::raw::c_uint,
}

/// Declares of a struct type to the local system time.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FPDF_SYSTEMTIME {
    /// years since 1900
    pub wYear: ::std::os::raw::c_ushort,
    /// months since January - \[0,11\]
    pub wMonth: ::std::os::raw::c_ushort,
    /// days since Sunday - \[0,6\]
    pub wDayOfWeek: ::std::os::raw::c_ushort,
    /// day of the month - \[1,31\]
    pub wDay: ::std::os::raw::c_ushort,
    /// hours since midnight - \[0,23\]
    pub wHour: ::std::os::raw::c_ushort,
    /// minutes after the hour - \[0,59\]
    pub wMinute: ::std::os::raw::c_ushort,
    /// seconds after the minute - \[0,59\]
    pub wSecond: ::std::os::raw::c_ushort,
    /// milliseconds after the second - \[0,999\]
    pub wMilliseconds: ::std::os::raw::c_ushort,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FPDF_FORMFILLINFO {
    /// Version number of the interface.
    ///
    /// C documentation:
    ///
    /// ```text
    /// Version number of the interface.
    /// Version 1 contains stable interfaces. Version 2 has additional
    /// experimental interfaces.
    /// When PDFium is built without the XFA module, version can be 1 or 2.
    /// With version 1, only stable interfaces are called. With version 2,
    /// additional experimental interfaces are also called.
    /// When PDFium is built with the XFA module, version must be 2.
    /// All the XFA related interfaces are experimental. If PDFium is built with
    /// the XFA module and version 1 then none of the XFA related interfaces
    /// would be called. When PDFium is built with XFA module then the version
    /// must be 2.
    /// ```
    pub version: ::std::os::raw::c_int,
    /// Method: Release
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: Release
    ///       Give the implementation a chance to release any resources after the
    ///       interface is no longer used.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       No
    /// Comments:
    ///       Called by PDFium during the final cleanup process.
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself
    /// Return Value:
    ///       None
    /// ```
    pub Release: ::std::option::Option<unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO)>,
    /// Method: FFI_Invalidate
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_Invalidate
    ///       Invalidate the client area within the specified rectangle.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    ///       page        -   Handle to the page. Returned by FPDF_LoadPage().
    ///       left        -   Left position of the client area in PDF page
    ///                       coordinates.
    ///       top         -   Top position of the client area in PDF page
    ///                       coordinates.
    ///       right       -   Right position of the client area in PDF page
    ///                       coordinates.
    ///       bottom      -   Bottom position of the client area in PDF page
    ///                       coordinates.
    /// Return Value:
    ///       None.
    /// Comments:
    ///       All positions are measured in PDF "user space".
    ///       Implementation should call FPDF_RenderPageBitmap() for repainting
    ///       the specified page area.
    /// ```
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
    /// Method: FFI_OutputSelectedRect
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_OutputSelectedRect
    ///       When the user selects text in form fields with the mouse, this
    ///       callback function will be invoked with the selected areas.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       No
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    ///       page        -   Handle to the page. Returned by FPDF_LoadPage()/
    ///       left        -   Left position of the client area in PDF page
    ///                       coordinates.
    ///       top         -   Top position of the client area in PDF page
    ///                       coordinates.
    ///       right       -   Right position of the client area in PDF page
    ///                       coordinates.
    ///       bottom      -   Bottom position of the client area in PDF page
    ///                       coordinates.
    /// Return Value:
    ///       None.
    /// Comments:
    ///       This callback function is useful for implementing special text
    ///       selection effects. An implementation should first record the
    ///       returned rectangles, then draw them one by one during the next
    ///       painting period. Lastly, it should remove all the recorded
    ///       rectangles when finished painting.
    /// ```
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
    /// Method: FFI_SetCursor
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_SetCursor
    ///       Set the Cursor shape.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    ///       nCursorType -   Cursor type, see Flags for Cursor type for details.
    /// Return value:
    ///       None.
    /// ```
    pub FFI_SetCursor: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, nCursorType: ::std::os::raw::c_int),
    >,
    /// Method: FFI_SetTimer
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_SetTimer
    ///       This method installs a system timer. An interval value is specified,
    ///       and every time that interval elapses, the system must call into the
    ///       callback function with the timer ID as returned by this function.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    ///       uElapse     -   Specifies the time-out value, in milliseconds.
    ///       lpTimerFunc -   A pointer to the callback function-TimerCallback.
    /// Return value:
    ///       The timer identifier of the new timer if the function is successful.
    ///       An application passes this value to the FFI_KillTimer method to kill
    ///       the timer. Nonzero if it is successful; otherwise, it is zero.
    /// ```
    pub FFI_SetTimer: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            uElapse: ::std::os::raw::c_int,
            lpTimerFunc: TimerCallback,
        ) -> ::std::os::raw::c_int,
    >,
    /// Method: FFI_KillTimer
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_KillTimer
    ///       This method uninstalls a system timer, as set by an earlier call to
    ///       FFI_SetTimer.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    ///       nTimerID    -   The timer ID returned by FFI_SetTimer function.
    /// Return value:
    ///       None.
    /// ```
    pub FFI_KillTimer: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, nTimerID: ::std::os::raw::c_int),
    >,
    /// Method: FFI_GetLocalTime
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_GetLocalTime
    ///       This method receives the current local time on the system.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    /// Return value:
    ///       The local time. See FPDF_SYSTEMTIME above for details.
    /// Note: Unused.
    /// ```
    pub FFI_GetLocalTime: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO) -> FPDF_SYSTEMTIME,
    >,
    /// Method: FFI_OnChange
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_OnChange
    ///       This method will be invoked to notify the implementation when the
    ///       value of any FormField on the document had been changed.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       no
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    /// Return value:
    ///       None.
    /// ```
    pub FFI_OnChange: ::std::option::Option<unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO)>,
    /// Method: FFI_GetPage
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_GetPage
    ///       This method receives the page handle associated with a specified
    ///       page index.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    ///       document    -   Handle to document. Returned by FPDF_LoadDocument().
    ///       nPageIndex  -   Index number of the page. 0 for the first page.
    /// Return value:
    ///       Handle to the page, as previously returned to the implementation by
    ///       FPDF_LoadPage().
    /// Comments:
    ///       The implementation is expected to keep track of the page handles it
    ///       receives from PDFium, and their mappings to page numbers. In some
    ///       cases, the document-level JavaScript action may refer to a page
    ///       which hadn't been loaded yet. To successfully run the Javascript
    ///       action, the implementation needs to load the page.
    /// ```
    pub FFI_GetPage: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
            nPageIndex: ::std::os::raw::c_int,
        ) -> FPDF_PAGE,
    >,
    /// Method: FFI_GetCurrentPage
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_GetCurrentPage
    ///       This method receives the handle to the current page.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       Yes when V8 support is present, otherwise unused.
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    ///       document    -   Handle to document. Returned by FPDF_LoadDocument().
    /// Return value:
    ///       Handle to the page. Returned by FPDF_LoadPage().
    /// Comments:
    ///       PDFium doesn't keep keep track of the "current page" (e.g. the one
    ///       that is most visible on screen), so it must ask the embedder for
    ///       this information.
    /// ```
    pub FFI_GetCurrentPage: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, document: FPDF_DOCUMENT) -> FPDF_PAGE,
    >,
    /// Method: FFI_GetRotation
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_GetRotation
    ///       This method receives currently rotation of the page view.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis       -   Pointer to the interface structure itself.
    ///       page        -   Handle to page, as returned by FPDF_LoadPage().
    /// Return value:
    ///       A number to indicate the page rotation in 90 degree increments
    ///       in a clockwise direction:
    ///         0 - 0 degrees
    ///         1 - 90 degrees
    ///         2 - 180 degrees
    ///         3 - 270 degrees
    /// Note: Unused.
    /// ```
    pub FFI_GetRotation: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page: FPDF_PAGE,
        ) -> ::std::os::raw::c_int,
    >,
    /// Method: FFI_ExecuteNamedAction
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_ExecuteNamedAction
    ///       This method will execute a named action.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       yes
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       namedAction     -   A byte string which indicates the named action,
    ///                           terminated by 0.
    /// Return value:
    ///       None.
    /// Comments:
    ///       See ISO 32000-1:2008, section 12.6.4.11 for descriptions of the
    ///       standard named actions, but note that a document may supply any
    ///       name of its choosing.
    /// ```
    pub FFI_ExecuteNamedAction: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, namedAction: FPDF_BYTESTRING),
    >,
    /// Method: FFI_SetTextFieldFocus
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_SetTextFieldFocus
    ///       Called when a text field is getting or losing focus.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       no
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       value           -   The string value of the form field, in UTF-16LE
    ///                           format.
    ///       valueLen        -   The length of the string value. This is the
    ///                           number of characters, not bytes.
    ///       is_focus        -   True if the form field is getting focus, false
    ///                           if the form field is losing focus.
    /// Return value:
    ///       None.
    /// Comments:
    ///       Only supports text fields and combobox fields.
    /// ```
    pub FFI_SetTextFieldFocus: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            value: FPDF_WIDESTRING,
            valueLen: FPDF_DWORD,
            is_focus: FPDF_BOOL,
        ),
    >,
    /// Method: FFI_DoURIAction
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_DoURIAction
    ///       Ask the implementation to navigate to a uniform resource identifier.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       No
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       bsURI           -   A byte string which indicates the uniform
    ///                           resource identifier, terminated by 0.
    /// Return value:
    ///       None.
    /// Comments:
    ///       If the embedder is version 2 or higher and have implementation for
    ///       FFI_DoURIActionWithKeyboardModifier, then
    ///       FFI_DoURIActionWithKeyboardModifier takes precedence over
    ///       FFI_DoURIAction.
    ///       See the URI actions description of <<PDF Reference, version 1.7>>
    ///       for more details.
    /// ```
    pub FFI_DoURIAction: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_FORMFILLINFO, bsURI: FPDF_BYTESTRING),
    >,
    /// Method: FFI_DoGoToAction
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_DoGoToAction
    ///       This action changes the view to a specified destination.
    /// Interface Version:
    ///       1
    /// Implementation Required:
    ///       No
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       nPageIndex      -   The index of the PDF page.
    ///       zoomMode        -   The zoom mode for viewing page. See below.
    ///       fPosArray       -   The float array which carries the position info.
    ///       sizeofArray     -   The size of float array.
    /// PDFZoom values:
    ///         - XYZ = 1
    ///         - FITPAGE = 2
    ///         - FITHORZ = 3
    ///         - FITVERT = 4
    ///         - FITRECT = 5
    ///         - FITBBOX = 6
    ///         - FITBHORZ = 7
    ///         - FITBVERT = 8
    /// Return value:
    ///       None.
    /// Comments:
    ///       See the Destinations description of <<PDF Reference, version 1.7>>
    ///       in 8.2.1 for more details.
    /// ```
    pub FFI_DoGoToAction: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            nPageIndex: ::std::os::raw::c_int,
            zoomMode: ::std::os::raw::c_int,
            fPosArray: *mut f32,
            sizeofArray: ::std::os::raw::c_int,
        ),
    >,
    /// Pointer to IPDF_JSPLATFORM interface.
    ///
    /// C documentation:
    ///
    /// ```text
    /// Pointer to IPDF_JSPLATFORM interface.
    /// Unused if PDFium is built without V8 support. Otherwise, if NULL, then
    /// JavaScript will be prevented from executing while rendering the document.
    /// ```
    pub m_pJsPlatform: *mut IPDF_JSPLATFORM,
    /// Whether the XFA module is disabled when built with the XFA module.
    ///
    /// C documentation:
    ///
    /// ```text
    /// Whether the XFA module is disabled when built with the XFA module.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// ```
    pub xfa_disabled: FPDF_BOOL,
    /// Method: FFI_DisplayCaret
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_DisplayCaret
    ///       This method will show the caret at specified position.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       page            -   Handle to page. Returned by FPDF_LoadPage().
    ///       left            -   Left position of the client area in PDF page
    ///                           coordinates.
    ///       top             -   Top position of the client area in PDF page
    ///                           coordinates.
    ///       right           -   Right position of the client area in PDF page
    ///                           coordinates.
    ///       bottom          -   Bottom position of the client area in PDF page
    ///                           coordinates.
    /// Return value:
    ///       None.
    /// ```
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
    /// Method: FFI_GetCurrentPageIndex
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_GetCurrentPageIndex
    ///       This method will get the current page index.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       document        -   Handle to document from FPDF_LoadDocument().
    /// Return value:
    ///       The index of current page.
    /// ```
    pub FFI_GetCurrentPageIndex: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
        ) -> ::std::os::raw::c_int,
    >,
    /// Method: FFI_SetCurrentPage
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_SetCurrentPage
    ///       This method will set the current page.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       document        -   Handle to document from FPDF_LoadDocument().
    ///       iCurPage        -   The index of the PDF page.
    /// Return value:
    ///       None.
    /// ```
    pub FFI_SetCurrentPage: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
            iCurPage: ::std::os::raw::c_int,
        ),
    >,
    /// Method: FFI_GotoURL
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_GotoURL
    ///       This method will navigate to the specified URL.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis            -   Pointer to the interface structure itself.
    ///       document         -   Handle to document from FPDF_LoadDocument().
    ///       wsURL            -   The string value of the URL, in UTF-16LE format.
    /// Return value:
    ///       None.
    /// ```
    pub FFI_GotoURL: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            document: FPDF_DOCUMENT,
            wsURL: FPDF_WIDESTRING,
        ),
    >,
    /// Method: FFI_GetPageViewRect
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_GetPageViewRect
    ///       This method will get the current page view rectangle.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       page            -   Handle to page. Returned by FPDF_LoadPage().
    ///       left            -   The pointer to receive left position of the page
    ///                           view area in PDF page coordinates.
    ///       top             -   The pointer to receive top position of the page
    ///                           view area in PDF page coordinates.
    ///       right           -   The pointer to receive right position of the
    ///                           page view area in PDF page coordinates.
    ///       bottom          -   The pointer to receive bottom position of the
    ///                           page view area in PDF page coordinates.
    /// Return value:
    ///     None.
    /// ```
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
    /// Method: FFI_PageEvent
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_PageEvent
    ///       This method fires when pages have been added to or deleted from
    ///       the XFA document.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       page_count      -   The number of pages to be added or deleted.
    ///       event_type      -   See FXFA_PAGEVIEWEVENT_* above.
    /// Return value:
    ///       None.
    /// Comments:
    ///       The pages to be added or deleted always start from the last page
    ///       of document. This means that if parameter page_count is 2 and
    ///       event type is FXFA_PAGEVIEWEVENT_POSTADDED, 2 new pages have been
    ///       appended to the tail of document; If page_count is 2 and
    ///       event type is FXFA_PAGEVIEWEVENT_POSTREMOVED, the last 2 pages
    ///       have been deleted.
    /// ```
    pub FFI_PageEvent: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            page_count: ::std::os::raw::c_int,
            event_type: FPDF_DWORD,
        ),
    >,
    /// Method: FFI_PopupMenu
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_PopupMenu
    ///       This method will track the right context menu for XFA fields.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       page            -   Handle to page. Returned by FPDF_LoadPage().
    ///       hWidget         -   Always null, exists for compatibility.
    ///       menuFlag        -   The menu flags. Please refer to macro definition
    ///                           of FXFA_MENU_XXX and this can be one or a
    ///                           combination of these macros.
    ///       x               -   X position of the client area in PDF page
    ///                           coordinates.
    ///       y               -   Y position of the client area in PDF page
    ///                           coordinates.
    /// Return value:
    ///       TRUE indicates success; otherwise false.
    /// ```
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
    /// Method: FFI_OpenFile
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_OpenFile
    ///       This method will open the specified file with the specified mode.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       fileFlag        -   The file flag. Please refer to macro definition
    ///                           of FXFA_SAVEAS_XXX and use one of these macros.
    ///       wsURL           -   The string value of the file URL, in UTF-16LE
    ///                           format.
    ///       mode            -   The mode for open file, e.g. "rb" or "wb".
    /// Return value:
    ///       The handle to FPDF_FILEHANDLER.
    /// ```
    pub FFI_OpenFile: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            fileFlag: ::std::os::raw::c_int,
            wsURL: FPDF_WIDESTRING,
            mode: *const ::std::os::raw::c_char,
        ) -> *mut FPDF_FILEHANDLER,
    >,
    /// Method: FFI_EmailTo
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_EmailTo
    ///       This method will email the specified file stream to the specified
    ///       contact.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       pFileHandler    -   Handle to the FPDF_FILEHANDLER.
    ///       pTo             -   A semicolon-delimited list of recipients for the
    ///                           message,in UTF-16LE format.
    ///       pSubject        -   The subject of the message,in UTF-16LE format.
    ///       pCC             -   A semicolon-delimited list of CC recipients for
    ///                           the message,in UTF-16LE format.
    ///       pBcc            -   A semicolon-delimited list of BCC recipients for
    ///                           the message,in UTF-16LE format.
    ///       pMsg            -   Pointer to the data buffer to be sent.Can be
    ///                           NULL,in UTF-16LE format.
    /// Return value:
    ///       None.
    /// ```
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
    /// Method: FFI_UploadTo
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_UploadTo
    ///       This method will upload the specified file stream to the
    ///       specified URL.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       pFileHandler    -   Handle to the FPDF_FILEHANDLER.
    ///       fileFlag        -   The file flag. Please refer to macro definition
    ///                           of FXFA_SAVEAS_XXX and use one of these macros.
    ///       uploadTo        -   Pointer to the URL path, in UTF-16LE format.
    /// Return value:
    ///       None.
    /// ```
    pub FFI_UploadTo: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            fileHandler: *mut FPDF_FILEHANDLER,
            fileFlag: ::std::os::raw::c_int,
            uploadTo: FPDF_WIDESTRING,
        ),
    >,
    /// Method: FFI_GetPlatform
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_GetPlatform
    ///       This method will get the current platform.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       platform        -   Pointer to the data buffer to receive the
    ///                           platform,in UTF-16LE format. Can be NULL.
    ///       length          -   The length of the buffer in bytes. Can be
    ///                           0 to query the required size.
    /// Return value:
    ///       The length of the buffer, number of bytes.
    /// ```
    pub FFI_GetPlatform: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            platform: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    /// Method: FFI_GetLanguage
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_GetLanguage
    ///       This method will get the current language.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       language        -   Pointer to the data buffer to receive the
    ///                           current language. Can be NULL.
    ///       length          -   The length of the buffer in bytes. Can be
    ///                           0 to query the required size.
    /// Return value:
    ///       The length of the buffer, number of bytes.
    /// ```
    pub FFI_GetLanguage: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            language: *mut ::std::os::raw::c_void,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    /// Method: FFI_DownloadFromURL
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_DownloadFromURL
    ///       This method will download the specified file from the URL.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       URL             -   The string value of the file URL, in UTF-16LE
    ///                           format.
    /// Return value:
    ///       The handle to FPDF_FILEHANDLER.
    /// ```
    pub FFI_DownloadFromURL: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            URL: FPDF_WIDESTRING,
        ) -> *mut FPDF_FILEHANDLER,
    >,
    /// Method: FFI_PostRequestURL
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_PostRequestURL
    ///       This method will post the request to the server URL.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       wsURL           -   The string value of the server URL, in UTF-16LE
    ///                           format.
    ///       wsData          -   The post data,in UTF-16LE format.
    ///       wsContentType   -   The content type of the request data, in
    ///                           UTF-16LE format.
    ///       wsEncode        -   The encode type, in UTF-16LE format.
    ///       wsHeader        -   The request header,in UTF-16LE format.
    ///       response        -   Pointer to the FPDF_BSTR to receive the response
    ///                           data from the server, in UTF-16LE format.
    /// Return value:
    ///       TRUE indicates success, otherwise FALSE.
    /// ```
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
    /// Method: FFI_PutRequestURL
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_PutRequestURL
    ///       This method will put the request to the server URL.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       Required for XFA, otherwise set to NULL.
    /// Parameters:
    ///       pThis           -   Pointer to the interface structure itself.
    ///       wsURL           -   The string value of the server URL, in UTF-16LE
    ///                           format.
    ///       wsData          -   The put data, in UTF-16LE format.
    ///       wsEncode        -   The encode type, in UTR-16LE format.
    /// Return value:
    ///       TRUE indicates success, otherwise FALSE.
    /// ```
    pub FFI_PutRequestURL: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_FORMFILLINFO,
            wsURL: FPDF_WIDESTRING,
            wsData: FPDF_WIDESTRING,
            wsEncode: FPDF_WIDESTRING,
        ) -> FPDF_BOOL,
    >,
    /// Method: FFI_OnFocusChange
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_OnFocusChange
    ///     Called when the focused annotation is updated.
    /// Interface Version:
    ///     Ignored if |version| < 2.
    /// Implementation Required:
    ///     No
    /// Parameters:
    ///     param           -   Pointer to the interface structure itself.
    ///     annot           -   The focused annotation.
    ///     page_index      -   Index number of the page which contains the
    ///                         focused annotation. 0 for the first page.
    /// Return value:
    ///     None.
    /// Comments:
    ///     This callback function is useful for implementing any view based
    ///     action such as scrolling the annotation rect into view. The
    ///     embedder should not copy and store the annot as its scope is
    ///     limited to this call only.
    /// ```
    pub FFI_OnFocusChange: ::std::option::Option<
        unsafe extern "C" fn(
            param: *mut _FPDF_FORMFILLINFO,
            annot: FPDF_ANNOTATION,
            page_index: ::std::os::raw::c_int,
        ),
    >,
    /// Method: FFI_DoURIActionWithKeyboardModifier
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: FFI_DoURIActionWithKeyboardModifier
    ///       Ask the implementation to navigate to a uniform resource identifier
    ///       with the specified modifiers.
    /// Interface Version:
    ///       Ignored if |version| < 2.
    /// Implementation Required:
    ///       No
    /// Parameters:
    ///       param           -   Pointer to the interface structure itself.
    ///       uri             -   A byte string which indicates the uniform
    ///                           resource identifier, terminated by 0.
    ///       modifiers       -   Keyboard modifier that indicates which of
    ///                           the virtual keys are down, if any.
    /// Return value:
    ///       None.
    /// Comments:
    ///       If the embedder who is version 2 and does not implement this API,
    ///       then a call will be redirected to FFI_DoURIAction.
    ///       See the URI actions description of <<PDF Reference, version 1.7>>
    ///       for more details.
    /// ```
    pub FFI_DoURIActionWithKeyboardModifier: ::std::option::Option<
        unsafe extern "C" fn(
            param: *mut _FPDF_FORMFILLINFO,
            uri: FPDF_BYTESTRING,
            modifiers: ::std::os::raw::c_int,
        ),
    >,
}

/// Interface for checking whether sections of the file are available.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FX_FILEAVAIL {
    /// Version number of the interface. Must be 1.
    pub version: ::std::os::raw::c_int,
    /// Reports if the specified data section is currently available
    ///
    /// C documentation:
    ///
    /// ```text
    /// Reports if the specified data section is currently available. A section is
    /// available if all bytes in the section are available.
    ///
    /// Interface Version: 1
    /// Implementation Required: Yes
    ///
    ///   pThis  - pointer to the interface structure.
    ///   offset - the offset of the data section in the file.
    ///   size   - the size of the data section.
    ///
    /// Returns true if the specified data section at |offset| of |size|
    /// is available.
    /// ```
    pub IsDataAvail: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FX_FILEAVAIL, offset: usize, size: usize) -> FPDF_BOOL,
    >,
}

/// Download hints interface. Used to receive hints for further downloading.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FX_DOWNLOADHINTS {
    /// Version number of the interface. Must be 1.
    pub version: ::std::os::raw::c_int,
    /// Add a section to be downloaded.
    ///
    /// C documentation:
    ///
    /// ```text
    /// Add a section to be downloaded.
    ///
    /// Interface Version: 1
    /// Implementation Required: Yes
    ///
    ///   pThis  - pointer to the interface structure.
    ///   offset - the offset of the hint reported to be downloaded.
    ///   size   - the size of the hint reported to be downloaded.
    ///
    /// The |offset| and |size| of the section may not be unique. Part of the
    /// section might be already available. The download manager must deal with
    /// overlapping sections.
    /// ```
    pub AddSegment: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FX_DOWNLOADHINTS, offset: usize, size: usize),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_IMAGEOBJ_METADATA {
    /// The image width in pixels.
    pub width: ::std::os::raw::c_uint,
    /// The image height in pixels.
    pub height: ::std::os::raw::c_uint,
    /// The image's horizontal pixel-per-inch.
    pub horizontal_dpi: f32,
    /// The image's vertical pixel-per-inch.
    pub vertical_dpi: f32,
    /// The number of bits used to represent each pixel.
    pub bits_per_pixel: ::std::os::raw::c_uint,
    /// The image's colorspace. See above for the list of FPDF_COLORSPACE_*.
    pub colorspace: ::std::os::raw::c_int,
    /// The image's marked content ID
    ///
    /// C documentation:
    ///
    /// ```text
    /// The image's marked content ID. Useful for pairing with associated alt-text.
    /// A value of -1 indicates no ID.
    /// ```
    pub marked_content_id: ::std::os::raw::c_int,
}

/// ISO C `broken-down time' structure.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    /// Seconds.\t\[0-60\] (1 leap second)
    pub tm_sec: ::std::os::raw::c_int,
    /// Minutes.\t\[0-59\]
    pub tm_min: ::std::os::raw::c_int,
    /// Hours.\t\[0-23\]
    pub tm_hour: ::std::os::raw::c_int,
    /// Day.\t\t\[1-31\]
    pub tm_mday: ::std::os::raw::c_int,
    /// Month.\t\[0-11\]
    pub tm_mon: ::std::os::raw::c_int,
    /// Year\t- 1900.
    pub tm_year: ::std::os::raw::c_int,
    /// Day of week.\t\[0-6\]
    pub tm_wday: ::std::os::raw::c_int,
    /// Days in year.\[0-365\]
    pub tm_yday: ::std::os::raw::c_int,
    /// DST.\t\t\[-1/0/1\]
    pub tm_isdst: ::std::os::raw::c_int,
    /// Seconds east of UTC.
    pub tm_gmtoff: ::std::os::raw::c_long,
    /// Timezone abbreviation.
    pub tm_zone: *const ::std::os::raw::c_char,
}

/// POSIX.1b structure for a time value
///
/// C documentation:
///
/// ```text
/// POSIX.1b structure for a time value.  This is like a `struct timeval' but
///has nanoseconds instead of microseconds.
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    /// Seconds.
    pub tv_sec: __time_t,
    /// Nanoseconds.
    pub tv_nsec: __syscall_slong_t,
}

/// POSIX.1b structure for timer start values and intervals.
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

/// Interface for unsupported feature notifications.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _UNSUPPORT_INFO {
    /// Version number of the interface. Must be 1.
    pub version: ::std::os::raw::c_int,
    /// Unsupported object notification function.
    ///
    /// C documentation:
    ///
    /// ```text
    /// Unsupported object notification function.
    /// Interface Version: 1
    /// Implementation Required: Yes
    ///
    ///   pThis - pointer to the interface structure.
    ///   nType - the type of unsupported object. One of the |FPDF_UNSP_*| entries.
    /// ```
    pub FSDK_UnSupport_Handler: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _UNSUPPORT_INFO, nType: ::std::os::raw::c_int),
    >,
}

/// IFPDF_RENDERINFO interface.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IFSDK_PAUSE {
    /// Version number of the interface. Currently must be 1.
    pub version: ::std::os::raw::c_int,
    /// Method: NeedToPauseNow
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: NeedToPauseNow
    ///           Check if we need to pause a progressive process now.
    /// Interface Version:
    ///           1
    /// Implementation Required:
    ///           yes
    /// Parameters:
    ///           pThis       -   Pointer to the interface structure itself
    /// Return Value:
    ///           Non-zero for pause now, 0 for continue.
    /// ```
    pub NeedToPauseNow:
        ::std::option::Option<unsafe extern "C" fn(pThis: *mut _IFSDK_PAUSE) -> FPDF_BOOL>,
    /// A user defined data pointer, used by user's application. Can be NULL.
    pub user: *mut ::std::os::raw::c_void,
}

/// Structure for custom file write
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_FILEWRITE_ {
    /// Version number of the interface. Currently must be 1.
    pub version: ::std::os::raw::c_int,
    /// Method: WriteBlock
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: WriteBlock
    ///          Output a block of data in your custom way.
    /// Interface Version:
    ///          1
    /// Implementation Required:
    ///          Yes
    /// Comments:
    ///          Called by function FPDF_SaveDocument
    /// Parameters:
    ///          pThis       -   Pointer to the structure itself
    ///          pData       -   Pointer to a buffer to output
    ///          size        -   The size of the buffer.
    /// Return value:
    ///          Should be non-zero if successful, zero for error.
    /// ```
    pub WriteBlock: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut FPDF_FILEWRITE_,
            pData: *const ::std::os::raw::c_void,
            size: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_int,
    >,
}

/// Interface: FPDF_SYSFONTINFO
///
/// C documentation:
///
/// ```text
/// Interface: FPDF_SYSFONTINFO
///          Interface for getting system font information and font mapping
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FPDF_SYSFONTINFO {
    /// Version number of the interface. Currently must be 1.
    pub version: ::std::os::raw::c_int,
    /// Method: Release
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: Release
    ///          Give implementation a chance to release any data after the
    ///          interface is no longer used.
    /// Interface Version:
    ///          1
    /// Implementation Required:
    ///          No
    /// Parameters:
    ///          pThis       -   Pointer to the interface structure itself
    /// Return Value:
    ///          None
    /// Comments:
    ///          Called by PDFium during the final cleanup process.
    /// ```
    pub Release: ::std::option::Option<unsafe extern "C" fn(pThis: *mut _FPDF_SYSFONTINFO)>,
    /// Method: EnumFonts
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: EnumFonts
    ///          Enumerate all fonts installed on the system
    /// Interface Version:
    ///          1
    /// Implementation Required:
    ///          No
    /// Parameters:
    ///          pThis       -   Pointer to the interface structure itself
    ///          pMapper     -   An opaque pointer to internal font mapper, used
    ///                          when calling FPDF_AddInstalledFont().
    /// Return Value:
    ///          None
    /// Comments:
    ///          Implementations should call FPDF_AddInstalledFont() function for
    ///          each font found. Only TrueType/OpenType and Type1 fonts are
    ///          accepted by PDFium.
    /// ```
    pub EnumFonts: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_SYSFONTINFO, pMapper: *mut ::std::os::raw::c_void),
    >,
    /// Method: MapFont
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: MapFont
    ///          Use the system font mapper to get a font handle from requested
    ///          parameters.
    /// Interface Version:
    ///          1
    /// Implementation Required:
    ///          Required if GetFont method is not implemented.
    /// Parameters:
    ///          pThis       -   Pointer to the interface structure itself
    ///          weight      -   Weight of the requested font. 400 is normal and
    ///                          700 is bold.
    ///          bItalic     -   Italic option of the requested font, TRUE or
    ///                          FALSE.
    ///          charset     -   Character set identifier for the requested font.
    ///                          See above defined constants.
    ///          pitch_family -  A combination of flags. See above defined
    ///                          constants.
    ///          face        -   Typeface name. Currently use system local encoding
    ///                          only.
    ///          bExact      -   Obsolete: this parameter is now ignored.
    /// Return Value:
    ///          An opaque pointer for font handle, or NULL if system mapping is
    ///          not supported.
    /// Comments:
    ///          If the system supports native font mapper (like Windows),
    ///          implementation can implement this method to get a font handle.
    ///          Otherwise, PDFium will do the mapping and then call GetFont
    ///          method. Only TrueType/OpenType and Type1 fonts are accepted
    ///          by PDFium.
    /// ```
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
    /// Method: GetFont
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: GetFont
    ///          Get a handle to a particular font by its internal ID
    /// Interface Version:
    ///          1
    /// Implementation Required:
    ///          Required if MapFont method is not implemented.
    /// Return Value:
    ///          An opaque pointer for font handle.
    /// Parameters:
    ///          pThis       -   Pointer to the interface structure itself
    ///          face        -   Typeface name in system local encoding.
    /// Comments:
    ///          If the system mapping not supported, PDFium will do the font
    ///          mapping and use this method to get a font handle.
    /// ```
    pub GetFont: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            face: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_void,
    >,
    /// Method: GetFontData
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: GetFontData
    ///          Get font data from a font
    /// Interface Version:
    ///          1
    /// Implementation Required:
    ///          Yes
    /// Parameters:
    ///          pThis       -   Pointer to the interface structure itself
    ///          hFont       -   Font handle returned by MapFont or GetFont method
    ///          table       -   TrueType/OpenType table identifier (refer to
    ///                          TrueType specification), or 0 for the whole file.
    ///          buffer      -   The buffer receiving the font data. Can be NULL if
    ///                          not provided.
    ///          buf_size    -   Buffer size, can be zero if not provided.
    /// Return Value:
    ///          Number of bytes needed, if buffer not provided or not large
    ///          enough, or number of bytes written into buffer otherwise.
    /// Comments:
    ///          Can read either the full font file, or a particular
    ///          TrueType/OpenType table.
    /// ```
    pub GetFontData: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            hFont: *mut ::std::os::raw::c_void,
            table: ::std::os::raw::c_uint,
            buffer: *mut ::std::os::raw::c_uchar,
            buf_size: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_ulong,
    >,
    /// Method: GetFaceName
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: GetFaceName
    ///          Get face name from a font handle
    /// Interface Version:
    ///          1
    /// Implementation Required:
    ///          No
    /// Parameters:
    ///          pThis       -   Pointer to the interface structure itself
    ///          hFont       -   Font handle returned by MapFont or GetFont method
    ///          buffer      -   The buffer receiving the face name. Can be NULL if
    ///                          not provided
    ///          buf_size    -   Buffer size, can be zero if not provided
    /// Return Value:
    ///          Number of bytes needed, if buffer not provided or not large
    ///          enough, or number of bytes written into buffer otherwise.
    /// ```
    pub GetFaceName: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            hFont: *mut ::std::os::raw::c_void,
            buffer: *mut ::std::os::raw::c_char,
            buf_size: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_ulong,
    >,
    /// Method: GetFontCharset
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: GetFontCharset
    ///          Get character set information for a font handle
    /// Interface Version:
    ///          1
    /// Implementation Required:
    ///          No
    /// Parameters:
    ///          pThis       -   Pointer to the interface structure itself
    ///          hFont       -   Font handle returned by MapFont or GetFont method
    /// Return Value:
    ///          Character set identifier. See defined constants above.
    /// ```
    pub GetFontCharset: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut _FPDF_SYSFONTINFO,
            hFont: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    /// Method: DeleteFont
    ///
    /// C documentation:
    ///
    /// ```text
    /// Method: DeleteFont
    ///          Delete a font handle
    /// Interface Version:
    ///          1
    /// Implementation Required:
    ///          Yes
    /// Parameters:
    ///          pThis       -   Pointer to the interface structure itself
    ///          hFont       -   Font handle returned by MapFont or GetFont method
    /// Return Value:
    ///          None
    /// ```
    pub DeleteFont: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut _FPDF_SYSFONTINFO, hFont: *mut ::std::os::raw::c_void),
    >,
}

/// Struct: FPDF_CharsetFontMap
///
/// C documentation:
///
/// ```text
/// Struct: FPDF_CharsetFontMap
///    Provides the name of a font to use for a given charset value.
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPDF_CharsetFontMap_ {
    /// Character Set Enum value, see FXFONT_*_CHARSET above.
    pub charset: ::std::os::raw::c_int,
    /// Name of default font to use with that charset.
    pub fontname: *const ::std::os::raw::c_char,
}

/// PDF text rendering modes
pub type FPDF_TEXT_RENDERMODE = ::std::os::raw::c_int;

/// PDF types - use incomplete types (never completed) to force API type safety.
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

/// Basic data types
pub type FPDF_BOOL = ::std::os::raw::c_int;

pub type FPDF_RESULT = ::std::os::raw::c_int;

pub type FPDF_DWORD = ::std::os::raw::c_ulong;

pub type FS_FLOAT = f32;

/// Duplex types
pub type _FPDF_DUPLEXTYPE_ = ::std::os::raw::c_uint;

/// Duplex types
pub use self::_FPDF_DUPLEXTYPE_ as FPDF_DUPLEXTYPE;

/// String types
pub type FPDF_WCHAR = ::std::os::raw::c_ushort;

/// Public PDFium API type for byte strings.
pub type FPDF_BYTESTRING = *const ::std::os::raw::c_char;

/// The public PDFium API always uses UTF-16LE encoded wide strings
///
/// C documentation:
///
/// ```text
/// The public PDFium API always uses UTF-16LE encoded wide strings, each
/// character uses 2 bytes (except surrogation), with the low byte first.
/// ```
pub type FPDF_WIDESTRING = *const FPDF_WCHAR;

/// Structure for persisting a string beyond the duration of a callback.
///
/// C documentation:
///
/// ```text
/// Structure for persisting a string beyond the duration of a callback.
/// Note: although represented as a char*, string may be interpreted as
/// a UTF-16LE formated string. Used only by XFA callbacks.
/// ```
pub type FPDF_BSTR = FPDF_BSTR_;

/// For Windows programmers: In most cases it's OK to treat FPDF_WIDESTRING as a
///
/// C documentation:
///
/// ```text
/// For Windows programmers: In most cases it's OK to treat FPDF_WIDESTRING as a
/// Windows unicode string, however, special care needs to be taken if you
/// expect to process Unicode larger than 0xffff.
///
/// For Linux/Unix programmers: most compiler/library environments use 4 bytes
/// for a Unicode character, and you have to convert between FPDF_WIDESTRING and
/// system wide string by yourself.
/// ```
pub type FPDF_STRING = *const ::std::os::raw::c_char;

/// Matrix for transformation
///
/// C documentation:
///
/// ```text
/// Matrix for transformation, in the form \[a b c d e f\], equivalent to:
/// | a  b  0 |
/// | c  d  0 |
/// | e  f  1 |
///
/// Translation is performed with \[1 0 0 1 tx ty\].
/// Scaling is performed with \[sx 0 0 sy 0 0\].
/// See PDF Reference 1.7, 4.2.2 Common Transformations for more.
/// ```
pub type FS_MATRIX = _FS_MATRIX_;

/// Rectangle area(float) in device or page coordinate system.
pub type FS_LPRECTF = *mut _FS_RECTF_;

/// Rectangle area(float) in device or page coordinate system.
pub type FS_RECTF = _FS_RECTF_;

/// Const Pointer to FS_RECTF structure.
pub type FS_LPCRECTF = *const FS_RECTF;

/// Rectangle size. Coordinate system agnostic.
pub type FS_LPSIZEF = *mut FS_SIZEF_;

/// Rectangle size. Coordinate system agnostic.
pub type FS_SIZEF = FS_SIZEF_;

/// Const Pointer to FS_SIZEF structure.
pub type FS_LPCSIZEF = *const FS_SIZEF;

/// 2D Point. Coordinate system agnostic.
pub type FS_LPPOINTF = *mut FS_POINTF_;

/// 2D Point. Coordinate system agnostic.
pub type FS_POINTF = FS_POINTF_;

/// Const Pointer to FS_POINTF structure.
pub type FS_LPCPOINTF = *const FS_POINTF;

pub type FS_QUADPOINTSF = _FS_QUADPOINTSF;

/// Annotation enums.
pub type FPDF_ANNOTATION_SUBTYPE = ::std::os::raw::c_int;

pub type FPDF_ANNOT_APPEARANCEMODE = ::std::os::raw::c_int;

/// Dictionary value types.
pub type FPDF_OBJECT_TYPE = ::std::os::raw::c_int;

/// PDF renderer types - Experimental.
///
/// C documentation:
///
/// ```text
/// PDF renderer types - Experimental.
/// Selection of 2D graphics library to use for rendering to FPDF_BITMAPs.
/// ```
pub type FPDF_RENDERER_TYPE = ::std::os::raw::c_uint;

/// Process-wide options for initializing the library.
pub type FPDF_LIBRARY_CONFIG = FPDF_LIBRARY_CONFIG_;

/// Structure for file reading or writing (I/O).
///
/// C documentation:
///
/// ```text
/// Structure for file reading or writing (I/O).
///
/// Note: This is a handler and should be implemented by callers,
/// and is only used from XFA.
/// ```
pub type FPDF_FILEHANDLER = FPDF_FILEHANDLER_;

/// Struct for color scheme.
///
/// C documentation:
///
/// ```text
/// Struct for color scheme.
/// Each should be a 32-bit value specifying the color, in 8888 ARGB format.
/// ```
pub type FPDF_COLORSCHEME = FPDF_COLORSCHEME_;

pub type IPDF_JSPLATFORM = _IPDF_JsPlatform;

/// Function signature for the callback function passed to the FFI_SetTimer
///
/// C documentation:
///
/// ```text
/// Function signature for the callback function passed to the FFI_SetTimer
/// method.
/// Parameters:
///          idEvent     -   Identifier of the timer.
/// Return value:
///          None.
/// ```
pub type TimerCallback =
    ::std::option::Option<unsafe extern "C" fn(idEvent: ::std::os::raw::c_int)>;

/// Declares of a struct type to the local system time.
pub type FPDF_SYSTEMTIME = _FPDF_SYSTEMTIME;

pub type FPDF_FORMFILLINFO = _FPDF_FORMFILLINFO;

pub type FPDFANNOT_COLORTYPE = ::std::os::raw::c_uint;

/// Interface for checking whether sections of the file are available.
pub type FX_FILEAVAIL = _FX_FILEAVAIL;

/// Download hints interface. Used to receive hints for further downloading.
pub type FX_DOWNLOADHINTS = _FX_DOWNLOADHINTS;

/// The file identifier entry type
///
/// C documentation:
///
/// ```text
/// The file identifier entry type. See section 14.4 "File Identifiers" of the
/// ISO 32000-1:2008 spec.
/// ```
pub type FPDF_FILEIDTYPE = ::std::os::raw::c_uint;

pub type __time_t = ::std::os::raw::c_long;

pub type __syscall_slong_t = ::std::os::raw::c_long;

pub type __syscall_ulong_t = ::std::os::raw::c_ulong;

pub type time_t = __time_t;

/// Interface for unsupported feature notifications.
pub type UNSUPPORT_INFO = _UNSUPPORT_INFO;

/// Key flags.
pub type FWL_EVENTFLAG = ::std::os::raw::c_uint;

/// Virtual keycodes.
pub type FWL_VKEYCODE = ::std::os::raw::c_uint;

/// IFPDF_RENDERINFO interface.
pub type IFSDK_PAUSE = _IFSDK_PAUSE;

/// Structure for custom file write
pub type FPDF_FILEWRITE = FPDF_FILEWRITE_;

/// Interface: FPDF_SYSFONTINFO
///
/// C documentation:
///
/// ```text
/// Interface: FPDF_SYSFONTINFO
///          Interface for getting system font information and font mapping
/// ```
pub type FPDF_SYSFONTINFO = _FPDF_SYSFONTINFO;

/// Struct: FPDF_CharsetFontMap
///
/// C documentation:
///
/// ```text
/// Struct: FPDF_CharsetFontMap
///    Provides the name of a font to use for a given charset value.
/// ```
pub type FPDF_CharsetFontMap = FPDF_CharsetFontMap_;

pub type ActionHandle = Handle<fpdf_action_t__>;
pub type AnnotationHandle = Handle<fpdf_annotation_t__>;
pub type AttachmentHandle = Handle<fpdf_attachment_t__>;
pub type AvailabilityHandle = Handle<fpdf_avail_t__>;
pub type BitmapHandle = Handle<fpdf_bitmap_t__>;
pub type BookmarkHandle = Handle<fpdf_bookmark_t__>;
pub type ClipPathHandle = Handle<fpdf_clippath_t__>;
pub type DestinationHandle = Handle<fpdf_dest_t__>;
pub type DocumentHandle = Handle<fpdf_document_t__>;
pub type FontHandle = Handle<fpdf_font_t__>;
pub type FormHandle = Handle<fpdf_form_handle_t__>;
pub type GlyphPathHandle = Handle<fpdf_glyphpath_t__>;
pub type JavascriptActionHandle = Handle<fpdf_javascript_action_t>;
pub type LinkHandle = Handle<fpdf_link_t__>;
pub type PageHandle = Handle<fpdf_page_t__>;
pub type PageLinkHandle = Handle<fpdf_pagelink_t__>;
pub type PageObjectHandle = Handle<fpdf_pageobject_t__>;
pub type PageObjectMarkHandle = Handle<fpdf_pageobjectmark_t__>;
pub type PageRangeHandle = Handle<fpdf_pagerange_t__>;
pub type PathSegmentHandle = Handle<fpdf_pathsegment_t>;
pub type SearchHandle = Handle<fpdf_schhandle_t__>;
pub type SignatureHandle = Handle<fpdf_signature_t__>;
pub type SkiaCanvasHandle = Handle<::std::os::raw::c_void>;
pub type StructElementHandle = Handle<fpdf_structelement_t__>;
pub type StructElementAttrHandle = Handle<fpdf_structelement_attr_t__>;
pub type StructElementAttrValueHandle = Handle<fpdf_structelement_attr_value_t__>;
pub type StructTreeHandle = Handle<fpdf_structtree_t__>;
pub type TextPageHandle = Handle<fpdf_textpage_t__>;
pub type WidgetHandle = Handle<fpdf_widget_t__>;
pub type XObjectHandle = Handle<fpdf_xobject_t__>;
