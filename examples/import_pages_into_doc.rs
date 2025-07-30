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
// This gives us access to PdfiumDocument and related functionality
use pdfium::*;

/// Demonstrates importing PDF pages using string-based page specification.
///
/// This function shows how to:
/// - Create a new empty PDF document
/// - Load an existing PDF file as a source
/// - Import specific pages using a human-readable string format
/// - Save the resulting document to disk
/// - Verify the operation was successful
///
/// The string format supports:
/// - Individual pages: "5" (imports page 5)
/// - Multiple pages: "1,3,7" (imports pages 1, 3, and 7)
/// - Page ranges: "10-15" (imports pages 10 through 15 inclusive)
/// - Mixed format: "1,5-8,12" (imports page 1, pages 5-8, and page 12)
///
/// Note: Page numbers in the string are 1-based (human-readable format)
pub fn example_import_pages() -> PdfiumResult<()> {
    // Create a new, empty PDF document that will serve as our destination
    // This document starts with 0 pages and we'll add pages to it
    let document = PdfiumDocument::new()?;

    // Load the source PDF document from which we'll extract pages
    // The second parameter (None) means we're not providing a password
    let src_doc = PdfiumDocument::new_from_path("resources/pg1342-images-3.pdf", None)?;

    // Import specific pages from the source document into our destination document
    // Parameters breakdown:
    // - &src_doc: Reference to the source document to import from
    // - "12,14,30-34": String specifying which pages to import
    //   * Page 12 (individual page)
    //   * Page 14 (individual page)
    //   * Pages 30-34 (range of 5 pages: 30, 31, 32, 33, 34)
    //   * Total: 7 pages will be imported
    // - 0: Index position where imported pages should be inserted
    //   * 0 means insert at the beginning of the destination document
    //   * If the destination had existing pages, imported pages would be inserted before them
    document.import_pages(&src_doc, "12,14,30-34", 0)?;

    // Save the destination document with imported pages to a new file
    // The second parameter (None) indicates we're not specifying a version
    document.save_to_path("pride-1.pdf", None)?;

    // Verification step: reload the saved document to confirm the operation
    let document = PdfiumDocument::new_from_path("pride-1.pdf", None)?;

    // Get the total number of pages in the saved document
    let page_count = document.page_count();

    // Assert that we have exactly 7 pages as expected
    // This confirms that all specified pages were imported correctly:
    // 1 page (12) + 1 page (14) + 5 pages (30-34) = 7 pages total
    assert_eq!(page_count, 7);

    Ok(())
}

/// Demonstrates importing PDF pages using index-based page specification.
///
/// This function shows an alternative approach to page importing using
/// explicit page indices rather than string specifications. This method
/// provides more programmatic control and is useful when:
/// - You have a dynamically generated list of page numbers
/// - You need to import non-contiguous pages with complex patterns
/// - You're working with 0-based indexing in your application logic
///
/// Key differences from string-based approach:
/// - Uses 0-based indexing (first page is index 0)
/// - Requires explicit specification of each page index
/// - More verbose but offers precise control
/// - Better for programmatic generation of page lists
pub fn example_import_pages_by_index() -> PdfiumResult<()> {
    // Create a new, empty PDF document to serve as our destination
    let document = PdfiumDocument::new()?;

    // Load the same source PDF document as in the previous example
    // We're extracting the same pages but using a different method
    let src_doc = PdfiumDocument::new_from_path("resources/pg1342-images-3.pdf", None)?;

    // Import pages using explicit 0-based indices
    // Parameters breakdown:
    // - &src_doc: Reference to the source document
    // - Some(&vec![11, 13, 29, 30, 31, 32, 33]): Vector of 0-based page indices
    //   * Index 11 = Page 12 in human numbering (11 + 1 = 12)
    //   * Index 13 = Page 14 in human numbering (13 + 1 = 14)
    //   * Indices 29-33 = Pages 30-34 in human numbering
    //   * Note: This matches exactly the same pages as "12,14,30-34" from the previous example
    //   * The Some() wrapper indicates we're providing a specific list of indices
    //   * Using None instead would import all pages from the source document
    // - 0: Insertion position (beginning of destination document)
    document.import_pages_by_index(&src_doc, Some(&[11, 13, 29, 30, 31, 32, 33]), 0)?;

    // Save the document with a different filename to distinguish from the first example
    // Even though the content should be identical, using different names helps with testing
    document.save_to_path("pride-2.pdf", None)?;

    // Verification: reload the saved document to ensure it was created correctly
    let document = PdfiumDocument::new_from_path("pride-2.pdf", None)?;

    // Count the pages in the saved document
    let page_count = document.page_count();

    // Verify that we have the same 7 pages as the string-based method
    // This confirms both methods produce identical results:
    // - 1 page at index 11 (page 12)
    // - 1 page at index 13 (page 14)
    // - 5 pages at indices 29-33 (pages 30-34)
    // Total: 7 pages

    assert_eq!(page_count, 7);

    Ok(())
}

fn main() -> PdfiumResult<()> {
    example_import_pages()?;
    example_import_pages_by_index()?;
    Ok(())
}
