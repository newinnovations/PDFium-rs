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

use parking_lot::{ReentrantMutex, ReentrantMutexGuard};
use std::sync::{atomic::AtomicBool, LazyLock, Mutex, OnceLock};

use crate::{c_api::pdfium_bindings::Pdfium, error::PdfiumResult, PdfiumError};

static PDFIUM: OnceLock<ReentrantMutex<PdfiumResult<Box<Pdfium>>>> = OnceLock::new();

static LIBRARY_LOCATION: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::from(".")));

static SKIA_RENDERER: AtomicBool = AtomicBool::new(false);

fn load_pdfium() -> &'static ReentrantMutex<PdfiumResult<Box<Pdfium>>> {
    PDFIUM.get_or_init(|| {
        let directory = LIBRARY_LOCATION
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .to_string();
        let use_skia = SKIA_RENDERER.load(std::sync::atomic::Ordering::Relaxed);
        let pdfium = Pdfium::load_from_directory(&directory).or_else(|_| Pdfium::load());

        if let Ok(pdfium) = &pdfium {
            pdfium.init(use_skia);
        }

        ReentrantMutex::new(pdfium)
    })
}

/// A guard that holds the reentrant mutex lock and provides access to [`PdfiumBindings`]
pub struct PdfiumGuard {
    _guard: ReentrantMutexGuard<'static, PdfiumResult<Box<Pdfium>>>,
    pdfium: *const Box<Pdfium>,
}

impl PdfiumGuard {
    fn new(guard: ReentrantMutexGuard<'static, PdfiumResult<Box<Pdfium>>>) -> PdfiumResult<Self> {
        match guard.as_ref() {
            Ok(p) => Ok(PdfiumGuard {
                pdfium: p as *const Box<Pdfium>,
                _guard: guard,
            }),
            Err(e) => Err(match e {
                PdfiumError::LibraryError(msg) => PdfiumError::LibraryError(msg.to_string()),
                _ => PdfiumError::Unknown,
            }),
        }
    }
}

impl std::ops::Deref for PdfiumGuard {
    type Target = Pdfium;

    fn deref(&self) -> &Self::Target {
        // Safety: The pointer is valid as long as the guard is held
        unsafe { &(*self.pdfium) }
    }
}

/// Access to the PDFium dynamic library with thread-safe reentrant locking
///
/// This function will return an error if the library is not available. If the library
/// is not available, future calls to this function will also fail. Testing for the
/// existence of the library is only done once in the lifetime of the application.
///
/// The returned PdfiumGuard ensures that only one thread can access the library at a time,
/// but the same thread can acquire the lock multiple times (reentrant behavior).
/// The guard will automatically release the lock when it goes out of scope.
///
/// **Reentrant behavior**: If the same thread calls this function multiple times while
/// already holding a guard, it will succeed without deadlocking. However, each call
/// must be matched with a corresponding drop of the guard.
pub fn try_lib() -> PdfiumResult<PdfiumGuard> {
    let mutex = load_pdfium();
    let guard = mutex.lock();
    PdfiumGuard::new(guard)
}

/// Access to the PDFium dynamic library with thread-safe reentrant locking
///
/// This function will panic if the library is not available. Use only when you are
/// sure the library is available, for example inside an object that can only exist
/// because it is created by the library before.
///
/// The returned PdfiumGuard ensures that only one thread can access the library at a time,
/// but the same thread can acquire the lock multiple times (reentrant behavior).
/// The guard will automatically release the lock when it goes out of scope.
///
/// **Reentrant behavior**: If the same thread calls this function multiple times while
/// already holding a guard, it will succeed without deadlocking. However, each call
/// must be matched with a corresponding drop of the guard.
pub fn lib() -> PdfiumGuard {
    try_lib().unwrap()
}

/// Set a specific location (directory) for the PDFium dynamic library (so/dll/dylib)
pub fn set_library_location(location: &str) {
    let mut guard = LIBRARY_LOCATION
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = location.to_string();
}

/// Enable the use of the Skia renderer. Default renderer is AGG (Aggregated Graphics).
pub fn set_use_skia(use_skia: bool) {
    SKIA_RENDERER.store(use_skia, std::sync::atomic::Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdfium_library_loading() {
        let lib = try_lib();
        assert!(lib.is_ok());
    }
}
