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
#![allow(clippy::too_many_arguments)]

use std::ffi::CString;
use std::os::raw::{c_char, c_long, c_ulong, c_ushort, c_void};
use std::ptr::null_mut;

use crate::{
    pdfium_types::*, Pdfium, PdfiumAction, PdfiumAnnotation, PdfiumAttachment, PdfiumAvailability,
    PdfiumBitmap, PdfiumBookmark, PdfiumClipPath, PdfiumDestination, PdfiumDocument, PdfiumError,
    PdfiumFont, PdfiumForm, PdfiumGlyphPath, PdfiumJavascriptAction, PdfiumLink, PdfiumPage,
    PdfiumPageLink, PdfiumPageObject, PdfiumPageObjectMark, PdfiumPageRange, PdfiumPathSegment,
    PdfiumReader, PdfiumResult, PdfiumSearch, PdfiumSignature, PdfiumStructElement,
    PdfiumStructElementAttr, PdfiumStructElementAttrValue, PdfiumStructTree, PdfiumTextPage,
    PdfiumXObject,
};

/// This is the memory-safe wrapper around PDFium's C API. All raw C pointers and manual
/// memory management have been replaced with safe Rust equivalents (`&str`, `Vec<u8>`,
/// references, etc.). This allows full access to PDFium functionality without requiring
/// `unsafe` blocks in client code. Memory management and null pointer checks are handled
/// internally.
impl Pdfium {
    /// C documentation for FORM_CanRedo:
    ///
    /// ```text
    /// Function: FORM_CanRedo
    ///       Find out if it is possible for the current focused widget in a given
    ///       form to perform a redo operation.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    /// Return Value:
    ///       True if it is possible to redo.
    /// ```
    #[inline]
    pub fn FORM_CanRedo(&self, hHandle: &PdfiumForm, page: &PdfiumPage) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FORM_CanRedo)(hHandle.into(), page.into()) })
    }

    /// C documentation for FORM_CanUndo:
    ///
    /// ```text
    /// Function: FORM_CanUndo
    ///       Find out if it is possible for the current focused widget in a given
    ///       form to perform an undo operation.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    /// Return Value:
    ///       True if it is possible to undo.
    /// ```
    #[inline]
    pub fn FORM_CanUndo(&self, hHandle: &PdfiumForm, page: &PdfiumPage) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FORM_CanUndo)(hHandle.into(), page.into()) })
    }

    /// C documentation for FORM_DoDocumentAAction:
    ///
    /// ```text
    /// Function: FORM_DoDocumentAAction
    ///       This method is required for performing the document's
    ///       additional-action.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module. Returned by
    ///                       FPDFDOC_InitFormFillEnvironment.
    ///       aaType      -   The type of the additional-actions which defined
    ///                       above.
    /// Return Value:
    ///       None.
    /// Comments:
    ///       This method will do nothing if there is no document
    ///       additional-action corresponding to the specified |aaType|.
    /// ```
    #[inline]
    pub fn FORM_DoDocumentAAction(&self, hHandle: &PdfiumForm, aaType: i32) {
        unsafe { (self.fn_FORM_DoDocumentAAction)(hHandle.into(), aaType) }
    }

    /// C documentation for FORM_DoDocumentJSAction:
    ///
    /// ```text
    /// Function: FORM_DoDocumentJSAction
    ///       This method is required for performing document-level JavaScript
    ///       actions. It should be invoked after the PDF document has been loaded.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    /// Return Value:
    ///       None.
    /// Comments:
    ///       If there is document-level JavaScript action embedded in the
    ///       document, this method will execute the JavaScript action. Otherwise,
    ///       the method will do nothing.
    /// ```
    #[inline]
    pub fn FORM_DoDocumentJSAction(&self, hHandle: &PdfiumForm) {
        unsafe { (self.fn_FORM_DoDocumentJSAction)(hHandle.into()) }
    }

    /// C documentation for FORM_DoDocumentOpenAction:
    ///
    /// ```text
    /// Function: FORM_DoDocumentOpenAction
    ///       This method is required for performing open-action when the document
    ///       is opened.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    /// Return Value:
    ///       None.
    /// Comments:
    ///       This method will do nothing if there are no open-actions embedded
    ///       in the document.
    /// ```
    #[inline]
    pub fn FORM_DoDocumentOpenAction(&self, hHandle: &PdfiumForm) {
        unsafe { (self.fn_FORM_DoDocumentOpenAction)(hHandle.into()) }
    }

    /// C documentation for FORM_DoPageAAction:
    ///
    /// ```text
    /// Function: FORM_DoPageAAction
    ///       This method is required for performing the page object's
    ///       additional-action when opened or closed.
    /// Parameters:
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       aaType      -   The type of the page object's additional-actions
    ///                       which defined above.
    /// Return Value:
    ///       None.
    /// Comments:
    ///       This method will do nothing if no additional-action corresponding
    ///       to the specified |aaType| exists.
    /// ```
    #[inline]
    pub fn FORM_DoPageAAction(&self, page: &PdfiumPage, hHandle: &PdfiumForm, aaType: i32) {
        unsafe { (self.fn_FORM_DoPageAAction)(page.into(), hHandle.into(), aaType) }
    }

    /// C documentation for FORM_ForceToKillFocus:
    ///
    /// ```text
    /// Function: FORM_ForceToKillFocus.
    ///       Call this member function to force to kill the focus of the form
    ///       field which has focus. If it would kill the focus of a form field,
    ///       save the value of form field if was changed by theuser.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    /// Return Value:
    ///       True indicates success; otherwise false.
    /// ```
    #[inline]
    pub fn FORM_ForceToKillFocus(&self, hHandle: &PdfiumForm) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FORM_ForceToKillFocus)(hHandle.into()) })
    }

    /// C documentation for FORM_GetFocusedAnnot:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FORM_GetFocusedAnnot.
    ///       Call this member function to get the currently focused annotation.
    /// Parameters:
    ///       handle      -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page_index  -   Buffer to hold the index number of the page which
    ///                       contains the focused annotation. 0 for the first page.
    ///                       Can't be NULL.
    ///       annot       -   Buffer to hold the focused annotation. Can't be NULL.
    /// Return Value:
    ///       On success, return true and write to the out parameters. Otherwise
    ///       return false and leave the out parameters unmodified.
    /// Comments:
    ///       Not currently supported for XFA forms - will report no focused
    ///       annotation.
    ///       Must call FPDFPage_CloseAnnot() when the annotation returned in |annot|
    ///       by this function is no longer needed.
    ///       This will return true and set |page_index| to -1 and |annot| to NULL,
    ///       if there is no focused annotation.
    /// ```
    #[inline]
    pub fn FORM_GetFocusedAnnot(
        &self,
        handle: &PdfiumForm,
        page_index: &mut i32,
    ) -> PdfiumResult<PdfiumAnnotation> {
        let mut annot: FPDF_ANNOTATION = null_mut();
        let result =
            unsafe { (self.fn_FORM_GetFocusedAnnot)(handle.into(), page_index, &mut annot) };
        if result == 0 {
            Err(PdfiumError::InvokationFailed)
        } else {
            PdfiumAnnotation::new_from_handle(annot)
        }
    }

    /// C documentation for FORM_GetFocusedText:
    ///
    /// ```text
    /// Experimental API
    /// Function: FORM_GetFocusedText
    ///       Call this function to obtain the text within the current focused
    ///       field, if any.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       buffer      -   Buffer for holding the form text, encoded in
    ///                       UTF-16LE. If NULL, |buffer| is not modified.
    ///       buflen      -   Length of |buffer| in bytes. If |buflen| is less
    ///                       than the length of the form text string, |buffer| is
    ///                       not modified.
    /// Return Value:
    ///       Length in bytes for the text in the focused field.
    /// ```
    #[inline]
    pub fn FORM_GetFocusedText(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FORM_GetFocusedText)(
                hHandle.into(),
                page.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FORM_GetSelectedText:
    ///
    /// ```text
    /// Function: FORM_GetSelectedText
    ///       Call this function to obtain selected text within a form text
    ///       field or form combobox text field.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       buffer      -   Buffer for holding the selected text, encoded in
    ///                       UTF-16LE. If NULL, |buffer| is not modified.
    ///       buflen      -   Length of |buffer| in bytes. If |buflen| is less
    ///                       than the length of the selected text string,
    ///                       |buffer| is not modified.
    /// Return Value:
    ///       Length in bytes of selected text in form text field or form combobox
    ///       text field.
    /// ```
    #[inline]
    pub fn FORM_GetSelectedText(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FORM_GetSelectedText)(
                hHandle.into(),
                page.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FORM_IsIndexSelected:
    ///
    /// ```text
    /// Experimental API
    /// Function: FORM_IsIndexSelected
    ///           Returns whether or not the value at |index| of the focused
    ///           annotation is currently selected.
    /// Parameters:
    ///           hHandle     -   Handle to the form fill module. Returned by
    ///                           FPDFDOC_InitFormFillEnvironment.
    ///           page        -   Handle to the page. Returned by FPDF_LoadPage
    ///           index       -   0-based Index of value to check
    /// Return Value:
    ///           TRUE if value at |index| is currently selected.
    ///           FALSE if value at |index| is not selected or widget is not a
    ///           supported type.
    /// Comments:
    ///           Intended for use with listbox/combobox widget types. Default
    ///           implementation is a no-op that will return false for other types.
    ///           Not currently supported for XFA forms - will return false.
    /// ```
    #[inline]
    pub fn FORM_IsIndexSelected(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        index: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FORM_IsIndexSelected)(hHandle.into(), page.into(), index) })
    }

    /// C documentation for FORM_OnAfterLoadPage:
    ///
    /// ```text
    /// Function: FORM_OnAfterLoadPage
    ///       This method is required for implementing all the form related
    ///       functions. Should be invoked after user successfully loaded a
    ///       PDF page, and FPDFDOC_InitFormFillEnvironment() has been invoked.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    /// Return Value:
    ///       None.
    /// ```
    #[inline]
    pub fn FORM_OnAfterLoadPage(&self, page: &PdfiumPage, hHandle: &PdfiumForm) {
        unsafe { (self.fn_FORM_OnAfterLoadPage)(page.into(), hHandle.into()) }
    }

    /// C documentation for FORM_OnBeforeClosePage:
    ///
    /// ```text
    /// Function: FORM_OnBeforeClosePage
    ///       This method is required for implementing all the form related
    ///       functions. Should be invoked before user closes the PDF page.
    /// Parameters:
    ///        page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///        hHandle     -   Handle to the form fill module, as returned by
    ///                        FPDFDOC_InitFormFillEnvironment().
    /// Return Value:
    ///        None.
    /// ```
    #[inline]
    pub fn FORM_OnBeforeClosePage(&self, page: &PdfiumPage, hHandle: &PdfiumForm) {
        unsafe { (self.fn_FORM_OnBeforeClosePage)(page.into(), hHandle.into()) }
    }

    /// C documentation for FORM_OnChar:
    ///
    /// ```text
    /// Function: FORM_OnChar
    ///       Call this member function when a keystroke translates to a
    ///       nonsystem character.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       nChar       -   The character code value itself.
    ///       modifier    -   Mask of key flags (see fpdf_fwlevent.h for key
    ///                       flag values).
    /// Return Value:
    ///       True indicates success; otherwise false.
    /// ```
    #[inline]
    pub fn FORM_OnChar(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        nChar: i32,
        modifier: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FORM_OnChar)(hHandle.into(), page.into(), nChar, modifier) })
    }

    /// C documentation for FORM_OnFocus:
    ///
    /// ```text
    /// Function: FORM_OnFocus
    ///       This function focuses the form annotation at a given point. If the
    ///       annotation at the point already has focus, nothing happens. If there
    ///       is no annotation at the point, removes form focus.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       modifier    -   Indicates whether various virtual keys are down.
    ///       page_x      -   Specifies the x-coordinate of the cursor in PDF user
    ///                       space.
    ///       page_y      -   Specifies the y-coordinate of the cursor in PDF user
    ///                       space.
    /// Return Value:
    ///       True if there is an annotation at the given point and it has focus.
    /// ```
    #[inline]
    pub fn FORM_OnFocus(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        modifier: i32,
        page_x: f64,
        page_y: f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FORM_OnFocus)(hHandle.into(), page.into(), modifier, page_x, page_y)
        })
    }

    /// C documentation for FORM_OnKeyDown:
    ///
    /// ```text
    /// Function: FORM_OnKeyDown
    ///       Call this member function when a nonsystem key is pressed.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, aseturned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       nKeyCode    -   The virtual-key code of the given key (see
    ///                       fpdf_fwlevent.h for virtual key codes).
    ///       modifier    -   Mask of key flags (see fpdf_fwlevent.h for key
    ///                       flag values).
    /// Return Value:
    ///       True indicates success; otherwise false.
    /// ```
    #[inline]
    pub fn FORM_OnKeyDown(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        nKeyCode: i32,
        modifier: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FORM_OnKeyDown)(hHandle.into(), page.into(), nKeyCode, modifier)
        })
    }

    /// C documentation for FORM_OnKeyUp:
    ///
    /// ```text
    /// Function: FORM_OnKeyUp
    ///       Call this member function when a nonsystem key is released.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       nKeyCode    -   The virtual-key code of the given key (see
    ///                       fpdf_fwlevent.h for virtual key codes).
    ///       modifier    -   Mask of key flags (see fpdf_fwlevent.h for key
    ///                       flag values).
    /// Return Value:
    ///       True indicates success; otherwise false.
    /// Comments:
    ///       Currently unimplemented and always returns false. PDFium reserves this
    ///       API and may implement it in the future on an as-needed basis.
    /// ```
    #[inline]
    pub fn FORM_OnKeyUp(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        nKeyCode: i32,
        modifier: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FORM_OnKeyUp)(hHandle.into(), page.into(), nKeyCode, modifier)
        })
    }

    /// C documentation for FORM_OnLButtonDoubleClick:
    ///
    /// ```text
    /// Function: FORM_OnLButtonDoubleClick
    ///       Call this member function when the user double clicks the
    ///       left mouse button.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       modifier    -   Indicates whether various virtual keys are down.
    ///       page_x      -   Specifies the x-coordinate of the cursor in PDF user
    ///                       space.
    ///       page_y      -   Specifies the y-coordinate of the cursor in PDF user
    ///                       space.
    /// Return Value:
    ///       True indicates success; otherwise false.
    /// ```
    #[inline]
    pub fn FORM_OnLButtonDoubleClick(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        modifier: i32,
        page_x: f64,
        page_y: f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FORM_OnLButtonDoubleClick)(
                hHandle.into(),
                page.into(),
                modifier,
                page_x,
                page_y,
            )
        })
    }

    /// C documentation for FORM_OnLButtonDown:
    ///
    /// ```text
    /// Function: FORM_OnLButtonDown
    ///       Call this member function when the user presses the left
    ///       mouse button.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       modifier    -   Indicates whether various virtual keys are down.
    ///       page_x      -   Specifies the x-coordinate of the cursor in PDF user
    ///                       space.
    ///       page_y      -   Specifies the y-coordinate of the cursor in PDF user
    ///                       space.
    /// Return Value:
    ///       True indicates success; otherwise false.
    /// ```
    #[inline]
    pub fn FORM_OnLButtonDown(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        modifier: i32,
        page_x: f64,
        page_y: f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FORM_OnLButtonDown)(hHandle.into(), page.into(), modifier, page_x, page_y)
        })
    }

    /// C documentation for FORM_OnLButtonUp:
    ///
    /// ```text
    /// Function: FORM_OnLButtonUp
    ///       Call this member function when the user releases the left
    ///       mouse button.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       modifier    -   Indicates whether various virtual keys are down.
    ///       page_x      -   Specifies the x-coordinate of the cursor in device.
    ///       page_y      -   Specifies the y-coordinate of the cursor in device.
    /// Return Value:
    ///       True indicates success; otherwise false.
    /// ```
    #[inline]
    pub fn FORM_OnLButtonUp(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        modifier: i32,
        page_x: f64,
        page_y: f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FORM_OnLButtonUp)(hHandle.into(), page.into(), modifier, page_x, page_y)
        })
    }

    /// C documentation for FORM_OnMouseMove:
    ///
    /// ```text
    /// Function: FORM_OnMouseMove
    ///       Call this member function when the mouse cursor moves.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       modifier    -   Indicates whether various virtual keys are down.
    ///       page_x      -   Specifies the x-coordinate of the cursor in PDF user
    ///                       space.
    ///       page_y      -   Specifies the y-coordinate of the cursor in PDF user
    ///                       space.
    /// Return Value:
    ///       True indicates success; otherwise false.
    /// ```
    #[inline]
    pub fn FORM_OnMouseMove(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        modifier: i32,
        page_x: f64,
        page_y: f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FORM_OnMouseMove)(hHandle.into(), page.into(), modifier, page_x, page_y)
        })
    }

    /// C documentation for FORM_OnMouseWheel:
    ///
    /// ```text
    /// Experimental API
    /// Function: FORM_OnMouseWheel
    ///       Call this member function when the user scrolls the mouse wheel.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    ///       modifier    -   Indicates whether various virtual keys are down.
    ///       page_coord  -   Specifies the coordinates of the cursor in PDF user
    ///                       space.
    ///       delta_x     -   Specifies the amount of wheel movement on the x-axis,
    ///                       in units of platform-agnostic wheel deltas. Negative
    ///                       values mean left.
    ///       delta_y     -   Specifies the amount of wheel movement on the y-axis,
    ///                       in units of platform-agnostic wheel deltas. Negative
    ///                       values mean down.
    /// Return Value:
    ///       True indicates success; otherwise false.
    /// Comments:
    ///       For |delta_x| and |delta_y|, the caller must normalize
    ///       platform-specific wheel deltas. e.g. On Windows, a delta value of 240
    ///       for a WM_MOUSEWHEEL event normalizes to 2, since Windows defines
    ///       WHEEL_DELTA as 120.
    /// ```
    #[inline]
    pub fn FORM_OnMouseWheel(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        modifier: i32,
        page_coord: &FS_POINTF,
        delta_x: i32,
        delta_y: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FORM_OnMouseWheel)(
                hHandle.into(),
                page.into(),
                modifier,
                page_coord,
                delta_x,
                delta_y,
            )
        })
    }

    /// C documentation for FORM_OnRButtonDown:
    ///
    /// ```text
    /// Function: FORM_OnRButtonDown
    ///       Same as above, execpt for the right mouse button.
    /// Comments:
    ///       At the present time, has no effect except in XFA builds, but is
    ///       included for the sake of symmetry.
    /// ```
    #[inline]
    pub fn FORM_OnRButtonDown(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        modifier: i32,
        page_x: f64,
        page_y: f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FORM_OnRButtonDown)(hHandle.into(), page.into(), modifier, page_x, page_y)
        })
    }

    /// C documentation for FORM_OnRButtonUp:
    ///
    /// ```text
    /// Function: FORM_OnRButtonUp
    ///       Same as above, execpt for the right mouse button.
    /// Comments:
    ///       At the present time, has no effect except in XFA builds, but is
    ///       included for the sake of symmetry.
    /// ```
    #[inline]
    pub fn FORM_OnRButtonUp(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        modifier: i32,
        page_x: f64,
        page_y: f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FORM_OnRButtonUp)(hHandle.into(), page.into(), modifier, page_x, page_y)
        })
    }

    /// C documentation for FORM_Redo:
    ///
    /// ```text
    /// Function: FORM_Redo
    ///       Make the current focused widget perform a redo operation.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    /// Return Value:
    ///       True if the redo operation succeeded.
    /// ```
    #[inline]
    pub fn FORM_Redo(&self, hHandle: &PdfiumForm, page: &PdfiumPage) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FORM_Redo)(hHandle.into(), page.into()) })
    }

    /// C documentation for FORM_ReplaceAndKeepSelection:
    ///
    /// ```text
    /// Experimental API
    /// Function: FORM_ReplaceAndKeepSelection
    ///       Call this function to replace the selected text in a form
    ///       text field or user-editable form combobox text field with another
    ///       text string (which can be empty or non-empty). If there is no
    ///       selected text, this function will append the replacement text after
    ///       the current caret position. After the insertion, the inserted text
    ///       will be selected.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as Returned by FPDF_LoadPage().
    ///       wsText      -   The text to be inserted, in UTF-16LE format.
    /// Return Value:
    ///       None.
    /// ```
    #[inline]
    pub fn FORM_ReplaceAndKeepSelection(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        wsText: &str,
    ) {
        let wsText = str_to_utf16le_vec(wsText);
        unsafe {
            (self.fn_FORM_ReplaceAndKeepSelection)(hHandle.into(), page.into(), wsText.as_ptr())
        }
    }

    /// C documentation for FORM_ReplaceSelection:
    ///
    /// ```text
    /// Function: FORM_ReplaceSelection
    ///       Call this function to replace the selected text in a form
    ///       text field or user-editable form combobox text field with another
    ///       text string (which can be empty or non-empty). If there is no
    ///       selected text, this function will append the replacement text after
    ///       the current caret position. After the insertion, the selection range
    ///       will be set to empty.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as Returned by FPDF_LoadPage().
    ///       wsText      -   The text to be inserted, in UTF-16LE format.
    /// Return Value:
    ///       None.
    /// ```
    #[inline]
    pub fn FORM_ReplaceSelection(&self, hHandle: &PdfiumForm, page: &PdfiumPage, wsText: &str) {
        let wsText = str_to_utf16le_vec(wsText);
        unsafe { (self.fn_FORM_ReplaceSelection)(hHandle.into(), page.into(), wsText.as_ptr()) }
    }

    /// C documentation for FORM_SelectAllText:
    ///
    /// ```text
    /// Experimental API
    /// Function: FORM_SelectAllText
    ///       Call this function to select all the text within the currently focused
    ///       form text field or form combobox text field.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    /// Return Value:
    ///       Whether the operation succeeded or not.
    /// ```
    #[inline]
    pub fn FORM_SelectAllText(&self, hHandle: &PdfiumForm, page: &PdfiumPage) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FORM_SelectAllText)(hHandle.into(), page.into()) })
    }

    /// C documentation for FORM_SetFocusedAnnot:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FORM_SetFocusedAnnot.
    ///       Call this member function to set the currently focused annotation.
    /// Parameters:
    ///       handle      -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       annot       -   Handle to an annotation.
    /// Return Value:
    ///       True indicates success; otherwise false.
    /// Comments:
    ///       |annot| can't be NULL. To kill focus, use FORM_ForceToKillFocus()
    ///       instead.
    /// ```
    #[inline]
    pub fn FORM_SetFocusedAnnot(
        &self,
        handle: &PdfiumForm,
        annot: &PdfiumAnnotation,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FORM_SetFocusedAnnot)(handle.into(), annot.into()) })
    }

    /// C documentation for FORM_SetIndexSelected:
    ///
    /// ```text
    /// Experimental API
    /// Function: FORM_SetIndexSelected
    ///           Selects/deselects the value at the given |index| of the focused
    ///           annotation.
    /// Parameters:
    ///           hHandle     -   Handle to the form fill module. Returned by
    ///                           FPDFDOC_InitFormFillEnvironment.
    ///           page        -   Handle to the page. Returned by FPDF_LoadPage
    ///           index       -   0-based index of value to be set as
    ///                           selected/unselected
    ///           selected    -   true to select, false to deselect
    /// Return Value:
    ///           TRUE if the operation succeeded.
    ///           FALSE if the operation failed or widget is not a supported type.
    /// Comments:
    ///           Intended for use with listbox/combobox widget types. Comboboxes
    ///           have at most a single value selected at a time which cannot be
    ///           deselected. Deselect on a combobox is a no-op that returns false.
    ///           Default implementation is a no-op that will return false for
    ///           other types.
    ///           Not currently supported for XFA forms - will return false.
    /// ```
    #[inline]
    pub fn FORM_SetIndexSelected(
        &self,
        hHandle: &PdfiumForm,
        page: &PdfiumPage,
        index: i32,
        selected: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FORM_SetIndexSelected)(hHandle.into(), page.into(), index, selected)
        })
    }

    /// C documentation for FORM_Undo:
    ///
    /// ```text
    /// Function: FORM_Undo
    ///       Make the current focused widget perform an undo operation.
    /// Parameters:
    ///       hHandle     -   Handle to the form fill module, as returned by
    ///                       FPDFDOC_InitFormFillEnvironment().
    ///       page        -   Handle to the page, as returned by FPDF_LoadPage().
    /// Return Value:
    ///       True if the undo operation succeeded.
    /// ```
    #[inline]
    pub fn FORM_Undo(&self, hHandle: &PdfiumForm, page: &PdfiumPage) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FORM_Undo)(hHandle.into(), page.into()) })
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
    #[inline]
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
    #[inline]
    pub fn FPDFAction_GetFilePath(
        &self,
        action: &PdfiumAction,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFAction_GetFilePath)(action.into(), to_void_ptr_mut(buffer), buflen) }
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
    #[inline]
    pub fn FPDFAction_GetType(&self, action: &PdfiumAction) -> c_ulong {
        unsafe { (self.fn_FPDFAction_GetType)(action.into()) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_AddInkStroke(
        &self,
        annot: &PdfiumAnnotation,
        points: &FS_POINTF,
        point_count: usize,
    ) -> i32 {
        unsafe { (self.fn_FPDFAnnot_AddInkStroke)(annot.into(), points, point_count) }
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
    #[inline]
    pub fn FPDFAnnot_AppendAttachmentPoints(
        &self,
        annot: &PdfiumAnnotation,
        quad_points: &FS_QUADPOINTSF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_AppendAttachmentPoints)(annot.into(), quad_points) })
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
    #[inline]
    pub fn FPDFAnnot_AppendObject(
        &self,
        annot: &PdfiumAnnotation,
        obj: &PdfiumPageObject,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_AppendObject)(annot.into(), obj.into()) })
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
    #[inline]
    pub fn FPDFAnnot_CountAttachmentPoints(&self, annot: &PdfiumAnnotation) -> usize {
        unsafe { (self.fn_FPDFAnnot_CountAttachmentPoints)(annot.into()) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_GetFileAttachment(
        &self,
        annot: &PdfiumAnnotation,
    ) -> PdfiumResult<PdfiumAttachment> {
        PdfiumAttachment::new_from_handle(unsafe {
            (self.fn_FPDFAnnot_GetFileAttachment)(annot.into())
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
    #[inline]
    pub fn FPDFAnnot_GetFlags(&self, annot: &PdfiumAnnotation) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetFlags)(annot.into()) }
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_GetFocusableSubtypesCount(&self, hHandle: &PdfiumForm) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetFocusableSubtypesCount)(hHandle.into()) }
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_GetFontSize(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
        value: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_GetFontSize)(hHandle.into(), annot.into(), value) })
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_GetFormControlIndex(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
    ) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetFormControlIndex)(hHandle.into(), annot.into()) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_GetFormFieldFlags(
        &self,
        handle: &PdfiumForm,
        annot: &PdfiumAnnotation,
    ) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetFormFieldFlags)(handle.into(), annot.into()) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_GetLine(
        &self,
        annot: &PdfiumAnnotation,
        start: &mut FS_POINTF,
        end: &mut FS_POINTF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_GetLine)(annot.into(), start, end) })
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
    #[inline]
    pub fn FPDFAnnot_GetLink(&self, annot: &PdfiumAnnotation) -> PdfiumResult<PdfiumLink> {
        PdfiumLink::new_from_handle(unsafe { (self.fn_FPDFAnnot_GetLink)(annot.into()) })
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
    #[inline]
    pub fn FPDFAnnot_GetLinkedAnnot(
        &self,
        annot: &PdfiumAnnotation,
        key: &CString,
    ) -> PdfiumResult<PdfiumAnnotation> {
        PdfiumAnnotation::new_from_handle(unsafe {
            (self.fn_FPDFAnnot_GetLinkedAnnot)(annot.into(), key.as_ptr())
        })
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
    #[inline]
    pub fn FPDFAnnot_GetNumberValue(
        &self,
        annot: &PdfiumAnnotation,
        key: &CString,
        value: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_GetNumberValue)(annot.into(), key.as_ptr(), value) })
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
    #[inline]
    pub fn FPDFAnnot_GetObject(
        &self,
        annot: &PdfiumAnnotation,
        index: i32,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDFAnnot_GetObject)(annot.into(), index)
        })
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
    #[inline]
    pub fn FPDFAnnot_GetObjectCount(&self, annot: &PdfiumAnnotation) -> i32 {
        unsafe { (self.fn_FPDFAnnot_GetObjectCount)(annot.into()) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_GetRect(
        &self,
        annot: &PdfiumAnnotation,
        rect: &mut FS_RECTF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_GetRect)(annot.into(), rect) })
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_GetSubtype(&self, annot: &PdfiumAnnotation) -> FPDF_ANNOTATION_SUBTYPE {
        unsafe { (self.fn_FPDFAnnot_GetSubtype)(annot.into()) }
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
    #[inline]
    pub fn FPDFAnnot_GetValueType(
        &self,
        annot: &PdfiumAnnotation,
        key: &CString,
    ) -> FPDF_OBJECT_TYPE {
        unsafe { (self.fn_FPDFAnnot_GetValueType)(annot.into(), key.as_ptr()) }
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
    #[inline]
    pub fn FPDFAnnot_GetVertices(
        &self,
        annot: &PdfiumAnnotation,
        buffer: &mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFAnnot_GetVertices)(annot.into(), buffer, length) }
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
    #[inline]
    pub fn FPDFAnnot_HasAttachmentPoints(&self, annot: &PdfiumAnnotation) -> i32 {
        unsafe { (self.fn_FPDFAnnot_HasAttachmentPoints)(annot.into()) }
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
    #[inline]
    pub fn FPDFAnnot_HasKey(&self, annot: &PdfiumAnnotation, key: &CString) -> i32 {
        unsafe { (self.fn_FPDFAnnot_HasKey)(annot.into(), key.as_ptr()) }
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
    #[inline]
    pub fn FPDFAnnot_IsChecked(
        &self,
        hHandle: &PdfiumForm,
        annot: &PdfiumAnnotation,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_IsChecked)(hHandle.into(), annot.into()) })
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
    #[inline]
    pub fn FPDFAnnot_IsObjectSupportedSubtype(
        &self,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_IsObjectSupportedSubtype)(subtype) })
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_IsSupportedSubtype(
        &self,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_IsSupportedSubtype)(subtype) })
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
    #[inline]
    pub fn FPDFAnnot_RemoveInkList(&self, annot: &PdfiumAnnotation) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_RemoveInkList)(annot.into()) })
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
    #[inline]
    pub fn FPDFAnnot_RemoveObject(&self, annot: &PdfiumAnnotation, index: i32) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_RemoveObject)(annot.into(), index) })
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_SetFlags(&self, annot: &PdfiumAnnotation, flags: i32) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_SetFlags)(annot.into(), flags) })
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_SetRect(&self, annot: &PdfiumAnnotation, rect: &FS_RECTF) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_SetRect)(annot.into(), rect) })
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
    #[inline]
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
    #[inline]
    pub fn FPDFAnnot_SetURI(
        &self,
        annot: &PdfiumAnnotation,
        uri: Option<&[i8]>,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_SetURI)(annot.into(), to_char_ptr(uri)) })
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
    #[inline]
    pub fn FPDFAnnot_UpdateObject(
        &self,
        annot: &PdfiumAnnotation,
        obj: &PdfiumPageObject,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFAnnot_UpdateObject)(annot.into(), obj.into()) })
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
    #[inline]
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
    #[inline]
    pub fn FPDFAttachment_GetName(
        &self,
        attachment: &PdfiumAttachment,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFAttachment_GetName)(attachment.into(), buffer.as_mut_ptr(), buflen) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFAttachment_GetValueType(
        &self,
        attachment: &PdfiumAttachment,
        key: &CString,
    ) -> FPDF_OBJECT_TYPE {
        unsafe { (self.fn_FPDFAttachment_GetValueType)(attachment.into(), key.as_ptr()) }
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
    #[inline]
    pub fn FPDFAttachment_HasKey(&self, attachment: &PdfiumAttachment, key: &CString) -> i32 {
        unsafe { (self.fn_FPDFAttachment_HasKey)(attachment.into(), key.as_ptr()) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFAvail_Create(
        &self,
        file_avail: &mut FX_FILEAVAIL,
        file: &mut Box<PdfiumReader>,
    ) -> PdfiumResult<PdfiumAvailability> {
        PdfiumAvailability::new_from_handle(unsafe {
            (self.fn_FPDFAvail_Create)(file_avail, file.as_mut().into())
        })
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
    #[inline]
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
    #[inline]
    pub fn FPDFAvail_GetFirstPageNum(&self, doc: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDFAvail_GetFirstPageNum)(doc.into()) }
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
    #[inline]
    pub fn FPDFAvail_IsDocAvail(
        &self,
        avail: &PdfiumAvailability,
        hints: &mut FX_DOWNLOADHINTS,
    ) -> i32 {
        unsafe { (self.fn_FPDFAvail_IsDocAvail)(avail.into(), hints) }
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
    #[inline]
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
    #[inline]
    pub fn FPDFAvail_IsLinearized(&self, avail: &PdfiumAvailability) -> i32 {
        unsafe { (self.fn_FPDFAvail_IsLinearized)(avail.into()) }
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
    #[inline]
    pub fn FPDFAvail_IsPageAvail(
        &self,
        avail: &PdfiumAvailability,
        page_index: i32,
        hints: &mut FX_DOWNLOADHINTS,
    ) -> i32 {
        unsafe { (self.fn_FPDFAvail_IsPageAvail)(avail.into(), page_index, hints) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFBitmap_GetFormat(&self, bitmap: &PdfiumBitmap) -> i32 {
        unsafe { (self.fn_FPDFBitmap_GetFormat)(bitmap.into()) }
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
    #[inline]
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
    #[inline]
    pub fn FPDFBitmap_GetStride(&self, bitmap: &PdfiumBitmap) -> i32 {
        unsafe { (self.fn_FPDFBitmap_GetStride)(bitmap.into()) }
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
    #[inline]
    pub fn FPDFBitmap_GetWidth(&self, bitmap: &PdfiumBitmap) -> i32 {
        unsafe { (self.fn_FPDFBitmap_GetWidth)(bitmap.into()) }
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
    #[inline]
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
    #[inline]
    pub fn FPDFBookmark_GetAction(&self, bookmark: &PdfiumBookmark) -> PdfiumResult<PdfiumAction> {
        PdfiumAction::new_from_handle(unsafe { (self.fn_FPDFBookmark_GetAction)(bookmark.into()) })
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
    #[inline]
    pub fn FPDFBookmark_GetCount(&self, bookmark: &PdfiumBookmark) -> i32 {
        unsafe { (self.fn_FPDFBookmark_GetCount)(bookmark.into()) }
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
    #[inline]
    pub fn FPDFBookmark_GetDest(
        &self,
        document: &PdfiumDocument,
        bookmark: &PdfiumBookmark,
    ) -> PdfiumResult<PdfiumDestination> {
        PdfiumDestination::new_from_handle(unsafe {
            (self.fn_FPDFBookmark_GetDest)(document.into(), bookmark.into())
        })
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFBookmark_GetTitle(
        &self,
        bookmark: &PdfiumBookmark,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFBookmark_GetTitle)(bookmark.into(), to_void_ptr_mut(buffer), buflen) }
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
    #[inline]
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
    #[inline]
    pub fn FPDFCatalog_SetLanguage(
        &self,
        document: &PdfiumDocument,
        language: &CString,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFCatalog_SetLanguage)(document.into(), language.as_ptr()) })
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
    #[inline]
    pub fn FPDFClipPath_CountPathSegments(
        &self,
        clip_path: &PdfiumClipPath,
        path_index: i32,
    ) -> i32 {
        unsafe { (self.fn_FPDFClipPath_CountPathSegments)(clip_path.into(), path_index) }
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
    #[inline]
    pub fn FPDFClipPath_CountPaths(&self, clip_path: &PdfiumClipPath) -> i32 {
        unsafe { (self.fn_FPDFClipPath_CountPaths)(clip_path.into()) }
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
    #[inline]
    pub fn FPDFClipPath_GetPathSegment(
        &self,
        clip_path: &PdfiumClipPath,
        path_index: i32,
        segment_index: i32,
    ) -> PdfiumResult<PdfiumPathSegment> {
        PdfiumPathSegment::new_from_handle(unsafe {
            (self.fn_FPDFClipPath_GetPathSegment)(clip_path.into(), path_index, segment_index)
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
    #[inline]
    pub fn FPDFDOC_ExitFormFillEnvironment(&self, hHandle: &PdfiumForm) {
        unsafe { (self.fn_FPDFDOC_ExitFormFillEnvironment)(hHandle.into()) }
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
    #[inline]
    pub fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: &PdfiumDocument,
        formInfo: &mut FPDF_FORMFILLINFO,
    ) -> PdfiumResult<PdfiumForm> {
        PdfiumForm::new_from_handle(unsafe {
            (self.fn_FPDFDOC_InitFormFillEnvironment)(document.into(), formInfo)
        })
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
    #[inline]
    pub fn FPDFDest_GetDestPageIndex(
        &self,
        document: &PdfiumDocument,
        dest: &PdfiumDestination,
    ) -> i32 {
        unsafe { (self.fn_FPDFDest_GetDestPageIndex)(document.into(), dest.into()) }
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
    #[inline]
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
    #[inline]
    pub fn FPDFDest_GetView(
        &self,
        dest: &PdfiumDestination,
        pNumParams: &mut c_ulong,
        pParams: &mut f32,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFDest_GetView)(dest.into(), pNumParams, pParams) }
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
    #[inline]
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
    #[inline]
    pub fn FPDFDoc_DeleteAttachment(
        &self,
        document: &PdfiumDocument,
        index: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFDoc_DeleteAttachment)(document.into(), index) })
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
    #[inline]
    pub fn FPDFDoc_GetAttachment(
        &self,
        document: &PdfiumDocument,
        index: i32,
    ) -> PdfiumResult<PdfiumAttachment> {
        PdfiumAttachment::new_from_handle(unsafe {
            (self.fn_FPDFDoc_GetAttachment)(document.into(), index)
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
    #[inline]
    pub fn FPDFDoc_GetAttachmentCount(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDFDoc_GetAttachmentCount)(document.into()) }
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
    #[inline]
    pub fn FPDFDoc_GetJavaScriptAction(
        &self,
        document: &PdfiumDocument,
        index: i32,
    ) -> PdfiumResult<PdfiumJavascriptAction> {
        PdfiumJavascriptAction::new_from_handle(unsafe {
            (self.fn_FPDFDoc_GetJavaScriptAction)(document.into(), index)
        })
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
    #[inline]
    pub fn FPDFDoc_GetJavaScriptActionCount(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDFDoc_GetJavaScriptActionCount)(document.into()) }
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
    #[inline]
    pub fn FPDFDoc_GetPageMode(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDFDoc_GetPageMode)(document.into()) }
    }

    /// C documentation for FPDFFont_GetAscent:
    ///
    /// ```text
    /// Experimental API.
    /// Get ascent distance of a font.
    ///
    /// font       - the handle to the font object.
    /// font_size  - the size of the |font|.
    /// ascent     - pointer where the font ascent will be stored
    ///
    /// Ascent is the maximum distance in points above the baseline reached by the
    /// glyphs of the |font|. One point is 1/72 inch (around 0.3528 mm).
    ///
    /// Returns TRUE on success; |ascent| unmodified on failure.
    /// ```
    #[inline]
    pub fn FPDFFont_GetAscent(
        &self,
        font: &PdfiumFont,
        font_size: f32,
        ascent: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFFont_GetAscent)(font.into(), font_size, ascent) })
    }

    /// C documentation for FPDFFont_GetBaseFontName:
    ///
    /// ```text
    /// Experimental API.
    /// Get the base name of a font.
    ///
    /// font   - the handle to the font object.
    /// buffer - the address of a buffer that receives the base font name.
    /// length - the size, in bytes, of |buffer|.
    ///
    /// Returns the number of bytes in the base name (including the trailing NUL
    /// character) on success, 0 on error. The base name is typically the font's
    /// PostScript name. See descriptions of "BaseFont" in ISO 32000-1:2008 spec.
    ///
    /// Regardless of the platform, the |buffer| is always in UTF-8 encoding.
    /// If |length| is less than the returned length, or |buffer| is NULL, |buffer|
    /// will not be modified.
    /// ```
    #[inline]
    pub fn FPDFFont_GetBaseFontName(
        &self,
        font: &PdfiumFont,
        buffer: Option<&mut [i8]>,
        length: usize,
    ) -> usize {
        unsafe { (self.fn_FPDFFont_GetBaseFontName)(font.into(), to_char_ptr_mut(buffer), length) }
    }

    /// C documentation for FPDFFont_GetDescent:
    ///
    /// ```text
    /// Experimental API.
    /// Get descent distance of a font.
    ///
    /// font       - the handle to the font object.
    /// font_size  - the size of the |font|.
    /// descent    - pointer where the font descent will be stored
    ///
    /// Descent is the maximum distance in points below the baseline reached by the
    /// glyphs of the |font|. One point is 1/72 inch (around 0.3528 mm).
    ///
    /// Returns TRUE on success; |descent| unmodified on failure.
    /// ```
    #[inline]
    pub fn FPDFFont_GetDescent(
        &self,
        font: &PdfiumFont,
        font_size: f32,
        descent: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFFont_GetDescent)(font.into(), font_size, descent) })
    }

    /// C documentation for FPDFFont_GetFamilyName:
    ///
    /// ```text
    /// Experimental API.
    /// Get the family name of a font.
    ///
    /// font   - the handle to the font object.
    /// buffer - the address of a buffer that receives the font name.
    /// length - the size, in bytes, of |buffer|.
    ///
    /// Returns the number of bytes in the family name (including the trailing NUL
    /// character) on success, 0 on error.
    ///
    /// Regardless of the platform, the |buffer| is always in UTF-8 encoding.
    /// If |length| is less than the returned length, or |buffer| is NULL, |buffer|
    /// will not be modified.
    /// ```
    #[inline]
    pub fn FPDFFont_GetFamilyName(
        &self,
        font: &PdfiumFont,
        buffer: Option<&mut [i8]>,
        length: usize,
    ) -> usize {
        unsafe { (self.fn_FPDFFont_GetFamilyName)(font.into(), to_char_ptr_mut(buffer), length) }
    }

    /// C documentation for FPDFFont_GetFlags:
    ///
    /// ```text
    /// Experimental API.
    /// Get the descriptor flags of a font.
    ///
    /// font - the handle to the font object.
    ///
    /// Returns the bit flags specifying various characteristics of the font as
    /// defined in ISO 32000-1:2008, table 123, -1 on failure.
    /// ```
    #[inline]
    pub fn FPDFFont_GetFlags(&self, font: &PdfiumFont) -> i32 {
        unsafe { (self.fn_FPDFFont_GetFlags)(font.into()) }
    }

    /// C documentation for FPDFFont_GetFontData:
    ///
    /// ```text
    /// Experimental API.
    /// Get the decoded data from the |font| object.
    ///
    /// font       - The handle to the font object. (Required)
    /// buffer     - The address of a buffer that receives the font data.
    /// buflen     - Length of the buffer.
    /// out_buflen - Pointer to variable that will receive the minimum buffer size
    ///              to contain the font data. Not filled if the return value is
    ///              FALSE. (Required)
    ///
    /// Returns TRUE on success. In which case, |out_buflen| will be filled, and
    /// |buffer| will be filled if it is large enough. Returns FALSE if any of the
    /// required parameters are null.
    ///
    /// The decoded data is the uncompressed font data. i.e. the raw font data after
    /// having all stream filters applied, when the data is embedded.
    ///
    /// If the font is not embedded, then this API will instead return the data for
    /// the substitution font it is using.
    /// ```
    #[inline]
    pub fn FPDFFont_GetFontData(
        &self,
        font: &PdfiumFont,
        buffer: &mut u8,
        buflen: usize,
        out_buflen: &mut usize,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFFont_GetFontData)(font.into(), buffer, buflen, out_buflen)
        })
    }

    /// C documentation for FPDFFont_GetGlyphPath:
    ///
    /// ```text
    /// Experimental API.
    /// Get the glyphpath describing how to draw a font glyph.
    ///
    /// font       - the handle to the font object.
    /// glyph      - the glyph being drawn.
    /// font_size  - the size of the font.
    ///
    /// Returns the handle to the segment, or NULL on faiure.
    /// ```
    #[inline]
    pub fn FPDFFont_GetGlyphPath(
        &self,
        font: &PdfiumFont,
        glyph: u32,
        font_size: f32,
    ) -> PdfiumResult<PdfiumGlyphPath> {
        PdfiumGlyphPath::new_from_handle(unsafe {
            (self.fn_FPDFFont_GetGlyphPath)(font.into(), glyph, font_size)
        })
    }

    /// C documentation for FPDFFont_GetGlyphWidth:
    ///
    /// ```text
    /// Experimental API.
    /// Get the width of a glyph in a font.
    ///
    /// font       - the handle to the font object.
    /// glyph      - the glyph.
    /// font_size  - the size of the font.
    /// width      - pointer where the glyph width will be stored
    ///
    /// Glyph width is the distance from the end of the prior glyph to the next
    /// glyph. This will be the vertical distance for vertical writing.
    ///
    /// Returns TRUE on success; |width| unmodified on failure.
    /// ```
    #[inline]
    pub fn FPDFFont_GetGlyphWidth(
        &self,
        font: &PdfiumFont,
        glyph: u32,
        font_size: f32,
        width: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFFont_GetGlyphWidth)(font.into(), glyph, font_size, width) })
    }

    /// C documentation for FPDFFont_GetIsEmbedded:
    ///
    /// ```text
    /// Experimental API.
    /// Get whether |font| is embedded or not.
    ///
    /// font - the handle to the font object.
    ///
    /// Returns 1 if the font is embedded, 0 if it not, and -1 on failure.
    /// ```
    #[inline]
    pub fn FPDFFont_GetIsEmbedded(&self, font: &PdfiumFont) -> i32 {
        unsafe { (self.fn_FPDFFont_GetIsEmbedded)(font.into()) }
    }

    /// C documentation for FPDFFont_GetItalicAngle:
    ///
    /// ```text
    /// Experimental API.
    /// Get the italic angle of a font.
    ///
    /// font  - the handle to the font object.
    /// angle - pointer where the italic angle will be stored
    ///
    /// The italic angle of a |font| is defined as degrees counterclockwise
    /// from vertical. For a font that slopes to the right, this will be negative.
    ///
    /// Returns TRUE on success; |angle| unmodified on failure.
    /// ```
    #[inline]
    pub fn FPDFFont_GetItalicAngle(&self, font: &PdfiumFont, angle: &mut i32) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFFont_GetItalicAngle)(font.into(), angle) })
    }

    /// C documentation for FPDFFont_GetWeight:
    ///
    /// ```text
    /// Experimental API.
    /// Get the font weight of a font.
    ///
    /// font - the handle to the font object.
    ///
    /// Returns the font weight, -1 on failure.
    /// Typical values are 400 (normal) and 700 (bold).
    /// ```
    #[inline]
    pub fn FPDFFont_GetWeight(&self, font: &PdfiumFont) -> i32 {
        unsafe { (self.fn_FPDFFont_GetWeight)(font.into()) }
    }

    /// C documentation for FPDFFormObj_CountObjects:
    ///
    /// ```text
    /// Get number of page objects inside |form_object|.
    ///
    ///   form_object - handle to a form object.
    ///
    /// Returns the number of objects in |form_object| on success, -1 on error.
    /// ```
    #[inline]
    pub fn FPDFFormObj_CountObjects(&self, form_object: &PdfiumPageObject) -> i32 {
        unsafe { (self.fn_FPDFFormObj_CountObjects)(form_object.into()) }
    }

    /// C documentation for FPDFFormObj_GetObject:
    ///
    /// ```text
    /// Get page object in |form_object| at |index|.
    ///
    ///   form_object - handle to a form object.
    ///   index       - the 0-based index of a page object.
    ///
    /// Returns the handle to the page object, or NULL on error.
    /// ```
    #[inline]
    pub fn FPDFFormObj_GetObject(
        &self,
        form_object: &PdfiumPageObject,
        index: c_ulong,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDFFormObj_GetObject)(form_object.into(), index)
        })
    }

    /// C documentation for FPDFFormObj_RemoveObject:
    ///
    /// ```text
    /// Experimental API.
    ///
    /// Remove |page_object| from |form_object|.
    ///
    ///   form_object - handle to a form object.
    ///   page_object - handle to a page object to be removed from the form.
    ///
    /// Returns TRUE on success.
    ///
    /// Ownership of the removed |page_object| is transferred to the caller.
    /// Call FPDFPageObj_Destroy() on the removed page_object to free it.
    /// ```
    #[inline]
    pub fn FPDFFormObj_RemoveObject(
        &self,
        form_object: &PdfiumPageObject,
        page_object: &PdfiumPageObject,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFFormObj_RemoveObject)(form_object.into(), page_object.into())
        })
    }

    /// C documentation for FPDFGlyphPath_CountGlyphSegments:
    ///
    /// ```text
    /// Experimental API.
    /// Get number of segments inside glyphpath.
    ///
    /// glyphpath - handle to a glyph path.
    ///
    /// Returns the number of objects in |glyphpath| or -1 on failure.
    /// ```
    #[inline]
    pub fn FPDFGlyphPath_CountGlyphSegments(&self, glyphpath: &PdfiumGlyphPath) -> i32 {
        unsafe { (self.fn_FPDFGlyphPath_CountGlyphSegments)(glyphpath.into()) }
    }

    /// C documentation for FPDFGlyphPath_GetGlyphPathSegment:
    ///
    /// ```text
    /// Experimental API.
    /// Get segment in glyphpath at index.
    ///
    /// glyphpath  - handle to a glyph path.
    /// index      - the index of a segment.
    ///
    /// Returns the handle to the segment, or NULL on faiure.
    /// ```
    #[inline]
    pub fn FPDFGlyphPath_GetGlyphPathSegment(
        &self,
        glyphpath: &PdfiumGlyphPath,
        index: i32,
    ) -> PdfiumResult<PdfiumPathSegment> {
        PdfiumPathSegment::new_from_handle(unsafe {
            (self.fn_FPDFGlyphPath_GetGlyphPathSegment)(glyphpath.into(), index)
        })
    }

    /// C documentation for FPDFImageObj_GetBitmap:
    ///
    /// ```text
    /// Get a bitmap rasterization of |image_object|. FPDFImageObj_GetBitmap() only
    /// operates on |image_object| and does not take the associated image mask into
    /// account. It also ignores the matrix for |image_object|.
    /// The returned bitmap will be owned by the caller, and FPDFBitmap_Destroy()
    /// must be called on the returned bitmap when it is no longer needed.
    ///
    ///   image_object - handle to an image object.
    ///
    /// Returns the bitmap.
    /// ```
    #[inline]
    pub fn FPDFImageObj_GetBitmap(
        &self,
        image_object: &PdfiumPageObject,
    ) -> PdfiumResult<PdfiumBitmap> {
        PdfiumBitmap::new_from_handle(unsafe {
            (self.fn_FPDFImageObj_GetBitmap)(image_object.into())
        })
    }

    /// C documentation for FPDFImageObj_GetIccProfileDataDecoded:
    ///
    /// ```text
    /// Experimental API.
    /// Get ICC profile decoded data of |image_object|. If the |image_object| is not
    /// an image object or if it does not have an image, then the return value will
    /// be false. It also returns false if the |image_object| has no ICC profile.
    /// |buffer| is only modified if ICC profile exists and |buflen| is longer than
    /// the length of the ICC profile decoded data.
    ///
    ///   image_object - handle to an image object; must not be NULL.
    ///   page         - handle to the page containing |image_object|; must not be
    ///                  NULL. Required for retrieving the image's colorspace.
    ///   buffer       - Buffer to receive ICC profile data; may be NULL if querying
    ///                  required size via |out_buflen|.
    ///   buflen       - Length of the buffer in bytes. Ignored if |buffer| is NULL.
    ///   out_buflen   - Pointer to receive the ICC profile data size in bytes; must
    ///                  not be NULL. Will be set if this API returns true.
    ///
    /// Returns true if |out_buflen| is not null and an ICC profile exists for the
    /// given |image_object|.
    /// ```
    #[inline]
    pub fn FPDFImageObj_GetIccProfileDataDecoded(
        &self,
        image_object: &PdfiumPageObject,
        page: &PdfiumPage,
        buffer: &mut u8,
        buflen: usize,
        out_buflen: &mut usize,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFImageObj_GetIccProfileDataDecoded)(
                image_object.into(),
                page.into(),
                buffer,
                buflen,
                out_buflen,
            )
        })
    }

    /// C documentation for FPDFImageObj_GetImageDataDecoded:
    ///
    /// ```text
    /// Get the decoded image data of |image_object|. The decoded data is the
    /// uncompressed image data, i.e. the raw image data after having all filters
    /// applied. |buffer| is only modified if |buflen| is longer than the length of
    /// the decoded image data.
    ///
    ///   image_object - handle to an image object.
    ///   buffer       - buffer for holding the decoded image data.
    ///   buflen       - length of the buffer in bytes.
    ///
    /// Returns the length of the decoded image data.
    /// ```
    #[inline]
    pub fn FPDFImageObj_GetImageDataDecoded(
        &self,
        image_object: &PdfiumPageObject,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFImageObj_GetImageDataDecoded)(
                image_object.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDFImageObj_GetImageDataRaw:
    ///
    /// ```text
    /// Get the raw image data of |image_object|. The raw data is the image data as
    /// stored in the PDF without applying any filters. |buffer| is only modified if
    /// |buflen| is longer than the length of the raw image data.
    ///
    ///   image_object - handle to an image object.
    ///   buffer       - buffer for holding the raw image data.
    ///   buflen       - length of the buffer in bytes.
    ///
    /// Returns the length of the raw image data.
    /// ```
    #[inline]
    pub fn FPDFImageObj_GetImageDataRaw(
        &self,
        image_object: &PdfiumPageObject,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFImageObj_GetImageDataRaw)(
                image_object.into(),
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDFImageObj_GetImageFilter:
    ///
    /// ```text
    /// Get the filter at |index| of |image_object|'s list of filters. Note that the
    /// filters need to be applied in order, i.e. the first filter should be applied
    /// first, then the second, etc. |buffer| is only modified if |buflen| is longer
    /// than the length of the filter string.
    ///
    ///   image_object - handle to an image object.
    ///   index        - the index of the filter requested.
    ///   buffer       - buffer for holding filter string, encoded in UTF-8.
    ///   buflen       - length of the buffer.
    ///
    /// Returns the length of the filter string.
    /// ```
    #[inline]
    pub fn FPDFImageObj_GetImageFilter(
        &self,
        image_object: &PdfiumPageObject,
        index: i32,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFImageObj_GetImageFilter)(
                image_object.into(),
                index,
                to_void_ptr_mut(buffer),
                buflen,
            )
        }
    }

    /// C documentation for FPDFImageObj_GetImageFilterCount:
    ///
    /// ```text
    /// Get the number of filters (i.e. decoders) of the image in |image_object|.
    ///
    ///   image_object - handle to an image object.
    ///
    /// Returns the number of |image_object|'s filters.
    /// ```
    #[inline]
    pub fn FPDFImageObj_GetImageFilterCount(&self, image_object: &PdfiumPageObject) -> i32 {
        unsafe { (self.fn_FPDFImageObj_GetImageFilterCount)(image_object.into()) }
    }

    /// C documentation for FPDFImageObj_GetImageMetadata:
    ///
    /// ```text
    /// Get the image metadata of |image_object|, including dimension, DPI, bits per
    /// pixel, and colorspace. If the |image_object| is not an image object or if it
    /// does not have an image, then the return value will be false. Otherwise,
    /// failure to retrieve any specific parameter would result in its value being 0.
    ///
    ///   image_object - handle to an image object.
    ///   page         - handle to the page that |image_object| is on. Required for
    ///                  retrieving the image's bits per pixel and colorspace.
    ///   metadata     - receives the image metadata; must not be NULL.
    ///
    /// Returns true if successful.
    /// ```
    #[inline]
    pub fn FPDFImageObj_GetImageMetadata(
        &self,
        image_object: &PdfiumPageObject,
        page: &PdfiumPage,
        metadata: &mut FPDF_IMAGEOBJ_METADATA,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFImageObj_GetImageMetadata)(image_object.into(), page.into(), metadata)
        })
    }

    /// C documentation for FPDFImageObj_GetImagePixelSize:
    ///
    /// ```text
    /// Experimental API.
    /// Get the image size in pixels. Faster method to get only image size.
    ///
    ///   image_object - handle to an image object.
    ///   width        - receives the image width in pixels; must not be NULL.
    ///   height       - receives the image height in pixels; must not be NULL.
    ///
    /// Returns true if successful.
    /// ```
    #[inline]
    pub fn FPDFImageObj_GetImagePixelSize(
        &self,
        image_object: &PdfiumPageObject,
        width: &mut u32,
        height: &mut u32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFImageObj_GetImagePixelSize)(image_object.into(), width, height)
        })
    }

    /// C documentation for FPDFImageObj_GetRenderedBitmap:
    ///
    /// ```text
    /// Experimental API.
    /// Get a bitmap rasterization of |image_object| that takes the image mask and
    /// image matrix into account. To render correctly, the caller must provide the
    /// |document| associated with |image_object|. If there is a |page| associated
    /// with |image_object|, the caller should provide that as well.
    /// The returned bitmap will be owned by the caller, and FPDFBitmap_Destroy()
    /// must be called on the returned bitmap when it is no longer needed.
    ///
    ///   document     - handle to a document associated with |image_object|.
    ///   page         - handle to an optional page associated with |image_object|.
    ///   image_object - handle to an image object.
    ///
    /// Returns the bitmap or NULL on failure.
    /// ```
    #[inline]
    pub fn FPDFImageObj_GetRenderedBitmap(
        &self,
        document: &PdfiumDocument,
        page: &PdfiumPage,
        image_object: &PdfiumPageObject,
    ) -> PdfiumResult<PdfiumBitmap> {
        PdfiumBitmap::new_from_handle(unsafe {
            (self.fn_FPDFImageObj_GetRenderedBitmap)(
                document.into(),
                page.into(),
                image_object.into(),
            )
        })
    }

    /// C documentation for FPDFImageObj_SetMatrix:
    ///
    /// ```text
    /// TODO(thestig): Start deprecating this once FPDFPageObj_SetMatrix() is stable.
    ///
    /// Set the transform matrix of |image_object|.
    ///
    ///   image_object - handle to an image object.
    ///   a            - matrix value.
    ///   b            - matrix value.
    ///   c            - matrix value.
    ///   d            - matrix value.
    ///   e            - matrix value.
    ///   f            - matrix value.
    ///
    /// The matrix is composed as:
    ///   |a c e|
    ///   |b d f|
    /// and can be used to scale, rotate, shear and translate the |image_object|.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFImageObj_SetMatrix(
        &self,
        image_object: &PdfiumPageObject,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFImageObj_SetMatrix)(image_object.into(), a, b, c, d, e, f)
        })
    }

    /// C documentation for FPDFJavaScriptAction_GetName:
    ///
    /// ```text
    /// Experimental API.
    /// Get the name from the |javascript| handle. |buffer| is only modified if
    /// |buflen| is longer than the length of the name. On errors, |buffer| is
    /// unmodified and the returned length is 0.
    ///
    ///   javascript - handle to an JavaScript action.
    ///   buffer     - buffer for holding the name, encoded in UTF-16LE.
    ///   buflen     - length of the buffer in bytes.
    ///
    /// Returns the length of the JavaScript action name in bytes.
    /// ```
    #[inline]
    pub fn FPDFJavaScriptAction_GetName(
        &self,
        javascript: &PdfiumJavascriptAction,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFJavaScriptAction_GetName)(javascript.into(), buffer.as_mut_ptr(), buflen)
        }
    }

    /// C documentation for FPDFJavaScriptAction_GetScript:
    ///
    /// ```text
    /// Experimental API.
    /// Get the script from the |javascript| handle. |buffer| is only modified if
    /// |buflen| is longer than the length of the script. On errors, |buffer| is
    /// unmodified and the returned length is 0.
    ///
    ///   javascript - handle to an JavaScript action.
    ///   buffer     - buffer for holding the name, encoded in UTF-16LE.
    ///   buflen     - length of the buffer in bytes.
    ///
    /// Returns the length of the JavaScript action name in bytes.
    /// ```
    #[inline]
    pub fn FPDFJavaScriptAction_GetScript(
        &self,
        javascript: &PdfiumJavascriptAction,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFJavaScriptAction_GetScript)(javascript.into(), buffer.as_mut_ptr(), buflen)
        }
    }

    /// C documentation for FPDFLink_CountQuadPoints:
    ///
    /// ```text
    /// Get the count of quadrilateral points to the |link_annot|.
    ///
    ///   link_annot - handle to the link annotation.
    ///
    /// Returns the count of quadrilateral points.
    /// ```
    #[inline]
    pub fn FPDFLink_CountQuadPoints(&self, link_annot: &PdfiumLink) -> i32 {
        unsafe { (self.fn_FPDFLink_CountQuadPoints)(link_annot.into()) }
    }

    /// C documentation for FPDFLink_CountRects:
    ///
    /// ```text
    /// Function: FPDFLink_CountRects
    ///          Count number of rectangular areas for the link.
    /// Parameters:
    ///          link_page   -   Handle returned by FPDFLink_LoadWebLinks.
    ///          link_index  -   Zero-based index for the link.
    /// Return Value:
    ///          Number of rectangular areas for the link.  If |link_index| does
    ///          not correspond to a valid link, then 0 is returned.
    /// ```
    #[inline]
    pub fn FPDFLink_CountRects(&self, link_page: &PdfiumPageLink, link_index: i32) -> i32 {
        unsafe { (self.fn_FPDFLink_CountRects)(link_page.into(), link_index) }
    }

    /// C documentation for FPDFLink_CountWebLinks:
    ///
    /// ```text
    /// Function: FPDFLink_CountWebLinks
    ///          Count number of detected web links.
    /// Parameters:
    ///          link_page   -   Handle returned by FPDFLink_LoadWebLinks.
    /// Return Value:
    ///          Number of detected web links.
    /// ```
    #[inline]
    pub fn FPDFLink_CountWebLinks(&self, link_page: &PdfiumPageLink) -> i32 {
        unsafe { (self.fn_FPDFLink_CountWebLinks)(link_page.into()) }
    }

    /// C documentation for FPDFLink_GetAction:
    ///
    /// ```text
    /// Get action info for |link|.
    ///
    ///   link - handle to the link.
    ///
    /// Returns a handle to the action associated to |link|, or NULL if no action.
    /// If this function returns a valid handle, it is valid as long as |link| is
    /// valid.
    /// ```
    #[inline]
    pub fn FPDFLink_GetAction(&self, link: &PdfiumLink) -> PdfiumResult<PdfiumAction> {
        PdfiumAction::new_from_handle(unsafe { (self.fn_FPDFLink_GetAction)(link.into()) })
    }

    /// C documentation for FPDFLink_GetAnnot:
    ///
    /// ```text
    /// Experimental API.
    /// Gets FPDF_ANNOTATION object for |link_annot|.
    ///
    ///   page       - handle to the page in which FPDF_LINK object is present.
    ///   link_annot - handle to link annotation.
    ///
    /// Returns FPDF_ANNOTATION from the FPDF_LINK and NULL on failure,
    /// if the input link annot or page is NULL.
    /// ```
    #[inline]
    pub fn FPDFLink_GetAnnot(
        &self,
        page: &PdfiumPage,
        link_annot: &PdfiumLink,
    ) -> PdfiumResult<PdfiumAnnotation> {
        PdfiumAnnotation::new_from_handle(unsafe {
            (self.fn_FPDFLink_GetAnnot)(page.into(), link_annot.into())
        })
    }

    /// C documentation for FPDFLink_GetAnnotRect:
    ///
    /// ```text
    /// Get the rectangle for |link_annot|.
    ///
    ///   link_annot - handle to the link annotation.
    ///   rect       - the annotation rectangle.
    ///
    /// Returns true on success.
    /// ```
    #[inline]
    pub fn FPDFLink_GetAnnotRect(
        &self,
        link_annot: &PdfiumLink,
        rect: &mut FS_RECTF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFLink_GetAnnotRect)(link_annot.into(), rect) })
    }

    /// C documentation for FPDFLink_GetDest:
    ///
    /// ```text
    /// Get destination info for |link|.
    ///
    ///   document - handle to the document.
    ///   link     - handle to the link.
    ///
    /// Returns a handle to the destination, or NULL if there is no destination
    /// associated with the link. In this case, you should call FPDFLink_GetAction()
    /// to retrieve the action associated with |link|.
    /// ```
    #[inline]
    pub fn FPDFLink_GetDest(
        &self,
        document: &PdfiumDocument,
        link: &PdfiumLink,
    ) -> PdfiumResult<PdfiumDestination> {
        PdfiumDestination::new_from_handle(unsafe {
            (self.fn_FPDFLink_GetDest)(document.into(), link.into())
        })
    }

    /// C documentation for FPDFLink_GetLinkAtPoint:
    ///
    /// ```text
    /// Find a link at point (|x|,|y|) on |page|.
    ///
    ///   page - handle to the document page.
    ///   x    - the x coordinate, in the page coordinate system.
    ///   y    - the y coordinate, in the page coordinate system.
    ///
    /// Returns a handle to the link, or NULL if no link found at the given point.
    ///
    /// You can convert coordinates from screen coordinates to page coordinates using
    /// FPDF_DeviceToPage().
    /// ```
    #[inline]
    pub fn FPDFLink_GetLinkAtPoint(
        &self,
        page: &PdfiumPage,
        x: f64,
        y: f64,
    ) -> PdfiumResult<PdfiumLink> {
        PdfiumLink::new_from_handle(unsafe { (self.fn_FPDFLink_GetLinkAtPoint)(page.into(), x, y) })
    }

    /// C documentation for FPDFLink_GetLinkZOrderAtPoint:
    ///
    /// ```text
    /// Find the Z-order of link at point (|x|,|y|) on |page|.
    ///
    ///   page - handle to the document page.
    ///   x    - the x coordinate, in the page coordinate system.
    ///   y    - the y coordinate, in the page coordinate system.
    ///
    /// Returns the Z-order of the link, or -1 if no link found at the given point.
    /// Larger Z-order numbers are closer to the front.
    ///
    /// You can convert coordinates from screen coordinates to page coordinates using
    /// FPDF_DeviceToPage().
    /// ```
    #[inline]
    pub fn FPDFLink_GetLinkZOrderAtPoint(&self, page: &PdfiumPage, x: f64, y: f64) -> i32 {
        unsafe { (self.fn_FPDFLink_GetLinkZOrderAtPoint)(page.into(), x, y) }
    }

    /// C documentation for FPDFLink_GetQuadPoints:
    ///
    /// ```text
    /// Get the quadrilateral points for the specified |quad_index| in |link_annot|.
    ///
    ///   link_annot  - handle to the link annotation.
    ///   quad_index  - the specified quad point index.
    ///   quad_points - receives the quadrilateral points.
    ///
    /// Returns true on success.
    /// ```
    #[inline]
    pub fn FPDFLink_GetQuadPoints(
        &self,
        link_annot: &PdfiumLink,
        quad_index: i32,
        quad_points: &mut FS_QUADPOINTSF,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFLink_GetQuadPoints)(link_annot.into(), quad_index, quad_points)
        })
    }

    /// C documentation for FPDFLink_GetRect:
    ///
    /// ```text
    /// Function: FPDFLink_GetRect
    ///          Fetch the boundaries of a rectangle for a link.
    /// Parameters:
    ///          link_page   -   Handle returned by FPDFLink_LoadWebLinks.
    ///          link_index  -   Zero-based index for the link.
    ///          rect_index  -   Zero-based index for a rectangle.
    ///          left        -   Pointer to a double value receiving the rectangle
    ///                          left boundary.
    ///          top         -   Pointer to a double value receiving the rectangle
    ///                          top boundary.
    ///          right       -   Pointer to a double value receiving the rectangle
    ///                          right boundary.
    ///          bottom      -   Pointer to a double value receiving the rectangle
    ///                          bottom boundary.
    /// Return Value:
    ///          On success, return TRUE and fill in |left|, |top|, |right|, and
    ///          |bottom|. If |link_page| is invalid or if |link_index| does not
    ///          correspond to a valid link, then return FALSE, and the out
    ///          parameters remain unmodified.
    /// ```
    #[inline]
    pub fn FPDFLink_GetRect(
        &self,
        link_page: &PdfiumPageLink,
        link_index: i32,
        rect_index: i32,
        left: &mut f64,
        top: &mut f64,
        right: &mut f64,
        bottom: &mut f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFLink_GetRect)(
                link_page.into(),
                link_index,
                rect_index,
                left,
                top,
                right,
                bottom,
            )
        })
    }

    /// C documentation for FPDFLink_GetTextRange:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFLink_GetTextRange
    ///          Fetch the start char index and char count for a link.
    /// Parameters:
    ///          link_page         -   Handle returned by FPDFLink_LoadWebLinks.
    ///          link_index        -   Zero-based index for the link.
    ///          start_char_index  -   pointer to int receiving the start char index
    ///          char_count        -   pointer to int receiving the char count
    /// Return Value:
    ///          On success, return TRUE and fill in |start_char_index| and
    ///          |char_count|. if |link_page| is invalid or if |link_index| does
    ///          not correspond to a valid link, then return FALSE and the out
    ///          parameters remain unmodified.
    /// ```
    #[inline]
    pub fn FPDFLink_GetTextRange(
        &self,
        link_page: &PdfiumPageLink,
        link_index: i32,
        start_char_index: &mut i32,
        char_count: &mut i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFLink_GetTextRange)(
                link_page.into(),
                link_index,
                start_char_index,
                char_count,
            )
        })
    }

    /// C documentation for FPDFLink_GetURL:
    ///
    /// ```text
    /// Function: FPDFLink_GetURL
    ///          Fetch the URL information for a detected web link.
    /// Parameters:
    ///          link_page   -   Handle returned by FPDFLink_LoadWebLinks.
    ///          link_index  -   Zero-based index for the link.
    ///          buffer      -   A unicode buffer for the result.
    ///          buflen      -   Number of 16-bit code units (not bytes) for the
    ///                          buffer, including an additional terminator.
    /// Return Value:
    ///          If |buffer| is NULL or |buflen| is zero, return the number of 16-bit
    ///          code units (not bytes) needed to buffer the result (an additional
    ///          terminator is included in this count).
    ///          Otherwise, copy the result into |buffer|, truncating at |buflen| if
    ///          the result is too large to fit, and return the number of 16-bit code
    ///          units actually copied into the buffer (the additional terminator is
    ///          also included in this count).
    ///          If |link_index| does not correspond to a valid link, then the result
    ///          is an empty string.
    /// ```
    #[inline]
    pub fn FPDFLink_GetURL(
        &self,
        link_page: &PdfiumPageLink,
        link_index: i32,
        buffer: &mut c_ushort,
        buflen: i32,
    ) -> i32 {
        unsafe { (self.fn_FPDFLink_GetURL)(link_page.into(), link_index, buffer, buflen) }
    }

    /// C documentation for FPDFLink_LoadWebLinks:
    ///
    /// ```text
    /// Function: FPDFLink_LoadWebLinks
    ///          Prepare information about weblinks in a page.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    /// Return Value:
    ///          A handle to the page's links information structure, or
    ///          NULL if something goes wrong.
    /// Comments:
    ///          Weblinks are those links implicitly embedded in PDF pages. PDF also
    ///          has a type of annotation called "link" (FPDFTEXT doesn't deal with
    ///          that kind of link). FPDFTEXT weblink feature is useful for
    ///          automatically detecting links in the page contents. For example,
    ///          things like "https://www.example.com" will be detected, so
    ///          applications can allow user to click on those characters to activate
    ///          the link, even the PDF doesn't come with link annotations.
    ///
    ///          FPDFLink_CloseWebLinks must be called to release resources.
    /// ```
    #[inline]
    pub fn FPDFLink_LoadWebLinks(
        &self,
        text_page: &PdfiumTextPage,
    ) -> PdfiumResult<PdfiumPageLink> {
        PdfiumPageLink::new_from_handle(unsafe {
            (self.fn_FPDFLink_LoadWebLinks)(text_page.into())
        })
    }

    /// C documentation for FPDFPageObjMark_CountParams:
    ///
    /// ```text
    /// Experimental API.
    /// Get the number of key/value pair parameters in |mark|.
    ///
    ///   mark   - handle to a content mark.
    ///
    /// Returns the number of key/value pair parameters |mark|, or -1 in case of
    /// failure.
    /// ```
    #[inline]
    pub fn FPDFPageObjMark_CountParams(&self, mark: &PdfiumPageObjectMark) -> i32 {
        unsafe { (self.fn_FPDFPageObjMark_CountParams)(mark.into()) }
    }

    /// C documentation for FPDFPageObjMark_GetName:
    ///
    /// ```text
    /// Experimental API.
    /// Get the name of a content mark.
    ///
    ///   mark       - handle to a content mark.
    ///   buffer     - buffer for holding the returned name in UTF-16LE. This is only
    ///                modified if |buflen| is large enough to store the name.
    ///                Optional, pass null to just retrieve the size of the buffer
    ///                needed.
    ///   buflen     - length of the buffer in bytes.
    ///   out_buflen - pointer to variable that will receive the minimum buffer size
    ///                in bytes to contain the name. This is a required parameter.
    ///                Not filled if FALSE is returned.
    ///
    /// Returns TRUE if the operation succeeded, FALSE if it failed.
    /// ```
    #[inline]
    pub fn FPDFPageObjMark_GetName(
        &self,
        mark: &PdfiumPageObjectMark,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
        out_buflen: &mut c_ulong,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObjMark_GetName)(mark.into(), buffer.as_mut_ptr(), buflen, out_buflen)
        })
    }

    /// C documentation for FPDFPageObjMark_GetParamBlobValue:
    ///
    /// ```text
    /// Experimental API.
    /// Get the value of a blob property in a content mark by key.
    ///
    ///   mark       - handle to a content mark.
    ///   key        - string key of the property.
    ///   buffer     - buffer for holding the returned value. This is only modified
    ///                if |buflen| is large enough to store the value.
    ///                Optional, pass null to just retrieve the size of the buffer
    ///                needed.
    ///   buflen     - length of the buffer in bytes.
    ///   out_buflen - pointer to variable that will receive the minimum buffer size
    ///                in bytes to contain the name. This is a required parameter.
    ///                Not filled if FALSE is returned.
    ///
    /// Returns TRUE if the key maps to a string/blob value, FALSE otherwise.
    /// ```
    #[inline]
    pub fn FPDFPageObjMark_GetParamBlobValue(
        &self,
        mark: &PdfiumPageObjectMark,
        key: &CString,
        buffer: &mut [u8],
        buflen: c_ulong,
        out_buflen: &mut c_ulong,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObjMark_GetParamBlobValue)(
                mark.into(),
                key.as_ptr(),
                buffer.as_mut_ptr(),
                buflen,
                out_buflen,
            )
        })
    }

    /// C documentation for FPDFPageObjMark_GetParamIntValue:
    ///
    /// ```text
    /// Experimental API.
    /// Get the value of a number property in a content mark by key as int.
    /// FPDFPageObjMark_GetParamValueType() should have returned FPDF_OBJECT_NUMBER
    /// for this property.
    ///
    ///   mark      - handle to a content mark.
    ///   key       - string key of the property.
    ///   out_value - pointer to variable that will receive the value. Not filled if
    ///               false is returned.
    ///
    /// Returns TRUE if the key maps to a number value, FALSE otherwise.
    /// ```
    #[inline]
    pub fn FPDFPageObjMark_GetParamIntValue(
        &self,
        mark: &PdfiumPageObjectMark,
        key: &CString,
        out_value: &mut i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObjMark_GetParamIntValue)(mark.into(), key.as_ptr(), out_value)
        })
    }

    /// C documentation for FPDFPageObjMark_GetParamKey:
    ///
    /// ```text
    /// Experimental API.
    /// Get the key of a property in a content mark.
    ///
    ///   mark       - handle to a content mark.
    ///   index      - index of the property.
    ///   buffer     - buffer for holding the returned key in UTF-16LE. This is only
    ///                modified if |buflen| is large enough to store the key.
    ///                Optional, pass null to just retrieve the size of the buffer
    ///                needed.
    ///   buflen     - length of the buffer in bytes.
    ///   out_buflen - pointer to variable that will receive the minimum buffer size
    ///                in bytes to contain the name. This is a required parameter.
    ///                Not filled if FALSE is returned.
    ///
    /// Returns TRUE if the operation was successful, FALSE otherwise.
    /// ```
    #[inline]
    pub fn FPDFPageObjMark_GetParamKey(
        &self,
        mark: &PdfiumPageObjectMark,
        index: c_ulong,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
        out_buflen: &mut c_ulong,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObjMark_GetParamKey)(
                mark.into(),
                index,
                buffer.as_mut_ptr(),
                buflen,
                out_buflen,
            )
        })
    }

    /// C documentation for FPDFPageObjMark_GetParamStringValue:
    ///
    /// ```text
    /// Experimental API.
    /// Get the value of a string property in a content mark by key.
    ///
    ///   mark       - handle to a content mark.
    ///   key        - string key of the property.
    ///   buffer     - buffer for holding the returned value in UTF-16LE. This is
    ///                only modified if |buflen| is large enough to store the value.
    ///                Optional, pass null to just retrieve the size of the buffer
    ///                needed.
    ///   buflen     - length of the buffer in bytes.
    ///   out_buflen - pointer to variable that will receive the minimum buffer size
    ///                in bytes to contain the name. This is a required parameter.
    ///                Not filled if FALSE is returned.
    ///
    /// Returns TRUE if the key maps to a string/blob value, FALSE otherwise.
    /// ```
    #[inline]
    pub fn FPDFPageObjMark_GetParamStringValue(
        &self,
        mark: &PdfiumPageObjectMark,
        key: &CString,
        buffer: &mut Vec<u16>,
        buflen: c_ulong,
        out_buflen: &mut c_ulong,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObjMark_GetParamStringValue)(
                mark.into(),
                key.as_ptr(),
                buffer.as_mut_ptr(),
                buflen,
                out_buflen,
            )
        })
    }

    /// C documentation for FPDFPageObjMark_GetParamValueType:
    ///
    /// ```text
    /// Experimental API.
    /// Get the type of the value of a property in a content mark by key.
    ///
    ///   mark   - handle to a content mark.
    ///   key    - string key of the property.
    ///
    /// Returns the type of the value, or FPDF_OBJECT_UNKNOWN in case of failure.
    /// ```
    #[inline]
    pub fn FPDFPageObjMark_GetParamValueType(
        &self,
        mark: &PdfiumPageObjectMark,
        key: &CString,
    ) -> FPDF_OBJECT_TYPE {
        unsafe { (self.fn_FPDFPageObjMark_GetParamValueType)(mark.into(), key.as_ptr()) }
    }

    /// C documentation for FPDFPageObjMark_RemoveParam:
    ///
    /// ```text
    /// Experimental API.
    /// Removes a property from a content mark by key.
    ///
    ///   page_object - handle to the page object with the mark.
    ///   mark        - handle to a content mark.
    ///   key         - string key of the property.
    ///
    /// Returns TRUE if the operation succeeded, FALSE otherwise.
    /// ```
    #[inline]
    pub fn FPDFPageObjMark_RemoveParam(
        &self,
        page_object: &PdfiumPageObject,
        mark: &PdfiumPageObjectMark,
        key: &CString,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObjMark_RemoveParam)(page_object.into(), mark.into(), key.as_ptr())
        })
    }

    /// C documentation for FPDFPageObjMark_SetBlobParam:
    ///
    /// ```text
    /// Experimental API.
    /// Set the value of a blob property in a content mark by key. If a parameter
    /// with key |key| exists, its value is set to |value|. Otherwise, it is added as
    /// a new parameter.
    ///
    ///   document    - handle to the document.
    ///   page_object - handle to the page object with the mark.
    ///   mark        - handle to a content mark.
    ///   key         - string key of the property.
    ///   value       - pointer to blob value to set.
    ///   value_len   - size in bytes of |value|.
    ///
    /// Returns TRUE if the operation succeeded, FALSE otherwise.
    /// ```
    #[inline]
    pub fn FPDFPageObjMark_SetBlobParam(
        &self,
        document: &PdfiumDocument,
        page_object: &PdfiumPageObject,
        mark: &PdfiumPageObjectMark,
        key: &CString,
        value: &[u8],
        value_len: c_ulong,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObjMark_SetBlobParam)(
                document.into(),
                page_object.into(),
                mark.into(),
                key.as_ptr(),
                value.as_ptr(),
                value_len,
            )
        })
    }

    /// C documentation for FPDFPageObjMark_SetIntParam:
    ///
    /// ```text
    /// Experimental API.
    /// Set the value of an int property in a content mark by key. If a parameter
    /// with key |key| exists, its value is set to |value|. Otherwise, it is added as
    /// a new parameter.
    ///
    ///   document    - handle to the document.
    ///   page_object - handle to the page object with the mark.
    ///   mark        - handle to a content mark.
    ///   key         - string key of the property.
    ///   value       - int value to set.
    ///
    /// Returns TRUE if the operation succeeded, FALSE otherwise.
    /// ```
    #[inline]
    pub fn FPDFPageObjMark_SetIntParam(
        &self,
        document: &PdfiumDocument,
        page_object: &PdfiumPageObject,
        mark: &PdfiumPageObjectMark,
        key: &CString,
        value: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObjMark_SetIntParam)(
                document.into(),
                page_object.into(),
                mark.into(),
                key.as_ptr(),
                value,
            )
        })
    }

    /// C documentation for FPDFPageObjMark_SetStringParam:
    ///
    /// ```text
    /// Experimental API.
    /// Set the value of a string property in a content mark by key. If a parameter
    /// with key |key| exists, its value is set to |value|. Otherwise, it is added as
    /// a new parameter.
    ///
    ///   document    - handle to the document.
    ///   page_object - handle to the page object with the mark.
    ///   mark        - handle to a content mark.
    ///   key         - string key of the property.
    ///   value       - string value to set.
    ///
    /// Returns TRUE if the operation succeeded, FALSE otherwise.
    /// ```
    #[inline]
    pub fn FPDFPageObjMark_SetStringParam(
        &self,
        document: &PdfiumDocument,
        page_object: &PdfiumPageObject,
        mark: &PdfiumPageObjectMark,
        key: &CString,
        value: &CString,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObjMark_SetStringParam)(
                document.into(),
                page_object.into(),
                mark.into(),
                key.as_ptr(),
                value.as_ptr(),
            )
        })
    }

    /// C documentation for FPDFPageObj_AddMark:
    ///
    /// ```text
    /// Experimental API.
    /// Add a new content mark to a |page_object|.
    ///
    ///   page_object - handle to a page object.
    ///   name        - the name (tag) of the mark.
    ///
    /// Returns the handle to the content mark, or NULL on failure. The handle is
    /// still owned by the library, and it should not be freed directly. It becomes
    /// invalid if the page object is destroyed, either directly or indirectly by
    /// unloading the page.
    /// ```
    #[inline]
    pub fn FPDFPageObj_AddMark(
        &self,
        page_object: &PdfiumPageObject,
        name: &CString,
    ) -> PdfiumResult<PdfiumPageObjectMark> {
        PdfiumPageObjectMark::new_from_handle(unsafe {
            (self.fn_FPDFPageObj_AddMark)(page_object.into(), name.as_ptr())
        })
    }

    /// C documentation for FPDFPageObj_CountMarks:
    ///
    /// ```text
    /// Experimental API.
    /// Get number of content marks in |page_object|.
    ///
    ///   page_object - handle to a page object.
    ///
    /// Returns the number of content marks in |page_object|, or -1 in case of
    /// failure.
    /// ```
    #[inline]
    pub fn FPDFPageObj_CountMarks(&self, page_object: &PdfiumPageObject) -> i32 {
        unsafe { (self.fn_FPDFPageObj_CountMarks)(page_object.into()) }
    }

    /// C documentation for FPDFPageObj_CreateNewPath:
    ///
    /// ```text
    /// Create a new path object at an initial position.
    ///
    ///   x - initial horizontal position.
    ///   y - initial vertical position.
    ///
    /// Returns a handle to a new path object.
    /// ```
    #[inline]
    pub fn FPDFPageObj_CreateNewPath(&self, x: f32, y: f32) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe { (self.fn_FPDFPageObj_CreateNewPath)(x, y) })
    }

    /// C documentation for FPDFPageObj_CreateNewRect:
    ///
    /// ```text
    /// Create a closed path consisting of a rectangle.
    ///
    ///   x - horizontal position for the left boundary of the rectangle.
    ///   y - vertical position for the bottom boundary of the rectangle.
    ///   w - width of the rectangle.
    ///   h - height of the rectangle.
    ///
    /// Returns a handle to the new path object.
    /// ```
    #[inline]
    pub fn FPDFPageObj_CreateNewRect(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDFPageObj_CreateNewRect)(x, y, w, h)
        })
    }

    /// C documentation for FPDFPageObj_CreateTextObj:
    ///
    /// ```text
    /// Create a new text object using a loaded font.
    ///
    /// document   - handle to the document.
    /// font       - handle to the font object.
    /// font_size  - the font size for the new text object.
    ///
    /// Returns a handle to a new text object, or NULL on failure
    /// ```
    #[inline]
    pub fn FPDFPageObj_CreateTextObj(
        &self,
        document: &PdfiumDocument,
        font: &PdfiumFont,
        font_size: f32,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDFPageObj_CreateTextObj)(document.into(), font.into(), font_size)
        })
    }

    /// C documentation for FPDFPageObj_GetBounds:
    ///
    /// ```text
    /// Get the bounding box of |page_object|.
    ///
    /// page_object  - handle to a page object.
    /// left         - pointer where the left coordinate will be stored
    /// bottom       - pointer where the bottom coordinate will be stored
    /// right        - pointer where the right coordinate will be stored
    /// top          - pointer where the top coordinate will be stored
    ///
    /// On success, returns TRUE and fills in the 4 coordinates.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetBounds(
        &self,
        page_object: &PdfiumPageObject,
        left: &mut f32,
        bottom: &mut f32,
        right: &mut f32,
        top: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObj_GetBounds)(page_object.into(), left, bottom, right, top)
        })
    }

    /// C documentation for FPDFPageObj_GetClipPath:
    ///
    /// ```text
    /// Experimental API.
    /// Get the clip path of the page object.
    ///
    ///   page object - Handle to a page object. Returned by e.g.
    ///                 FPDFPage_GetObject().
    ///
    /// Returns the handle to the clip path, or NULL on failure. The caller does not
    /// take ownership of the returned FPDF_CLIPPATH. Instead, it remains valid until
    /// FPDF_ClosePage() is called for the page containing |page_object|.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetClipPath(
        &self,
        page_object: &PdfiumPageObject,
    ) -> PdfiumResult<PdfiumClipPath> {
        PdfiumClipPath::new_from_handle(unsafe {
            (self.fn_FPDFPageObj_GetClipPath)(page_object.into())
        })
    }

    /// C documentation for FPDFPageObj_GetDashArray:
    ///
    /// ```text
    /// Experimental API.
    /// Get the line dash array of |page_object|.
    ///
    /// page_object - handle to a page object.
    /// dash_array - pointer where the dashing array will be stored.
    /// dash_count - number of elements in |dash_array|.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetDashArray(
        &self,
        page_object: &PdfiumPageObject,
        dash_array: &mut f32,
        dash_count: usize,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObj_GetDashArray)(page_object.into(), dash_array, dash_count)
        })
    }

    /// C documentation for FPDFPageObj_GetDashCount:
    ///
    /// ```text
    /// Experimental API.
    /// Get the line dash array of |page_object|.
    ///
    /// page_object - handle to a page object.
    ///
    /// Returns the line dash array size or -1 on failure.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetDashCount(&self, page_object: &PdfiumPageObject) -> i32 {
        unsafe { (self.fn_FPDFPageObj_GetDashCount)(page_object.into()) }
    }

    /// C documentation for FPDFPageObj_GetDashPhase:
    ///
    /// ```text
    /// Experimental API.
    /// Get the line dash |phase| of |page_object|.
    ///
    /// page_object - handle to a page object.
    /// phase - pointer where the dashing phase will be stored.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetDashPhase(
        &self,
        page_object: &PdfiumPageObject,
        phase: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_GetDashPhase)(page_object.into(), phase) })
    }

    /// C documentation for FPDFPageObj_GetFillColor:
    ///
    /// ```text
    /// Get the fill RGBA of a page object. Range of values: 0 - 255.
    ///
    /// page_object  - the handle to the page object.
    /// R            - the red component of the object's fill color.
    /// G            - the green component of the object's fill color.
    /// B            - the blue component of the object's fill color.
    /// A            - the fill alpha of the object.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetFillColor(
        &self,
        page_object: &PdfiumPageObject,
        R: &mut u32,
        G: &mut u32,
        B: &mut u32,
        A: &mut u32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_GetFillColor)(page_object.into(), R, G, B, A) })
    }

    /// C documentation for FPDFPageObj_GetIsActive:
    ///
    /// ```text
    /// Experimental API.
    /// Gets active state for |page_object| within page.
    ///
    ///   page_object - handle to a page object.
    ///   active      - pointer to variable that will receive if the page object is
    ///                 active. This is a required parameter. Not filled if FALSE
    ///                 is returned.
    ///
    /// For page objects where |active| is filled with FALSE, the |page_object| is
    /// treated as if it wasn't in the document even though it is still held
    /// internally.
    ///
    /// Returns TRUE if the operation succeeded, FALSE if it failed.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetIsActive(
        &self,
        page_object: &PdfiumPageObject,
        active: &mut FPDF_BOOL,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_GetIsActive)(page_object.into(), active) })
    }

    /// C documentation for FPDFPageObj_GetLineCap:
    ///
    /// ```text
    /// Get the line cap of |page_object|.
    ///
    /// page_object - handle to a page object.
    ///
    /// Returns the line cap, or -1 on failure.
    /// Line cap can be one of following: FPDF_LINECAP_BUTT, FPDF_LINECAP_ROUND,
    /// FPDF_LINECAP_PROJECTING_SQUARE
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetLineCap(&self, page_object: &PdfiumPageObject) -> i32 {
        unsafe { (self.fn_FPDFPageObj_GetLineCap)(page_object.into()) }
    }

    /// C documentation for FPDFPageObj_GetLineJoin:
    ///
    /// ```text
    /// Get the line join of |page_object|.
    ///
    /// page_object  - handle to a page object.
    ///
    /// Returns the line join, or -1 on failure.
    /// Line join can be one of following: FPDF_LINEJOIN_MITER, FPDF_LINEJOIN_ROUND,
    /// FPDF_LINEJOIN_BEVEL
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetLineJoin(&self, page_object: &PdfiumPageObject) -> i32 {
        unsafe { (self.fn_FPDFPageObj_GetLineJoin)(page_object.into()) }
    }

    /// C documentation for FPDFPageObj_GetMark:
    ///
    /// ```text
    /// Experimental API.
    /// Get content mark in |page_object| at |index|.
    ///
    ///   page_object - handle to a page object.
    ///   index       - the index of a page object.
    ///
    /// Returns the handle to the content mark, or NULL on failure. The handle is
    /// still owned by the library, and it should not be freed directly. It becomes
    /// invalid if the page object is destroyed, either directly or indirectly by
    /// unloading the page.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetMark(
        &self,
        page_object: &PdfiumPageObject,
        index: c_ulong,
    ) -> PdfiumResult<PdfiumPageObjectMark> {
        PdfiumPageObjectMark::new_from_handle(unsafe {
            (self.fn_FPDFPageObj_GetMark)(page_object.into(), index)
        })
    }

    /// C documentation for FPDFPageObj_GetMarkedContentID:
    ///
    /// ```text
    /// Experimental API.
    /// Get the marked content ID for the object.
    ///
    ///   page_object - handle to a page object.
    ///
    /// Returns the page object's marked content ID, or -1 on error.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetMarkedContentID(&self, page_object: &PdfiumPageObject) -> i32 {
        unsafe { (self.fn_FPDFPageObj_GetMarkedContentID)(page_object.into()) }
    }

    /// C documentation for FPDFPageObj_GetMatrix:
    ///
    /// ```text
    /// Experimental API.
    /// Get the transform matrix of a page object.
    ///
    ///   page_object - handle to a page object.
    ///   matrix      - pointer to struct to receive the matrix value.
    ///
    /// The matrix is composed as:
    ///   |a c e|
    ///   |b d f|
    /// and used to scale, rotate, shear and translate the page object.
    ///
    /// For page objects outside form objects, the matrix values are relative to the
    /// page that contains it.
    /// For page objects inside form objects, the matrix values are relative to the
    /// form that contains it.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetMatrix(
        &self,
        page_object: &PdfiumPageObject,
        matrix: &mut FS_MATRIX,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_GetMatrix)(page_object.into(), matrix) })
    }

    /// C documentation for FPDFPageObj_GetRotatedBounds:
    ///
    /// ```text
    /// Experimental API.
    /// Get the quad points that bounds |page_object|.
    ///
    /// page_object  - handle to a page object.
    /// quad_points  - pointer where the quadrilateral points will be stored.
    ///
    /// On success, returns TRUE and fills in |quad_points|.
    ///
    /// Similar to FPDFPageObj_GetBounds(), this returns the bounds of a page
    /// object. When the object is rotated by a non-multiple of 90 degrees, this API
    /// returns a tighter bound that cannot be represented with just the 4 sides of
    /// a rectangle.
    ///
    /// Currently only works the following |page_object| types: FPDF_PAGEOBJ_TEXT and
    /// FPDF_PAGEOBJ_IMAGE.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetRotatedBounds(
        &self,
        page_object: &PdfiumPageObject,
        quad_points: &mut FS_QUADPOINTSF,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObj_GetRotatedBounds)(page_object.into(), quad_points)
        })
    }

    /// C documentation for FPDFPageObj_GetStrokeColor:
    ///
    /// ```text
    /// Get the stroke RGBA of a page object. Range of values: 0 - 255.
    ///
    /// page_object  - the handle to the page object.
    /// R            - the red component of the path stroke color.
    /// G            - the green component of the object's stroke color.
    /// B            - the blue component of the object's stroke color.
    /// A            - the stroke alpha of the object.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetStrokeColor(
        &self,
        page_object: &PdfiumPageObject,
        R: &mut u32,
        G: &mut u32,
        B: &mut u32,
        A: &mut u32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_GetStrokeColor)(page_object.into(), R, G, B, A) })
    }

    /// C documentation for FPDFPageObj_GetStrokeWidth:
    ///
    /// ```text
    /// Get the stroke width of a page object.
    ///
    /// path   - the handle to the page object.
    /// width  - the width of the stroke.
    ///
    /// Returns TRUE on success
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetStrokeWidth(
        &self,
        page_object: &PdfiumPageObject,
        width: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_GetStrokeWidth)(page_object.into(), width) })
    }

    /// C documentation for FPDFPageObj_GetType:
    ///
    /// ```text
    /// Get type of |page_object|.
    ///
    ///   page_object - handle to a page object.
    ///
    /// Returns one of the FPDF_PAGEOBJ_* values on success, FPDF_PAGEOBJ_UNKNOWN on
    /// error.
    /// ```
    #[inline]
    pub fn FPDFPageObj_GetType(&self, page_object: &PdfiumPageObject) -> i32 {
        unsafe { (self.fn_FPDFPageObj_GetType)(page_object.into()) }
    }

    /// C documentation for FPDFPageObj_HasTransparency:
    ///
    /// ```text
    /// Checks if |page_object| contains transparency.
    ///
    ///   page_object - handle to a page object.
    ///
    /// Returns TRUE if |page_object| contains transparency.
    /// ```
    #[inline]
    pub fn FPDFPageObj_HasTransparency(&self, page_object: &PdfiumPageObject) -> i32 {
        unsafe { (self.fn_FPDFPageObj_HasTransparency)(page_object.into()) }
    }

    /// C documentation for FPDFPageObj_NewImageObj:
    ///
    /// ```text
    /// Create a new image object.
    ///
    ///   document - handle to a document.
    ///
    /// Returns a handle to a new image object.
    /// ```
    #[inline]
    pub fn FPDFPageObj_NewImageObj(
        &self,
        document: &PdfiumDocument,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDFPageObj_NewImageObj)(document.into())
        })
    }

    /// C documentation for FPDFPageObj_NewTextObj:
    ///
    /// ```text
    /// Create a new text object using one of the standard PDF fonts.
    ///
    /// document   - handle to the document.
    /// font       - string containing the font name, without spaces.
    /// font_size  - the font size for the new text object.
    ///
    /// Returns a handle to a new text object, or NULL on failure
    /// ```
    #[inline]
    pub fn FPDFPageObj_NewTextObj(
        &self,
        document: &PdfiumDocument,
        font: &CString,
        font_size: f32,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDFPageObj_NewTextObj)(document.into(), font.as_ptr(), font_size)
        })
    }

    /// C documentation for FPDFPageObj_RemoveMark:
    ///
    /// ```text
    /// Experimental API.
    /// Removes a content |mark| from a |page_object|.
    /// The mark handle will be invalid after the removal.
    ///
    ///   page_object - handle to a page object.
    ///   mark        - handle to a content mark in that object to remove.
    ///
    /// Returns TRUE if the operation succeeded, FALSE if it failed.
    /// ```
    #[inline]
    pub fn FPDFPageObj_RemoveMark(
        &self,
        page_object: &PdfiumPageObject,
        mark: &PdfiumPageObjectMark,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_RemoveMark)(page_object.into(), mark.into()) })
    }

    /// C documentation for FPDFPageObj_SetBlendMode:
    ///
    /// ```text
    /// Set the blend mode of |page_object|.
    ///
    /// page_object  - handle to a page object.
    /// blend_mode   - string containing the blend mode.
    ///
    /// Blend mode can be one of following: Color, ColorBurn, ColorDodge, Darken,
    /// Difference, Exclusion, HardLight, Hue, Lighten, Luminosity, Multiply, Normal,
    /// Overlay, Saturation, Screen, SoftLight
    /// ```
    #[inline]
    pub fn FPDFPageObj_SetBlendMode(&self, page_object: &PdfiumPageObject, blend_mode: &CString) {
        unsafe { (self.fn_FPDFPageObj_SetBlendMode)(page_object.into(), blend_mode.as_ptr()) }
    }

    /// C documentation for FPDFPageObj_SetDashArray:
    ///
    /// ```text
    /// Experimental API.
    /// Set the line dash array of |page_object|.
    ///
    /// page_object - handle to a page object.
    /// dash_array - the dash array.
    /// dash_count - number of elements in |dash_array|.
    /// phase - the line dash phase.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFPageObj_SetDashArray(
        &self,
        page_object: &PdfiumPageObject,
        dash_array: &[f32],
        dash_count: usize,
        phase: f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFPageObj_SetDashArray)(
                page_object.into(),
                dash_array.as_ptr(),
                dash_count,
                phase,
            )
        })
    }

    /// C documentation for FPDFPageObj_SetDashPhase:
    ///
    /// ```text
    /// Experimental API.
    /// Set the line dash phase of |page_object|.
    ///
    /// page_object - handle to a page object.
    /// phase - line dash phase.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFPageObj_SetDashPhase(
        &self,
        page_object: &PdfiumPageObject,
        phase: f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_SetDashPhase)(page_object.into(), phase) })
    }

    /// C documentation for FPDFPageObj_SetFillColor:
    ///
    /// ```text
    /// Set the fill RGBA of a page object. Range of values: 0 - 255.
    ///
    /// page_object  - the handle to the page object.
    /// R            - the red component for the object's fill color.
    /// G            - the green component for the object's fill color.
    /// B            - the blue component for the object's fill color.
    /// A            - the fill alpha for the object.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFPageObj_SetFillColor(
        &self,
        page_object: &PdfiumPageObject,
        R: u32,
        G: u32,
        B: u32,
        A: u32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_SetFillColor)(page_object.into(), R, G, B, A) })
    }

    /// C documentation for FPDFPageObj_SetIsActive:
    ///
    /// ```text
    /// Experimental API.
    /// Sets if |page_object| is active within page.
    ///
    ///   page_object - handle to a page object.
    ///   active      - a boolean specifying if the object is active.
    ///
    /// Returns TRUE on success.
    ///
    /// Page objects all start in the active state by default, and remain in that
    /// state unless this function is called.
    ///
    /// When |active| is false, this makes the |page_object| be treated as if it
    /// wasn't in the document even though it is still held internally.
    /// ```
    #[inline]
    pub fn FPDFPageObj_SetIsActive(
        &self,
        page_object: &PdfiumPageObject,
        active: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_SetIsActive)(page_object.into(), active) })
    }

    /// C documentation for FPDFPageObj_SetLineCap:
    ///
    /// ```text
    /// Set the line cap of |page_object|.
    ///
    /// page_object - handle to a page object.
    /// line_cap    - line cap
    ///
    /// Line cap can be one of following: FPDF_LINECAP_BUTT, FPDF_LINECAP_ROUND,
    /// FPDF_LINECAP_PROJECTING_SQUARE
    /// ```
    #[inline]
    pub fn FPDFPageObj_SetLineCap(
        &self,
        page_object: &PdfiumPageObject,
        line_cap: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_SetLineCap)(page_object.into(), line_cap) })
    }

    /// C documentation for FPDFPageObj_SetLineJoin:
    ///
    /// ```text
    /// Set the line join of |page_object|.
    ///
    /// page_object  - handle to a page object.
    /// line_join    - line join
    ///
    /// Line join can be one of following: FPDF_LINEJOIN_MITER, FPDF_LINEJOIN_ROUND,
    /// FPDF_LINEJOIN_BEVEL
    /// ```
    #[inline]
    pub fn FPDFPageObj_SetLineJoin(
        &self,
        page_object: &PdfiumPageObject,
        line_join: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_SetLineJoin)(page_object.into(), line_join) })
    }

    /// C documentation for FPDFPageObj_SetMatrix:
    ///
    /// ```text
    /// Experimental API.
    /// Set the transform matrix of a page object.
    ///
    ///   page_object - handle to a page object.
    ///   matrix      - pointer to struct with the matrix value.
    ///
    /// The matrix is composed as:
    ///   |a c e|
    ///   |b d f|
    /// and can be used to scale, rotate, shear and translate the page object.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFPageObj_SetMatrix(
        &self,
        page_object: &PdfiumPageObject,
        matrix: &FS_MATRIX,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_SetMatrix)(page_object.into(), matrix) })
    }

    /// C documentation for FPDFPageObj_SetStrokeColor:
    ///
    /// ```text
    /// Set the stroke RGBA of a page object. Range of values: 0 - 255.
    ///
    /// page_object  - the handle to the page object.
    /// R            - the red component for the object's stroke color.
    /// G            - the green component for the object's stroke color.
    /// B            - the blue component for the object's stroke color.
    /// A            - the stroke alpha for the object.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFPageObj_SetStrokeColor(
        &self,
        page_object: &PdfiumPageObject,
        R: u32,
        G: u32,
        B: u32,
        A: u32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_SetStrokeColor)(page_object.into(), R, G, B, A) })
    }

    /// C documentation for FPDFPageObj_SetStrokeWidth:
    ///
    /// ```text
    /// Set the stroke width of a page object.
    ///
    /// path   - the handle to the page object.
    /// width  - the width of the stroke.
    ///
    /// Returns TRUE on success
    /// ```
    #[inline]
    pub fn FPDFPageObj_SetStrokeWidth(
        &self,
        page_object: &PdfiumPageObject,
        width: f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_SetStrokeWidth)(page_object.into(), width) })
    }

    /// C documentation for FPDFPageObj_Transform:
    ///
    /// ```text
    /// Transform |page_object| by the given matrix.
    ///
    ///   page_object - handle to a page object.
    ///   a           - matrix value.
    ///   b           - matrix value.
    ///   c           - matrix value.
    ///   d           - matrix value.
    ///   e           - matrix value.
    ///   f           - matrix value.
    ///
    /// The matrix is composed as:
    ///   |a c e|
    ///   |b d f|
    /// and can be used to scale, rotate, shear and translate the |page_object|.
    /// ```
    #[inline]
    pub fn FPDFPageObj_Transform(
        &self,
        page_object: &PdfiumPageObject,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) {
        unsafe { (self.fn_FPDFPageObj_Transform)(page_object.into(), a, b, c, d, e, f) }
    }

    /// C documentation for FPDFPageObj_TransformClipPath:
    ///
    /// ```text
    /// Transform (scale, rotate, shear, move) the clip path of page object.
    /// page_object - Handle to a page object. Returned by
    /// FPDFPageObj_NewImageObj().
    ///
    /// a  - The coefficient "a" of the matrix.
    /// b  - The coefficient "b" of the matrix.
    /// c  - The coefficient "c" of the matrix.
    /// d  - The coefficient "d" of the matrix.
    /// e  - The coefficient "e" of the matrix.
    /// f  - The coefficient "f" of the matrix.
    /// ```
    #[inline]
    pub fn FPDFPageObj_TransformClipPath(
        &self,
        page_object: &PdfiumPageObject,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) {
        unsafe { (self.fn_FPDFPageObj_TransformClipPath)(page_object.into(), a, b, c, d, e, f) }
    }

    /// C documentation for FPDFPageObj_TransformF:
    ///
    /// ```text
    /// Experimental API.
    /// Transform |page_object| by the given matrix.
    ///
    ///   page_object - handle to a page object.
    ///   matrix      - the transform matrix.
    ///
    /// Returns TRUE on success.
    ///
    /// This can be used to scale, rotate, shear and translate the |page_object|.
    /// It is an improved version of FPDFPageObj_Transform() that does not do
    /// unnecessary double to float conversions, and only uses 1 parameter for the
    /// matrix. It also returns whether the operation succeeded or not.
    /// ```
    #[inline]
    pub fn FPDFPageObj_TransformF(
        &self,
        page_object: &PdfiumPageObject,
        matrix: &FS_MATRIX,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPageObj_TransformF)(page_object.into(), matrix) })
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
    #[inline]
    pub fn FPDFPage_CountObjects(&self, page: &PdfiumPage) -> i32 {
        unsafe { (self.fn_FPDFPage_CountObjects)(page.into()) }
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
    #[inline]
    pub fn FPDFPage_CreateAnnot(
        &self,
        page: &PdfiumPage,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> PdfiumResult<PdfiumAnnotation> {
        PdfiumAnnotation::new_from_handle(unsafe {
            (self.fn_FPDFPage_CreateAnnot)(page.into(), subtype)
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
    #[inline]
    pub fn FPDFPage_Delete(&self, document: &PdfiumDocument, page_index: i32) {
        unsafe { (self.fn_FPDFPage_Delete)(document.into(), page_index) }
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
    #[inline]
    pub fn FPDFPage_Flatten(&self, page: &PdfiumPage, nFlag: i32) -> i32 {
        unsafe { (self.fn_FPDFPage_Flatten)(page.into(), nFlag) }
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
    #[inline]
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
    #[inline]
    pub fn FPDFPage_GenerateContent(&self, page: &PdfiumPage) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_GenerateContent)(page.into()) })
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
    #[inline]
    pub fn FPDFPage_GetAnnot(
        &self,
        page: &PdfiumPage,
        index: i32,
    ) -> PdfiumResult<PdfiumAnnotation> {
        PdfiumAnnotation::new_from_handle(unsafe {
            (self.fn_FPDFPage_GetAnnot)(page.into(), index)
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
    #[inline]
    pub fn FPDFPage_GetAnnotCount(&self, page: &PdfiumPage) -> i32 {
        unsafe { (self.fn_FPDFPage_GetAnnotCount)(page.into()) }
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
    #[inline]
    pub fn FPDFPage_GetAnnotIndex(&self, page: &PdfiumPage, annot: &PdfiumAnnotation) -> i32 {
        unsafe { (self.fn_FPDFPage_GetAnnotIndex)(page.into(), annot.into()) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFPage_GetObject(
        &self,
        page: &PdfiumPage,
        index: i32,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDFPage_GetObject)(page.into(), index)
        })
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
    #[inline]
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
    #[inline]
    pub fn FPDFPage_GetRotation(&self, page: &PdfiumPage) -> i32 {
        unsafe { (self.fn_FPDFPage_GetRotation)(page.into()) }
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
    #[inline]
    pub fn FPDFPage_GetThumbnailAsBitmap(&self, page: &PdfiumPage) -> PdfiumResult<PdfiumBitmap> {
        PdfiumBitmap::new_from_handle(unsafe {
            (self.fn_FPDFPage_GetThumbnailAsBitmap)(page.into())
        })
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
    #[inline]
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
    #[inline]
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

    /// C documentation for FPDFPage_HasTransparency:
    ///
    /// ```text
    /// Checks if |page| contains transparency.
    ///
    ///   page - handle to a page.
    ///
    /// Returns TRUE if |page| contains transparency.
    /// ```
    #[inline]
    pub fn FPDFPage_HasTransparency(&self, page: &PdfiumPage) -> i32 {
        unsafe { (self.fn_FPDFPage_HasTransparency)(page.into()) }
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
    #[inline]
    pub fn FPDFPage_InsertClipPath(&self, page: &PdfiumPage, clipPath: &PdfiumClipPath) {
        unsafe { (self.fn_FPDFPage_InsertClipPath)(page.into(), clipPath.into()) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFPage_RemoveAnnot(&self, page: &PdfiumPage, index: i32) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_RemoveAnnot)(page.into(), index) })
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
    #[inline]
    pub fn FPDFPage_RemoveObject(
        &self,
        page: &PdfiumPage,
        page_object: &PdfiumPageObject,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_RemoveObject)(page.into(), page_object.into()) })
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDFPage_SetRotation(&self, page: &PdfiumPage, rotate: i32) {
        unsafe { (self.fn_FPDFPage_SetRotation)(page.into(), rotate) }
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
    #[inline]
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
    #[inline]
    pub fn FPDFPage_TransFormWithClip(
        &self,
        page: &PdfiumPage,
        matrix: &FS_MATRIX,
        clipRect: &FS_RECTF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPage_TransFormWithClip)(page.into(), matrix, clipRect) })
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
    #[inline]
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

    /// C documentation for FPDFPathSegment_GetClose:
    ///
    /// ```text
    /// Gets if the |segment| closes the current subpath of a given path.
    ///
    ///   segment - handle to a segment.
    ///
    /// Returns close flag for non-NULL segment, FALSE otherwise.
    /// ```
    #[inline]
    pub fn FPDFPathSegment_GetClose(&self, segment: &PdfiumPathSegment) -> i32 {
        unsafe { (self.fn_FPDFPathSegment_GetClose)(segment.into()) }
    }

    /// C documentation for FPDFPathSegment_GetPoint:
    ///
    /// ```text
    /// Get coordinates of |segment|.
    ///
    ///   segment  - handle to a segment.
    ///   x      - the horizontal position of the segment.
    ///   y      - the vertical position of the segment.
    ///
    /// Returns TRUE on success, otherwise |x| and |y| is not set.
    /// ```
    #[inline]
    pub fn FPDFPathSegment_GetPoint(
        &self,
        segment: &PdfiumPathSegment,
        x: &mut f32,
        y: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPathSegment_GetPoint)(segment.into(), x, y) })
    }

    /// C documentation for FPDFPathSegment_GetType:
    ///
    /// ```text
    /// Get type of |segment|.
    ///
    ///   segment - handle to a segment.
    ///
    /// Returns one of the FPDF_SEGMENT_* values on success,
    /// FPDF_SEGMENT_UNKNOWN on error.
    /// ```
    #[inline]
    pub fn FPDFPathSegment_GetType(&self, segment: &PdfiumPathSegment) -> i32 {
        unsafe { (self.fn_FPDFPathSegment_GetType)(segment.into()) }
    }

    /// C documentation for FPDFPath_BezierTo:
    ///
    /// ```text
    /// Add a cubic Bezier curve to the given path, starting at the current point.
    ///
    /// path   - the handle to the path object.
    /// x1     - the horizontal position of the first Bezier control point.
    /// y1     - the vertical position of the first Bezier control point.
    /// x2     - the horizontal position of the second Bezier control point.
    /// y2     - the vertical position of the second Bezier control point.
    /// x3     - the horizontal position of the ending point of the Bezier curve.
    /// y3     - the vertical position of the ending point of the Bezier curve.
    ///
    /// Returns TRUE on success
    /// ```
    #[inline]
    pub fn FPDFPath_BezierTo(
        &self,
        path: &PdfiumPageObject,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPath_BezierTo)(path.into(), x1, y1, x2, y2, x3, y3) })
    }

    /// C documentation for FPDFPath_Close:
    ///
    /// ```text
    /// Close the current subpath of a given path.
    ///
    /// path   - the handle to the path object.
    ///
    /// This will add a line between the current point and the initial point of the
    /// subpath, thus terminating the current subpath.
    ///
    /// Returns TRUE on success
    /// ```
    #[inline]
    pub fn FPDFPath_Close(&self, path: &PdfiumPageObject) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPath_Close)(path.into()) })
    }

    /// C documentation for FPDFPath_CountSegments:
    ///
    /// ```text
    /// Get number of segments inside |path|.
    ///
    ///   path - handle to a path.
    ///
    /// A segment is a command, created by e.g. FPDFPath_MoveTo(),
    /// FPDFPath_LineTo() or FPDFPath_BezierTo().
    ///
    /// Returns the number of objects in |path| or -1 on failure.
    /// ```
    #[inline]
    pub fn FPDFPath_CountSegments(&self, path: &PdfiumPageObject) -> i32 {
        unsafe { (self.fn_FPDFPath_CountSegments)(path.into()) }
    }

    /// C documentation for FPDFPath_GetDrawMode:
    ///
    /// ```text
    /// Get the drawing mode of a path.
    ///
    /// path     - the handle to the path object.
    /// fillmode - the filling mode of the path: one of the FPDF_FILLMODE_* flags.
    /// stroke   - a boolean specifying if the path is stroked or not.
    ///
    /// Returns TRUE on success
    /// ```
    #[inline]
    pub fn FPDFPath_GetDrawMode(
        &self,
        path: &PdfiumPageObject,
        fillmode: &mut i32,
        stroke: &mut FPDF_BOOL,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPath_GetDrawMode)(path.into(), fillmode, stroke) })
    }

    /// C documentation for FPDFPath_GetPathSegment:
    ///
    /// ```text
    /// Get segment in |path| at |index|.
    ///
    ///   path  - handle to a path.
    ///   index - the index of a segment.
    ///
    /// Returns the handle to the segment, or NULL on faiure.
    /// ```
    #[inline]
    pub fn FPDFPath_GetPathSegment(
        &self,
        path: &PdfiumPageObject,
        index: i32,
    ) -> PdfiumResult<PdfiumPathSegment> {
        PdfiumPathSegment::new_from_handle(unsafe {
            (self.fn_FPDFPath_GetPathSegment)(path.into(), index)
        })
    }

    /// C documentation for FPDFPath_LineTo:
    ///
    /// ```text
    /// Add a line between the current point and a new point in the path.
    ///
    /// path   - the handle to the path object.
    /// x      - the horizontal position of the new point.
    /// y      - the vertical position of the new point.
    ///
    /// The path's current point is changed to (x, y).
    ///
    /// Returns TRUE on success
    /// ```
    #[inline]
    pub fn FPDFPath_LineTo(&self, path: &PdfiumPageObject, x: f32, y: f32) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPath_LineTo)(path.into(), x, y) })
    }

    /// C documentation for FPDFPath_MoveTo:
    ///
    /// ```text
    /// Move a path's current point.
    ///
    /// path   - the handle to the path object.
    /// x      - the horizontal position of the new current point.
    /// y      - the vertical position of the new current point.
    ///
    /// Note that no line will be created between the previous current point and the
    /// new one.
    ///
    /// Returns TRUE on success
    /// ```
    #[inline]
    pub fn FPDFPath_MoveTo(&self, path: &PdfiumPageObject, x: f32, y: f32) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPath_MoveTo)(path.into(), x, y) })
    }

    /// C documentation for FPDFPath_SetDrawMode:
    ///
    /// ```text
    /// Set the drawing mode of a path.
    ///
    /// path     - the handle to the path object.
    /// fillmode - the filling mode to be set: one of the FPDF_FILLMODE_* flags.
    /// stroke   - a boolean specifying if the path should be stroked or not.
    ///
    /// Returns TRUE on success
    /// ```
    #[inline]
    pub fn FPDFPath_SetDrawMode(
        &self,
        path: &PdfiumPageObject,
        fillmode: i32,
        stroke: i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFPath_SetDrawMode)(path.into(), fillmode, stroke) })
    }

    /// C documentation for FPDFSignatureObj_GetByteRange:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFSignatureObj_GetByteRange
    ///          Get the byte range of a signature object.
    /// Parameters:
    ///          signature   -   Handle to the signature object. Returned by
    ///                          FPDF_GetSignatureObject().
    ///          buffer      -   The address of a buffer that receives the
    ///                          byte range.
    ///          length      -   The size, in ints, of |buffer|.
    /// Return value:
    ///          Returns the number of ints in the byte range on
    ///          success, 0 on error.
    ///
    /// |buffer| is an array of pairs of integers (starting byte offset,
    /// length in bytes) that describes the exact byte range for the digest
    /// calculation. If |length| is less than the returned length, or
    /// |buffer| is NULL, |buffer| will not be modified.
    /// ```
    #[inline]
    pub fn FPDFSignatureObj_GetByteRange(
        &self,
        signature: &PdfiumSignature,
        buffer: &mut i32,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDFSignatureObj_GetByteRange)(signature.into(), buffer, length) }
    }

    /// C documentation for FPDFSignatureObj_GetContents:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFSignatureObj_GetContents
    ///          Get the contents of a signature object.
    /// Parameters:
    ///          signature   -   Handle to the signature object. Returned by
    ///                          FPDF_GetSignatureObject().
    ///          buffer      -   The address of a buffer that receives the contents.
    ///          length      -   The size, in bytes, of |buffer|.
    /// Return value:
    ///          Returns the number of bytes in the contents on success, 0 on error.
    ///
    /// For public-key signatures, |buffer| is either a DER-encoded PKCS#1 binary or
    /// a DER-encoded PKCS#7 binary. If |length| is less than the returned length, or
    /// |buffer| is NULL, |buffer| will not be modified.
    /// ```
    #[inline]
    pub fn FPDFSignatureObj_GetContents(
        &self,
        signature: &PdfiumSignature,
        buffer: Option<&mut [u8]>,
        length: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFSignatureObj_GetContents)(
                signature.into(),
                to_void_ptr_mut(buffer),
                length,
            )
        }
    }

    /// C documentation for FPDFSignatureObj_GetDocMDPPermission:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFSignatureObj_GetDocMDPPermission
    ///          Get the DocMDP permission of a signature object.
    /// Parameters:
    ///          signature   -   Handle to the signature object. Returned by
    ///                          FPDF_GetSignatureObject().
    /// Return value:
    ///          Returns the permission (1, 2 or 3) on success, 0 on error.
    /// ```
    #[inline]
    pub fn FPDFSignatureObj_GetDocMDPPermission(&self, signature: &PdfiumSignature) -> u32 {
        unsafe { (self.fn_FPDFSignatureObj_GetDocMDPPermission)(signature.into()) }
    }

    /// C documentation for FPDFSignatureObj_GetReason:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFSignatureObj_GetReason
    ///          Get the reason (comment) of the signature object.
    /// Parameters:
    ///          signature   -   Handle to the signature object. Returned by
    ///                          FPDF_GetSignatureObject().
    ///          buffer      -   The address of a buffer that receives the reason.
    ///          length      -   The size, in bytes, of |buffer|.
    /// Return value:
    ///          Returns the number of bytes in the reason on success, 0 on error.
    ///
    /// Regardless of the platform, the |buffer| is always in UTF-16LE encoding. The
    /// string is terminated by a UTF16 NUL character. If |length| is less than the
    /// returned length, or |buffer| is NULL, |buffer| will not be modified.
    /// ```
    #[inline]
    pub fn FPDFSignatureObj_GetReason(
        &self,
        signature: &PdfiumSignature,
        buffer: Option<&mut [u8]>,
        length: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFSignatureObj_GetReason)(signature.into(), to_void_ptr_mut(buffer), length)
        }
    }

    /// C documentation for FPDFSignatureObj_GetSubFilter:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFSignatureObj_GetSubFilter
    ///          Get the encoding of the value of a signature object.
    /// Parameters:
    ///          signature   -   Handle to the signature object. Returned by
    ///                          FPDF_GetSignatureObject().
    ///          buffer      -   The address of a buffer that receives the encoding.
    ///          length      -   The size, in bytes, of |buffer|.
    /// Return value:
    ///          Returns the number of bytes in the encoding name (including the
    ///          trailing NUL character) on success, 0 on error.
    ///
    /// The |buffer| is always encoded in 7-bit ASCII. If |length| is less than the
    /// returned length, or |buffer| is NULL, |buffer| will not be modified.
    /// ```
    #[inline]
    pub fn FPDFSignatureObj_GetSubFilter(
        &self,
        signature: &PdfiumSignature,
        buffer: Option<&mut [i8]>,
        length: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFSignatureObj_GetSubFilter)(
                signature.into(),
                to_char_ptr_mut(buffer),
                length,
            )
        }
    }

    /// C documentation for FPDFSignatureObj_GetTime:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFSignatureObj_GetTime
    ///          Get the time of signing of a signature object.
    /// Parameters:
    ///          signature   -   Handle to the signature object. Returned by
    ///                          FPDF_GetSignatureObject().
    ///          buffer      -   The address of a buffer that receives the time.
    ///          length      -   The size, in bytes, of |buffer|.
    /// Return value:
    ///          Returns the number of bytes in the encoding name (including the
    ///          trailing NUL character) on success, 0 on error.
    ///
    /// The |buffer| is always encoded in 7-bit ASCII. If |length| is less than the
    /// returned length, or |buffer| is NULL, |buffer| will not be modified.
    ///
    /// The format of time is expected to be D:YYYYMMDDHHMMSS+XX'YY', i.e. it's
    /// percision is seconds, with timezone information. This value should be used
    /// only when the time of signing is not available in the (PKCS#7 binary)
    /// signature.
    /// ```
    #[inline]
    pub fn FPDFSignatureObj_GetTime(
        &self,
        signature: &PdfiumSignature,
        buffer: Option<&mut [i8]>,
        length: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFSignatureObj_GetTime)(signature.into(), to_char_ptr_mut(buffer), length)
        }
    }

    /// C documentation for FPDFTextObj_GetFont:
    ///
    /// ```text
    /// Experimental API.
    /// Get the font of a text object.
    ///
    /// text - the handle to the text object.
    ///
    /// Returns a handle to the font object held by |text| which retains ownership.
    /// ```
    #[inline]
    pub fn FPDFTextObj_GetFont(&self, text: &PdfiumPageObject) -> PdfiumResult<PdfiumFont> {
        PdfiumFont::new_from_handle(unsafe { (self.fn_FPDFTextObj_GetFont)(text.into()) })
    }

    /// C documentation for FPDFTextObj_GetFontSize:
    ///
    /// ```text
    /// Get the font size of a text object.
    ///
    ///   text - handle to a text.
    ///   size - pointer to the font size of the text object, measured in points
    ///   (about 1/72 inch)
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFTextObj_GetFontSize(
        &self,
        text: &PdfiumPageObject,
        size: &mut f32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFTextObj_GetFontSize)(text.into(), size) })
    }

    /// C documentation for FPDFTextObj_GetRenderedBitmap:
    ///
    /// ```text
    /// Experimental API.
    /// Get a bitmap rasterization of |text_object|. To render correctly, the caller
    /// must provide the |document| associated with |text_object|. If there is a
    /// |page| associated with |text_object|, the caller should provide that as well.
    /// The returned bitmap will be owned by the caller, and FPDFBitmap_Destroy()
    /// must be called on the returned bitmap when it is no longer needed.
    ///
    ///   document    - handle to a document associated with |text_object|.
    ///   page        - handle to an optional page associated with |text_object|.
    ///   text_object - handle to a text object.
    ///   scale       - the scaling factor, which must be greater than 0.
    ///
    /// Returns the bitmap or NULL on failure.
    /// ```
    #[inline]
    pub fn FPDFTextObj_GetRenderedBitmap(
        &self,
        document: &PdfiumDocument,
        page: &PdfiumPage,
        text_object: &PdfiumPageObject,
        scale: f32,
    ) -> PdfiumResult<PdfiumBitmap> {
        PdfiumBitmap::new_from_handle(unsafe {
            (self.fn_FPDFTextObj_GetRenderedBitmap)(
                document.into(),
                page.into(),
                text_object.into(),
                scale,
            )
        })
    }

    /// C documentation for FPDFTextObj_GetText:
    ///
    /// ```text
    /// Get the text of a text object.
    ///
    /// text_object      - the handle to the text object.
    /// text_page        - the handle to the text page.
    /// buffer           - the address of a buffer that receives the text.
    /// length           - the size, in bytes, of |buffer|.
    ///
    /// Returns the number of bytes in the text (including the trailing NUL
    /// character) on success, 0 on error.
    ///
    /// Regardless of the platform, the |buffer| is always in UTF-16LE encoding.
    /// If |length| is less than the returned length, or |buffer| is NULL, |buffer|
    /// will not be modified.
    /// ```
    #[inline]
    pub fn FPDFTextObj_GetText(
        &self,
        text_object: &PdfiumPageObject,
        text_page: &PdfiumTextPage,
        buffer: &mut Vec<u16>,
        length: c_ulong,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFTextObj_GetText)(
                text_object.into(),
                text_page.into(),
                buffer.as_mut_ptr(),
                length,
            )
        }
    }

    /// C documentation for FPDFTextObj_GetTextRenderMode:
    ///
    /// ```text
    /// Get the text rendering mode of a text object.
    ///
    /// text     - the handle to the text object.
    ///
    /// Returns one of the known FPDF_TEXT_RENDERMODE enum values on success,
    /// FPDF_TEXTRENDERMODE_UNKNOWN on error.
    /// ```
    #[inline]
    pub fn FPDFTextObj_GetTextRenderMode(&self, text: &PdfiumPageObject) -> FPDF_TEXT_RENDERMODE {
        unsafe { (self.fn_FPDFTextObj_GetTextRenderMode)(text.into()) }
    }

    /// C documentation for FPDFTextObj_SetTextRenderMode:
    ///
    /// ```text
    /// Experimental API.
    /// Set the text rendering mode of a text object.
    ///
    /// text         - the handle to the text object.
    /// render_mode  - the FPDF_TEXT_RENDERMODE enum value to be set (cannot set to
    ///                FPDF_TEXTRENDERMODE_UNKNOWN).
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FPDFTextObj_SetTextRenderMode(
        &self,
        text: &PdfiumPageObject,
        render_mode: FPDF_TEXT_RENDERMODE,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFTextObj_SetTextRenderMode)(text.into(), render_mode) })
    }

    /// C documentation for FPDFText_CountChars:
    ///
    /// ```text
    /// Function: FPDFText_CountChars
    ///          Get number of characters in a page.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    /// Return value:
    ///          Number of characters in the page. Return -1 for error.
    ///          Generated characters, like additional space characters, new line
    ///          characters, are also counted.
    /// Comments:
    ///          Characters in a page form a "stream", inside the stream, each
    ///          character has an index.
    ///          We will use the index parameters in many of FPDFTEXT functions. The
    ///          first character in the page
    ///          has an index value of zero.
    /// ```
    #[inline]
    pub fn FPDFText_CountChars(&self, text_page: &PdfiumTextPage) -> i32 {
        unsafe { (self.fn_FPDFText_CountChars)(text_page.into()) }
    }

    /// C documentation for FPDFText_CountRects:
    ///
    /// ```text
    /// Function: FPDFText_CountRects
    ///          Counts number of rectangular areas occupied by a segment of text,
    ///          and caches the result for subsequent FPDFText_GetRect() calls.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          start_index -   Index for the start character.
    ///          count       -   Number of characters, or -1 for all remaining.
    /// Return value:
    ///          Number of rectangles, 0 if text_page is null, or -1 on bad
    ///          start_index.
    /// Comments:
    ///          This function, along with FPDFText_GetRect can be used by
    ///          applications to detect the position on the page for a text segment,
    ///          so proper areas can be highlighted. The FPDFText_* functions will
    ///          automatically merge small character boxes into bigger one if those
    ///          characters are on the same line and use same font settings.
    /// ```
    #[inline]
    pub fn FPDFText_CountRects(
        &self,
        text_page: &PdfiumTextPage,
        start_index: i32,
        count: i32,
    ) -> i32 {
        unsafe { (self.fn_FPDFText_CountRects)(text_page.into(), start_index, count) }
    }

    /// C documentation for FPDFText_FindNext:
    ///
    /// ```text
    /// Function: FPDFText_FindNext
    ///          Search in the direction from page start to end.
    /// Parameters:
    ///          handle      -   A search context handle returned by
    ///                          FPDFText_FindStart.
    /// Return Value:
    ///          Whether a match is found.
    /// ```
    #[inline]
    pub fn FPDFText_FindNext(&self, handle: &PdfiumSearch) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFText_FindNext)(handle.into()) })
    }

    /// C documentation for FPDFText_FindPrev:
    ///
    /// ```text
    /// Function: FPDFText_FindPrev
    ///          Search in the direction from page end to start.
    /// Parameters:
    ///          handle      -   A search context handle returned by
    ///                          FPDFText_FindStart.
    /// Return Value:
    ///          Whether a match is found.
    /// ```
    #[inline]
    pub fn FPDFText_FindPrev(&self, handle: &PdfiumSearch) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFText_FindPrev)(handle.into()) })
    }

    /// C documentation for FPDFText_FindStart:
    ///
    /// ```text
    /// Function: FPDFText_FindStart
    ///          Start a search.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          findwhat    -   A unicode match pattern.
    ///          flags       -   Option flags.
    ///          start_index -   Start from this character. -1 for end of the page.
    /// Return Value:
    ///          A handle for the search context. FPDFText_FindClose must be called
    ///          to release this handle.
    /// ```
    #[inline]
    pub fn FPDFText_FindStart(
        &self,
        text_page: &PdfiumTextPage,
        findwhat: &str,
        flags: c_ulong,
        start_index: i32,
    ) -> PdfiumResult<PdfiumSearch> {
        let findwhat = str_to_utf16le_vec(findwhat);
        PdfiumSearch::new_from_handle(unsafe {
            (self.fn_FPDFText_FindStart)(text_page.into(), findwhat.as_ptr(), flags, start_index)
        })
    }

    /// C documentation for FPDFText_GetBoundedText:
    ///
    /// ```text
    /// Function: FPDFText_GetBoundedText
    ///          Extract unicode text within a rectangular boundary on the page.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          left        -   Left boundary.
    ///          top         -   Top boundary.
    ///          right       -   Right boundary.
    ///          bottom      -   Bottom boundary.
    ///          buffer      -   Caller-allocated buffer to receive UTF-16 values.
    ///          buflen      -   Number of UTF-16 values (not bytes) that `buffer`
    ///                          is capable of holding.
    /// Return Value:
    ///          If buffer is NULL or buflen is zero, return number of UTF-16
    ///          values (not bytes) of text present within the rectangle, excluding
    ///          a terminating NUL. Generally you should pass a buffer at least one
    ///          larger than this if you want a terminating NUL, which will be
    ///          provided if space is available. Otherwise, return number of UTF-16
    ///          values copied into the buffer, including the terminating NUL when
    ///          space for it is available.
    /// Comment:
    ///          If the buffer is too small, as much text as will fit is copied into
    ///          it. May return a split surrogate in that case.
    /// ```
    #[inline]
    pub fn FPDFText_GetBoundedText(
        &self,
        text_page: &PdfiumTextPage,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
        buffer: &mut c_ushort,
        buflen: i32,
    ) -> i32 {
        unsafe {
            (self.fn_FPDFText_GetBoundedText)(
                text_page.into(),
                left,
                top,
                right,
                bottom,
                buffer,
                buflen,
            )
        }
    }

    /// C documentation for FPDFText_GetCharAngle:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFText_GetCharAngle
    ///          Get character rotation angle.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          index       -   Zero-based index of the character.
    /// Return Value:
    ///          On success, return the angle value in radian. Value will always be
    ///          greater or equal to 0. If |text_page| is invalid, or if |index| is
    ///          out of bounds, then return -1.
    /// ```
    #[inline]
    pub fn FPDFText_GetCharAngle(&self, text_page: &PdfiumTextPage, index: i32) -> f32 {
        unsafe { (self.fn_FPDFText_GetCharAngle)(text_page.into(), index) }
    }

    /// C documentation for FPDFText_GetCharBox:
    ///
    /// ```text
    /// Function: FPDFText_GetCharBox
    ///          Get bounding box of a particular character.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          index       -   Zero-based index of the character.
    ///          left        -   Pointer to a double number receiving left position
    ///                          of the character box.
    ///          right       -   Pointer to a double number receiving right position
    ///                          of the character box.
    ///          bottom      -   Pointer to a double number receiving bottom position
    ///                          of the character box.
    ///          top         -   Pointer to a double number receiving top position of
    ///                          the character box.
    /// Return Value:
    ///          On success, return TRUE and fill in |left|, |right|, |bottom|, and
    ///          |top|. If |text_page| is invalid, or if |index| is out of bounds,
    ///          then return FALSE, and the out parameters remain unmodified.
    /// Comments:
    ///          All positions are measured in PDF "user space".
    /// ```
    #[inline]
    pub fn FPDFText_GetCharBox(
        &self,
        text_page: &PdfiumTextPage,
        index: i32,
        left: &mut f64,
        right: &mut f64,
        bottom: &mut f64,
        top: &mut f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFText_GetCharBox)(text_page.into(), index, left, right, bottom, top)
        })
    }

    /// C documentation for FPDFText_GetCharIndexAtPos:
    ///
    /// ```text
    /// Function: FPDFText_GetCharIndexAtPos
    ///          Get the index of a character at or nearby a certain position on the
    ///          page.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          x           -   X position in PDF "user space".
    ///          y           -   Y position in PDF "user space".
    ///          xTolerance  -   An x-axis tolerance value for character hit
    ///                          detection, in point units.
    ///          yTolerance  -   A y-axis tolerance value for character hit
    ///                          detection, in point units.
    /// Return Value:
    ///          The zero-based index of the character at, or nearby the point (x,y).
    ///          If there is no character at or nearby the point, return value will
    ///          be -1. If an error occurs, -3 will be returned.
    /// ```
    #[inline]
    pub fn FPDFText_GetCharIndexAtPos(
        &self,
        text_page: &PdfiumTextPage,
        x: f64,
        y: f64,
        xTolerance: f64,
        yTolerance: f64,
    ) -> i32 {
        unsafe {
            (self.fn_FPDFText_GetCharIndexAtPos)(text_page.into(), x, y, xTolerance, yTolerance)
        }
    }

    /// C documentation for FPDFText_GetCharIndexFromTextIndex:
    ///
    /// ```text
    /// Get the character index in |text_page| internal character list.
    ///
    ///   text_page  - a text page information structure.
    ///   nTextIndex - index of the text returned from FPDFText_GetText().
    ///
    /// Returns the index of the character in internal character list. -1 for error.
    /// ```
    #[inline]
    pub fn FPDFText_GetCharIndexFromTextIndex(
        &self,
        text_page: &PdfiumTextPage,
        nTextIndex: i32,
    ) -> i32 {
        unsafe { (self.fn_FPDFText_GetCharIndexFromTextIndex)(text_page.into(), nTextIndex) }
    }

    /// C documentation for FPDFText_GetCharOrigin:
    ///
    /// ```text
    /// Function: FPDFText_GetCharOrigin
    ///          Get origin of a particular character.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          index       -   Zero-based index of the character.
    ///          x           -   Pointer to a double number receiving x coordinate of
    ///                          the character origin.
    ///          y           -   Pointer to a double number receiving y coordinate of
    ///                          the character origin.
    /// Return Value:
    ///          Whether the call succeeded. If false, x and y are unchanged.
    /// Comments:
    ///          All positions are measured in PDF "user space".
    /// ```
    #[inline]
    pub fn FPDFText_GetCharOrigin(
        &self,
        text_page: &PdfiumTextPage,
        index: i32,
        x: &mut f64,
        y: &mut f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFText_GetCharOrigin)(text_page.into(), index, x, y) })
    }

    /// C documentation for FPDFText_GetFillColor:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFText_GetFillColor
    ///          Get the fill color of a particular character.
    /// Parameters:
    ///          text_page      -   Handle to a text page information structure.
    ///                             Returned by FPDFText_LoadPage function.
    ///          index          -   Zero-based index of the character.
    ///          R              -   Pointer to an unsigned int number receiving the
    ///                             red value of the fill color.
    ///          G              -   Pointer to an unsigned int number receiving the
    ///                             green value of the fill color.
    ///          B              -   Pointer to an unsigned int number receiving the
    ///                             blue value of the fill color.
    ///          A              -   Pointer to an unsigned int number receiving the
    ///                             alpha value of the fill color.
    /// Return value:
    ///          Whether the call succeeded. If false, |R|, |G|, |B| and |A| are
    ///          unchanged.
    /// ```
    #[inline]
    pub fn FPDFText_GetFillColor(
        &self,
        text_page: &PdfiumTextPage,
        index: i32,
        R: &mut u32,
        G: &mut u32,
        B: &mut u32,
        A: &mut u32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFText_GetFillColor)(text_page.into(), index, R, G, B, A) })
    }

    /// C documentation for FPDFText_GetFontInfo:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFText_GetFontInfo
    ///          Get the font name and flags of a particular character.
    /// Parameters:
    ///          text_page - Handle to a text page information structure.
    ///                      Returned by FPDFText_LoadPage function.
    ///          index     - Zero-based index of the character.
    ///          buffer    - A buffer receiving the font name.
    ///          buflen    - The length of |buffer| in bytes.
    ///          flags     - Optional pointer to an int receiving the font flags.
    ///                      These flags should be interpreted per PDF spec 1.7
    ///                      Section 5.7.1 Font Descriptor Flags.
    /// Return value:
    ///          On success, return the length of the font name, including the
    ///          trailing NUL character, in bytes. If this length is less than or
    ///          equal to |length|, |buffer| is set to the font name, |flags| is
    ///          set to the font flags. |buffer| is in UTF-8 encoding. Return 0 on
    ///          failure.
    /// ```
    #[inline]
    pub fn FPDFText_GetFontInfo(
        &self,
        text_page: &PdfiumTextPage,
        index: i32,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
        flags: &mut i32,
    ) -> c_ulong {
        unsafe {
            (self.fn_FPDFText_GetFontInfo)(
                text_page.into(),
                index,
                to_void_ptr_mut(buffer),
                buflen,
                flags,
            )
        }
    }

    /// C documentation for FPDFText_GetFontSize:
    ///
    /// ```text
    /// Function: FPDFText_GetFontSize
    ///          Get the font size of a particular character.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          index       -   Zero-based index of the character.
    /// Return value:
    ///          The font size of the particular character, measured in points (about
    ///          1/72 inch). This is the typographic size of the font (so called
    ///          "em size").
    /// ```
    #[inline]
    pub fn FPDFText_GetFontSize(&self, text_page: &PdfiumTextPage, index: i32) -> f64 {
        unsafe { (self.fn_FPDFText_GetFontSize)(text_page.into(), index) }
    }

    /// C documentation for FPDFText_GetFontWeight:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFText_GetFontWeight
    ///          Get the font weight of a particular character.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          index       -   Zero-based index of the character.
    /// Return value:
    ///          On success, return the font weight of the particular character. If
    ///          |text_page| is invalid, if |index| is out of bounds, or if the
    ///          character's text object is undefined, return -1.
    /// ```
    #[inline]
    pub fn FPDFText_GetFontWeight(&self, text_page: &PdfiumTextPage, index: i32) -> i32 {
        unsafe { (self.fn_FPDFText_GetFontWeight)(text_page.into(), index) }
    }

    /// C documentation for FPDFText_GetLooseCharBox:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFText_GetLooseCharBox
    ///          Get a "loose" bounding box of a particular character, i.e., covering
    ///          the entire glyph bounds, without taking the actual glyph shape into
    ///          account.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          index       -   Zero-based index of the character.
    ///          rect        -   Pointer to a FS_RECTF receiving the character box.
    /// Return Value:
    ///          On success, return TRUE and fill in |rect|. If |text_page| is
    ///          invalid, or if |index| is out of bounds, then return FALSE, and the
    ///          |rect| out parameter remains unmodified.
    /// Comments:
    ///          All positions are measured in PDF "user space".
    /// ```
    #[inline]
    pub fn FPDFText_GetLooseCharBox(
        &self,
        text_page: &PdfiumTextPage,
        index: i32,
        rect: &mut FS_RECTF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFText_GetLooseCharBox)(text_page.into(), index, rect) })
    }

    /// C documentation for FPDFText_GetMatrix:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFText_GetMatrix
    ///          Get the effective transformation matrix for a particular character.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage().
    ///          index       -   Zero-based index of the character.
    ///          matrix      -   Pointer to a FS_MATRIX receiving the transformation
    ///                          matrix.
    /// Return Value:
    ///          On success, return TRUE and fill in |matrix|. If |text_page| is
    ///          invalid, or if |index| is out of bounds, or if |matrix| is NULL,
    ///          then return FALSE, and |matrix| remains unmodified.
    /// ```
    #[inline]
    pub fn FPDFText_GetMatrix(
        &self,
        text_page: &PdfiumTextPage,
        index: i32,
        matrix: &mut FS_MATRIX,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFText_GetMatrix)(text_page.into(), index, matrix) })
    }

    /// C documentation for FPDFText_GetRect:
    ///
    /// ```text
    /// Function: FPDFText_GetRect
    ///          Get a rectangular area from the result generated by
    ///          FPDFText_CountRects.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          rect_index  -   Zero-based index for the rectangle.
    ///          left        -   Pointer to a double value receiving the rectangle
    ///                          left boundary.
    ///          top         -   Pointer to a double value receiving the rectangle
    ///                          top boundary.
    ///          right       -   Pointer to a double value receiving the rectangle
    ///                          right boundary.
    ///          bottom      -   Pointer to a double value receiving the rectangle
    ///                          bottom boundary.
    /// Return Value:
    ///          On success, return TRUE and fill in |left|, |top|, |right|, and
    ///          |bottom|. If |text_page| is invalid then return FALSE, and the out
    ///          parameters remain unmodified. If |text_page| is valid but
    ///          |rect_index| is out of bounds, then return FALSE and set the out
    ///          parameters to 0.
    /// ```
    #[inline]
    pub fn FPDFText_GetRect(
        &self,
        text_page: &PdfiumTextPage,
        rect_index: i32,
        left: &mut f64,
        top: &mut f64,
        right: &mut f64,
        bottom: &mut f64,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFText_GetRect)(text_page.into(), rect_index, left, top, right, bottom)
        })
    }

    /// C documentation for FPDFText_GetSchCount:
    ///
    /// ```text
    /// Function: FPDFText_GetSchCount
    ///          Get the number of matched characters in the search result.
    /// Parameters:
    ///          handle      -   A search context handle returned by
    ///                          FPDFText_FindStart.
    /// Return Value:
    ///          Number of matched characters.
    /// ```
    #[inline]
    pub fn FPDFText_GetSchCount(&self, handle: &PdfiumSearch) -> i32 {
        unsafe { (self.fn_FPDFText_GetSchCount)(handle.into()) }
    }

    /// C documentation for FPDFText_GetSchResultIndex:
    ///
    /// ```text
    /// Function: FPDFText_GetSchResultIndex
    ///          Get the starting character index of the search result.
    /// Parameters:
    ///          handle      -   A search context handle returned by
    ///                          FPDFText_FindStart.
    /// Return Value:
    ///          Index for the starting character.
    /// ```
    #[inline]
    pub fn FPDFText_GetSchResultIndex(&self, handle: &PdfiumSearch) -> i32 {
        unsafe { (self.fn_FPDFText_GetSchResultIndex)(handle.into()) }
    }

    /// C documentation for FPDFText_GetStrokeColor:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFText_GetStrokeColor
    ///          Get the stroke color of a particular character.
    /// Parameters:
    ///          text_page      -   Handle to a text page information structure.
    ///                             Returned by FPDFText_LoadPage function.
    ///          index          -   Zero-based index of the character.
    ///          R              -   Pointer to an unsigned int number receiving the
    ///                             red value of the stroke color.
    ///          G              -   Pointer to an unsigned int number receiving the
    ///                             green value of the stroke color.
    ///          B              -   Pointer to an unsigned int number receiving the
    ///                             blue value of the stroke color.
    ///          A              -   Pointer to an unsigned int number receiving the
    ///                             alpha value of the stroke color.
    /// Return value:
    ///          Whether the call succeeded. If false, |R|, |G|, |B| and |A| are
    ///          unchanged.
    /// ```
    #[inline]
    pub fn FPDFText_GetStrokeColor(
        &self,
        text_page: &PdfiumTextPage,
        index: i32,
        R: &mut u32,
        G: &mut u32,
        B: &mut u32,
        A: &mut u32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDFText_GetStrokeColor)(text_page.into(), index, R, G, B, A) })
    }

    /// C documentation for FPDFText_GetText:
    ///
    /// ```text
    /// Function: FPDFText_GetText
    ///          Extract unicode text string from the page.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          start_index -   Index for the start characters.
    ///          count       -   Number of UCS-2 values to be extracted.
    ///          result      -   A buffer (allocated by application) receiving the
    ///                          extracted UCS-2 values. The buffer must be able to
    ///                          hold `count` UCS-2 values plus a terminator.
    /// Return Value:
    ///          Number of characters written into the result buffer, including the
    ///          trailing terminator.
    /// Comments:
    ///          This function ignores characters without UCS-2 representations.
    ///          It considers all characters on the page, even those that are not
    ///          visible when the page has a cropbox. To filter out the characters
    ///          outside of the cropbox, use FPDF_GetPageBoundingBox() and
    ///          FPDFText_GetCharBox().
    /// ```
    #[inline]
    pub fn FPDFText_GetText(
        &self,
        text_page: &PdfiumTextPage,
        start_index: i32,
        count: i32,
        result: &mut c_ushort,
    ) -> i32 {
        unsafe { (self.fn_FPDFText_GetText)(text_page.into(), start_index, count, result) }
    }

    /// C documentation for FPDFText_GetTextIndexFromCharIndex:
    ///
    /// ```text
    /// Get the text index in |text_page| internal character list.
    ///
    ///   text_page  - a text page information structure.
    ///   nCharIndex - index of the character in internal character list.
    ///
    /// Returns the index of the text returned from FPDFText_GetText(). -1 for error.
    /// ```
    #[inline]
    pub fn FPDFText_GetTextIndexFromCharIndex(
        &self,
        text_page: &PdfiumTextPage,
        nCharIndex: i32,
    ) -> i32 {
        unsafe { (self.fn_FPDFText_GetTextIndexFromCharIndex)(text_page.into(), nCharIndex) }
    }

    /// C documentation for FPDFText_GetTextObject:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFText_GetTextObject
    ///          Get the FPDF_PAGEOBJECT associated with a given character.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          index       -   Zero-based index of the character.
    /// Return value:
    ///          The associated text object for the character at |index|, or NULL on
    ///          error. The returned text object, if non-null, is of type
    ///          |FPDF_PAGEOBJ_TEXT|. The caller does not own the returned object.
    /// ```
    #[inline]
    pub fn FPDFText_GetTextObject(
        &self,
        text_page: &PdfiumTextPage,
        index: i32,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDFText_GetTextObject)(text_page.into(), index)
        })
    }

    /// C documentation for FPDFText_GetUnicode:
    ///
    /// ```text
    /// Function: FPDFText_GetUnicode
    ///          Get Unicode of a character in a page.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          index       -   Zero-based index of the character.
    /// Return value:
    ///          The Unicode of the particular character.
    ///          If a character is not encoded in Unicode and Foxit engine can't
    ///          convert to Unicode,
    ///          the return value will be zero.
    /// ```
    #[inline]
    pub fn FPDFText_GetUnicode(&self, text_page: &PdfiumTextPage, index: i32) -> u32 {
        unsafe { (self.fn_FPDFText_GetUnicode)(text_page.into(), index) }
    }

    /// C documentation for FPDFText_HasUnicodeMapError:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFText_HasUnicodeMapError
    ///          Get if a character in a page has an invalid unicode mapping.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          index       -   Zero-based index of the character.
    /// Return value:
    ///          1 if the character has an invalid unicode mapping.
    ///          0 if the character has no known unicode mapping issues.
    ///          -1 if there was an error.
    /// ```
    #[inline]
    pub fn FPDFText_HasUnicodeMapError(&self, text_page: &PdfiumTextPage, index: i32) -> i32 {
        unsafe { (self.fn_FPDFText_HasUnicodeMapError)(text_page.into(), index) }
    }

    /// C documentation for FPDFText_IsGenerated:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFText_IsGenerated
    ///          Get if a character in a page is generated by PDFium.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          index       -   Zero-based index of the character.
    /// Return value:
    ///          1 if the character is generated by PDFium.
    ///          0 if the character is not generated by PDFium.
    ///          -1 if there was an error.
    /// ```
    #[inline]
    pub fn FPDFText_IsGenerated(&self, text_page: &PdfiumTextPage, index: i32) -> i32 {
        unsafe { (self.fn_FPDFText_IsGenerated)(text_page.into(), index) }
    }

    /// C documentation for FPDFText_IsHyphen:
    ///
    /// ```text
    /// Experimental API.
    /// Function: FPDFText_IsHyphen
    ///          Get if a character in a page is a hyphen.
    /// Parameters:
    ///          text_page   -   Handle to a text page information structure.
    ///                          Returned by FPDFText_LoadPage function.
    ///          index       -   Zero-based index of the character.
    /// Return value:
    ///          1 if the character is a hyphen.
    ///          0 if the character is not a hyphen.
    ///          -1 if there was an error.
    /// ```
    #[inline]
    pub fn FPDFText_IsHyphen(&self, text_page: &PdfiumTextPage, index: i32) -> i32 {
        unsafe { (self.fn_FPDFText_IsHyphen)(text_page.into(), index) }
    }

    /// C documentation for FPDFText_LoadCidType2Font:
    ///
    /// ```text
    /// Experimental API.
    /// Returns a font object loaded from a stream of data for a type 2 CID font. The
    /// font is loaded into the document. Unlike FPDFText_LoadFont(), the ToUnicode
    /// data and the CIDToGIDMap data are caller provided, instead of auto-generated.
    ///
    /// document                 - handle to the document.
    /// font_data                - the stream of font data, which will be copied by
    ///                            the font object.
    /// font_data_size           - the size of the font data, in bytes.
    /// to_unicode_cmap          - the ToUnicode data.
    /// cid_to_gid_map_data      - the stream of CIDToGIDMap data.
    /// cid_to_gid_map_data_size - the size of the CIDToGIDMap data, in bytes.
    ///
    /// The loaded font can be closed using FPDFFont_Close().
    ///
    /// Returns NULL on failure.
    /// ```
    #[inline]
    pub fn FPDFText_LoadCidType2Font(
        &self,
        document: &PdfiumDocument,
        font_data: &[u8],
        font_data_size: u32,
        to_unicode_cmap: &CString,
        cid_to_gid_map_data: &[u8],
        cid_to_gid_map_data_size: u32,
    ) -> PdfiumResult<PdfiumFont> {
        PdfiumFont::new_from_handle(unsafe {
            (self.fn_FPDFText_LoadCidType2Font)(
                document.into(),
                font_data.as_ptr(),
                font_data_size,
                to_unicode_cmap.as_ptr(),
                cid_to_gid_map_data.as_ptr(),
                cid_to_gid_map_data_size,
            )
        })
    }

    /// C documentation for FPDFText_LoadFont:
    ///
    /// ```text
    /// Returns a font object loaded from a stream of data. The font is loaded
    /// into the document. Various font data structures, such as the ToUnicode data,
    /// are auto-generated based on the inputs.
    ///
    /// document  - handle to the document.
    /// data      - the stream of font data, which will be copied by the font object.
    /// size      - the size of the font data, in bytes.
    /// font_type - FPDF_FONT_TYPE1 or FPDF_FONT_TRUETYPE depending on the font type.
    /// cid       - a boolean specifying if the font is a CID font or not.
    ///
    /// The loaded font can be closed using FPDFFont_Close().
    ///
    /// Returns NULL on failure
    /// ```
    #[inline]
    pub fn FPDFText_LoadFont(
        &self,
        document: &PdfiumDocument,
        data: &[u8],
        size: u32,
        font_type: i32,
        cid: i32,
    ) -> PdfiumResult<PdfiumFont> {
        PdfiumFont::new_from_handle(unsafe {
            (self.fn_FPDFText_LoadFont)(document.into(), data.as_ptr(), size, font_type, cid)
        })
    }

    /// C documentation for FPDFText_LoadPage:
    ///
    /// ```text
    /// Function: FPDFText_LoadPage
    ///          Prepare information about all characters in a page.
    /// Parameters:
    ///          page    -   Handle to the page. Returned by FPDF_LoadPage function
    ///                      (in FPDFVIEW module).
    /// Return value:
    ///          A handle to the text page information structure.
    ///          NULL if something goes wrong.
    /// Comments:
    ///          Application must call FPDFText_ClosePage to release the text page
    ///          information.
    /// ```
    #[inline]
    pub fn FPDFText_LoadPage(&self, page: &PdfiumPage) -> PdfiumResult<PdfiumTextPage> {
        PdfiumTextPage::new_from_handle(unsafe { (self.fn_FPDFText_LoadPage)(page.into()) })
    }

    /// C documentation for FPDFText_LoadStandardFont:
    ///
    /// ```text
    /// Experimental API.
    /// Loads one of the standard 14 fonts per PDF spec 1.7 page 416. The preferred
    /// way of using font style is using a dash to separate the name from the style,
    /// for example 'Helvetica-BoldItalic'.
    ///
    /// document   - handle to the document.
    /// font       - string containing the font name, without spaces.
    ///
    /// The loaded font can be closed using FPDFFont_Close().
    ///
    /// Returns NULL on failure.
    /// ```
    #[inline]
    pub fn FPDFText_LoadStandardFont(
        &self,
        document: &PdfiumDocument,
        font: &CString,
    ) -> PdfiumResult<PdfiumFont> {
        PdfiumFont::new_from_handle(unsafe {
            (self.fn_FPDFText_LoadStandardFont)(document.into(), font.as_ptr())
        })
    }

    /// C documentation for FPDFText_SetCharcodes:
    ///
    /// ```text
    /// Experimental API.
    /// Set the text using charcodes for a text object. If it had text, it will be
    /// replaced.
    ///
    /// text_object  - handle to the text object.
    /// charcodes    - pointer to an array of charcodes to be added.
    /// count        - number of elements in |charcodes|.
    ///
    /// Returns TRUE on success
    /// ```
    #[inline]
    pub fn FPDFText_SetCharcodes(
        &self,
        text_object: &PdfiumPageObject,
        charcodes: &[u32],
        count: usize,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDFText_SetCharcodes)(text_object.into(), charcodes.as_ptr(), count)
        })
    }

    /// C documentation for FPDFText_SetText:
    ///
    /// ```text
    /// Set the text for a text object. If it had text, it will be replaced.
    ///
    /// text_object  - handle to the text object.
    /// text         - the UTF-16LE encoded string containing the text to be added.
    ///
    /// Returns TRUE on success
    /// ```
    #[inline]
    pub fn FPDFText_SetText(&self, text_object: &PdfiumPageObject, text: &str) -> PdfiumResult<()> {
        let text = str_to_utf16le_vec(text);
        to_result(unsafe { (self.fn_FPDFText_SetText)(text_object.into(), text.as_ptr()) })
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
    #[inline]
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
    #[inline]
    pub fn FPDF_CopyViewerPreferences(
        &self,
        dest_doc: &PdfiumDocument,
        src_doc: &PdfiumDocument,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_CopyViewerPreferences)(dest_doc.into(), src_doc.into()) })
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
    #[inline]
    pub fn FPDF_CountNamedDests(&self, document: &PdfiumDocument) -> FPDF_DWORD {
        unsafe { (self.fn_FPDF_CountNamedDests)(document.into()) }
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
    #[inline]
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

    /// C documentation for FPDF_CreateNewDocument:
    ///
    /// ```text
    /// Create a new PDF document.
    ///
    /// Returns a handle to a new document, or NULL on failure.
    /// ```
    #[inline]
    pub fn FPDF_CreateNewDocument(&self) -> FPDF_DOCUMENT {
        unsafe { (self.fn_FPDF_CreateNewDocument)() }
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
    #[inline]
    pub fn FPDF_DestroyLibrary(&self) {
        unsafe { (self.fn_FPDF_DestroyLibrary)() }
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
    #[inline]
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
    #[inline]
    pub fn FPDF_DocumentHasValidCrossReferenceTable(
        &self,
        document: &PdfiumDocument,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_DocumentHasValidCrossReferenceTable)(document.into()) })
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
    #[inline]
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
    #[inline]
    pub fn FPDF_GetDefaultTTFMapCount(&self) -> usize {
        unsafe { (self.fn_FPDF_GetDefaultTTFMapCount)() }
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
    #[inline]
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
    #[inline]
    pub fn FPDF_GetDocUserPermissions(&self, document: &PdfiumDocument) -> c_ulong {
        unsafe { (self.fn_FPDF_GetDocUserPermissions)(document.into()) }
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
    #[inline]
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
    #[inline]
    pub fn FPDF_GetFileVersion(
        &self,
        doc: &PdfiumDocument,
        fileVersion: &mut i32,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_GetFileVersion)(doc.into(), fileVersion) })
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
    #[inline]
    pub fn FPDF_GetFormType(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDF_GetFormType)(document.into()) }
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
    #[inline]
    pub fn FPDF_GetLastError(&self) -> c_ulong {
        unsafe { (self.fn_FPDF_GetLastError)() }
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDF_GetNamedDestByName(
        &self,
        document: &PdfiumDocument,
        name: &CString,
    ) -> PdfiumResult<PdfiumDestination> {
        PdfiumDestination::new_from_handle(unsafe {
            (self.fn_FPDF_GetNamedDestByName)(document.into(), name.as_ptr())
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
    #[inline]
    pub fn FPDF_GetPageAAction(
        &self,
        page: &PdfiumPage,
        aa_type: i32,
    ) -> PdfiumResult<PdfiumAction> {
        PdfiumAction::new_from_handle(unsafe {
            (self.fn_FPDF_GetPageAAction)(page.into(), aa_type)
        })
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
    #[inline]
    pub fn FPDF_GetPageBoundingBox(
        &self,
        page: &PdfiumPage,
        rect: &mut FS_RECTF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_GetPageBoundingBox)(page.into(), rect) })
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
    #[inline]
    pub fn FPDF_GetPageCount(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDF_GetPageCount)(document.into()) }
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
    #[inline]
    pub fn FPDF_GetPageHeight(&self, page: &PdfiumPage) -> f64 {
        unsafe { (self.fn_FPDF_GetPageHeight)(page.into()) }
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
    #[inline]
    pub fn FPDF_GetPageHeightF(&self, page: &PdfiumPage) -> f32 {
        unsafe { (self.fn_FPDF_GetPageHeightF)(page.into()) }
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
    #[inline]
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
    #[inline]
    pub fn FPDF_GetPageSizeByIndex(
        &self,
        document: &PdfiumDocument,
        page_index: i32,
        width: &mut f64,
        height: &mut f64,
    ) -> i32 {
        unsafe { (self.fn_FPDF_GetPageSizeByIndex)(document.into(), page_index, width, height) }
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
    #[inline]
    pub fn FPDF_GetPageSizeByIndexF(
        &self,
        document: &PdfiumDocument,
        page_index: i32,
        size: &mut FS_SIZEF,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_GetPageSizeByIndexF)(document.into(), page_index, size) })
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
    #[inline]
    pub fn FPDF_GetPageWidth(&self, page: &PdfiumPage) -> f64 {
        unsafe { (self.fn_FPDF_GetPageWidth)(page.into()) }
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
    #[inline]
    pub fn FPDF_GetPageWidthF(&self, page: &PdfiumPage) -> f32 {
        unsafe { (self.fn_FPDF_GetPageWidthF)(page.into()) }
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
    #[inline]
    pub fn FPDF_GetSecurityHandlerRevision(&self, document: &PdfiumDocument) -> i32 {
        unsafe { (self.fn_FPDF_GetSecurityHandlerRevision)(document.into()) }
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
    #[inline]
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
    #[inline]
    pub fn FPDF_GetSignatureObject(
        &self,
        document: &PdfiumDocument,
        index: i32,
    ) -> PdfiumResult<PdfiumSignature> {
        PdfiumSignature::new_from_handle(unsafe {
            (self.fn_FPDF_GetSignatureObject)(document.into(), index)
        })
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
    #[inline]
    pub fn FPDF_GetTrailerEnds(
        &self,
        document: &PdfiumDocument,
        buffer: &mut u32,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.fn_FPDF_GetTrailerEnds)(document.into(), buffer, length) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDF_InitLibrary(&self) {
        unsafe { (self.fn_FPDF_InitLibrary)() }
    }

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
    #[inline]
    pub fn FPDF_InitLibraryWithConfig(&self, config: &FPDF_LIBRARY_CONFIG) {
        unsafe { (self.fn_FPDF_InitLibraryWithConfig)(config) }
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
    #[inline]
    pub fn FPDF_LoadCustomDocument(
        &self,
        pFileAccess: &mut Box<PdfiumReader>,
        password: &CString,
    ) -> FPDF_DOCUMENT {
        unsafe { (self.fn_FPDF_LoadCustomDocument)(pFileAccess.as_mut().into(), password.as_ptr()) }
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
    #[inline]
    pub fn FPDF_LoadPage(
        &self,
        document: &PdfiumDocument,
        page_index: i32,
    ) -> PdfiumResult<PdfiumPage> {
        PdfiumPage::new_from_handle(unsafe { (self.fn_FPDF_LoadPage)(document.into(), page_index) })
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
    #[inline]
    pub fn FPDF_LoadXFA(&self, document: &PdfiumDocument) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_LoadXFA)(document.into()) })
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
    #[inline]
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

    /// C documentation for FPDF_NewFormObjectFromXObject:
    ///
    /// ```text
    /// Experimental API.
    /// Create a new form object from an FPDF_XOBJECT object.
    ///
    /// Returns a new form object on success, or NULL on failure. Caller owns the
    /// newly created object.
    /// ```
    #[inline]
    pub fn FPDF_NewFormObjectFromXObject(
        &self,
        xobject: &PdfiumXObject,
    ) -> PdfiumResult<PdfiumPageObject> {
        PdfiumPageObject::new_from_handle(unsafe {
            (self.fn_FPDF_NewFormObjectFromXObject)(xobject.into())
        })
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDF_RemoveFormFieldHighlight(&self, hHandle: &PdfiumForm) {
        unsafe { (self.fn_FPDF_RemoveFormFieldHighlight)(hHandle.into()) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDF_RenderPage_Close(&self, page: &PdfiumPage) {
        unsafe { (self.fn_FPDF_RenderPage_Close)(page.into()) }
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
    #[inline]
    pub fn FPDF_RenderPage_Continue(&self, page: &PdfiumPage, pause: &mut IFSDK_PAUSE) -> i32 {
        unsafe { (self.fn_FPDF_RenderPage_Continue)(page.into(), pause) }
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
    #[inline]
    pub fn FPDF_SetFormFieldHighlightAlpha(
        &self,
        hHandle: &PdfiumForm,
        alpha: ::std::os::raw::c_uchar,
    ) {
        unsafe { (self.fn_FPDF_SetFormFieldHighlightAlpha)(hHandle.into(), alpha) }
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
    #[inline]
    pub fn FPDF_SetFormFieldHighlightColor(
        &self,
        hHandle: &PdfiumForm,
        fieldType: i32,
        color: c_ulong,
    ) {
        unsafe { (self.fn_FPDF_SetFormFieldHighlightColor)(hHandle.into(), fieldType, color) }
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
    #[inline]
    pub fn FPDF_SetSandBoxPolicy(&self, policy: FPDF_DWORD, enable: i32) {
        unsafe { (self.fn_FPDF_SetSandBoxPolicy)(policy, enable) }
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
    #[inline]
    pub fn FPDF_StructElement_Attr_CountChildren(
        &self,
        value: &PdfiumStructElementAttrValue,
    ) -> i32 {
        unsafe { (self.fn_FPDF_StructElement_Attr_CountChildren)(value.into()) }
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
    #[inline]
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
    #[inline]
    pub fn FPDF_StructElement_Attr_GetBooleanValue(
        &self,
        value: &PdfiumStructElementAttrValue,
        out_value: &mut FPDF_BOOL,
    ) -> PdfiumResult<()> {
        to_result(unsafe {
            (self.fn_FPDF_StructElement_Attr_GetBooleanValue)(value.into(), out_value)
        })
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
    #[inline]
    pub fn FPDF_StructElement_Attr_GetChildAtIndex(
        &self,
        value: &PdfiumStructElementAttrValue,
        index: i32,
    ) -> PdfiumResult<PdfiumStructElementAttrValue> {
        PdfiumStructElementAttrValue::new_from_handle(unsafe {
            (self.fn_FPDF_StructElement_Attr_GetChildAtIndex)(value.into(), index)
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDF_StructElement_Attr_GetType(
        &self,
        value: &PdfiumStructElementAttrValue,
    ) -> FPDF_OBJECT_TYPE {
        unsafe { (self.fn_FPDF_StructElement_Attr_GetType)(value.into()) }
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
    #[inline]
    pub fn FPDF_StructElement_Attr_GetValue(
        &self,
        struct_attribute: &PdfiumStructElementAttr,
        name: &CString,
    ) -> PdfiumResult<PdfiumStructElementAttrValue> {
        PdfiumStructElementAttrValue::new_from_handle(unsafe {
            (self.fn_FPDF_StructElement_Attr_GetValue)(struct_attribute.into(), name.as_ptr())
        })
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
    #[inline]
    pub fn FPDF_StructElement_CountChildren(&self, struct_element: &PdfiumStructElement) -> i32 {
        unsafe { (self.fn_FPDF_StructElement_CountChildren)(struct_element.into()) }
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDF_StructElement_GetAttributeAtIndex(
        &self,
        struct_element: &PdfiumStructElement,
        index: i32,
    ) -> PdfiumResult<PdfiumStructElementAttr> {
        PdfiumStructElementAttr::new_from_handle(unsafe {
            (self.fn_FPDF_StructElement_GetAttributeAtIndex)(struct_element.into(), index)
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
    #[inline]
    pub fn FPDF_StructElement_GetAttributeCount(
        &self,
        struct_element: &PdfiumStructElement,
    ) -> i32 {
        unsafe { (self.fn_FPDF_StructElement_GetAttributeCount)(struct_element.into()) }
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
    #[inline]
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
    #[inline]
    pub fn FPDF_StructElement_GetChildMarkedContentID(
        &self,
        struct_element: &PdfiumStructElement,
        index: i32,
    ) -> i32 {
        unsafe {
            (self.fn_FPDF_StructElement_GetChildMarkedContentID)(struct_element.into(), index)
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDF_StructElement_GetMarkedContentID(
        &self,
        struct_element: &PdfiumStructElement,
    ) -> i32 {
        unsafe { (self.fn_FPDF_StructElement_GetMarkedContentID)(struct_element.into()) }
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
    #[inline]
    pub fn FPDF_StructElement_GetMarkedContentIdAtIndex(
        &self,
        struct_element: &PdfiumStructElement,
        index: i32,
    ) -> i32 {
        unsafe {
            (self.fn_FPDF_StructElement_GetMarkedContentIdAtIndex)(struct_element.into(), index)
        }
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
    #[inline]
    pub fn FPDF_StructElement_GetMarkedContentIdCount(
        &self,
        struct_element: &PdfiumStructElement,
    ) -> i32 {
        unsafe { (self.fn_FPDF_StructElement_GetMarkedContentIdCount)(struct_element.into()) }
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
    #[inline]
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
    #[inline]
    pub fn FPDF_StructElement_GetParent(
        &self,
        struct_element: &PdfiumStructElement,
    ) -> PdfiumResult<PdfiumStructElement> {
        PdfiumStructElement::new_from_handle(unsafe {
            (self.fn_FPDF_StructElement_GetParent)(struct_element.into())
        })
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDF_StructTree_GetChildAtIndex(
        &self,
        struct_tree: &PdfiumStructTree,
        index: i32,
    ) -> PdfiumResult<PdfiumStructElement> {
        PdfiumStructElement::new_from_handle(unsafe {
            (self.fn_FPDF_StructTree_GetChildAtIndex)(struct_tree.into(), index)
        })
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
    #[inline]
    pub fn FPDF_StructTree_GetForPage(&self, page: &PdfiumPage) -> PdfiumResult<PdfiumStructTree> {
        PdfiumStructTree::new_from_handle(unsafe {
            (self.fn_FPDF_StructTree_GetForPage)(page.into())
        })
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn FPDF_VIEWERREF_GetPrintPageRangeElement(
        &self,
        pagerange: &PdfiumPageRange,
        index: usize,
    ) -> i32 {
        unsafe { (self.fn_FPDF_VIEWERREF_GetPrintPageRangeElement)(pagerange.into(), index) }
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
    #[inline]
    pub fn FPDF_VIEWERREF_GetPrintScaling(&self, document: &PdfiumDocument) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FPDF_VIEWERREF_GetPrintScaling)(document.into()) })
    }

    /// C documentation for FSDK_SetUnSpObjProcessHandler:
    ///
    /// ```text
    /// Setup an unsupported object handler.
    ///
    ///   unsp_info - Pointer to an UNSUPPORT_INFO structure.
    ///
    /// Returns TRUE on success.
    /// ```
    #[inline]
    pub fn FSDK_SetUnSpObjProcessHandler(
        &self,
        unsp_info: &mut UNSUPPORT_INFO,
    ) -> PdfiumResult<()> {
        to_result(unsafe { (self.fn_FSDK_SetUnSpObjProcessHandler)(unsp_info) })
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

pub fn to_result(b: FPDF_BOOL) -> PdfiumResult<()> {
    if b == 0 {
        Err(PdfiumError::InvokationFailed)
    } else {
        Ok(())
    }
}

fn str_to_utf16le_vec(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
