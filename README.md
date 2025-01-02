[![CI](https://github.com/lpenz/enumchar/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/enumchar/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/enumchar/badge.svg?branch=main)](https://coveralls.io/github/lpenz/enumchar?branch=main)
[![dependency status](https://deps.rs/repo/github/lpenz/enumchar/status.svg)](https://deps.rs/repo/github/lpenz/enumchar)
[![crates.io](https://img.shields.io/crates/v/enumchar)](https://crates.io/crates/enumchar)


# enumchar

**enumchar** is a rust derive macro for enums where each variant is
represented by a single char. The macro automatically derives
`TryFrom<char>` for the enum, allowing the user to do the following:

```rust
#[derive(EnumChar)]
pub enum Cell {
    #[char('#')]
    Wall,
    #[char('.')]
    Empty,
}

let cell = Cell::try_from('.')?;
```

I've been using this macro to parse the 2D mazes in [adventofcode].


[adventofcode]: https://adventofcode.com/
