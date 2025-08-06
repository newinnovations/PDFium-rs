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

use crate::{
    lib, pdfium_constants,
    pdfium_types::{FS_MATRIX, FS_RECTF},
    PdfiumBitmap, PdfiumBitmapFormat, PdfiumColor, PdfiumError, PdfiumMatrix, PdfiumPage,
    PdfiumRect, PdfiumResult,
};

use bitflags::bitflags;

bitflags! {
    /// Flags controlling the PDFium rendering behavior.
    ///
    /// These flags can be combined using bitwise OR operations to customize
    /// the rendering process for different use cases such as printing,
    /// debugging, or optimizing for specific display types.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PdfiumRenderFlags: i32 {
        /// ANNOT: Render annotations (comments, highlights, form fields, etc.)
        /// Enable this to include interactive elements in the rendered output.
        const ANNOT = pdfium_constants::FPDF_ANNOT;

        /// LCD_TEXT: Use LCD text rendering for better clarity on LCD screens.
        /// This uses subpixel rendering to improve text sharpness on LCD displays.
        const LCD_TEXT = pdfium_constants::FPDF_LCD_TEXT;

        /// NO_NATIVETEXT: Disable native text rendering optimizations.
        /// Forces PDFium to render text as paths instead of using system fonts.
        const NO_NATIVETEXT = pdfium_constants::FPDF_NO_NATIVETEXT;

        /// GRAYSCALE: Render in grayscale mode.
        /// Converts all colors to grayscale values, useful for black & white printing.
        const GRAYSCALE = pdfium_constants::FPDF_GRAYSCALE;

        /// REVERSE_BYTE_ORDER: Reverse byte order for pixel data.
        /// Changes from BGRA to RGBA or vice versa, depending on platform requirements.
        const REVERSE_BYTE_ORDER = pdfium_constants::FPDF_REVERSE_BYTE_ORDER;

        /// CONVERT_FILL_TO_STROKE: Convert filled paths to stroked paths.
        /// Useful for creating outlines or wireframe representations.
        const CONVERT_FILL_TO_STROKE = pdfium_constants::FPDF_CONVERT_FILL_TO_STROKE;

        /// DEBUG_INFO: Include debug information in the rendering process.
        /// May affect performance but provides additional diagnostic information.
        const DEBUG_INFO = pdfium_constants::FPDF_DEBUG_INFO;

        /// NO_CATCH: Don't catch exceptions during rendering.
        /// Allows exceptions to propagate for debugging purposes - use with caution.
        const NO_CATCH = pdfium_constants::FPDF_NO_CATCH;

        /// RENDER_LIMITEDIMAGECACHE: Limit image cache size during rendering.
        /// Reduces memory usage at the cost of potential performance impact.
        const RENDER_LIMITEDIMAGECACHE = pdfium_constants::FPDF_RENDER_LIMITEDIMAGECACHE;

        /// RENDER_FORCEHALFTONE: Force halftone rendering for images.
        /// Applies halftone patterns, typically used for print output optimization.
        const RENDER_FORCEHALFTONE = pdfium_constants::FPDF_RENDER_FORCEHALFTONE;

        /// PRINTING: Optimize rendering for printing output.
        /// Applies print-specific optimizations and color handling.
        const PRINTING = pdfium_constants::FPDF_PRINTING;

        /// RENDER_NO_SMOOTHTEXT: Disable anti-aliasing for text rendering.
        /// Results in pixelated but potentially faster text rendering.
        const RENDER_NO_SMOOTHTEXT = pdfium_constants::FPDF_RENDER_NO_SMOOTHTEXT;

        /// RENDER_NO_SMOOTHIMAGE: Disable anti-aliasing for image rendering.
        /// Images will appear pixelated but render faster with less memory usage.
        const RENDER_NO_SMOOTHIMAGE = pdfium_constants::FPDF_RENDER_NO_SMOOTHIMAGE;

        /// RENDER_NO_SMOOTHPATH: Disable anti-aliasing for vector path rendering.
        /// Paths and shapes will have jagged edges but render more quickly.
        const RENDER_NO_SMOOTHPATH = pdfium_constants::FPDF_RENDER_NO_SMOOTHPATH;
    }
}

impl PdfiumRenderFlags {
    /// Returns flags optimized for fast rendering with minimal quality.
    /// Disables anti-aliasing and other quality features for maximum speed.
    pub fn fast_rendering() -> Self {
        Self::RENDER_NO_SMOOTHTEXT | Self::RENDER_NO_SMOOTHIMAGE | Self::RENDER_NO_SMOOTHPATH
    }

    /// Returns flags for debugging purposes.
    /// Includes debug information and disables exception catching.
    pub fn debug() -> Self {
        Self::DEBUG_INFO | Self::NO_CATCH
    }
}

/// Configuration for PDF page rendering operations.
///
/// This struct provides a flexible way to configure how PDF pages are rendered
/// to bitmaps, allowing control over dimensions, appearance, and performance
/// characteristics of the rendering process.
///
/// # Examples
///
/// ```rust
/// use pdfium::*;
///
/// // Render at specific width, height calculated automatically, include
/// // annotations
/// let config = PdfiumRenderConfig::new()
///     .with_width(800)
///     .with_format(PdfiumBitmapFormat::Bgra)
///     .with_flags(PdfiumRenderFlags::ANNOT);
///
/// // Render with custom scaling and clipping
/// let config = PdfiumRenderConfig::new()
///     .with_size(1920, 1080)
///     .with_scale(2.0)
///     .with_clipping(PdfiumRect::new(0.0, 0.0, 400.0, 300.0));
/// ```
#[derive(Debug, Clone)]
pub struct PdfiumRenderConfig {
    /// Target width in pixels. If None, calculated from height and aspect ratio.
    pub width: Option<i32>,
    /// Target height in pixels. If None, calculated from width and aspect ratio.
    pub height: Option<i32>,
    /// The pixel format for the rendered bitmap (BGRA, RGB, etc.).
    pub format: PdfiumBitmapFormat,
    /// Background color for the bitmap. If None, background is transparent.
    pub background: Option<PdfiumColor>,
    /// Bitflags controlling various rendering behaviors and optimizations.
    pub flags: PdfiumRenderFlags,
    /// Scaling factor. If None and both width/height specified, calculated automatically.
    pub scale: Option<f32>,
    /// Translation offset (pan_x, pan_y) in bitmap coordinates.
    pub pan: Option<(f32, f32)>,
    /// Custom transformation matrix. Takes precedence over scale and pan if specified.
    pub matrix: Option<PdfiumMatrix>,
    /// Clipping rectangle to restrict rendering to a specific area of the page.
    pub clipping: Option<PdfiumRect>,
}

impl Default for PdfiumRenderConfig {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            format: PdfiumBitmapFormat::Bgra,
            background: Some(PdfiumColor::WHITE),
            flags: PdfiumRenderFlags::ANNOT | PdfiumRenderFlags::LCD_TEXT,
            scale: None,
            pan: None,
            matrix: None,
            clipping: None,
        }
    }
}

impl PdfiumRenderConfig {
    /// Creates a new render configuration with default values.
    ///
    /// Default configuration uses BGRA format with white background and no special flags.
    /// You must specify at least width or height before rendering.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets exact width and height in pixels.
    ///
    /// When both dimensions are specified, you must also provide either a scale
    /// factor or a transformation matrix to define how the page maps to the bitmap.
    ///
    /// # Arguments
    /// * `width` - Target bitmap width in pixels (must be > 0)
    /// * `height` - Target bitmap height in pixels (must be > 0)
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Sets the target width. Height will be calculated automatically to preserve aspect ratio.
    ///
    /// This is the most common way to specify dimensions when you want to maintain
    /// the original page proportions.
    ///
    /// # Arguments
    /// * `width` - Target bitmap width in pixels (must be > 0)
    pub fn with_width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the target height. Width will be calculated automatically to preserve aspect ratio.
    ///
    /// Useful when you have height constraints but want to maintain proportions.
    ///
    /// # Arguments
    /// * `height` - Target bitmap height in pixels (must be > 0)
    pub fn with_height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the pixel format for the rendered bitmap.
    ///
    /// Different formats have different memory requirements and compatibility:
    /// - BGRA: 32-bit with alpha, most common for display
    /// - RGB: 24-bit without alpha, smaller memory footprint
    /// - Gray: 8-bit grayscale, smallest memory usage
    pub fn with_format(mut self, format: PdfiumBitmapFormat) -> Self {
        self.format = format;
        self
    }

    /// Sets the background color for areas not covered by page content.
    ///
    /// # Arguments
    /// * `color` - The background color to use
    pub fn with_background(mut self, color: PdfiumColor) -> Self {
        self.background = Some(color);
        self
    }

    /// Removes the background color, resulting in a transparent background.
    ///
    /// Only meaningful when using a pixel format that supports transparency (e.g., BGRA).
    pub fn with_transparent_background(mut self) -> Self {
        self.background = None;
        self
    }

    /// Sets the rendering flags to control various behaviors.
    ///
    /// You can combine multiple flags using the bitwise OR operator (|).
    /// Default flags are `PdfiumRenderFlags::ANNOT` and `PdfiumRenderFlags::LCD_TEXT`
    ///
    /// # Arguments
    /// * `flags` - Combination of PdfiumRenderFlags
    pub fn with_flags(mut self, flags: PdfiumRenderFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Adds additional flags to the existing configuration.
    ///
    /// This is useful when you want to add flags without replacing existing ones.
    pub fn add_flags(mut self, flags: PdfiumRenderFlags) -> Self {
        self.flags |= flags;
        self
    }

    /// Sets a clipping rectangle to render only a portion of the page.
    ///
    /// The rectangle is specified in page coordinates (points), not bitmap pixels.
    ///
    /// # Arguments
    /// * `rect` - The clipping rectangle in page coordinate system
    pub fn with_clipping(mut self, rect: PdfiumRect) -> Self {
        self.clipping = Some(rect);
        self
    }

    /// Sets the scaling factor for the rendered bitmap.
    ///
    /// Scale of 1.0 means 72 DPI (1 point = 1 pixel).
    /// Scale of 2.0 means 144 DPI, etc.
    ///
    /// Cannot be used with custom transformation matrices.
    ///
    /// # Arguments
    /// * `scale` - Scaling factor (must be > 0.0)
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = Some(scale);
        self
    }

    /// Sets the pan (translation) values for the rendered bitmap.
    ///
    /// Pan values are in bitmap pixel coordinates and are applied after scaling.
    /// Positive values move the content right/down, negative values move left/up.
    ///
    /// Cannot be used with custom transformation matrices.
    ///
    /// # Arguments
    /// * `pan_x` - Horizontal translation in pixels
    /// * `pan_y` - Vertical translation in pixels
    pub fn with_pan(mut self, pan_x: f32, pan_y: f32) -> Self {
        self.pan = Some((pan_x, pan_y));
        self
    }

    /// Sets a custom transformation matrix for advanced rendering control.
    ///
    /// The matrix transforms from page coordinates to bitmap coordinates.
    /// When specified, scale and pan parameters are ignored.
    ///
    /// # Arguments
    /// * `matrix` - The transformation matrix to apply
    pub fn with_matrix(mut self, matrix: PdfiumMatrix) -> Self {
        self.matrix = Some(matrix);
        self
    }

    /// Validates the configuration for internal consistency.
    ///
    /// This method checks for conflicting or impossible parameter combinations
    /// and returns descriptive error messages for invalid configurations.
    pub fn validate(&self) -> PdfiumResult<()> {
        // Check for basic dimension requirements
        if self.width.is_none() && self.height.is_none() {
            return Err(PdfiumError::InvalidConfiguration(
                "At least width or height must be specified".to_string(),
            ));
        }

        // Check for positive dimensions
        if let Some(w) = self.width {
            if w <= 0 {
                return Err(PdfiumError::InvalidConfiguration(
                    "Width must be greater than 0".to_string(),
                ));
            }
        }

        if let Some(h) = self.height {
            if h <= 0 {
                return Err(PdfiumError::InvalidConfiguration(
                    "Height must be greater than 0".to_string(),
                ));
            }
        }

        // Check scale parameter validity
        if let Some(scale) = self.scale {
            if scale <= 0.0 || !scale.is_finite() {
                return Err(PdfiumError::InvalidConfiguration(
                    "Scale must be a positive finite number".to_string(),
                ));
            }
        }

        // Check for conflicting transformation parameters
        if self.matrix.is_some() && self.scale.is_some() {
            return Err(PdfiumError::InvalidConfiguration(
                "Cannot specify both matrix and scale parameters".to_string(),
            ));
        }

        if self.matrix.is_some() && self.pan.is_some() {
            return Err(PdfiumError::InvalidConfiguration(
                "Cannot specify both matrix and pan parameters".to_string(),
            ));
        }

        // Check for dimension/transformation compatibility
        if self.width.is_some()
            && self.height.is_some()
            && self.matrix.is_none()
            && self.scale.is_none()
        {
            return Err(PdfiumError::InvalidConfiguration(
                "When both width and height are specified, scale or matrix must be provided"
                    .to_string(),
            ));
        }

        Ok(())
    }
}

impl PdfiumPage {
    /// Renders this [`PdfiumPage`] into a new [`PdfiumBitmap`] using the specified configuration.
    ///
    /// This method handles the complex logic of translating the configuration parameters
    /// into the appropriate PDFium rendering calls, including dimension calculations,
    /// transformation matrix setup, and bitmap creation.
    ///
    /// # Arguments
    /// * `config` - The rendering configuration to use
    ///
    /// # Returns
    /// * `Ok(PdfiumBitmap)` - The rendered bitmap on success
    /// * `Err(PdfiumError)` - An error describing what went wrong
    ///
    /// # Errors
    /// This method can return errors for:
    /// - Invalid configuration parameters
    /// - PDFium rendering failures
    /// - Memory allocation failures
    /// - Page boundary calculation errors
    pub fn render(&self, config: &PdfiumRenderConfig) -> PdfiumResult<PdfiumBitmap> {
        // Validate configuration first
        config.validate()?;

        // Calculate final dimensions and transformation matrix
        let (width, height, matrix) = self.calculate_render_parameters(config)?;

        // Create the target bitmap
        let bitmap = PdfiumBitmap::empty(width, height, config.format)?;

        // Fill background if specified
        if let Some(color) = config.background {
            bitmap.fill(&color)?;
        };

        // Set up clipping rectangle (default to full bitmap if not specified)
        let clipping = config.clipping.unwrap_or(PdfiumRect::new(
            0.0,
            0.0,
            bitmap.width() as f32,
            bitmap.height() as f32,
        ));

        // Convert to PDFium types and render
        let clipping: FS_RECTF = (&clipping).into();
        let matrix: FS_MATRIX = (&matrix).into();

        lib().FPDF_RenderPageBitmapWithMatrix(
            &bitmap,
            self,
            &matrix,
            &clipping,
            config.flags.bits(),
        );

        Ok(bitmap)
    }

    /// Calculates the final rendering parameters (width, height, matrix) from the configuration.
    ///
    /// This internal method handles the complex logic of determining final dimensions
    /// and transformation based on the various configuration options.
    fn calculate_render_parameters(
        &self,
        config: &PdfiumRenderConfig,
    ) -> PdfiumResult<(i32, i32, PdfiumMatrix)> {
        match (config.width, config.height) {
            (None, None) => {
                // This should be caught by validate(), but just in case
                Err(PdfiumError::InvalidConfiguration(
                    "At least width or height needs to be specified".to_string(),
                ))
            }
            (None, Some(h)) => {
                // Height specified, calculate width from aspect ratio
                if config.matrix.is_some() || config.scale.is_some() {
                    return Err(PdfiumError::InvalidConfiguration(
                        "Cannot specify matrix or scale when only height is provided".to_string(),
                    ));
                }
                let bounds = self.boundaries().default()?;
                let scale = h as f32 / bounds.height();
                let w = (bounds.width() * scale) as i32;
                let m = PdfiumMatrix::new_scale_opt_pan(scale, config.pan);
                Ok((w, h, m))
            }
            (Some(w), None) => {
                // Width specified, calculate height from aspect ratio
                if config.matrix.is_some() || config.scale.is_some() {
                    return Err(PdfiumError::InvalidConfiguration(
                        "Cannot specify matrix or scale when only width is provided".to_string(),
                    ));
                }
                let bounds = self.boundaries().default()?;
                let scale = w as f32 / bounds.width();
                let h = (bounds.height() * scale) as i32;
                let m = PdfiumMatrix::new_scale_opt_pan(scale, config.pan);
                Ok((w, h, m))
            }
            (Some(w), Some(h)) => {
                // Both dimensions specified, need explicit transformation
                let m = match (config.matrix, config.scale) {
                    (None, None) => return Err(PdfiumError::InvalidConfiguration(
                        "When both width and height are specified, scale or matrix must be provided"
                            .to_string(),
                    )),
                    (None, Some(s)) => PdfiumMatrix::new_scale_opt_pan(s, config.pan),
                    (Some(m), None) => {
                        if config.pan.is_some() {
                            return Err(PdfiumError::InvalidConfiguration(
                                "Cannot specify both matrix and pan parameters".to_string(),
                            ));
                        };
                        m
                    }
                    (Some(_), Some(_)) => {
                        return Err(PdfiumError::InvalidConfiguration(
                            "Cannot specify both matrix and scale parameters".to_string(),
                        ))
                    }
                };
                Ok((w, h, m))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{document::PdfiumDocument, PdfiumColor};

    #[test]
    fn test_render_at_height() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let config = PdfiumRenderConfig::new().with_height(1080);
        let bitmap = page.render(&config).unwrap();
        assert_eq!(bitmap.width(), 763);
        assert_eq!(bitmap.height(), 1080);
    }

    #[test]
    fn test_render_at_width() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(1).unwrap();
        let config = PdfiumRenderConfig::new().with_width(1920);
        let bitmap = page.render(&config).unwrap();
        assert_eq!(bitmap.width(), 1920);
        assert_eq!(bitmap.height(), 2716);
    }

    #[test]
    fn test_invalid_config_no_dimensions() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let config = PdfiumRenderConfig::new();
        let result = page.render(&config);
        assert!(result.is_err());
        if let Err(PdfiumError::InvalidConfiguration(msg)) = result {
            assert!(msg.contains("At least width or height"));
        }
    }

    #[test]
    fn test_invalid_config_negative_dimensions() {
        let config = PdfiumRenderConfig::new().with_width(-100);
        assert!(config.validate().is_err());

        let config = PdfiumRenderConfig::new().with_height(-50);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_config_zero_dimensions() {
        let config = PdfiumRenderConfig::new().with_width(0);
        assert!(config.validate().is_err());

        let config = PdfiumRenderConfig::new().with_height(0);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_config_matrix_and_scale() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let matrix = PdfiumMatrix::identity();
        let config = PdfiumRenderConfig::new()
            .with_size(800, 600)
            .with_scale(2.0)
            .with_matrix(matrix);
        let result = page.render(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_config_matrix_and_pan() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let matrix = PdfiumMatrix::identity();
        let config = PdfiumRenderConfig::new()
            .with_size(800, 600)
            .with_matrix(matrix)
            .with_pan(10.0, 20.0);
        let result = page.render(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_config_both_dimensions_no_transform() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let config = PdfiumRenderConfig::new().with_size(800, 600);
        let result = page.render(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_config_matrix_with_single_dimension() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let matrix = PdfiumMatrix::identity();
        let config = PdfiumRenderConfig::new()
            .with_width(800)
            .with_matrix(matrix);
        let result = page.render(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_config_scale_with_single_dimension() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let config = PdfiumRenderConfig::new().with_height(600).with_scale(1.5);
        let result = page.render(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_scale_values() {
        let config = PdfiumRenderConfig::new().with_scale(0.0);
        assert!(config.validate().is_err());

        let config = PdfiumRenderConfig::new().with_scale(-1.0);
        assert!(config.validate().is_err());

        let config = PdfiumRenderConfig::new().with_scale(f32::INFINITY);
        assert!(config.validate().is_err());

        let config = PdfiumRenderConfig::new().with_scale(f32::NAN);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_valid_config_with_scale() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let config = PdfiumRenderConfig::new()
            .with_size(800, 600)
            .with_scale(1.5);
        let result = page.render(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_config_with_matrix() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let matrix = PdfiumMatrix::identity();
        let config = PdfiumRenderConfig::new()
            .with_size(800, 600)
            .with_matrix(matrix);
        let result = page.render(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transparent_background() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let config = PdfiumRenderConfig::new()
            .with_width(800)
            .with_transparent_background()
            .with_format(PdfiumBitmapFormat::Bgra);
        let result = page.render(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_custom_background_color() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let config = PdfiumRenderConfig::new()
            .with_width(800)
            .with_background(PdfiumColor::new(255, 0, 0, 255)); // Red background
        let result = page.render(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_clipping_rectangle() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let clipping_rect = PdfiumRect::new(0.0, 0.0, 400.0, 300.0);
        let config = PdfiumRenderConfig::new()
            .with_width(800)
            .with_clipping(clipping_rect);
        let result = page.render(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_flags() {
        let config = PdfiumRenderConfig::new()
            .with_width(800)
            .with_flags(PdfiumRenderFlags::ANNOT)
            .add_flags(PdfiumRenderFlags::LCD_TEXT | PdfiumRenderFlags::GRAYSCALE);

        assert!(config.flags.contains(PdfiumRenderFlags::ANNOT));
        assert!(config.flags.contains(PdfiumRenderFlags::LCD_TEXT));
        assert!(config.flags.contains(PdfiumRenderFlags::GRAYSCALE));
    }

    #[test]
    fn test_pan_with_scale() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let config = PdfiumRenderConfig::new()
            .with_size(800, 600)
            .with_scale(1.5)
            .with_pan(50.0, -25.0);
        let result = page.render(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_validation_passes_valid_config() {
        let config = PdfiumRenderConfig::new()
            .with_width(800)
            .with_flags(PdfiumRenderFlags::ANNOT)
            .with_background(PdfiumColor::WHITE);

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_different_bitmap_formats() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();

        // Test BGRA format
        let config = PdfiumRenderConfig::new()
            .with_width(400)
            .with_format(PdfiumBitmapFormat::Bgra);
        let result = page.render(&config);
        assert!(result.is_ok());

        // Test RGB format (assuming it exists)
        // let config = PdfiumRenderConfig::new()
        //     .with_width(400)
        //     .with_format(PdfiumBitmapFormat::Rgb);
        // let result = page.render(&config);
        // assert!(result.is_ok());
    }

    #[test]
    fn test_error_message_content() {
        let config = PdfiumRenderConfig::new();
        if let Err(PdfiumError::InvalidConfiguration(msg)) = config.validate() {
            assert!(msg.contains("At least width or height must be specified"));
        } else {
            panic!("Expected InvalidConfiguration error");
        }

        let config = PdfiumRenderConfig::new().with_width(-5);
        if let Err(PdfiumError::InvalidConfiguration(msg)) = config.validate() {
            assert!(msg.contains("Width must be greater than 0"));
        } else {
            panic!("Expected InvalidConfiguration error");
        }
    }

    #[test]
    fn test_builder_pattern_chaining() {
        let config = PdfiumRenderConfig::new()
            .with_width(1920)
            .with_format(PdfiumBitmapFormat::Bgra)
            .with_background(PdfiumColor::new(240, 240, 240, 255))
            .with_flags(PdfiumRenderFlags::ANNOT | PdfiumRenderFlags::LCD_TEXT)
            .add_flags(PdfiumRenderFlags::PRINTING);

        assert_eq!(config.width, Some(1920));
        assert_eq!(config.format, PdfiumBitmapFormat::Bgra);
        assert!(config.flags.contains(PdfiumRenderFlags::ANNOT));
        assert!(config.flags.contains(PdfiumRenderFlags::LCD_TEXT));
        assert!(config.flags.contains(PdfiumRenderFlags::PRINTING));
    }

    #[test]
    fn test_edge_case_very_small_dimensions() {
        let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None).unwrap();
        let page = document.page(0).unwrap();
        let config = PdfiumRenderConfig::new().with_width(1); // Very small
        let result = page.render(&config);
        assert!(result.is_ok());
        let bitmap = result.unwrap();
        assert_eq!(bitmap.width(), 1);
        assert!(bitmap.height() > 0); // Height should be calculated
    }

    #[test]
    fn test_edge_case_very_large_scale() {
        let config = PdfiumRenderConfig::new()
            .with_size(100, 100)
            .with_scale(1000.0); // Very large scale

        // Should validate successfully
        assert!(config.validate().is_ok());
    }
}
