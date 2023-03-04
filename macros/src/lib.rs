//! This crate provides procedural macros for [quit].
//!
//! **Do not add this crate as a dependency.** It has no backward compatibility
//! guarantees. Use the macros re-exported from [quit] instead.
//!
//! [quit]: https://crates.io/crates/quit

#![forbid(unsafe_code)]
#![warn(unused_results)]

use std::iter;
use std::result;

use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::Literal;
use proc_macro::Punct;
use proc_macro::Spacing;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

trait TokenStreamExt {
    fn push<T>(&mut self, token: T)
    where
        T: Into<TokenTree>;
}

impl TokenStreamExt for TokenStream {
    fn push<T>(&mut self, token: T)
    where
        T: Into<TokenTree>,
    {
        self.extend(iter::once(token.into()));
    }
}

fn macro_path(module: &str, name: &str) -> impl Iterator<Item = TokenTree> {
    [
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new(module, Span::call_site()).into(),
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new(name, Span::call_site()).into(),
        Punct::new('!', Spacing::Alone).into(),
    ]
    .into_iter()
}

// https://docs.rs/syn/1.0/syn/struct.Error.html
#[derive(Copy, Clone)]
struct Error {
    start: Span,
    end: Span,
    message: &'static str,
}

impl Error {
    const fn new(span: Span, message: &'static str) -> Self {
        Self {
            start: span,
            end: span,
            message,
        }
    }

    fn new_spanned<T>(tokens: T, message: &'static str) -> Self
    where
        T: Into<TokenStream>,
    {
        let mut tokens = tokens.into().into_iter();
        let start = tokens
            .next()
            .map(|x| x.span())
            .unwrap_or_else(Span::call_site);
        Self {
            start,
            end: tokens.last().map(|x| x.span()).unwrap_or(start),
            message,
        }
    }

    fn to_compile_error(self) -> TokenStream {
        let mut result: TokenStream = macro_path("std", "compile_error")
            .map(|mut token| {
                token.set_span(self.start);
                token
            })
            .collect();

        let mut literal = Literal::string(self.message);
        literal.set_span(self.end);

        let mut group =
            Group::new(Delimiter::Brace, TokenTree::Literal(literal).into());
        group.set_span(self.end);

        result.push(group);
        result
    }
}

// https://docs.rs/syn/1.0/syn/type.Result.html
type Result<T> = result::Result<T, Error>;

fn take_signature<I>(mut tokens: &mut I) -> Result<TokenStream>
where
    I: Iterator<Item = TokenTree>,
{
    let mut signature = TokenStream::new();

    for token in &mut tokens {
        let matched =
            matches!(&token, TokenTree::Ident(x) if x.to_string() == "fn");
        signature.push(token);
        if matched {
            break;
        }
    }

    let name = tokens.next().ok_or_else(|| {
        Error::new(
            Span::call_site(),
            "`#[quit::main]` can only be attached to functions",
        )
    })?;
    if !matches!(&name, TokenTree::Ident(x) if x.to_string() == "main") {
        return Err(Error::new_spanned(
            name,
            "`#[quit::main]` can only be attached to `main`",
        ));
    }
    signature.push(name);

    Ok(signature)
}

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return Error::new_spanned(args, "arguments are not accepted")
            .to_compile_error();
    }

    let mut item = item.into_iter();
    take_signature(&mut item)
        .map(|signature| {
            macro_path("quit", "__main")
                .chain(iter::once(
                    Group::new(
                        Delimiter::Brace,
                        iter::once(
                            Group::new(Delimiter::Parenthesis, signature)
                                .into(),
                        )
                        .chain(item)
                        .collect(),
                    )
                    .into(),
                ))
                .collect()
        })
        .unwrap_or_else(Error::to_compile_error)
}
