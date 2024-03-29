//! This crate provides a simple way to debug proc-macro2 spans. It is useful when you are working
//! with procedural macros and you want to see the location of a span in the source code. It can be
//! used for testing or debugging.
//!
//! # Example
//!
//! ```rust
//! use debug_span::debug_span;
//! use syn::spanned::Spanned;
//! use syn::Data;
//! use unindent::Unindent;
//!
//! let input = r###"
//!     struct Foo {
//!         a: i32,
//!         b: i32,
//!     }
//! "###
//! .unindent();
//! let derive_input: syn::DeriveInput = syn::parse_str(&input).unwrap();
//! let span = match derive_input.data {
//!     Data::Struct(s) => s.fields.span(),
//!     _ => panic!("expected struct"),
//! };
//!     
//! let output = debug_span(span, &input);
//! insta::assert_snapshot!(output, @r###"
//!  --> 1:11..4:1
//!   |
//!   |            ┌────╮
//! 1 | struct Foo {    │
//! 2 |     a: i32,     │
//! 3 |     b: i32,     │
//! 4 | }               │
//!   | └───────────────╯
//!   |
//! "###);
//! ```
//!

/// A trait for types that represent a span in the source code.
///
/// This trait is implemented for `proc_macro2::Span`
pub trait Span {
    fn start_line(&self) -> usize;
    fn end_line(&self) -> usize;
    fn start_column(&self) -> usize;
    fn end_column(&self) -> usize;

    #[doc(hidden)]
    fn is_empty(&self) -> bool {
        self.start_line() == self.end_line() && self.start_column() == self.end_column()
    }
    #[doc(hidden)]
    fn is_single_line(&self) -> bool {
        self.start_line() == self.end_line()
    }

    /// Returns a string representation of the span in the format `start_line:start_column..end_line:end_column`
    ///
    /// # Example
    ///
    /// ```text
    /// 1:7..1:10
    /// ```
    fn to_range(&self) -> String {
        format!(
            "{}:{}..{}:{}",
            self.start_line(),
            self.start_column(),
            self.end_line(),
            self.end_column(),
        )
    }

    /// Generate a debug representation of the span and the source code it points to.
    ///
    /// see [`debug_span`] for more information.
    fn debug(&self, code: &str) -> String {
        internal::debug_span(self, code)
    }
}

#[cfg(feature = "proc-macro2")]
mod proc_macro2_span {
    impl crate::Span for proc_macro2::Span {
        fn start_line(&self) -> usize {
            self.start().line
        }
        fn end_line(&self) -> usize {
            self.end().line
        }
        fn start_column(&self) -> usize {
            self.start().column
        }
        fn end_column(&self) -> usize {
            self.end().column
        }
    }
}

/// Generate a debug representation of a span and the source code it points to.
///
/// It accepts any type that implements the [`Span`] trait. `Span` is implemented for [`proc_macro2::Span`].
///
/// ## Single line span example
///
/// ```text
///  --> 1:7..1:10
///   |
/// 1 | struct Foo;
///   |        ^^^
/// ````
/// ## Multi line span example
///
/// ```text
/// --> 1:11..4:1
///   |
///   |            ┌────╮
/// 1 | struct Foo {    │
/// 2 |     a: i32,     │
/// 3 |     b: i32,     │
/// 4 | }               │
///   | └───────────────╯
/// ```
///
pub fn debug_span(span: impl Span, code: &str) -> String {
    internal::debug_span(&span, code)
}

#[doc(hidden)]
pub mod internal {
    use crate::Span;

    pub fn debug_span(span: &(impl Span + ?Sized), code: &str) -> String {
        if span.is_empty() {
            debug_empty_span(span, code)
        } else if span.is_single_line() {
            debug_single_line_span(span, code)
        } else {
            debug_multi_line_span(span, code)
        }
    }

    pub fn debug_empty_span(_span: &(impl Span + ?Sized), _code: &str) -> String {
        "".to_string()
    }

    pub fn debug_single_line_span(span: &(impl Span + ?Sized), code: &str) -> String {
        let empty_line = empty_line(span);
        let range_line = range_line(span);
        let code_line = code_line(span, code);
        let marker_line = marker_line(span);
        format!(
            "{}\n{}\n{}\n{}\n{}\n",
            range_line, empty_line, code_line, marker_line, empty_line,
        )
    }

    pub fn debug_multi_line_span(span: &(impl Span + ?Sized), code: &str) -> String {
        let empty_line = empty_line(span);
        let range_line = range_line(span);
        let start_line = start_line(span, code);
        let code_lines = code_lines(span, code);
        let end_line = end_line(span, code);
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n",
            range_line, empty_line, start_line, code_lines, end_line, empty_line,
        )
    }

    pub fn range_line(span: &(impl Span + ?Sized)) -> String {
        let line_number_width = span.end_line().to_string().len();
        let range = span.to_range();
        format!("{:width$}--> {}", "", range, width = line_number_width,)
    }

    pub fn empty_line(span: &(impl Span + ?Sized)) -> String {
        let line_number_width = span.end_line().to_string().len();
        format!("{:width$} |", "", width = line_number_width)
    }

    pub fn marker_line(span: &(impl Span + ?Sized)) -> String {
        let line_number_width = span.end_line().to_string().len();
        let start_column = span.start_column();
        let end_column = span.end_column();

        let marker = "^".repeat(end_column - start_column);
        format!(
            "{:width$} | {:space$}{}",
            "",
            "",
            marker,
            space = start_column,
            width = line_number_width,
        )
    }

    pub fn code_line(span: &(impl Span + ?Sized), code: &str) -> String {
        let line_number_width = span.end_line().to_string().len();
        let line = code.lines().nth(span.start_line() - 1).unwrap();
        format!(
            "{:width$} | {}",
            span.start_line(),
            line,
            width = line_number_width,
        )
    }

    const PADDING: usize = 3;

    pub fn start_line(span: &(impl Span + ?Sized), code: &str) -> String {
        let line_number_width = span.end_line().to_string().len();
        let start_line = span.start_line();
        let end_line = span.end_line();
        let start_column = span.start_column();

        let lines = code
            .lines()
            .skip(start_line - 1)
            .take(end_line - start_line + 1);
        let max_line_len = lines.map(|line| line.len()).max().unwrap();
        format!(
            "{:width$} | {}┌{}╮",
            "",
            " ".repeat(start_column),
            "─".repeat(max_line_len + PADDING - start_column),
            width = line_number_width,
        )
    }

    pub fn code_lines(span: &(impl Span + ?Sized), code: &str) -> String {
        let line_number_width = span.end_line().to_string().len();
        let start_line = span.start_line();
        let end_line = span.end_line();
        let lines = code
            .lines()
            .skip(start_line - 1)
            .take(end_line - start_line + 1);
        let max_line_len = lines.clone().map(|line| line.len()).max().unwrap();
        lines
            .into_iter()
            .enumerate()
            .map(|(i, line)| {
                let line_number = start_line + i;
                format!(
                    "{: >line_number_width$} | {}{}│",
                    line_number,
                    line,
                    " ".repeat(max_line_len + PADDING + 1 - line.len()),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn end_line(span: &(impl Span + ?Sized), code: &str) -> String {
        let line_number_width = span.end_line().to_string().len();
        let start_line = span.start_line();
        let end_line = span.end_line();
        let end_column = span.end_column();

        let lines = code
            .lines()
            .skip(start_line - 1)
            .take(end_line - start_line + 1);
        let max_line_len = lines.map(|line| line.len()).max().unwrap();
        format!(
            "{:width$} | {}└{}╯",
            "",
            " ".repeat(end_column - 1),
            "─".repeat(max_line_len + PADDING - end_column + 1),
            width = line_number_width,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::spanned::Spanned;
    use syn::Data;
    use unindent::Unindent;

    #[test]
    fn test_empty_span() {
        let input = r###"
            struct Foo;
        "###
        .unindent();
        let span = proc_macro2::Span::call_site();
        let output = debug_span(span, &input);
        insta::assert_snapshot!(output, @"");
    }

    #[test]
    fn test_single_line() {
        let input = r###"
            struct Foo;
        "###
        .unindent();
        let derive_input: syn::DeriveInput = syn::parse_str(&input).unwrap();
        let span = derive_input.ident.span();
        let output = debug_span(span, &input);
        insta::assert_snapshot!(output, @r###"
         --> 1:7..1:10
          |
        1 | struct Foo;
          |        ^^^
          |
        "###);
    }
    #[test]
    fn test_single_line_large_line_number() {
        let input = r###"
            struct Foo;
        "###
        .unindent();
        let input = "\n".repeat(120) + &input;
        let derive_input: syn::DeriveInput = syn::parse_str(&input).unwrap();
        let span = derive_input.ident.span();
        let output = debug_span(span, &input);
        insta::assert_snapshot!(output, @r###"
           --> 121:7..121:10
            |
        121 | struct Foo;
            |        ^^^
            |
        "###);
    }

    #[test]
    fn test_multi_line() {
        let input = r###"
            struct Foo {
                a: i32,
                b: i32,
            }
        "###
        .unindent();
        let derive_input: syn::DeriveInput = syn::parse_str(&input).unwrap();
        let span = match derive_input.data {
            Data::Struct(s) => s.fields.span(),
            _ => panic!("expected struct"),
        };

        let output = debug_span(span, &input);
        insta::assert_snapshot!(output, @r###"
         --> 1:11..4:1
          |
          |            ┌────╮
        1 | struct Foo {    │
        2 |     a: i32,     │
        3 |     b: i32,     │
        4 | }               │
          | └───────────────╯
          |
        "###);
    }

    #[test]
    fn test_multi_line_large_line_number() {
        let input = r###"
            struct Foo {
                a: i32,
                b: i32,
            }
        "###
        .unindent();
        let input = "\n".repeat(120) + &input;
        let derive_input: syn::DeriveInput = syn::parse_str(&input).unwrap();
        let span = match derive_input.data {
            Data::Struct(s) => s.fields.span(),
            _ => panic!("expected struct"),
        };

        let output = debug_span(span, &input);
        insta::assert_snapshot!(output, @r###"
           --> 121:11..124:1
            |
            |            ┌────╮
        121 | struct Foo {    │
        122 |     a: i32,     │
        123 |     b: i32,     │
        124 | }               │
            | └───────────────╯
            |
        "###);
    }

    #[test]
    fn test_multi_line_large_line() {
        let input = r###"
            struct Foo {
                a: std::collections::HashMap<i32, i32>,
                b: i32,
            }
        "###
        .unindent();
        let derive_input: syn::DeriveInput = syn::parse_str(&input).unwrap();
        let span = match derive_input.data {
            Data::Struct(s) => s.fields.span(),
            _ => panic!("expected struct"),
        };

        let output = debug_span(span, &input);
        insta::assert_snapshot!(output, @r###"
         --> 1:11..4:1
          |
          |            ┌───────────────────────────────────╮
        1 | struct Foo {                                   │
        2 |     a: std::collections::HashMap<i32, i32>,    │
        3 |     b: i32,                                    │
        4 | }                                              │
          | └──────────────────────────────────────────────╯
          |
        "###);
    }

    #[test]
    fn test_syn_error() {
        let input = r###"
            struct Foo {
                a: i32
                bar: i32,
            }
        "###
        .unindent();
        let derive_input: Result<syn::DeriveInput, _> = syn::parse_str(&input);
        let error = match derive_input {
            Ok(_) => panic!("expected error"),
            Err(e) => e,
        };
        let span = error.span();
        let output = debug_span(span, &input);
        insta::assert_snapshot!(error.to_string(), @"expected `,`");
        insta::assert_snapshot!(output, @r###"
         --> 3:4..3:7
          |
        3 |     bar: i32,
          |     ^^^
          |
        "###);
    }

    #[test]
    fn test_debug_method() {
        let input = r###"
            struct Foo;
        "###
        .unindent();
        let derive_input: syn::DeriveInput = syn::parse_str(&input).unwrap();
        let span = derive_input.ident.span();
        let output = span.debug(&input);
        insta::assert_snapshot!(output, @r###"
         --> 1:7..1:10
          |
        1 | struct Foo;
          |        ^^^
          |
        "###);
    }
}
