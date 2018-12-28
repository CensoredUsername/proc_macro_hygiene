# Error description:

when compiling normal_macro, the following error will be raised:

```rust
$ cargo build
   Compiling normal_macro v0.1.0 (C:\Users\CensoredUsername\Projects\macro_bug\normal_macro)
error[E0425]: cannot find value `a` in this scope
  --> src\main.rs:5:43
   |
5  |     ($test:expr) => {proc_macro::forward!($test)}
   |                                           ^^^^^ not found in this scope
...
12 |     println!("{}", forward!(a));
   |                    ----------- in this macro invocation

error: aborting due to previous error
```
