//! This crate provides procedural macros for [Quit].
//!
//! **Do not add this crate as a dependency.** It has no backward compatibility
//! guarantees. Use the macros re-exported from [Quit] instead.
//!
//! [Quit]: https://crates.io/crates/quit

#![doc(html_root_url = "https://docs.rs/quit_macros/*")]
#![forbid(unsafe_code)]
#![warn(unused_results)]

#[allow(unused_extern_crates)]
extern crate proc_macro;

use std::fmt::Display;

use proc_macro::TokenStream;

use quote::quote;
use quote::ToTokens;

use syn::parse_macro_input;
use syn::Error as SynError;
use syn::ItemFn;
use syn::ReturnType;

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
///
/// [`with_code`]: fn.with_code.html
#[inline]
#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    if let Some(arg) = args.into_iter().next() {
        return SynError::new(arg.span().into(), "arguments are not accepted")
            .to_compile_error()
            .into();
    }

    let input = parse_macro_input!(item as ItemFn);
    let signature = &input.sig;

    let name = &signature.ident;
    if name != "main" {
        return error(name, "`quit::main` can only be attached to `main`");
    }

    let return_type = &signature.output;
    if let ReturnType::Type(_, return_type) = return_type {
        return error(
            return_type,
            "`quit::main` function cannot have a return type",
        );
    }

    let attrs = &input.attrs;
    let visibility = &input.vis;
    let body = &input.block;
    return quote! {
        #(#attrs)*
        #visibility #signature {
            let exit_code = ::std::panic::catch_unwind(|| { #body })
                .map(|()| 0)
                .unwrap_or_else(|payload| {
                    match payload.downcast_ref() {
                        Some(&::quit::_ExitCode(exit_code)) => exit_code,
                        None => ::std::panic::resume_unwind(payload),
                    }
                });
            if exit_code != 0 {
                ::std::process::exit(exit_code);
            }
        }
    }
    .into();

    fn error<TMessage, TTokens>(
        tokens: TTokens,
        message: TMessage,
    ) -> TokenStream
    where
        TMessage: Display,
        TTokens: ToTokens,
    {
        SynError::new_spanned(tokens, message)
            .to_compile_error()
            .into()
    }
}
