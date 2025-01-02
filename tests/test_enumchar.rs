// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use enumchar::EnumChar;

#[derive(EnumChar, Default, Debug, PartialEq, Eq)]
pub enum ECAllVariants {
    #[default]
    #[char('#')]
    Wall,
    #[char('.')]
    Empty,
    Other,
}

#[test]
pub fn test_tryfrom_char() -> Result<(), String> {
    assert_eq!(ECAllVariants::try_from('.')?, ECAllVariants::Empty);
    assert_eq!(ECAllVariants::try_from('#')?, ECAllVariants::Wall);
    assert!(matches!(ECAllVariants::try_from('z'), Err(_)));
    Ok(())
}
