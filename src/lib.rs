//! This crate allows cleanly exiting a program using a custom exit code,
//! without the drawbacks of [`process::exit`]. Destructors will be called as
//! usual, and the stack will be unwound to the main function.
//!
//! It is always required to attach [`#[main]`][attribute] to the main
//! function. Then, [`with_code`] can be called from almost anywhere in the
//! program. Restrictions are noted in the documentation for that function.
//!
//! # Implementation
//!
//! Internally, this crate uses panicking to unwind the stack. Thus, if
//! panicking is set to "abort" instead of the default "unwind", setting the
//! exit status will not work correctly. Changing this option may become an
//! error in the future, if it can be detected at compile time.
//!
//! Additionally, the program will not exit if [`with_code`] is called from a
//! spawned thread, unless panics are propagated from that thread.
//!
//! # Examples
//!
//! ```
//! use std::env;
//!
//! fn read_args() {
//!     if env::args_os().nth(1).is_some() {
//!         eprintln!("too many arguments");
//!         quit::with_code(1);
//!     }
//! }
//!
//! #[quit::main]
//! fn main() {
//!     read_args();
//! }
//! ```
//!
//! [attribute]: main

#![forbid(unsafe_code)]
#![warn(unused_results)]

use std::panic;
use std::panic::UnwindSafe;
use std::process;

/// Modifies the main function to exit with the code passed to [`with_code`].
///
/// This attribute should always be attached to the main function. Otherwise,
/// the exit code of the program may be incorrect.
///
/// # Examples
///
/// ```
/// #[quit::main]
/// fn main() {}
/// ```
pub use quit_macros::main;

#[must_use]
struct ExitCode(i32);

#[doc(hidden)]
#[inline]
pub fn __run<F, R>(main_fn: F) -> R
where
    F: FnOnce() -> R + UnwindSafe,
{
    panic::catch_unwind(main_fn).unwrap_or_else(|payload| {
        if let Some(&ExitCode(exit_code)) = payload.downcast_ref() {
            process::exit(exit_code);
        }
        panic::resume_unwind(payload);
    })
}

/// Cleanly exits the program with an exit code.
///
/// Calling this function from within an FFI boundary invokes undefined
/// behavior. Because panics are used internally to unwind the stack, the exit
/// code cannot be passed safely. [`process::exit`] should be used instead in
/// that case.
///
/// This function will not behave as expected unless [`#[main]`][attribute] is
/// attached to the main function. Other implementation notes are mentioned in
/// [the module-level documentation][implementation].
///
/// # Examples
///
/// ```should_panic
/// fn exit() -> ! {
///     quit::with_code(1);
/// }
/// # exit();
/// ```
///
/// [attribute]: main
/// [implementation]: self#implementation
#[inline]
pub fn with_code(exit_code: i32) -> ! {
    panic::resume_unwind(Box::new(ExitCode(exit_code)));
}
