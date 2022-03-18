# Notes on Rust topics

## Lifetime annotations
- [strsplit](https://gist.github.com/jonhoo/2a7fdcf79be03e51a5f95cd326f2a1e8) example by [@jonhoo](https://github.com/jonhoo) / Crust of Rust [episode on YouTube](https://youtu.be/rAl-9HwD858)

## Macros

- https://github.com/dtolnay/cargo-expand is a handy tool to view the expanded form of macros.
- Use general metaprogramming to keep code DRY.

### Declarative Macros with `macro_rules!` (for General Metaprogramming)

### Procedural Macros

Refer to [the docs](https://doc.rust-lang.org/reference/procedural-macros.html).

Learn to use the following in the [proc-macro-workshop](https://github.com/dtolnay/proc-macro-workshop) by [@dtolnay](https://github.com/dtolnay)
- [syn](https://crates.io/crates/syn)
- [quote](https://crates.io/crates/quote)
- Crust of Rust [episode on YouTube](https://youtu.be/geovSK3wMB8)