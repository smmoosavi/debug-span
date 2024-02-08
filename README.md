# Debug span

[<img alt="github" src="https://img.shields.io/badge/github-smmoosavi/debug--span-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/smmoosavi/debug-span)
[<img alt="crates.io" src="https://img.shields.io/crates/v/debug-span.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/debug-span)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-debug--span-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/debug-span)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/smmoosavi/debug-span/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/smmoosavi/debug-span/actions?query=branch%3Amain)
[<img alt="coverage" src="https://img.shields.io/codecov/c/github/smmoosavi/debug-span?style=for-the-badge&logo=codecov" height="20">](https://app.codecov.io/gh/smmoosavi/debug-span)

This crate provides a simple way to debug proc-macro2 spans. It is useful when you are working with procedural macros and you want to see the location of a span in the source code. It can be used for testing or debugging.

```toml
[dev-dependencies]
debug-span = "0.1"

# used to parse code and assert output, not required for the crate itself
syn = "2"
insta = "1"
unindent = "0.2"

```

## Usage

```rust
use debug_span::debug_span;
use syn::spanned::Spanned;
use syn::Data;
use unindent::Unindent;

#[test]
fn test_single_line_span() {
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
fn test_multi_line_span() {
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
```
