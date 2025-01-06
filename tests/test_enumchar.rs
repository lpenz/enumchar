// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use enumchar::EnumChar;

#[derive(EnumChar, Debug, PartialEq, Eq)]
pub enum ECAllVariants {
    #[char('#')]
    Wall,
    #[char('.')]
    Empty,
}

#[test]
pub fn test_allvariants_tryfrom_char() -> Result<(), String> {
    assert_eq!(ECAllVariants::try_from('.')?, ECAllVariants::Empty);
    assert_eq!(ECAllVariants::try_from('#')?, ECAllVariants::Wall);
    assert_eq!("#".parse::<ECAllVariants>()?, ECAllVariants::Wall);
    assert!(matches!(ECAllVariants::try_from('z'), Err(_)));
    assert!(matches!("z".parse::<ECAllVariants>(), Err(_)));
    assert_eq!(char::try_from(ECAllVariants::Empty), Ok('.'));
    assert_eq!(char::try_from(ECAllVariants::Wall), Ok('#'));
    // As we have defined all enums, we also get char::from
    assert_eq!(char::from(ECAllVariants::Empty), '.');
    assert_eq!(char::from(ECAllVariants::Wall), '#');
    assert_eq!(format!("{}", ECAllVariants::Wall), "#");
    Ok(())
}

#[derive(EnumChar, Debug, PartialEq, Eq)]
pub enum ECSomeVariants {
    #[char('#')]
    Wall,
    #[char('.')]
    Empty,
    Other,
}

#[test]
pub fn test_somevariants_tryfrom_char() -> Result<(), String> {
    assert_eq!(ECSomeVariants::try_from('.')?, ECSomeVariants::Empty);
    assert_eq!(ECSomeVariants::try_from('#')?, ECSomeVariants::Wall);
    assert_eq!("#".parse::<ECSomeVariants>()?, ECSomeVariants::Wall);
    assert!(matches!(ECSomeVariants::try_from('z'), Err(_)));
    assert!(matches!("z".parse::<ECSomeVariants>(), Err(_)));
    assert_eq!(char::try_from(ECSomeVariants::Empty), Ok('.'));
    assert_eq!(char::try_from(ECSomeVariants::Wall), Ok('#'));
    assert!(matches!(char::try_from(ECSomeVariants::Other), Err(_)));
    Ok(())
}
