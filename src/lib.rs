#![no_std]
#![warn(missing_docs)]
#![allow(nonstandard_style)]

//! Crate with a struct that can load GL function pointers, and that lets you
//! call them.
//!
//! ## Typical Initialization
//!
//! ```rust,no_run
//! # use gl_struct_loader::*;
//! let gl = {
//!   let user_loader = |name| unimplemented!("call some OS fn here");
//!   let mut gl = GlFns::new_boxed();
//!   unsafe { gl.load(user_loader) };
//!   gl
//! };
//! ```
//!
//! The [new_boxed](GlFns::new_boxed) function makes the struct directly on the
//! heap (it's kinda large to keep it on the stack), and requires the crate's
//! `alloc` feature (which is on by default). If you somehow can use GL but
//! can't use the `alloc` crate you can use the [`BLANK_GL_FNS`] constant to
//! make an empty value of the struct.
//!
//! ## Global GL
//!
//! If you'd like to call the GL API from anywhere (like in a C program) it's
//! still possible using this crate, though I'll admit it's a hair clunky.
//!
//! When the crate's `std` feature (on by default) is enabled there is a static
//! [RwLock](std::sync::RwLock) available called [`GL`]:
//!
//! ```rust,no_run
//! # use gl_struct_loader::*;
//! unsafe {
//!   // Grab a write lock to load the functions
//!   GL.write().unwrap().load(|name| core::ptr::null());
//!
//!   // Then a read lock lets you call the functions
//!   GL.read().unwrap().ClearColor(1.0, 0.0, 0.5, 1.0);
//! }
//! ```
//!
//! However you'd wrap up and hide the unsafety of the GL API can also just hide
//! the lock grabbing and all that, so it's not too bad in practice.
//!
//! ## What Was Loaded
//!
//! If you attempt to call a function that is not loaded, it will cause a panic.
//!
//! If you want to check that a function is loaded you can call the
//! [`has_loaded`](GlFns::has_loaded) method (giving a [`FnLoadedChecker`]) and
//! then the method that you want to check the status of:
//!
//! ```rust
//! # use gl_struct_loader::*;
//! let gl = BLANK_GL_FNS;
//! assert_eq!(gl.has_loaded().ClearColor(), false);
//! ```
//!
//! That said, on some platforms it's *possible* for a function to load even
//! though the current context's API level and allowed extensions don't actually
//! let you legally call that function. As with most any improper uses of a C
//! API, actually doing this is Undefined Behavior.
//!
//! **I want to be extra clear:** You should **only** call functions that are
//! allowed by the current context's API level and extension list.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use gl_types::*;

mod struct_decl;
pub use struct_decl::*;

mod creation;
pub use creation::*;

mod calling;

mod fn_ty_aliases;
use fn_ty_aliases::*;

mod loaded_checker;
pub use loaded_checker::*;
