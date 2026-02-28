use crate::{PdfiumError, pdfium_types::FPDF_FILEIDTYPE};

/// Form types as returned by FPDF_GetFormType
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(i32)]
pub enum PdfiumFormType {
    #[default]
    /// Document contains no forms
    None = 0,
    /// Forms are specified using AcroForm spec
    AcroForm = 1,
    /// Forms are specified using entire XFA spec
    XFAFull = 2,
    /// Forms are specified using the XFAF subset of XFA spec
    XFAForeground = 3,
}

impl TryFrom<i32> for PdfiumFormType {
    type Error = PdfiumError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PdfiumFormType::None),
            1 => Ok(PdfiumFormType::AcroForm),
            2 => Ok(PdfiumFormType::XFAFull),
            3 => Ok(PdfiumFormType::XFAForeground),
            _ => Err(PdfiumError::InvalidEnumValue),
        }
    }
}

/// Page modes as returned by FPDFDoc_GetPageMode
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(i32)]
pub enum PdfiumPageMode {
    #[default]
    /// Unknown page mode.
    Unknown = -1,
    /// Document outline, and thumbnails hidden.
    UseNone = 0,
    /// Document outline visible.
    UseOutlines = 1,
    /// Thumbnail images visible.
    UseThumbs = 2,
    /// Full-screen mode, no menu bar, window controls, or other decorations visible.
    UseFullscreen = 3,
    /// Document outline visible, and thumbnail images visible.
    UseOC = 4,
    /// Attachments panel visible.
    UseAttachments = 5,
}

impl TryFrom<i32> for PdfiumPageMode {
    type Error = PdfiumError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(PdfiumPageMode::Unknown),
            0 => Ok(PdfiumPageMode::UseNone),
            1 => Ok(PdfiumPageMode::UseOutlines),
            2 => Ok(PdfiumPageMode::UseThumbs),
            3 => Ok(PdfiumPageMode::UseFullscreen),
            4 => Ok(PdfiumPageMode::UseOC),
            5 => Ok(PdfiumPageMode::UseAttachments),
            _ => Err(PdfiumError::InvalidEnumValue),
        }
    }
}

/// The file identifier entry type. See section 14.4 "File Identifiers" of the
/// ISO 32000-1:2008 spec.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(u32)]
pub enum PdfiumFileIdType {
    #[default]
    /// Permanent file identifier.
    Permanent = 0,
    /// Changing file identifier.
    Changing = 1,
}

impl From<PdfiumFileIdType> for FPDF_FILEIDTYPE {
    fn from(val: PdfiumFileIdType) -> Self {
        val as FPDF_FILEIDTYPE
    }
}
