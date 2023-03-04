//! This crate allows cleanly exiting a program using a custom exit code,
//! without the drawbacks of [`process::exit`]. Destructors will be called as
//! usual, and the stack will be completely unwound.
//!
//! It is always required to attach [`#[main]`][attribute] to the main
//! function. Then, [`with_code`] can be called from almost anywhere in the
//! program. Restrictions are noted in the documentation for that function.
//!
//! # Implementation
//!
//! Internally, this crate uses panicking to unwind the stack. Thus, if
//! panicking were set to "abort" instead of the default "unwind", setting the
//! exit status would not work correctly. This crate will cause a compile error
//! in that case, to avoid silent incorrect behavior. Further information can
//! be found in the [Rustc Development Guide][panic_runtime].
//!
//! Additionally, the program will not exit if [`with_code`] is called from a
//! spawned thread, unless panics are propagated from that thread. However,
//! propagating panics is usually recommended regardless.
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
//! [panic_runtime]: https://rustc-dev-guide.rust-lang.org/panic-implementation.html#step-2-the-panic-runtime

#![forbid(unsafe_code)]
#![warn(unused_results)]

#[cfg(not(panic = "unwind"))]
compile_error!(
    r#"Quit requires unwinding panics:
https://docs.rs/quit/latest/quit/#implementation"#
);

use std::panic;
use std::panic::UnwindSafe;
use std::process;
use std::process::Termination;

#[cfg(all(feature = "__unstable_tests", not(quit_docs_rs)))]
#[doc(hidden)]
pub mod tests_common;

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

struct ExitCode(process::ExitCode);

enum ResultInner<T>
where
    T: Termination,
{
    Result(T),
    ExitCode(process::ExitCode),
}

#[doc(hidden)]
pub struct __Result<T>(ResultInner<T>)
where
    T: Termination;

impl<T> Termination for __Result<T>
where
    T: Termination,
{
    #[inline]
    fn report(self) -> process::ExitCode {
        use ResultInner as Inner;

        match self.0 {
            Inner::Result(result) => result.report(),
            Inner::ExitCode(exit_code) => exit_code,
        }
    }
}

#[doc(hidden)]
#[inline]
pub fn __run<F, R>(main_fn: F) -> __Result<R>
where
    F: FnOnce() -> R + UnwindSafe,
    R: Termination,
{
    panic::catch_unwind(main_fn)
        .map(|x| __Result(ResultInner::Result(x)))
        .unwrap_or_else(|payload| {
            if let Some(&ExitCode(exit_code)) = payload.downcast_ref() {
                __Result(ResultInner::ExitCode(exit_code))
            } else {
                panic::resume_unwind(payload);
            }
        })
}

#[doc(hidden)]
#[macro_export]
macro_rules! __main {
    (
        ( $($signature_token:tt)+ )
        ( $($args_token:tt)* ) -> $return_type:ty { $($body_token:tt)* }
    ) => {
        $($signature_token)+ (
            $($args_token)*
        ) -> $crate::__Result<$return_type> {
            $crate::__run(|| { $($body_token)* })
        }
    };
    ( $signature:tt $args:tt $body:tt ) => {
        $crate::__main!($signature $args -> () $body);
    }
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
pub fn with_code<T>(exit_code: T) -> !
where
    T: Into<process::ExitCode>,
{
    panic::resume_unwind(Box::new(ExitCode(exit_code.into())));
}
