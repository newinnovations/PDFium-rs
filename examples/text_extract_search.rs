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

use pdfium::*;

/// Extracts and prints text from all pages of a PDF document
pub fn example_extract_text() -> PdfiumResult<()> {
    // Load the PDF document from the specified file path
    // The second parameter (None) indicates no password is required
    let document = PdfiumDocument::new_from_path("resources/chapter1.pdf", None)?;

    // Iterate through all pages in the document
    // enumerate() provides both the index and the page object
    for (index, page) in document.pages().enumerate() {
        // Extract the full text content from the current page
        // The ?. operators handle potential errors at each step:
        // - page? ensures the page loaded successfully
        // - .text()? extracts text objects from the page
        // - .full() gets the complete text content as a string
        let text = page?.text()?.full();

        // Print formatted output for each page
        println!("Page {}", index + 1); // Pages are 1-indexed for user display
        println!("------");
        println!("{text}");
        println!() // Empty line for separation between pages
    }

    Ok(())
}

/// Demonstrates text search functionality within a PDF document
pub fn example_search() -> PdfiumResult<()> {
    // Load the PDF document to search within
    let document = PdfiumDocument::new_from_path("resources/groningen.pdf", None)?;

    // Get the first page (index 0) for searching
    let page = document.page(0)?;

    // Extract text objects from the page for searching
    let text = page.text()?;

    // Search for "amsterdam" with case-insensitive matching
    // PdfiumSearchFlags::empty() means no special search flags (case-insensitive by default)
    // The last parameter (0) is the starting position for the search
    let search = text.find("amsterdam", PdfiumSearchFlags::empty(), 0);
    println!("Found amsterdam {} times", search.count());

    // Search for "groningen" with case-insensitive matching
    let search = text.find("groningen", PdfiumSearchFlags::empty(), 0);
    println!(
        "Found groningen {} times (case insensitive)",
        search.count()
    );

    // Search for "Groningen" with case-sensitive matching
    // MATCH_CASE flag enforces exact case matching
    let search = text.find("Groningen", PdfiumSearchFlags::MATCH_CASE, 0);
    println!("Found Groningen {} times (case sensitive)", search.count());

    // Perform another case-insensitive search to iterate through results
    let search = text.find("groningen", PdfiumSearchFlags::empty(), 0);

    // Iterate through each search result to extract detailed information
    for result in search {
        // Extract the text fragment at the found position
        // result.index() gives the character position where the match starts
        // result.count() gives the length of the matched text
        let fragment = text.extract(result.index(), result.count());
        println!(
            "Found groningen (case insensitive) at {}, fragment = '{fragment}'",
            result.index()
        );
    }

    // Expected output:
    //
    // Found amsterdam 0 times
    // Found groningen 5 times (case insensitive)
    // Found Groningen 5 times (case sensitive)
    // Found groningen (case insensitive) at 14, fragment = 'Groningen'
    // Found groningen (case insensitive) at 232, fragment = 'Groningen'
    // Found groningen (case insensitive) at 475, fragment = 'Groningen'
    // Found groningen (case insensitive) at 920, fragment = 'Groningen'
    // Found groningen (case insensitive) at 1050, fragment = 'Groningen'

    Ok(())
}

/// Main function that demonstrates both text extraction and search functionality
fn main() -> PdfiumResult<()> {
    example_extract_text()?;
    example_search()?;
    Ok(())
}
