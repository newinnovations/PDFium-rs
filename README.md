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

A modern, streamlined Rust wrapper for the PDFium C library, designed for simplicity and thread safety in interactive applications. PDFium is Google's PDF library developed for (and used in) the Chromium and Chrome web browsers.

PDFium-rs serves as one of the two PDF engines behind [MView6](https://github.com/newinnovations/mview6), a PDF and photo viewer written in Rust and GTK4. With a single keypress, you can switch engines and compare the rendering quality differences between mupdf and PDFium.

## Features

- **Thread-safe static library access** - Initialize once, use everywhere
- **Lifetime-free API** - No complex lifetime management for documents, pages, and bitmaps
- **Access to C API** - Safe access to the low-level C API is [possible](#using-the-pdfium-c-api)
- **Renderer selection** - [Select](#renderer-selection-skia-or-agg) either `Skia` or `AGG` (Anti-Grain Geometry) as renderer for PDFium
- **Interactive rendering focus** - Optimized render functions for real-time applications

## Why This Crate?

While existing PDFium bindings for Rust are available, this crate takes a different approach focused on ease of use and thread safety:

### Thread-Safe Static Access

This library uses a modern, static, and thread-safe initialization pattern with `parking_lot::ReentrantMutex`. On first use, it checks for the availability of the PDFium dynamic library on your system or in a specified directory, and stores the library reference statically for the application's lifetime. This prevents deadlocks when used multiple times in the same thread and eliminates the need for complex library management.

### No Lifetime Complexity

Unlike other implementations, this crate doesn't impose lifetimes on structs representing documents, pages, bitmaps, and other structures. This makes integration into your application much simpler - you can store these objects wherever you need them without fighting the borrow checker. This makes it ideal for interactive use cases, such as PDF viewers, editors, and other real-time applications.

### Access to C API without `unsafe` Functions

While focusing on high-level safe Rust integration, PDFium-rs also provides safe public access to the C API. Unsafe pointers to C structures and memory are transparently replaced with their safe Rust counterparts.

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
            PdfiumRenderFlags::empty(),
            None,
        )?;
        bitmap.save(filename, image::ImageFormat::Png)
    }
}

fn main() -> PdfiumResult<()> {
    let app = App::new("resources/groningen.pdf")?;
    app.render_to_file("groningen-page-1-rust.png", 0)
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
        let matrix = pdfium_types::FS_MATRIX {
            a: zoom,
            b: 0.0,
            c: 0.0,
            d: zoom,
            e: 0.0,
            f: 0.0,
        };
        let bitmap = lib().FPDFBitmap_Create(width, height, 1)?;
        lib().FPDFBitmap_FillRect(&bitmap, 0, 0, width, height, 0xffffffff)?;
        let clipping = pdfium_types::FS_RECTF {
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
pdfium = "0.5.1"  # Check crates.io for the latest version
```

For the latest version, visit [crates.io](https://crates.io/crates/pdfium) or use `cargo search pdfium`.

### Dynamic Library Requirements

PDFium-rs requires the PDFium dynamic library to be available on your system:

- **Linux**: `libpdfium.so`
- **Windows**: `pdfium.dll`
- **macOS**: `libpdfium.dylib`

#### Obtaining PDFium Libraries

Pre-built libraries for all major platforms are available from:
<https://github.com/bblanchon/pdfium-binaries/releases>

Choose the appropriate build for your target platform and architecture (x64, arm64, etc.).

#### Library Installation

##### Option 1: System Installation (Recommended for Linux/macOS)

- Linux: Place in `/usr/lib/`, `/usr/local/lib/`, or `/usr/lib/your_app/`
- macOS: Place in `/usr/local/lib/` or `/opt/homebrew/lib/`
- Use [`set_library_location`] to specify custom paths

##### Option 2: Application Bundle (Recommended for Windows)

- Windows: Place `pdfium.dll` in the same directory as your executable

##### Option 3: Custom Location

```rust
use pdfium::*;

// Set custom library path before first use
set_library_location("/path/to/your/pdfium/library/");

// Your application code...
let doc = PdfiumDocument::new_from_path("document.pdf", None);
```

### Renderer Selection: `Skia` or `AGG`

PDFium currently supports two rendering backends. You can select `Skia` over `AGG` (Anti-Grain Geometry) using the [`set_use_skia`] function:

```rust
use pdfium::*;

// Use Skia renderer (must be called before first PDFium usage)
set_use_skia(true);

// Your application code...
let doc = PdfiumDocument::new_from_path("document.pdf", None);
```

## Supported Platforms

- **Linux**: x86_64, aarch64 (ARM64)
- **Windows**: x86_64, x86 (32-bit)
- **macOS**: x86_64, aarch64 (Apple Silicon)

All platforms require their respective PDFium dynamic libraries.

## Troubleshooting

### Library Loading Errors

```text
/// Error loading or initializing the PDFium library
PdfiumError::LibraryError(String)
```

#### Solutions

1. Verify the PDFium library is in the correct location
2. Check that the library architecture matches your application (64-bit vs 32-bit)
3. On Linux, verify library dependencies with `ldd libpdfium.so`
4. Use [`set_library_location`] to specify the exact path
5. Ensure file permissions allow reading the library

#### Windows-Specific Issues

- Install Visual C++ Redistributable if you encounter DLL loading errors
- Ensure `pdfium.dll` is in your PATH or application directory

#### macOS-Specific Issues

- You may need to remove quarantine attributes: `xattr -d com.apple.quarantine libpdfium.dylib`
- For unsigned libraries, you might need to approve them in System Preferences > Security & Privacy

### Runtime Errors

This might indicate an incompatible PDFium library version. Try downloading a different build from the pdfium-binaries releases.

### Getting Help

If you encounter issues not covered here:

1. Check the [GitHub Issues](https://github.com/newinnovations/PDFium-rs/issues)
2. Verify your PDFium library version compatibility
3. Include your platform, Rust version, and PDFium library source when reporting issues

## Current Status

**Work in Progress** - This crate is actively being developed and currently implements a focused subset of PDFium's functionality. While it covers the core rendering and document manipulation features needed for most interactive applications, it doesn't yet provide the complete feature set available in more comprehensive PDFium bindings.

### What's Available

- Document loading and basic properties
- Page rendering with customizable parameters
- Bitmap handling and export
- Thread-safe access patterns
- Safe C API access

### Planned Features

- Text extraction and search
- Form field support
- Annotation handling
- Additional rendering options
- Security and encryption support
- Bookmarks and navigation

## Requirements

- Rust 1.85 or later
- PDFium dynamic library (see [installation instructions](#dynamic-library-requirements))

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

After cloning the repository, install the git hooks:

```bash
./scripts/install-hooks.sh
```

## License

PDFium-rs is free software: you can redistribute it and/or modify it under the terms of
the GNU General Public License as published by the Free Software Foundation, either version 3
of the License, or (at your option) any later version.
