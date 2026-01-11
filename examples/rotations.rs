// PDFium-rs -- Modern Rust interface to PDFium, the PDF library from Google
//
// Copyright (c) 2025-2026 Martin van der Werff <github (at) newinnovations.nl>
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

/// Demonstrate rotation operations.
///
/// This example demonstrates how to:
/// - Load a PDF document from disk
/// - Rotating a page
/// - Rendering with rotation
/// - Save each rendered page as an image file
///
/// The example uses proper error handling with Result types, allowing
/// errors to propagate up the call stack rather than panicking.
pub fn example_rotations() -> PdfiumResult<()> {
    // Load the PDF document from the specified file path
    // Parameters:
    // - "resources/groningen.pdf": Path to the PDF file (relative to current working directory)
    // - None: No password required for this PDF (use Some("password") if needed)
    let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None)?;

    let config = PdfiumRenderConfig::new().with_height(1080);

    // Method 1: Rotate page before rendering

    let page_1 = document.page(0)?;

    assert!(!page_1.is_landscape());
    assert!(page_1.is_portrait());

    page_1.set_rotation(PdfiumRotation::Cw90);
    let bitmap = page_1.render(&config)?;
    bitmap.save("groningen-rotation-90-1.jpg", image::ImageFormat::Jpeg)?;

    assert!(page_1.is_landscape());

    page_1.set_rotation(PdfiumRotation::Cw180);
    let bitmap = page_1.render(&config)?;
    bitmap.save("groningen-rotation-180-1.jpg", image::ImageFormat::Jpeg)?;

    page_1.set_rotation(PdfiumRotation::Cw270);
    let bitmap = page_1.render(&config)?;
    bitmap.save("groningen-rotation-270-1.jpg", image::ImageFormat::Jpeg)?;

    // Method 2 (better): Use `with_rotation` when creating the `PdfiumRenderConfig`

    let page_2 = document.page(1)?;

    let bitmap = page_2.render(&config.clone().with_rotation(PdfiumRotation::Cw90))?;
    bitmap.save("groningen-rotation-90-2.jpg", image::ImageFormat::Jpeg)?;

    let bitmap = page_2.render(&config.clone().with_rotation(PdfiumRotation::Cw180))?;
    bitmap.save("groningen-rotation-180-2.jpg", image::ImageFormat::Jpeg)?;

    let bitmap = page_2.render(&config.clone().with_rotation(PdfiumRotation::Cw270))?;
    bitmap.save("groningen-rotation-270-2.jpg", image::ImageFormat::Jpeg)?;

    // Return success - all operations were successful
    Ok(())
}

fn main() -> PdfiumResult<()> {
    example_rotations()?;
    Ok(())
}
