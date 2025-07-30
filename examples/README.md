# How to use `PDFium-rs`

PDFium-rs requires the PDFium dynamic library to be available on your system:

- **Linux**: `libpdfium.so`
- **Windows**: `pdfium.dll`
- **macOS**: `libpdfium.dylib`

See the main `README.md` how to obtain and install these.

## export_pages_to_images.rs

Run with `cargo run --example export_pages_to_images`

- Exports all pages from a PDF document as individual JPEG images.

## import_pages_into_doc.rs

Run with `cargo run --example import_pages_into_doc`

- Demonstrates importing PDF pages using
  - *string*-based page specification
  - *index*-based page specification
