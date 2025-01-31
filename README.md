[![CI](https://github.com/lpenz/enumchar/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/enumchar/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/enumchar/badge.svg?branch=main)](https://coveralls.io/github/lpenz/enumchar?branch=main)
[![dependency status](https://deps.rs/repo/github/lpenz/enumchar/status.svg)](https://deps.rs/repo/github/lpenz/enumchar)
[![crates.io](https://img.shields.io/crates/v/enumchar)](https://crates.io/crates/enumchar)


# enumchar

*enumchar* is a simple rust derive proc_macro for `enums` where each
variant corresponds to a `char`.

Example usage:

```rust
use enumchar::EnumChar;

#[derive(EnumChar)]
pub enum Cell {
    #[char('#')]
    Wall,
    #[char('.')]
    Empty,
}
```

The effect of the code above is the automatic `impl` of `TryFrom<char>`,
`TryInto<char>` and `std::fmt::Display`. It also implements
`Into<char>` if all variants have a corresponding `char`, as we
can't return an error from that one.

I've been using this macro to parse all those 2D mazes in
[adventofcode](https://adventofcode.com/) - feel free to use it too.
