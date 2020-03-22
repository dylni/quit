//! This crate allows cleanly exiting a program using a custom exit code,
//! without the drawbacks of [`exit`]. Destructors will be called as usual, and
//! the stack will be unwound to the main function.
//!
//! It is always required to attach [`main`] to the main function. Then,
//! [`with_code`] can be called from almost anywhere in the program.
//! Restrictions are noted in the documentation for that function.
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
//! [`exit`]: https://doc.rust-lang.org/std/process/fn.exit.html
//! [`main`]: attr.main.html
//! [`with_code`]: fn.with_code.html

#![doc(html_root_url = "https://docs.rs/quit/*")]
#![forbid(unsafe_code)]
#![warn(unused_results)]

use std::panic;

// https://github.com/rust-lang/rust/issues/62127
#[cfg(not(test))]
pub use quit_macros::main;

#[derive(Copy, Clone, Debug)]
#[doc(hidden)]
#[must_use]
pub struct _ExitCode(pub i32);

/// Cleanly exits the program with an exit code.
///
/// Calling this function from within an FFI boundary invokes undefined
/// behavior. Because panics are used internally to unwind the stack, the exit
/// code cannot be passed safely. [`exit`] should be used instead in that case.
///
/// This function will not behave as expected unless [`main`] is attached to
/// the main function. Other implementation notes are mentioned in [the
/// module-level documentation][implementation].
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
/// [`exit`]: https://doc.rust-lang.org/std/process/fn.exit.html
/// [`main`]: attr.main.html
/// [implementation]: index.html#implementation
#[inline]
pub fn with_code(exit_code: i32) -> ! {
    panic::resume_unwind(Box::new(_ExitCode(exit_code)));
}
