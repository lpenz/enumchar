// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#[test]
fn test_enumchar_errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/errors/*.rs");
}
