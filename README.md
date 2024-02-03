# Debug span

This crate provides a simple way to debug proc-macro2 spans. It is useful when you are working with procedural macros
and you want to see the location of a span in the source code.

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
