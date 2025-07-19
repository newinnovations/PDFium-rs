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

A modern streamlined Rust wrapper for the PDFium C library, designed for simplicity and thread safety in interactive applications. PDFium is the PDF library from Google developed for (and used in) the Chromium and Chrome web browsers.

PDFium-rs is used as one of the two PDF engines behind [MView6](https://github.com/newinnovations/mview6), a PDF and photo viewer written in Rust and GTK4. With a single keypress you can switch engines and see the rendering (quality) differences between mupdf and PDFium.

## Features

- **Thread-safe static library access** - Initialize once, use everywhere
- **Lifetime-free API** - No complex lifetime management for documents, pages, and bitmaps
- **Access to C API** - Safe access to the low level C API is [possible](#using-the-pdfium-c-api)
- **Renderer selection** - [Select](#skia-or-agg) either `Skia` or `AGG` (Aggregated Graphics) as renderer for PDFium
- **Interactive rendering focus** - Optimized render functions for real-time applications

## Why This Crate?

While there are existing PDFium bindings for Rust, this crate takes a different approach focused on ease of use and (thread) safety:

### Thread-Safe Static Access

This library uses a modern, static and thread-safe initialization pattern with `parking_lot::ReentrantMutex`. On first use, it checks for the availability of the PDFium dynamic library on your system or in a provided directory; and stores the library reference statically for the application's lifetime. This prevents deadlocks when used multiple times in the same thread and eliminates the need for complex library management.

### No Lifetime Complexity

Unlike other implementations, this crate doesn't impose lifetimes on structs representing documents, pages, bitmaps and other structures. This makes integration into your application much simpler - you can store these objects wherever you need them without fighting the borrow checker.

### Access to C API without `unsafe` functions

While focussing on the high level safe Rust integration, PDFium-rs also provides safe public access to the C API. Unsafe pointers to C structures and memory are transparently replaced with their safe Rust counterparts.

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
    app.render_to_file("groningen-page-1.png", 0)
}
```

## Using the PDFium C API

This is the same example, but now using the C API of PDFium directly.

- Using the C API is safe, no `unsafe` code blocks in your code
- Access the C API through [`lib`] or [`try_lib`]
- You can mix high-level Rust and C, shown here with the `bitmap.save` operation

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
        let page = lib().FPDF_LoadPage(&self.doc, index)?;
        let mut left = 0.0;
        let mut bottom = 0.0;
        let mut right = 0.0;
        let mut top = 0.0;
        lib().FPDFPage_GetMediaBox(&page, &mut left, &mut bottom, &mut right, &mut top)?;
        let height = 1080;
        let zoom = height as f32 / (top - bottom);
        let width = ((right - left) * zoom) as i32;
        let matrix = PdfiumMatrix {
            a: zoom,
            b: 0.0,
            c: 0.0,
            d: zoom,
            e: 0.0,
            f: 0.0,
        };
        let bitmap = lib().FPDFBitmap_Create(width, height, 1)?;
        lib().FPDFBitmap_FillRect(&bitmap, 0, 0, width, height, 0xffffffff)?;
        let clipping = PdfiumRect {
            left: 0.0,
            top: height as f32,
            right: width as f32,
            bottom: 0.0,
        };
        lib().FPDF_RenderPageBitmapWithMatrix(&bitmap, &page, &matrix, &clipping, 0);
        bitmap.save(filename, image::ImageFormat::Png)?;
        Ok(())
    }
}

fn main() -> PdfiumResult<()> {
    let app = App::new("resources/groningen.pdf")?;
    app.render_to_file("groningen-page-1-c.png", 0)
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pdfium = "0.4.2"
```

### Dynamic library

PDFium-rs needs to load the PDFium dynamic library (`libpdfium.so`, `pdfium.dll` or `libpdfium.dylib`) from your system.

Pre-built libraries are available for all major platforms from <https://github.com/bblanchon/pdfium-binaries/releases>.

You can place the library in a system location, with your application or in a custom location. You can specify the location using [`set_library_location`] (default is the current directory). PDFium-rs will try the specified location first, and then seach system locations. On windows you can place it in the same directory as the executable, on linux it is best to store it in a location like `/usr/lib/your_app` and specify it using [`set_library_location`].

### `Skia` or `AGG`

PDFium currently supports both renderers. You can select `Skia` over `AGG` (Aggregated Graphics) using the [`set_use_skia`] function

## Current Status

**Work in Progress** - This crate is actively being developed and currently implements a focused subset of PDFium's functionality. While it covers the core rendering and document manipulation features needed for most interactive applications, it doesn't yet provide the full feature set available in more comprehensive PDFium bindings.

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

After cloning the repository, please install the git hooks:

```bash
./scripts/install-hooks.sh
```

## License

PDFium-rs is free software: you can redistribute it and/or modify it under the terms of
the GNU General Public License as published by the Free Software Foundation, either version 3
of the License, or (at your option) any later version.
