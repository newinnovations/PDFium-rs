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

#![allow(non_upper_case_globals)]
#![allow(dead_code)]

//! Constants used by the PDFium C library

use crate::pdfium_types::*;

/// False value for PDFium boolean operations
pub const FALSE: i32 = 0;

/// Unknown or unrecognized PDF object type
pub const FPDF_OBJECT_UNKNOWN: i32 = 0;

/// PDF boolean object (true/false)
pub const FPDF_OBJECT_BOOLEAN: i32 = 1;

/// PDF numeric object (integer or real)
pub const FPDF_OBJECT_NUMBER: i32 = 2;

/// PDF string object (literal or hex string)
pub const FPDF_OBJECT_STRING: i32 = 3;

/// PDF name object (identifier starting with /)
pub const FPDF_OBJECT_NAME: i32 = 4;

/// PDF array object (ordered collection)
pub const FPDF_OBJECT_ARRAY: i32 = 5;

/// PDF dictionary object (key-value pairs)
pub const FPDF_OBJECT_DICTIONARY: i32 = 6;

/// PDF stream object (dictionary with binary data)
pub const FPDF_OBJECT_STREAM: i32 = 7;

/// PDF null object
pub const FPDF_OBJECT_NULLOBJ: i32 = 8;

/// PDF indirect object reference
pub const FPDF_OBJECT_REFERENCE: i32 = 9;

/// Policy for machine time access control
pub const FPDF_POLICY_MACHINETIME_ACCESS: i32 = 0;

/// Operation completed successfully
pub const FPDF_ERR_SUCCESS: i32 = 0;

/// Unknown error occurred
pub const FPDF_ERR_UNKNOWN: i32 = 1;

/// File not found or could not be opened
pub const FPDF_ERR_FILE: i32 = 2;

/// File not in PDF format or corrupted
pub const FPDF_ERR_FORMAT: i32 = 3;

/// Password required or incorrect
pub const FPDF_ERR_PASSWORD: i32 = 4;

/// Unsupported security scheme
pub const FPDF_ERR_SECURITY: i32 = 5;

/// Page not found or content error
pub const FPDF_ERR_PAGE: i32 = 6;

/// Render annotations
pub const FPDF_ANNOT: i32 = 1;

/// Optimize for LCD text rendering
pub const FPDF_LCD_TEXT: i32 = 2;

/// Don't use device native text
pub const FPDF_NO_NATIVETEXT: i32 = 4;

/// Render in grayscale
pub const FPDF_GRAYSCALE: i32 = 8;

/// Include debug information
pub const FPDF_DEBUG_INFO: i32 = 128;

/// Don't catch exceptions
pub const FPDF_NO_CATCH: i32 = 256;

/// Limit image cache size
pub const FPDF_RENDER_LIMITEDIMAGECACHE: i32 = 512;

/// Force halftone rendering
pub const FPDF_RENDER_FORCEHALFTONE: i32 = 1024;

/// Optimize for printing
pub const FPDF_PRINTING: i32 = 2048;

/// Disable text smoothing
pub const FPDF_RENDER_NO_SMOOTHTEXT: i32 = 4096;

/// Disable image smoothing
pub const FPDF_RENDER_NO_SMOOTHIMAGE: i32 = 8192;

/// Disable path smoothing
pub const FPDF_RENDER_NO_SMOOTHPATH: i32 = 16384;

/// Reverse byte order for bitmap
pub const FPDF_REVERSE_BYTE_ORDER: i32 = 16;

/// Convert fill operations to stroke
pub const FPDF_CONVERT_FILL_TO_STROKE: i32 = 32;

/// Unknown bitmap format
pub const FPDFBitmap_Unknown: i32 = 0;

/// 8-bit grayscale
pub const FPDFBitmap_Gray: i32 = 1;

/// 24-bit BGR (blue-green-red)
pub const FPDFBitmap_BGR: i32 = 2;

/// 32-bit BGR with unused alpha
pub const FPDFBitmap_BGRx: i32 = 3;

/// 32-bit BGRA with alpha
pub const FPDFBitmap_BGRA: i32 = 4;

/// 32-bit BGRA with premultiplied alpha
pub const FPDFBitmap_BGRA_Premul: i32 = 5;

/// Will close action
pub const FPDFDOC_AACTION_WC: i32 = 16;

/// Will save action
pub const FPDFDOC_AACTION_WS: i32 = 17;

/// Did save action
pub const FPDFDOC_AACTION_DS: i32 = 18;

/// Will print action
pub const FPDFDOC_AACTION_WP: i32 = 19;

/// Did print action
pub const FPDFDOC_AACTION_DP: i32 = 20;

/// Page open action
pub const FPDFPAGE_AACTION_OPEN: i32 = 0;

/// Page close action
pub const FPDFPAGE_AACTION_CLOSE: i32 = 1;

/// Unknown form field type
pub const FPDF_FORMFIELD_UNKNOWN: i32 = 0;

/// Push button field
pub const FPDF_FORMFIELD_PUSHBUTTON: i32 = 1;

/// Checkbox field
pub const FPDF_FORMFIELD_CHECKBOX: i32 = 2;

/// Radio button field
pub const FPDF_FORMFIELD_RADIOBUTTON: i32 = 3;

/// Combo box field
pub const FPDF_FORMFIELD_COMBOBOX: i32 = 4;

/// List box field
pub const FPDF_FORMFIELD_LISTBOX: i32 = 5;

/// Text field
pub const FPDF_FORMFIELD_TEXTFIELD: i32 = 6;

/// Digital signature field
pub const FPDF_FORMFIELD_SIGNATURE: i32 = 7;

/// Total number of form field types
pub const FPDF_FORMFIELD_COUNT: i32 = 8;

/// Unknown annotation type
pub const FPDF_ANNOT_UNKNOWN: i32 = 0;

/// Text annotation (sticky note)
pub const FPDF_ANNOT_TEXT: i32 = 1;

/// Link annotation
pub const FPDF_ANNOT_LINK: i32 = 2;

/// Free text annotation
pub const FPDF_ANNOT_FREETEXT: i32 = 3;

/// Line annotation
pub const FPDF_ANNOT_LINE: i32 = 4;

/// Square/rectangle annotation
pub const FPDF_ANNOT_SQUARE: i32 = 5;

/// Circle/ellipse annotation
pub const FPDF_ANNOT_CIRCLE: i32 = 6;

/// Polygon annotation
pub const FPDF_ANNOT_POLYGON: i32 = 7;

/// Polyline annotation
pub const FPDF_ANNOT_POLYLINE: i32 = 8;

/// Highlight annotation
pub const FPDF_ANNOT_HIGHLIGHT: i32 = 9;

/// Underline annotation
pub const FPDF_ANNOT_UNDERLINE: i32 = 10;

/// Squiggly underline annotation
pub const FPDF_ANNOT_SQUIGGLY: i32 = 11;

/// Strike-out annotation
pub const FPDF_ANNOT_STRIKEOUT: i32 = 12;

/// Rubber stamp annotation
pub const FPDF_ANNOT_STAMP: i32 = 13;

/// Caret annotation
pub const FPDF_ANNOT_CARET: i32 = 14;

/// Ink annotation (freehand drawing)
pub const FPDF_ANNOT_INK: i32 = 15;

/// Pop-up annotation
pub const FPDF_ANNOT_POPUP: i32 = 16;

/// File attachment annotation
pub const FPDF_ANNOT_FILEATTACHMENT: i32 = 17;

/// Sound annotation
pub const FPDF_ANNOT_SOUND: i32 = 18;

/// Movie annotation
pub const FPDF_ANNOT_MOVIE: i32 = 19;

/// Interactive form widget
pub const FPDF_ANNOT_WIDGET: i32 = 20;

/// Screen annotation
pub const FPDF_ANNOT_SCREEN: i32 = 21;

/// Printer's mark annotation
pub const FPDF_ANNOT_PRINTERMARK: i32 = 22;

/// Trap network annotation
pub const FPDF_ANNOT_TRAPNET: i32 = 23;

/// Watermark annotation
pub const FPDF_ANNOT_WATERMARK: i32 = 24;

/// 3D annotation
pub const FPDF_ANNOT_THREED: i32 = 25;

/// Rich media annotation
pub const FPDF_ANNOT_RICHMEDIA: i32 = 26;

/// XFA widget annotation
pub const FPDF_ANNOT_XFAWIDGET: i32 = 27;

/// Redaction annotation
pub const FPDF_ANNOT_REDACT: i32 = 28;

/// No annotation flags
pub const FPDF_ANNOT_FLAG_NONE: i32 = 0;

/// Annotation is invisible
pub const FPDF_ANNOT_FLAG_INVISIBLE: i32 = 1;

/// Annotation is hidden
pub const FPDF_ANNOT_FLAG_HIDDEN: i32 = 2;

/// Print annotation when printing page
pub const FPDF_ANNOT_FLAG_PRINT: i32 = 4;

/// Don't scale annotation with page zoom
pub const FPDF_ANNOT_FLAG_NOZOOM: i32 = 8;

/// Don't rotate annotation with page
pub const FPDF_ANNOT_FLAG_NOROTATE: i32 = 16;

/// Don't display annotation on screen
pub const FPDF_ANNOT_FLAG_NOVIEW: i32 = 32;

/// Annotation is read-only
pub const FPDF_ANNOT_FLAG_READONLY: i32 = 64;

/// Annotation contents are locked
pub const FPDF_ANNOT_FLAG_LOCKED: i32 = 128;

/// Invert NOVIEW flag on mouse click
pub const FPDF_ANNOT_FLAG_TOGGLENOVIEW: i32 = 256;

/// Normal appearance
pub const FPDF_ANNOT_APPEARANCEMODE_NORMAL: i32 = 0;

/// Rollover/hover appearance
pub const FPDF_ANNOT_APPEARANCEMODE_ROLLOVER: i32 = 1;

/// Down/pressed appearance
pub const FPDF_ANNOT_APPEARANCEMODE_DOWN: i32 = 2;

/// Total number of appearance modes
pub const FPDF_ANNOT_APPEARANCEMODE_COUNT: i32 = 3;

/// No form flags
pub const FPDF_FORMFLAG_NONE: i32 = 0;

/// Form field is read-only
pub const FPDF_FORMFLAG_READONLY: i32 = 1;

/// Form field is required
pub const FPDF_FORMFLAG_REQUIRED: i32 = 2;

/// Don't export form field value
pub const FPDF_FORMFLAG_NOEXPORT: i32 = 4;

/// Text field allows multiple lines
pub const FPDF_FORMFLAG_TEXT_MULTILINE: i32 = 4096;

/// Text field is password field
pub const FPDF_FORMFLAG_TEXT_PASSWORD: i32 = 8192;

/// Choice field is combo box
pub const FPDF_FORMFLAG_CHOICE_COMBO: i32 = 131072;

/// Choice field is editable
pub const FPDF_FORMFLAG_CHOICE_EDIT: i32 = 262144;

/// Choice field allows multiple selection
pub const FPDF_FORMFLAG_CHOICE_MULTI_SELECT: i32 = 2097152;

/// Key stroke action
pub const FPDF_ANNOT_AACTION_KEY_STROKE: i32 = 12;

/// Format action
pub const FPDF_ANNOT_AACTION_FORMAT: i32 = 13;

/// Validate action
pub const FPDF_ANNOT_AACTION_VALIDATE: i32 = 14;

/// Calculate action
pub const FPDF_ANNOT_AACTION_CALCULATE: i32 = 15;

/// Unknown color space
pub const FPDF_COLORSPACE_UNKNOWN: i32 = 0;

/// Device Gray color space
pub const FPDF_COLORSPACE_DEVICEGRAY: i32 = 1;

/// Device RGB color space
pub const FPDF_COLORSPACE_DEVICERGB: i32 = 2;

/// Device CMYK color space
pub const FPDF_COLORSPACE_DEVICECMYK: i32 = 3;

/// Calibrated Gray color space
pub const FPDF_COLORSPACE_CALGRAY: i32 = 4;

/// Calibrated RGB color space
pub const FPDF_COLORSPACE_CALRGB: i32 = 5;

/// CIE Lab color space
pub const FPDF_COLORSPACE_LAB: i32 = 6;

/// ICC-based color space
pub const FPDF_COLORSPACE_ICCBASED: i32 = 7;

/// Separation color space
pub const FPDF_COLORSPACE_SEPARATION: i32 = 8;

/// DeviceN color space
pub const FPDF_COLORSPACE_DEVICEN: i32 = 9;

/// Indexed color space
pub const FPDF_COLORSPACE_INDEXED: i32 = 10;

/// Pattern color space
pub const FPDF_COLORSPACE_PATTERN: i32 = 11;

/// Unknown page object type
pub const FPDF_PAGEOBJ_UNKNOWN: i32 = 0;

/// Text object
pub const FPDF_PAGEOBJ_TEXT: i32 = 1;

/// Path object (vector graphics)
pub const FPDF_PAGEOBJ_PATH: i32 = 2;

/// Image object
pub const FPDF_PAGEOBJ_IMAGE: i32 = 3;

/// Shading object
pub const FPDF_PAGEOBJ_SHADING: i32 = 4;

/// Form XObject
pub const FPDF_PAGEOBJ_FORM: i32 = 5;

/// Unknown path segment
pub const FPDF_SEGMENT_UNKNOWN: i32 = -1;

/// Line to segment
pub const FPDF_SEGMENT_LINETO: i32 = 0;

/// Bézier curve segment
pub const FPDF_SEGMENT_BEZIERTO: i32 = 1;

/// Move to segment
pub const FPDF_SEGMENT_MOVETO: i32 = 2;

/// No fill
pub const FPDF_FILLMODE_NONE: i32 = 0;

/// Alternate (even-odd) fill rule
pub const FPDF_FILLMODE_ALTERNATE: i32 = 1;

/// Winding (non-zero) fill rule
pub const FPDF_FILLMODE_WINDING: i32 = 2;

/// Type1 font
pub const FPDF_FONT_TYPE1: i32 = 1;

/// TrueType font
pub const FPDF_FONT_TRUETYPE: i32 = 2;

/// Butt cap (square end at line endpoint)
pub const FPDF_LINECAP_BUTT: i32 = 0;

/// Round cap (semicircle at line endpoint)
pub const FPDF_LINECAP_ROUND: i32 = 1;

/// Square cap (square extending beyond endpoint)
pub const FPDF_LINECAP_PROJECTING_SQUARE: i32 = 2;

/// Miter join (sharp corner)
pub const FPDF_LINEJOIN_MITER: i32 = 0;

/// Round join (rounded corner)
pub const FPDF_LINEJOIN_ROUND: i32 = 1;

/// Bevel join (flattened corner)
pub const FPDF_LINEJOIN_BEVEL: i32 = 2;

/// Enhanced Metafile (EMF) print mode
pub const FPDF_PRINTMODE_EMF: i32 = 0;

/// Text-only print mode
pub const FPDF_PRINTMODE_TEXTONLY: i32 = 1;

/// PostScript Level 2 print mode
pub const FPDF_PRINTMODE_POSTSCRIPT2: i32 = 2;

/// PostScript Level 3 print mode
pub const FPDF_PRINTMODE_POSTSCRIPT3: i32 = 3;

/// PostScript Level 2 passthrough mode
pub const FPDF_PRINTMODE_POSTSCRIPT2_PASSTHROUGH: i32 = 4;

/// PostScript Level 3 passthrough mode
pub const FPDF_PRINTMODE_POSTSCRIPT3_PASSTHROUGH: i32 = 5;

/// EMF with image masks print mode
pub const FPDF_PRINTMODE_EMF_IMAGE_MASKS: i32 = 6;

/// PostScript Level 3 with Type 42 fonts
pub const FPDF_PRINTMODE_POSTSCRIPT3_TYPE42: i32 = 7;

/// PostScript Level 3 Type 42 passthrough
pub const FPDF_PRINTMODE_POSTSCRIPT3_TYPE42_PASSTHROUGH: i32 = 8;

/// XFA form (unsupported)
pub const FPDF_UNSP_DOC_XFAFORM: i32 = 1;

/// Portable collection (unsupported)
pub const FPDF_UNSP_DOC_PORTABLECOLLECTION: i32 = 2;

/// Attachment (unsupported)
pub const FPDF_UNSP_DOC_ATTACHMENT: i32 = 3;

/// Security (unsupported)
pub const FPDF_UNSP_DOC_SECURITY: i32 = 4;

/// Shared review (unsupported)
pub const FPDF_UNSP_DOC_SHAREDREVIEW: i32 = 5;

/// Shared form Acrobat (unsupported)
pub const FPDF_UNSP_DOC_SHAREDFORM_ACROBAT: i32 = 6;

/// Shared form filesystem (unsupported)
pub const FPDF_UNSP_DOC_SHAREDFORM_FILESYSTEM: i32 = 7;

/// Shared form email (unsupported)
pub const FPDF_UNSP_DOC_SHAREDFORM_EMAIL: i32 = 8;

/// 3D annotation (unsupported)
pub const FPDF_UNSP_ANNOT_3DANNOT: i32 = 11;

/// Movie annotation (unsupported)
pub const FPDF_UNSP_ANNOT_MOVIE: i32 = 12;

/// Sound annotation (unsupported)
pub const FPDF_UNSP_ANNOT_SOUND: i32 = 13;

/// Screen media annotation (unsupported)
pub const FPDF_UNSP_ANNOT_SCREEN_MEDIA: i32 = 14;

/// Screen rich media annotation (unsupported)
pub const FPDF_UNSP_ANNOT_SCREEN_RICHMEDIA: i32 = 15;

/// Attachment annotation (unsupported)
pub const FPDF_UNSP_ANNOT_ATTACHMENT: i32 = 16;

/// Signature annotation (unsupported)
pub const FPDF_UNSP_ANNOT_SIG: i32 = 17;

/// Ready to start rendering
pub const FPDF_RENDER_READY: i32 = 0;

/// Rendering in progress, should be continued
pub const FPDF_RENDER_TOBECONTINUED: i32 = 1;

/// Rendering completed successfully
pub const FPDF_RENDER_DONE: i32 = 2;

/// Rendering failed
pub const FPDF_RENDER_FAILED: i32 = 3;

/// Save incrementally (append changes)
pub const FPDF_INCREMENTAL: i32 = 1;

/// Save as complete new file
pub const FPDF_NO_INCREMENTAL: i32 = 2;

/// Remove security settings when saving
pub const FPDF_REMOVE_SECURITY: i32 = 3;

/// Case-sensitive search
pub const FPDF_MATCHCASE: i32 = 1;

/// Match whole words only
pub const FPDF_MATCHWHOLEWORD: i32 = 2;

/// Find consecutive matches
pub const FPDF_CONSECUTIVE: i32 = 4;

/// Unknown text render mode
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_UNKNOWN: FPDF_TEXT_RENDERMODE = -1;

/// Fill text
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL: FPDF_TEXT_RENDERMODE = 0;

/// Stroke text outline
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE: FPDF_TEXT_RENDERMODE = 1;

/// Fill and stroke text
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE: FPDF_TEXT_RENDERMODE = 2;

/// Invisible text
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_INVISIBLE: FPDF_TEXT_RENDERMODE = 3;

/// Fill text and add to clipping path
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_CLIP: FPDF_TEXT_RENDERMODE = 4;

/// Stroke text and add to clipping path
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE_CLIP: FPDF_TEXT_RENDERMODE = 5;

/// Fill, stroke text and add to clipping path
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE_CLIP: FPDF_TEXT_RENDERMODE = 6;

/// Add text to clipping path only
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_CLIP: FPDF_TEXT_RENDERMODE = 7;

/// Last valid text render mode
pub const FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_LAST: FPDF_TEXT_RENDERMODE = 7;

/// Duplex type undefined
pub const _FPDF_DUPLEXTYPE__DuplexUndefined: _FPDF_DUPLEXTYPE_ = 0;

/// Single-sided printing
pub const _FPDF_DUPLEXTYPE__Simplex: _FPDF_DUPLEXTYPE_ = 1;

/// Double-sided, flip on short edge
pub const _FPDF_DUPLEXTYPE__DuplexFlipShortEdge: _FPDF_DUPLEXTYPE_ = 2;

/// Double-sided, flip on long edge
pub const _FPDF_DUPLEXTYPE__DuplexFlipLongEdge: _FPDF_DUPLEXTYPE_ = 3;

/// Anti-Grain Geometry (AGG) renderer
pub const FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_AGG: FPDF_RENDERER_TYPE = 0;

/// Skia graphics renderer
pub const FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_SKIA: FPDF_RENDERER_TYPE = 1;

/// Annotation border/text color
pub const FPDFANNOT_COLORTYPE_FPDFANNOT_COLORTYPE_Color: FPDFANNOT_COLORTYPE = 0;

/// Annotation interior/fill color
pub const FPDFANNOT_COLORTYPE_FPDFANNOT_COLORTYPE_InteriorColor: FPDFANNOT_COLORTYPE = 1;

/// Permanent file identifier
pub const FPDF_FILEIDTYPE_FILEIDTYPE_PERMANENT: FPDF_FILEIDTYPE = 0;

/// Changing file identifier
pub const FPDF_FILEIDTYPE_FILEIDTYPE_CHANGING: FPDF_FILEIDTYPE = 1;
