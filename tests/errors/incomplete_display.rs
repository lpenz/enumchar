// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use enumchar::EnumChar;

#[derive(EnumChar)]
pub enum E {
    #[char('.')]
    Dot,
    Other,
}

fn main() {
    println!("{}", E::Dot);
}
