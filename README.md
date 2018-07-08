# Repro for missing span information of inner attributes

This is a simple repro test case for inner attributes for Rust procedural attribute macros.

# Repro steps

1. $ `cargo clean && cargo build`

# Expected results

The output from the printing of the input `TokenStream` should contain valid spans for the `magic` attribute that the `repro` macro will strip from the output `TokenStream`.

# Actual results

The span information is all `#0 bytes(0..0)`:

```
TokenStream [Punct { ch: '#', spacing: Alone, span: #0 bytes(0..0) }, Group { delimiter: Bracket, stream: TokenStream [Ident { sym: magic, span: #0 bytes(0..0), is_raw: false }, Group { delimiter: Parenthesis, stream: TokenStream [Ident { sym: foo, span: #0bytes(0..0), is_raw: false }, Punct { ch: '=', spacing: Alone, span: #0 bytes(0..0) }, Literal { lit: Str_(bar), suffix: None, span: #0 bytes(0..0) }], span: #0 bytes(0..0) }], span: #0 bytes(0..0) }, Ident { sym: pub, span: #0 bytes(94..97), is_raw: false }, Ident { sym: fn, span: #0 bytes(98..100), is_raw: false }, Ident { sym: test, span: #0 bytes(101..105), is_raw: false }, Group{ delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(105..107) }, Group { delimiter: Brace, stream: TokenStream [], span: #0 bytes(108..110) }]
```

