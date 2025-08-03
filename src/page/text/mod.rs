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

pub mod search;

use std::{
    os::raw::{c_ulong, c_ushort},
    vec,
};

use crate::{
    c_api::{i32_to_bool_result, i32_to_result},
    error::{PdfiumError, PdfiumResult},
    lib,
    page::text::search::PdfiumSearchFlags,
    pdfium_types::{Handle, TextPageHandle, FPDF_TEXTPAGE, FS_MATRIX, FS_RECTF},
    PdfiumPageLink, PdfiumPageObject, PdfiumRect, PdfiumSearch,
};

/// # Rust interface to FPDF_TEXTPAGE
#[derive(Debug, Clone)]
pub struct PdfiumTextPage {
    handle: TextPageHandle,
}

impl PdfiumTextPage {
    pub(crate) fn new_from_handle(handle: FPDF_TEXTPAGE) -> PdfiumResult<Self> {
        if handle.is_null() {
            Err(PdfiumError::NullHandle)
        } else {
            Ok(Self {
                handle: Handle::new(handle, Some(close_text_page)),
            })
        }
    }

    /// Get information about weblinks in a page.
    ///
    /// Comments:
    /// * Weblinks are those links implicitly embedded in PDF pages. PDF also
    ///   has a type of annotation called "link" (FPDFTEXT doesn't deal with
    ///   that kind of link). FPDFTEXT weblink feature is useful for
    ///   automatically detecting links in the page contents. For example,
    ///   things like <https://www.example.com> will be detected, so
    ///   applications can allow user to click on those characters to activate
    ///   the link, even the PDF doesn't come with link annotations.
    pub fn load_web_links(&self) -> PdfiumResult<PdfiumPageLink> {
        lib().FPDFLink_LoadWebLinks(self)
    }

    /// Get number of characters in a page.
    ///
    /// Generated characters, like additional space characters, new line
    /// characters, are also counted.
    ///
    /// Comments:
    /// * Characters in a page form a "stream", inside the stream, each
    ///   character has an index. We will use the index parameters in many
    ///   of FPDFTEXT functions. The first character in the page has an index
    ///   value of zero.
    pub fn char_count(&self) -> PdfiumResult<i32> {
        i32_to_result(lib().FPDFText_CountChars(self))
    }

    /// Counts number of rectangular areas occupied by a segment of text
    ///
    /// Parameters:
    /// * start_index -   Index for the start character.
    /// * count       -   Number of characters, or -1 for all remaining.
    ///
    /// Returns:
    /// * Number of rectangles, Err -1 on bad start_index.
    ///
    /// Comments:
    /// * This function, along with FPDFText_GetRect can be used by
    ///   applications to detect the position on the page for a text segment,
    ///   so proper areas can be highlighted. The FPDFText_* functions will
    ///   automatically merge small character boxes into bigger one if those
    ///   characters are on the same line and use same font settings.
    /// * Caches the result for subsequent FPDFText_GetRect() calls.
    pub fn count_rects(&self, start_index: i32, count: i32) -> PdfiumResult<i32> {
        i32_to_result(lib().FPDFText_CountRects(self, start_index, count))
    }

    /// Start a search.
    ///
    /// Parameters:
    /// * findwhat    -   A unicode match pattern.
    /// * flags       -   Option flags.
    /// * start_index -   Start from this character. -1 for end of the page.
    pub fn find_start(
        &self,
        findwhat: &str,
        flags: PdfiumSearchFlags,
        start_index: i32,
    ) -> PdfiumResult<PdfiumSearch> {
        lib().FPDFText_FindStart(self, findwhat, flags.bits() as c_ulong, start_index)
    }

    /// Function: FPDFText_GetBoundedText
    /// Extract unicode text within a rectangular boundary on the page.
    /// Parameters:
    /// text_page   -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// left        -   Left boundary.
    /// top         -   Top boundary.
    /// right       -   Right boundary.
    /// bottom      -   Bottom boundary.
    /// buffer      -   Caller-allocated buffer to receive UTF-16 values.
    /// buflen      -   Number of UTF-16 values (not bytes) that `buffer`
    /// is capable of holding.
    /// Returns:
    /// If buffer is NULL or buflen is zero, return number of UTF-16
    /// values (not bytes) of text present within the rectangle, excluding
    /// a terminating NUL. Generally you should pass a buffer at least one
    /// larger than this if you want a terminating NUL, which will be
    /// provided if space is available. Otherwise, return number of UTF-16
    /// values copied into the buffer, including the terminating NUL when
    /// space for it is available.
    /// Comment:
    /// If the buffer is too small, as much text as will fit is copied into
    /// it. May return a split surrogate in that case.
    pub fn get_bounded_text(
        &self,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
        buffer: &mut c_ushort,
        buflen: i32,
    ) -> i32 {
        lib().FPDFText_GetBoundedText(self, left, top, right, bottom, buffer, buflen)
    }

    /// Get character rotation angle.
    ///
    /// Parameters:
    /// * index       -   Zero-based index of the character.
    ///
    /// Returns:
    /// On success, return the angle value in radian. Value will always be
    /// greater or equal to 0. If `index` is out of bounds, then return -1.
    pub fn get_char_angle(&self, index: i32) -> f32 {
        lib().FPDFText_GetCharAngle(self, index)
    }

    /// Get bounding box of a particular character.
    ///
    /// Parameters:
    /// * index       -   Zero-based index of the character.
    ///
    /// Returns:
    /// * The position of the character box as PdfiumRect. An Err if `index`
    ///   is out of bounds
    ///
    /// Comments:
    /// * All positions are measured in PDF "user space"
    pub fn get_char_box(&self, index: i32) -> PdfiumResult<PdfiumRect> {
        let mut left = 0.0;
        let mut right = 0.0;
        let mut bottom = 0.0;
        let mut top = 0.0;

        lib().FPDFText_GetCharBox(self, index, &mut left, &mut right, &mut bottom, &mut top)?;

        Ok(PdfiumRect {
            left: left as f32,
            top: top as f32,
            right: right as f32,
            bottom: bottom as f32,
        })
    }

    /// Function: FPDFText_GetCharIndexAtPos
    /// Get the index of a character at or nearby a certain position on the
    /// page.
    /// Parameters:
    /// text_page   -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// x           -   X position in PDF "user space".
    /// y           -   Y position in PDF "user space".
    /// xTolerance  -   An x-axis tolerance value for character hit
    /// detection, in point units.
    /// yTolerance  -   A y-axis tolerance value for character hit
    /// detection, in point units.
    /// Returns:
    /// The zero-based index of the character at, or nearby the point (x,y).
    /// If there is no character at or nearby the point, Returns will
    /// be -1. If an error occurs, -3 will be returned.
    pub fn get_char_index_at_pos(&self, x: f64, y: f64, x_tolerance: f64, y_tolerance: f64) -> i32 {
        lib().FPDFText_GetCharIndexAtPos(self, x, y, x_tolerance, y_tolerance)
    }

    /// Get the character index in this [`PdfiumTextPage`] internal character list.
    ///
    /// nTextIndex - index of the text returned from FPDFText_GetText().
    ///
    /// Returns the index of the character in internal character list. -1 for error.
    pub fn get_char_index_from_text_index(&self, n_text_index: i32) -> PdfiumResult<i32> {
        i32_to_result(lib().FPDFText_GetCharIndexFromTextIndex(self, n_text_index))
    }

    /// Function: FPDFText_GetCharOrigin
    /// Get origin of a particular character.
    /// Parameters:
    /// text_page   -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// index       -   Zero-based index of the character.
    /// x           -   Pointer to a double number receiving x coordinate of
    /// the character origin.
    /// y           -   Pointer to a double number receiving y coordinate of
    /// the character origin.
    /// Returns:
    /// Whether the call succeeded. If false, x and y are unchanged.
    /// Comments:
    /// All positions are measured in PDF "user space".
    pub fn get_char_origin(&self, index: i32, x: &mut f64, y: &mut f64) -> PdfiumResult<()> {
        lib().FPDFText_GetCharOrigin(self, index, x, y)
    }

    /// Function: FPDFText_GetFillColor
    /// Get the fill color of a particular character.
    /// Parameters:
    /// text_page      -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// index          -   Zero-based index of the character.
    /// R              -   Pointer to an unsigned int number receiving the
    /// red value of the fill color.
    /// G              -   Pointer to an unsigned int number receiving the
    /// green value of the fill color.
    /// B              -   Pointer to an unsigned int number receiving the
    /// blue value of the fill color.
    /// A              -   Pointer to an unsigned int number receiving the
    /// alpha value of the fill color.
    /// Returns:
    /// Whether the call succeeded. If false, |R|, |G|, |B| and |A| are
    /// unchanged.
    pub fn get_fill_color(
        &self,
        index: i32,
        r: &mut u32,
        g: &mut u32,
        b: &mut u32,
        a: &mut u32,
    ) -> PdfiumResult<()> {
        lib().FPDFText_GetFillColor(self, index, r, g, b, a)
    }

    /// Function: FPDFText_GetFontInfo
    /// Get the font name and flags of a particular character.
    /// Parameters:
    /// text_page - Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// index     - Zero-based index of the character.
    /// buffer    - A buffer receiving the font name.
    /// buflen    - The length of |buffer| in bytes.
    /// flags     - Optional pointer to an int receiving the font flags.
    /// These flags should be interpreted per PDF spec 1.7
    /// Section 5.7.1 Font Descriptor Flags.
    /// Returns:
    /// On success, return the length of the font name, including the
    /// trailing NUL character, in bytes. If this length is less than or
    /// equal to |length|, |buffer| is set to the font name, |flags| is
    /// set to the font flags. |buffer| is in UTF-8 encoding. Return 0 on
    /// failure.
    pub fn get_font_info(
        &self,
        index: i32,
        buffer: Option<&mut [u8]>,
        buflen: c_ulong,
        flags: &mut i32,
    ) -> c_ulong {
        lib().FPDFText_GetFontInfo(self, index, buffer, buflen, flags)
    }

    /// Function: FPDFText_GetFontSize
    /// Get the font size of a particular character.
    /// Parameters:
    /// text_page   -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// index       -   Zero-based index of the character.
    /// Returns:
    /// The font size of the particular character, measured in points (about
    /// 1/72 inch). This is the typographic size of the font (so called
    /// "em size").
    pub fn get_font_size(&self, index: i32) -> f64 {
        lib().FPDFText_GetFontSize(self, index)
    }

    /// Function: FPDFText_GetFontWeight
    /// Get the font weight of a particular character.
    /// Parameters:
    /// text_page   -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// index       -   Zero-based index of the character.
    /// Returns:
    /// On success, return the font weight of the particular character. If
    /// |text_page| is invalid, if `index` is out of bounds, or if the
    /// character's text object is undefined, return -1.
    pub fn get_font_weight(&self, index: i32) -> PdfiumResult<i32> {
        i32_to_result(lib().FPDFText_GetFontWeight(self, index))
    }

    /// Function: FPDFText_GetLooseCharBox
    /// Get a "loose" bounding box of a particular character, i.e., covering
    /// the entire glyph bounds, without taking the actual glyph shape into
    /// account.
    /// Parameters:
    /// text_page   -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// index       -   Zero-based index of the character.
    /// rect        -   Pointer to a FS_RECTF receiving the character box.
    /// Returns:
    /// On success, return TRUE and fill in |rect|. If |text_page| is
    /// invalid, or if `index` is out of bounds, then return FALSE, and the
    /// |rect| out parameter remains unmodified.
    /// Comments:
    /// All positions are measured in PDF "user space".
    pub fn get_loose_char_box(&self, index: i32, rect: &mut FS_RECTF) -> PdfiumResult<()> {
        lib().FPDFText_GetLooseCharBox(self, index, rect)
    }

    /// Function: FPDFText_GetMatrix
    /// Get the effective transformation matrix for a particular character.
    /// Parameters:
    /// text_page   -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage().
    /// index       -   Zero-based index of the character.
    /// matrix      -   Pointer to a FS_MATRIX receiving the transformation
    /// matrix.
    /// Returns:
    /// On success, return TRUE and fill in |matrix|. If |text_page| is
    /// invalid, or if `index` is out of bounds, or if |matrix| is NULL,
    /// then return FALSE, and |matrix| remains unmodified.
    pub fn get_matrix(&self, index: i32, matrix: &mut FS_MATRIX) -> PdfiumResult<()> {
        lib().FPDFText_GetMatrix(self, index, matrix)
    }

    /// Function: FPDFText_GetRect
    /// Get a rectangular area from the result generated by
    /// FPDFText_CountRects.
    /// Parameters:
    /// text_page   -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// rect_index  -   Zero-based index for the rectangle.
    /// left        -   Pointer to a double value receiving the rectangle
    /// left boundary.
    /// top         -   Pointer to a double value receiving the rectangle
    /// top boundary.
    /// right       -   Pointer to a double value receiving the rectangle
    /// right boundary.
    /// bottom      -   Pointer to a double value receiving the rectangle
    /// bottom boundary.
    /// Returns:
    /// On success, return TRUE and fill in |left|, |top|, |right|, and
    /// |bottom|. If |text_page| is invalid then return FALSE, and the out
    /// parameters remain unmodified. If |text_page| is valid but
    /// |rect_index| is out of bounds, then return FALSE and set the out
    /// parameters to 0.
    pub fn get_rect(
        &self,
        rect_index: i32,
        left: &mut f64,
        top: &mut f64,
        right: &mut f64,
        bottom: &mut f64,
    ) -> PdfiumResult<()> {
        lib().FPDFText_GetRect(self, rect_index, left, top, right, bottom)
    }

    /// Function: FPDFText_GetStrokeColor
    /// Get the stroke color of a particular character.
    /// Parameters:
    /// text_page      -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// index          -   Zero-based index of the character.
    /// R              -   Pointer to an unsigned int number receiving the
    /// red value of the stroke color.
    /// G              -   Pointer to an unsigned int number receiving the
    /// green value of the stroke color.
    /// B              -   Pointer to an unsigned int number receiving the
    /// blue value of the stroke color.
    /// A              -   Pointer to an unsigned int number receiving the
    /// alpha value of the stroke color.
    /// Returns:
    /// Whether the call succeeded. If false, |R|, |G|, |B| and |A| are
    /// unchanged.
    pub fn get_stroke_color(
        &self,
        index: i32,
        r: &mut u32,
        g: &mut u32,
        b: &mut u32,
        a: &mut u32,
    ) -> PdfiumResult<()> {
        lib().FPDFText_GetStrokeColor(self, index, r, g, b, a)
    }

    /// Extract unicode text section from the page as string.
    ///
    /// Parameters:
    /// * start_index -   Index for the start characters.
    /// * count       -   Number of UCS-2 values to be extracted.
    ///
    /// Returns:
    /// * String containing the requested text part
    ///
    /// Comments:
    /// * UTF-16 and UCS-2 are both character encoding schemes for representing Unicode text
    ///   * UCS-2: stands for Universal Character Set-2
    ///     - Fixed-length encoding that uses 2 bytes (16 bits) per character.
    ///     - Supports only the Basic Multilingual Plane (BMP), which includes Unicode code points
    ///       from U+0000 to U+FFFF (65,536 characters).
    ///   * UTF-16: stands for Unicode Transformation Format-16.
    ///     - Variable-length encoding that uses 2 or 4 bytes per character.
    ///     - Can represent all Unicode code points (U+0000 to U+10FFFF), including those outside the BMP
    ///     - Backward compatible with UCS-2 for BMP characters, as they are encoded identically
    /// * If the page contains UTF-16 4-byte characters they are handled as two UCS-2 values, and may get
    ///   split up depending on `start_index` and `count`. This will result into an invalid UTF-16
    ///   character and returned as `REPLACEMENT_CHARACTER`. See the test-case.
    /// * This function ignores characters without UCS-2 representations.
    ///   It considers all characters on the page, even those that are not
    ///   visible when the page has a cropbox. To filter out the characters
    ///   outside of the cropbox, use FPDF_GetPageBoundingBox() and
    ///   FPDFText_GetCharBox().
    pub fn extract(&self, start_index: i32, count: i32) -> String {
        if count < 1 {
            return String::default();
        }
        let mut vec_utf16 = vec![0u16; count as usize + 1];
        let num = lib().FPDFText_GetText(self.into(), start_index, count, vec_utf16.as_mut_ptr());
        if num < 1 {
            return String::default();
        }
        let vec_utf16 = &vec_utf16[..num as usize - 1];
        String::from_utf16_lossy(vec_utf16)
    }

    /// Gets the full text of the page as string.
    pub fn full(&self) -> String {
        self.extract(0, self.char_count().unwrap_or_default())
    }

    /// Get the text index in this [`PdfiumTextPage`] internal character list.
    ///
    /// nCharIndex - index of the character in internal character list.
    ///
    /// Returns the index of the text returned from FPDFText_GetText(). -1 for error.
    pub fn get_text_index_from_char_index(&self, n_char_index: i32) -> PdfiumResult<i32> {
        i32_to_result(lib().FPDFText_GetTextIndexFromCharIndex(self, n_char_index))
    }

    /// Function: FPDFText_GetTextObject
    ///
    /// Get the FPDF_PAGEOBJECT associated with a given character.
    ///
    /// Parameters:
    /// * index       -   Zero-based index of the character.
    ///
    /// Returns:
    /// * The associated text object for the character at `index`, or NULL on
    ///   error. The returned text object, if non-null, is of type
    ///   |FPDF_PAGEOBJ_TEXT|. The caller does not own the returned object.
    pub fn get_text_object(&self, index: i32) -> PdfiumResult<PdfiumPageObject> {
        lib().FPDFText_GetTextObject(self, index)
    }

    /// Get Unicode of a character in a page.
    ///
    /// Parameters:
    /// * index       -   Zero-based index of the character.
    ///
    /// Returns:
    /// * The Unicode of the particular character.
    ///
    /// Notes:
    /// * If a character is not encoded in Unicode and Foxit engine can't
    ///   convert to Unicode, the Returns will be zero.
    /// * This does not support UTF-16 4-byte characters
    pub fn get_unicode(&self, index: i32) -> u32 {
        lib().FPDFText_GetUnicode(self, index)
    }

    /// Function: FPDFText_HasUnicodeMapError
    /// Get if a character in a page has an invalid unicode mapping.
    /// Parameters:
    /// text_page   -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// index       -   Zero-based index of the character.
    /// Returns:
    /// 1 if the character has an invalid unicode mapping.
    /// 0 if the character has no known unicode mapping issues.
    /// -1 if there was an error.
    pub fn has_unicode_map_error(&self, index: i32) -> PdfiumResult<bool> {
        i32_to_bool_result(lib().FPDFText_HasUnicodeMapError(self, index))
    }

    /// Function: FPDFText_IsGenerated
    /// Get if a character in a page is generated by PDFium.
    /// Parameters:
    /// text_page   -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// index       -   Zero-based index of the character.
    /// Returns:
    /// 1 if the character is generated by PDFium.
    /// 0 if the character is not generated by PDFium.
    /// -1 if there was an error.
    pub fn is_generated(&self, index: i32) -> PdfiumResult<bool> {
        i32_to_bool_result(lib().FPDFText_IsGenerated(self, index))
    }

    /// Function: FPDFText_IsHyphen
    /// Get if a character in a page is a hyphen.
    /// Parameters:
    /// text_page   -   Handle to a text page information structure.
    /// Returned by FPDFText_LoadPage function.
    /// index       -   Zero-based index of the character.
    /// Returns:
    /// 1 if the character is a hyphen.
    /// 0 if the character is not a hyphen.
    /// -1 if there was an error.
    pub fn is_hyphen(&self, index: i32) -> PdfiumResult<bool> {
        i32_to_bool_result(lib().FPDFText_IsHyphen(self, index))
    }
}

impl From<&PdfiumTextPage> for FPDF_TEXTPAGE {
    fn from(text_page: &PdfiumTextPage) -> Self {
        text_page.handle.handle()
    }
}

fn close_text_page(text_page: FPDF_TEXTPAGE) {
    lib().FPDFText_ClosePage(text_page);
}

#[cfg(test)]
mod tests {
    use std::{char::REPLACEMENT_CHARACTER, fs};

    use crate::*;

    #[test]
    fn test_text_page() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();

        let page = document.page(0).unwrap();

        let text = page.text().unwrap();

        let num_chars = text.char_count().unwrap();

        #[cfg(target_os = "windows")]
        assert_eq!(num_chars, 1104);

        #[cfg(not(target_os = "windows"))]
        assert_eq!(num_chars, 1102);

        let t = text.extract(0, 27);
        assert_eq!(t, "🌟 Welcome to Groningen! 🌟");
        assert_eq!(t.chars().count(), 25);
        assert_eq!(t.len(), 31);

        let t = text.extract(0, 1);
        assert_eq!(t.chars().next().unwrap(), REPLACEMENT_CHARACTER);
        assert_eq!(t.chars().count(), 1);
        assert_eq!(t.len(), 3);

        let t = text.extract(0, 2);
        assert_eq!(t.chars().next().unwrap() as u32, 0x1F31F); // Glowing Star
        assert_eq!(t.chars().count(), 1);
        assert_eq!(t.len(), 4);

        #[cfg(target_os = "windows")]
        let t = text.extract(1093, 100);

        #[cfg(not(target_os = "windows"))]
        let t = text.extract(1091, 100);

        assert_eq!(t, "Netherlands");
        assert_eq!(t.chars().count(), 11);
        assert_eq!(t.len(), 11);

        let t = text.extract(1200, 100);
        assert_eq!(t, "");

        let full_text = text.full();
        fs::write("groningen-page-1-full.txt", full_text).unwrap();
    }
}
