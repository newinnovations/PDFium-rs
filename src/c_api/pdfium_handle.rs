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

use std::{fmt::Debug, rc::Rc};

/// Inner wrapper that owns the raw handle and its cleanup function.
/// This struct is not directly exposed to users - it's wrapped in Rc<> for safe sharing.
struct HandleWrapper<T> {
    /// Raw pointer to the PDFium C handle/resource
    handle: *mut T,
    /// Optional cleanup function to call when the handle is dropped.
    /// Takes the raw pointer as parameter to perform resource cleanup (e.g., free, close, etc.)
    cleanup_fn: Option<fn(*mut T)>,
}

/// Implements automatic cleanup when the last reference to the handle is dropped
impl<T> Drop for HandleWrapper<T> {
    fn drop(&mut self) {
        // If a cleanup function was provided, call it with the raw handle
        if let Some(cleanup_fn) = self.cleanup_fn {
            (cleanup_fn)(self.handle);
            #[cfg(feature = "debug_print")]
            println!("  close Handle<{}>({:p})", name::<T>(), self.handle);
        } else {
            // No cleanup function - this is likely a const handle or one that doesn't need cleanup
            #[cfg(feature = "debug_print")]
            println!("   drop Handle<{}>({:p})", name::<T>(), self.handle);
        }
    }
}

/// Public handle type that provides safe, reference-counted access to PDFium C resources.
///
/// This type can be cloned cheaply (only increments reference count) and automatically
/// cleans up the underlying resource when the last reference is dropped.
#[derive(Clone)]
pub struct Handle<T> {
    /// Reference-counted wrapper around the actual handle data
    /// Using Rc allows multiple Handle instances to safely share the same underlying resource
    inner: Rc<HandleWrapper<T>>,
}

impl<T> Handle<T> {
    /// Creates a new handle from a mutable raw pointer with an optional cleanup function.
    ///
    /// # Arguments
    /// * `handle` - Raw pointer to the PDFium C resource
    /// * `cleanup_fn` - Optional function to call when cleaning up the resource
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The handle pointer is valid for the lifetime of this Handle
    /// - The cleanup function (if provided) is appropriate for this resource type
    /// - No other code will free this resource while Handle instances exist
    pub fn new(handle: *mut T, cleanup_fn: Option<fn(*mut T)>) -> Self {
        #[cfg(feature = "debug_print")]
        println!("    new Handle<{}>({:p})", name::<T>(), handle);

        Handle {
            inner: Rc::new(HandleWrapper { handle, cleanup_fn }),
        }
    }

    /// Creates a new handle from a const raw pointer.
    ///
    /// This is typically used for handles that don't need cleanup (e.g., global constants,
    /// or resources managed elsewhere). No cleanup function will be called when dropped.
    ///
    /// # Arguments
    /// * `handle` - Raw const pointer to the PDFium C resource
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The handle pointer is valid for the lifetime of this Handle
    /// - The resource will remain valid until all Handle instances are dropped
    pub fn new_const(handle: *const T) -> Self {
        #[cfg(feature = "debug_print")]
        println!("  const Handle<{}>({:p})", name::<T>(), handle);

        Handle {
            inner: Rc::new(HandleWrapper {
                // Cast const to mut pointer - safe because cleanup_fn is None
                // so we'll never actually mutate through this pointer
                handle: handle as *mut T,
                cleanup_fn: None, // No cleanup for const handles
            }),
        }
    }

    /// Returns the raw handle pointer for use in C FFI calls.
    ///
    /// # Safety
    /// The returned pointer should only be used while this Handle (or a clone) exists.
    /// The caller should not free or modify the resource through this pointer.
    pub fn handle(&self) -> *mut T {
        self.inner.handle
    }
}

/// Custom Debug implementation that shows the type name and pointer address
impl<T> Debug for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handle<{}>({:p})", name::<T>(), self.inner.handle)
    }
}

/// Helper function to get a clean, readable type name for debugging.
///
/// Removes common FFI-related prefixes and suffixes to make debug output more readable.
/// For example: "pdfium::c_api::pdfium_types::fpdf_page_t__" becomes "page"
fn name<T>() -> String {
    std::any::type_name::<T>()
        .replace("pdfium::c_api::pdfium_types::fpdf_", "") // Remove module path
        .replace("_t__", "") // Remove common C type suffixes
}
