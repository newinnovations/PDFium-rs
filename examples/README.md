# How to use `PDFium-rs`

PDFium-rs requires the PDFium dynamic library to be available:

- **Linux**: `libpdfium.so`
- **Windows**: `pdfium.dll`
- **macOS**: `libpdfium.dylib`

See the main `README.md` how to obtain and install for your system.

## Export pages to images

File: `export_pages.rs`

Run with: `cargo run --example export_pages`

- Exports all pages from a PDF document as individual JPEG images.

## Import pages from another document

File: `import_pages.rs`

Run with: `cargo run --example import_pages`

- Demonstrates importing PDF pages using
  - *string*-based page specification
  - *index*-based page specification
