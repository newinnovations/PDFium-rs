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

// Import all necessary items from the pdfium crate
// This provides access to PDF document handling, rendering, and bitmap operations
use pdfium::*;

/// Exports all pages from a PDF document as individual JPEG images.
///
/// This function demonstrates how to:
/// - Load a PDF document from disk
/// - Iterate through all pages in the document
/// - Render each page as a bitmap with specific dimensions and format
/// - Save each rendered page as a separate image file
///
/// The function uses proper error handling with Result types, allowing
/// errors to propagate up the call stack rather than panicking.
pub fn example_export_pages_to_images() -> PdfiumResult<()> {
    // Load the PDF document from the specified file path
    // Parameters:
    // - "resources/groningen.pdf": Path to the PDF file (relative to current working directory)
    // - None: No password required for this PDF (use Some("password") if needed)
    let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None)?;

    // Iterate through all pages in the document
    // document.pages() returns an iterator over all pages
    // enumerate() adds an index counter starting from 0
    // This gives us both the page object and its 0-based index
    for (index, page) in document.pages().enumerate() {
        // Render the current page as a bitmap image
        // This is where the PDF content gets converted to a raster image
        //
        // Parameters breakdown:
        // - 1080: Target height in pixels
        //   * The width will be calculated automatically to maintain aspect ratio
        //
        // - PdfiumBitmapFormat::Bgra: Pixel format specification
        //   * BGRA = Blue, Green, Red, Alpha (transparency) channels
        //   * Each channel uses 8 bits, resulting in 32 bits per pixel
        //   * This format is commonly used and well-supported by image libraries
        //
        // - PdfiumRenderFlags::empty(): Rendering options/flags
        //   * empty() means use default rendering settings
        //   * Other options include: REVERSE_BYTE_ORDER, ANNOTATIONS, LCD_TEXT, etc.
        //   * Multiple flags can be combined
        let bitmap =
            page.render_at_height(1080, PdfiumBitmapFormat::Bgra, PdfiumRenderFlags::empty())?;

        // Verify that the bitmap was rendered at the requested height
        // This assertion ensures the rendering process worked as expected
        // If this fails, it indicates a bug in the rendering logic
        assert_eq!(bitmap.height(), 1080);

        // Generate a unique filename for this page
        // Format: "groningen-page-{page_number}.jpg"
        // - index + 1 converts from 0-based index to 1-based page numbers
        //   * Page 0 becomes "groningen-page-1.jpg"
        //   * Page 1 becomes "groningen-page-2.jpg", etc.
        // - The .jpg extension indicates JPEG format will be used
        let filename = format!("groningen-page-{}.jpg", index + 1);

        // Save the rendered bitmap to disk as a JPEG image
        // Parameters:
        // - &filename: Reference to the generated filename string
        // - image::ImageFormat::Jpeg: Specifies JPEG compression format
        //   * Alternative format: Png (lossless)
        //   * JPEG provides good compression but is lossy (some quality loss)
        //
        // The save operation handles:
        // - Converting from BGRA format to JPEG-compatible format
        // - Applying JPEG compression
        // - Writing the file to disk
        bitmap.save(&filename, image::ImageFormat::Jpeg)?;

        // Note: No explicit cleanup needed - Rust's ownership system automatically
        // deallocates the bitmap memory when it goes out of scope at the end of this iteration
    }

    // Return success - all pages have been successfully exported
    Ok(())
}

fn main() -> PdfiumResult<()> {
    example_export_pages_to_images()?;
    Ok(())
}
