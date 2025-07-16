# PDFium-rs

[![Crates.io][crates-badge]][crates-url]
[![GitHub Actions Workflow Status][build-badge]][build-url]
[![Docs.io][docs-badge]][docs-url]
[![Crates.io License][lic-badge]][lic-url]

[crates-badge]: https://img.shields.io/crates/v/pdfium.svg
[crates-url]: https://crates.io/crates/pdfium
[docs-badge]: https://img.shields.io/docsrs/pdfium
[docs-url]: https://docs.rs/pdfium
[lic-badge]: https://img.shields.io/crates/l/pdfium
[lic-url]: https://github.com/newinnovations/PDFium-rs/blob/main/README.md
[build-badge]: https://img.shields.io/github/actions/workflow/status/newinnovations/PDFium-rs/cargo_test.yml
[build-url]: https://github.com/newinnovations/PDFium-rs/actions/workflows/cargo_test.yml

A streamlined Rust wrapper for the PDFium C library, designed for simplicity and thread safety in interactive applications. PDFium is the PDF library from Google developed for and used in Chromium and Chrome.

PDFium-rs is used as one of the pdf engines in [MView6](https://github.com/newinnovations/mview6), a PDF and photo viewer written in Rust and GTK4.

## Features

- **Thread-safe static library access** - Initialize once, use everywhere
- **Lifetime-free API** - No complex lifetime management for documents, pages, and bitmaps
- **Interactive rendering focus** - Optimized render functions for real-time applications
- **Simplified design** - Clean API that closely follows PDFium's C interface

## Why This Crate?

While there are existing PDFium bindings for Rust, this crate takes a different approach focused on ease of use and thread safety:

### Thread-Safe Static Access

The library uses a static, thread-safe initialization pattern with `parking_lot::ReentrantMutex`. On first use, it checks for PDFium availability on your system or a provided directory and stores the library reference statically for the application's lifetime. This prevents deadlocks when used multiple times in the same thread and eliminates the need for complex library management.

### No Lifetime Complexity

Unlike other implementations, this crate doesn't impose lifetimes on structs representing documents, pages, and bitmaps. This makes integration into your application much simpler - you can store these objects wherever you need them without fighting the borrow checker.

### Clean Integration

Aims to follow the PDFium C-language interface, while providing a high level Rust integration.

### Interactive Application Focus

Provides access to rendering functions specifically optimized for interactive use cases, making it ideal for PDF viewers, editors, and other real-time applications.

## Quick Start

```rust
use pdfium::*;

struct App {
    doc: PdfiumDocument,
}

impl App {
    pub fn new(filename: &str) -> PdfiumResult<Self> {
        Ok(App {
            doc: PdfiumDocument::new_from_path(filename, None)?,
        })
    }
    pub fn render_to_file(&self, filename: &str, index: i32) -> PdfiumResult<()> {
        let page = self.doc.page(index)?;
        let bounds = page.boundaries().media()?;
        let height = 1080;
        let zoom = height as f32 / bounds.height();
        let width = (bounds.width() * zoom) as i32;
        let matrix = PdfiumMatrix::new_scale(zoom);
        let bitmap = page.render_with_matrix(
            width,
            height,
            PdfiumBitmapFormat::Bgra,
            Some(PdfiumColor::WHITE),
            &matrix,
            0,
            None,
        )?;
        bitmap.save(filename, image::ImageFormat::Png)
    }
}

fn main() -> PdfiumResult<()> {
    let app = App::new("resources/groningen.pdf")?;
    app.render_to_file("groningen.png", 0)
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pdfium = "0.3.0"
```

## Current Status

⚠️ **Work in Progress** - This crate is actively being developed and currently implements a focused subset of PDFium's functionality. While it covers the core rendering and document manipulation features needed for most interactive applications, it doesn't yet provide the full feature set available in more comprehensive PDFium bindings.

### What's Available

- Document loading and basic properties
- Page rendering with customizable parameters
- Bitmap handling
- Thread-safe access patterns

### Planned Features

- Text extraction
- Form support
- Annotation support
- Additional rendering options
- Security and encryption support

## Requirements

- PDFium dynamic library must be available on your system
- Rust 1.85 or later

## Contributing

Contributions are welcome!

## License

PDFium-rs is free software: you can redistribute it and/or modify it under the terms of
the GNU General Public License as published by the Free Software Foundation, either version 3
of the License, or (at your option) any later version.
