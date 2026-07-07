// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use document_tree::{
    HasChildren,
    element_categories::{BodyElement, StructuralSubElement, SubStructure, TextOrInlineElement},
};
use rst_parser::parse;
use std::fs;
use std::path::{Path, PathBuf};

fn test_data_path(filename: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data").join(filename)
}

#[test]
#[ignore = "rst_parser does not yet reject short title underlines"]
fn test_failure_short_underline() {
    // GIVEN An RST file with short underline (invalid)
    // WHEN The file is parsed
    // THEN Parsing fails with an error
    let contents = fs::read_to_string(test_data_path("nok_short_title_underline.rst"))
        .expect("failed to read nok rst file");
    
    let result = parse(&contents);
    
    assert!(result.is_err(), "short underline");
}
