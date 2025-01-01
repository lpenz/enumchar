// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use enumchar::EnumChar;

#[derive(EnumChar, Default)]
pub enum Cell {
    #[default]
    #[char('#')]
    Wall,
    #[char('.')]
    Empty,
}

#[test]
pub fn test_tryfrom_char() {
    let _c = Cell::try_from('.').expect("try_from to work");
}
