# Error description:

when compiling normal_macro, the following output and error will be raised:

```
$ cargo build
   Compiling normal_macro v0.1.0 (C:\Users\CensoredUsername\Projects\macro_bug\normal_macro)
input: TokenStream [Ident { ident: "a", span: #0 bytes(563..564) }]
output: TokenStream [Ident { ident: "a", span: #0 bytes(563..564) }]

input: TokenStream [Group { delimiter: None, stream: TokenStream [Ident { ident: "a", span: #27 bytes(341..346) }], span: #27 bytes(341..346) }]
output: TokenStream [Ident { ident: "a", span: #27 bytes(341..346) }]

input: TokenStream [Group { delimiter: None, stream: TokenStream [Ident { ident: "a", span: #0 bytes(741..742) }],span: #33 bytes(429..434) }]
output: TokenStream [Ident { ident: "a", span: #0 bytes(741..742) }]

error[E0425]: cannot find value `a` in this scope
  --> src\main.rs:17:43
   |
17 |     ($test:expr) => {proc_macro::forward!($test)}
   |                                           ^^^^^ not found in this scope
...
31 |     println!("{}", forward_proc_expr!(a));
   |                    --------------------- in this macro invocation

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
error: Could not compile `normal_macro`.
```
