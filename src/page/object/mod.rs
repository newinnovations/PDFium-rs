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

pub mod mark;
pub mod objects;

use std::{ffi::CString, os::raw::c_ulong};

use crate::{
    error::{PdfiumError, PdfiumResult},
    lib,
    pdfium_constants::{
        FPDF_PAGEOBJ_FORM, FPDF_PAGEOBJ_IMAGE, FPDF_PAGEOBJ_PATH, FPDF_PAGEOBJ_SHADING,
        FPDF_PAGEOBJ_TEXT, FPDF_PAGEOBJ_UNKNOWN,
    },
    pdfium_types::{
        Handle, PageObjectHandle, FPDF_BOOL, FPDF_PAGEOBJECT, FS_MATRIX, FS_QUADPOINTSF,
    },
    PdfiumClipPath, PdfiumPage, PdfiumPageObjectMark,
};

/// # Rust interface to FPDF_PAGEOBJECT
#[derive(Debug, Clone)]
pub struct PdfiumPageObject {
    handle: PageObjectHandle,
    owner: Option<PdfiumPage>,
}

impl PdfiumPageObject {
    pub(crate) fn new_from_handle(handle: FPDF_PAGEOBJECT) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            Ok(Self {
                handle: Handle::new(handle, Some(close_page_object)),
                owner: None,
            })
        }
    }

    pub(crate) fn set_owner(&mut self, owner: PdfiumPage) {
        self.owner = Some(owner);
    }

    /// Removes a property from a content mark by key.
    ///
    /// page_object - handle to the page object with the mark.
    /// mark        - handle to a content mark.
    /// key         - string key of the property.
    ///
    /// Returns TRUE if the operation succeeded, FALSE otherwise.
    pub fn remove_param(&self, mark: &PdfiumPageObjectMark, key: &CString) -> PdfiumResult<()> {
        lib().FPDFPageObjMark_RemoveParam(self, mark, key)
    }

    /// Add a new content mark to a this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// name        - the name (tag) of the mark.
    ///
    /// Returns the handle to the content mark, or NULL on failure. The handle is
    /// still owned by the library, and it should not be freed directly. It becomes
    /// invalid if the page object is destroyed, either directly or indirectly by
    /// unloading the page.
    pub fn add_mark(&self, name: &CString) -> PdfiumResult<PdfiumPageObjectMark> {
        lib().FPDFPageObj_AddMark(self, name)
    }

    /// Get number of content marks in this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    ///
    /// Returns the number of content marks in this [`PdfiumPageObject`], or -1 in case of
    /// failure.
    pub fn count_marks(&self) -> i32 {
        lib().FPDFPageObj_CountMarks(self)
    }

    /// Get the bounding box of this [`PdfiumPageObject`].
    ///
    /// page_object  - handle to this [`PdfiumPageObject`].
    /// left         - pointer where the left coordinate will be stored
    /// bottom       - pointer where the bottom coordinate will be stored
    /// right        - pointer where the right coordinate will be stored
    /// top          - pointer where the top coordinate will be stored
    ///
    /// On success, returns TRUE and fills in the 4 coordinates.
    pub fn get_bounds(
        &self,
        left: &mut f32,
        bottom: &mut f32,
        right: &mut f32,
        top: &mut f32,
    ) -> PdfiumResult<()> {
        lib().FPDFPageObj_GetBounds(self, left, bottom, right, top)
    }

    /// Get the clip path of the page object.
    ///
    /// page object - Handle to this [`PdfiumPageObject`]. Returned by e.g.
    /// FPDFPage_GetObject().
    ///
    /// Returns the handle to the clip path, or NULL on failure. The caller does not
    /// take ownership of the returned FPDF_CLIPPATH. Instead, it remains valid until
    /// FPDF_ClosePage() is called for the page containing this [`PdfiumPageObject`].
    pub fn get_clip_path(&self) -> PdfiumResult<PdfiumClipPath> {
        lib().FPDFPageObj_GetClipPath(self)
    }

    /// Get the line dash array of this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// dash_array - pointer where the dashing array will be stored.
    /// dash_count - number of elements in `dash_array`.
    ///
    /// Returns TRUE on success.
    pub fn get_dash_array(&self, dash_array: &mut f32, dash_count: usize) -> PdfiumResult<()> {
        lib().FPDFPageObj_GetDashArray(self, dash_array, dash_count)
    }

    /// Get the line dash array of this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    ///
    /// Returns the line dash array size or -1 on failure.
    pub fn get_dash_count(&self) -> i32 {
        lib().FPDFPageObj_GetDashCount(self)
    }

    /// Get the line dash `phase` of this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// phase - pointer where the dashing phase will be stored.
    ///
    /// Returns TRUE on success.
    pub fn get_dash_phase(&self, phase: &mut f32) -> PdfiumResult<()> {
        lib().FPDFPageObj_GetDashPhase(self, phase)
    }

    /// Get the fill RGBA of this [`PdfiumPageObject`]. Range of values: 0 - 255.
    ///
    /// page_object  - the handle to the page object.
    /// R            - the red component of the object's fill color.
    /// G            - the green component of the object's fill color.
    /// B            - the blue component of the object's fill color.
    /// A            - the fill alpha of the object.
    ///
    /// Returns TRUE on success.
    pub fn get_fill_color(
        &self,
        r: &mut u32,
        g: &mut u32,
        b: &mut u32,
        a: &mut u32,
    ) -> PdfiumResult<()> {
        lib().FPDFPageObj_GetFillColor(self, r, g, b, a)
    }

    /// Gets active state for this [`PdfiumPageObject`] within page.
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// active      - pointer to variable that will receive if the page object is
    /// active. This is a required parameter. Not filled if FALSE
    /// is returned.
    ///
    /// For page objects where `active` is filled with FALSE, the this [`PdfiumPageObject`] is
    /// treated as if it wasn't in the document even though it is still held
    /// internally.
    ///
    /// Returns TRUE if the operation succeeded, FALSE if it failed.
    pub fn get_is_active(&self, active: &mut FPDF_BOOL) -> PdfiumResult<()> {
        lib().FPDFPageObj_GetIsActive(self, active)
    }

    /// Get the line cap of this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    ///
    /// Returns the line cap, or -1 on failure.
    /// Line cap can be one of following: FPDF_LINECAP_BUTT, FPDF_LINECAP_ROUND,
    /// FPDF_LINECAP_PROJECTING_SQUARE
    pub fn get_line_cap(&self) -> i32 {
        lib().FPDFPageObj_GetLineCap(self)
    }

    /// Get the line join of this [`PdfiumPageObject`].
    ///
    /// page_object  - handle to this [`PdfiumPageObject`].
    ///
    /// Returns the line join, or -1 on failure.
    /// Line join can be one of following: FPDF_LINEJOIN_MITER, FPDF_LINEJOIN_ROUND,
    /// FPDF_LINEJOIN_BEVEL
    pub fn get_line_join(&self) -> i32 {
        lib().FPDFPageObj_GetLineJoin(self)
    }

    /// Get content mark in this [`PdfiumPageObject`] at `index`.
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// index       - the index of this [`PdfiumPageObject`].
    ///
    /// Returns the handle to the content mark, or NULL on failure. The handle is
    /// still owned by the library, and it should not be freed directly. It becomes
    /// invalid if the page object is destroyed, either directly or indirectly by
    /// unloading the page.
    pub fn get_mark(&self, index: c_ulong) -> PdfiumResult<PdfiumPageObjectMark> {
        lib().FPDFPageObj_GetMark(self, index)
    }

    /// Get the marked content ID for the object.
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    ///
    /// Returns the page object's marked content ID, or -1 on error.
    pub fn get_marked_content_id(&self) -> i32 {
        lib().FPDFPageObj_GetMarkedContentID(self)
    }

    /// Get the transform matrix of this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// matrix      - pointer to struct to receive the matrix value.
    ///
    /// The matrix is composed as:
    /// `a c e`
    /// `b d f`
    /// and used to scale, rotate, shear and translate the page object.
    ///
    /// For page objects outside form objects, the matrix values are relative to the
    /// page that contains it.
    /// For page objects inside form objects, the matrix values are relative to the
    /// form that contains it.
    ///
    /// Returns TRUE on success.
    pub fn get_matrix(&self, matrix: &mut FS_MATRIX) -> PdfiumResult<()> {
        lib().FPDFPageObj_GetMatrix(self, matrix)
    }

    /// Get the quad points that bounds this [`PdfiumPageObject`].
    ///
    /// page_object  - handle to this [`PdfiumPageObject`].
    /// quad_points  - pointer where the quadrilateral points will be stored.
    ///
    /// On success, returns TRUE and fills in `quad_points`.
    ///
    /// Similar to FPDFPageObj_GetBounds(), this returns the bounds of a page
    /// object. When the object is rotated by a non-multiple of 90 degrees, this API
    /// returns a tighter bound that cannot be represented with just the 4 sides of
    /// a rectangle.
    ///
    /// Currently only works the following this [`PdfiumPageObject`] types: FPDF_PAGEOBJ_TEXT and
    /// FPDF_PAGEOBJ_IMAGE.
    pub fn get_rotated_bounds(&self, quad_points: &mut FS_QUADPOINTSF) -> PdfiumResult<()> {
        lib().FPDFPageObj_GetRotatedBounds(self, quad_points)
    }

    /// Get the stroke RGBA of this [`PdfiumPageObject`]. Range of values: 0 - 255.
    ///
    /// page_object  - the handle to the page object.
    /// R            - the red component of the path stroke color.
    /// G            - the green component of the object's stroke color.
    /// B            - the blue component of the object's stroke color.
    /// A            - the stroke alpha of the object.
    ///
    /// Returns TRUE on success.
    pub fn get_stroke_color(
        &self,
        r: &mut u32,
        g: &mut u32,
        b: &mut u32,
        a: &mut u32,
    ) -> PdfiumResult<()> {
        lib().FPDFPageObj_GetStrokeColor(self, r, g, b, a)
    }

    /// Get the stroke width of this [`PdfiumPageObject`].
    ///
    /// path   - the handle to the page object.
    /// width  - the width of the stroke.
    ///
    /// Returns TRUE on success
    pub fn get_stroke_width(&self, width: &mut f32) -> PdfiumResult<()> {
        lib().FPDFPageObj_GetStrokeWidth(self, width)
    }

    /// Get type of this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    ///
    /// Returns one of the FPDF_PAGEOBJ_* values on success, FPDF_PAGEOBJ_UNKNOWN on
    /// error.
    pub fn get_type(&self) -> ObjectType {
        lib().FPDFPageObj_GetType(self).into()
    }

    /// Checks if this [`PdfiumPageObject`] contains transparency.
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    ///
    /// Returns TRUE if this [`PdfiumPageObject`] contains transparency.
    pub fn has_transparency(&self) -> i32 {
        lib().FPDFPageObj_HasTransparency(self)
    }

    /// Removes a content `mark` from a this [`PdfiumPageObject`].
    /// The mark handle will be invalid after the removal.
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// mark        - handle to a content mark in that object to remove.
    ///
    /// Returns TRUE if the operation succeeded, FALSE if it failed.
    pub fn remove_mark(&self, mark: &PdfiumPageObjectMark) -> PdfiumResult<()> {
        lib().FPDFPageObj_RemoveMark(self, mark)
    }

    /// Set the blend mode of this [`PdfiumPageObject`].
    ///
    /// page_object  - handle to this [`PdfiumPageObject`].
    /// blend_mode   - string containing the blend mode.
    ///
    /// Blend mode can be one of following: Color, ColorBurn, ColorDodge, Darken,
    /// Difference, Exclusion, HardLight, Hue, Lighten, Luminosity, Multiply, Normal,
    /// Overlay, Saturation, Screen, SoftLight
    pub fn set_blend_mode(&self, blend_mode: &CString) {
        lib().FPDFPageObj_SetBlendMode(self, blend_mode)
    }

    /// Set the line dash array of this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// dash_array - the dash array.
    /// dash_count - number of elements in `dash_array`.
    /// phase - the line dash phase.
    ///
    /// Returns TRUE on success.
    pub fn set_dash_array(
        &self,
        dash_array: &[f32],
        dash_count: usize,
        phase: f32,
    ) -> PdfiumResult<()> {
        lib().FPDFPageObj_SetDashArray(self, dash_array, dash_count, phase)
    }

    /// Set the line dash phase of this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// phase - line dash phase.
    ///
    /// Returns TRUE on success.
    pub fn set_dash_phase(&self, phase: f32) -> PdfiumResult<()> {
        lib().FPDFPageObj_SetDashPhase(self, phase)
    }

    /// Set the fill RGBA of this [`PdfiumPageObject`]. Range of values: 0 - 255.
    ///
    /// page_object  - the handle to the page object.
    /// R            - the red component for the object's fill color.
    /// G            - the green component for the object's fill color.
    /// B            - the blue component for the object's fill color.
    /// A            - the fill alpha for the object.
    ///
    /// Returns TRUE on success.
    pub fn set_fill_color(&self, r: u32, g: u32, b: u32, a: u32) -> PdfiumResult<()> {
        lib().FPDFPageObj_SetFillColor(self, r, g, b, a)
    }

    /// Sets if this [`PdfiumPageObject`] is active within page.
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// active      - a boolean specifying if the object is active.
    ///
    /// Returns TRUE on success.
    ///
    /// Page objects all start in the active state by default, and remain in that
    /// state unless this function is called.
    ///
    /// When `active` is false, this makes the this [`PdfiumPageObject`] be treated as if it
    /// wasn't in the document even though it is still held internally.
    pub fn set_is_active(&self, active: i32) -> PdfiumResult<()> {
        lib().FPDFPageObj_SetIsActive(self, active)
    }

    /// Set the line cap of this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// line_cap    - line cap
    ///
    /// Line cap can be one of following: FPDF_LINECAP_BUTT, FPDF_LINECAP_ROUND,
    /// FPDF_LINECAP_PROJECTING_SQUARE
    pub fn set_line_cap(&self, line_cap: i32) -> PdfiumResult<()> {
        lib().FPDFPageObj_SetLineCap(self, line_cap)
    }

    /// Set the line join of this [`PdfiumPageObject`].
    ///
    /// page_object  - handle to this [`PdfiumPageObject`].
    /// line_join    - line join
    ///
    /// Line join can be one of following: FPDF_LINEJOIN_MITER, FPDF_LINEJOIN_ROUND,
    /// FPDF_LINEJOIN_BEVEL
    pub fn set_line_join(&self, line_join: i32) -> PdfiumResult<()> {
        lib().FPDFPageObj_SetLineJoin(self, line_join)
    }

    /// Set the transform matrix of this [`PdfiumPageObject`].
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// matrix      - pointer to struct with the matrix value.
    ///
    /// The matrix is composed as:
    /// `a c e`
    /// `b d f`
    /// and can be used to scale, rotate, shear and translate the page object.
    ///
    /// Returns TRUE on success.
    pub fn set_matrix(&self, matrix: &FS_MATRIX) -> PdfiumResult<()> {
        lib().FPDFPageObj_SetMatrix(self, matrix)
    }

    /// Set the stroke RGBA of this [`PdfiumPageObject`]. Range of values: 0 - 255.
    ///
    /// page_object  - the handle to the page object.
    /// R            - the red component for the object's stroke color.
    /// G            - the green component for the object's stroke color.
    /// B            - the blue component for the object's stroke color.
    /// A            - the stroke alpha for the object.
    ///
    /// Returns TRUE on success.
    pub fn set_stroke_color(&self, r: u32, g: u32, b: u32, a: u32) -> PdfiumResult<()> {
        lib().FPDFPageObj_SetStrokeColor(self, r, g, b, a)
    }

    /// Set the stroke width of this [`PdfiumPageObject`].
    ///
    /// path   - the handle to the page object.
    /// width  - the width of the stroke.
    ///
    /// Returns TRUE on success
    pub fn set_stroke_width(&self, width: f32) -> PdfiumResult<()> {
        lib().FPDFPageObj_SetStrokeWidth(self, width)
    }

    /// Transform this [`PdfiumPageObject`] by the given matrix.
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// a           - matrix value.
    /// b           - matrix value.
    /// c           - matrix value.
    /// d           - matrix value.
    /// e           - matrix value.
    /// f           - matrix value.
    ///
    /// The matrix is composed as:
    /// `a c e`
    /// `b d f`
    /// and can be used to scale, rotate, shear and translate the this [`PdfiumPageObject`].
    pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        lib().FPDFPageObj_Transform(self, a, b, c, d, e, f)
    }

    /// Transform (scale, rotate, shear, move) the clip path of page object.
    /// page_object - Handle to this [`PdfiumPageObject`]. Returned by
    /// FPDFPageObj_NewImageObj().
    ///
    /// a  - The coefficient "a" of the matrix.
    /// b  - The coefficient "b" of the matrix.
    /// c  - The coefficient "c" of the matrix.
    /// d  - The coefficient "d" of the matrix.
    /// e  - The coefficient "e" of the matrix.
    /// f  - The coefficient "f" of the matrix.
    pub fn transform_clip_path(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        lib().FPDFPageObj_TransformClipPath(self, a, b, c, d, e, f)
    }

    /// Transform this [`PdfiumPageObject`] by the given matrix.
    ///
    /// page_object - handle to this [`PdfiumPageObject`].
    /// matrix      - the transform matrix.
    ///
    /// Returns TRUE on success.
    ///
    /// This can be used to scale, rotate, shear and translate the this [`PdfiumPageObject`].
    /// It is an improved version of FPDFPageObj_Transform() that does not do
    /// unnecessary double to float conversions, and only uses 1 parameter for the
    /// matrix. It also returns whether the operation succeeded or not.
    pub fn transform_f(&self, matrix: &FS_MATRIX) -> PdfiumResult<()> {
        lib().FPDFPageObj_TransformF(self, matrix)
    }
}

impl From<&PdfiumPageObject> for FPDF_PAGEOBJECT {
    fn from(page_object: &PdfiumPageObject) -> Self {
        page_object.handle.handle()
    }
}

fn close_page_object(_page_object: FPDF_PAGEOBJECT) {
    // Destroy `page_object` by releasing its resources. `page_object` must have
    // been created by FPDFPageObj_CreateNew{Path`Rect}() or
    // FPDFPageObj_New{Text`Image}Obj(). This function must be called on
    // newly-created objects if they are not added to a page through
    // FPDFPage_InsertObject() or to an annotation through FPDFAnnot_AppendObject().
    // lib().FPDFPageObj_Destroy(page_object);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum ObjectType {
    Unsupported = FPDF_PAGEOBJ_UNKNOWN,
    Text = FPDF_PAGEOBJ_TEXT,
    Path = FPDF_PAGEOBJ_PATH,
    Image = FPDF_PAGEOBJ_IMAGE,
    Shading = FPDF_PAGEOBJ_SHADING,
    Form = FPDF_PAGEOBJ_FORM,
}

impl From<i32> for ObjectType {
    fn from(value: i32) -> Self {
        match value {
            FPDF_PAGEOBJ_TEXT => ObjectType::Text,
            FPDF_PAGEOBJ_PATH => ObjectType::Path,
            FPDF_PAGEOBJ_IMAGE => ObjectType::Image,
            FPDF_PAGEOBJ_SHADING => ObjectType::Shading,
            FPDF_PAGEOBJ_FORM => ObjectType::Form,
            _ => ObjectType::Unsupported,
        }
    }
}

impl From<ObjectType> for i32 {
    fn from(value: ObjectType) -> Self {
        match value {
            ObjectType::Text => FPDF_PAGEOBJ_TEXT,
            ObjectType::Path => FPDF_PAGEOBJ_PATH,
            ObjectType::Image => FPDF_PAGEOBJ_IMAGE,
            ObjectType::Shading => FPDF_PAGEOBJ_SHADING,
            ObjectType::Form => FPDF_PAGEOBJ_FORM,
            ObjectType::Unsupported => FPDF_PAGEOBJ_UNKNOWN,
        }
    }
}
