//! This crate provides procedural macros for [quit].
//!
//! **Do not add this crate as a dependency.** It has no backward compatibility
//! guarantees. Use the macros re-exported from [quit] instead.
//!
//! [quit]: https://crates.io/crates/quit

#![forbid(unsafe_code)]
#![warn(unused_results)]

use std::iter;
use std::iter::FromIterator;
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

fn path(module: &str, name: &str) -> impl Iterator<Item = TokenTree> {
    vec![
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new(module, Span::call_site()).into(),
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new(name, Span::call_site()).into(),
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
        let mut result: TokenStream = path("std", "compile_error")
            .chain(iter::once(Punct::new('!', Spacing::Alone).into()))
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

fn parse_main_fn(tokens: TokenStream) -> Result<(TokenStream, TokenTree)> {
    let mut tokens = tokens.into_iter();
    let mut signature = TokenStream::new();

    loop {
        let token = tokens.next().ok_or_else(|| {
            Error::new(
                Span::call_site(),
                "`#[quit::main]` can only be attached to functions",
            )
        })?;

        let found =
            matches!(&token, TokenTree::Ident(x) if x.to_string() == "fn");
        signature.push(token);
        if found {
            break;
        }
    }

    if let Some(name) = tokens.next() {
        if name.to_string() != "main" {
            return Err(Error::new_spanned(
                name,
                "`#[quit::main]` can only be attached to `main`",
            ));
        }
        signature.push(name);
    }

    let body = loop {
        let token = tokens.next().ok_or_else(|| {
            Error::new(
                Span::call_site(),
                "`#[quit::main]` can only be attached to functions with a \
                body",
            )
        })?;

        if matches!(
            &token,
            TokenTree::Group(x) if x.delimiter() == Delimiter::Brace,
        ) {
            break token;
        }
        signature.push(token);
    };

    assert!(tokens.next().is_none());

    Ok((signature, body))
}

#[inline]
#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return Error::new_spanned(args, "arguments are not accepted")
            .to_compile_error();
    }

    let (mut result, body) = match parse_main_fn(item) {
        Ok(result) => result,
        Err(error) => return error.to_compile_error(),
    };

    let mut args = TokenStream::from_iter(vec![
        TokenTree::Punct(Punct::new(
            '|',
            Spacing::Alone,
        ));
        2
    ]);
    args.push(body);

    let mut body: TokenStream = path("quit", "__run").collect();
    body.push(Group::new(Delimiter::Parenthesis, args));

    result.push(Group::new(Delimiter::Brace, body));
    result
}
