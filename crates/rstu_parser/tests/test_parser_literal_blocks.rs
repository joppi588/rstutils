// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::parse;
use rstu_parser::parser_errors::ParserErrorKind as PE;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_err;
use test_parser::rst_vs_yaml;

#[rstest]
#[case::i00("indented_00")]
#[case::i01("indented_01")]
#[case::i02("indented_02")]
#[case::i05("indented_05")]
#[case::i06("indented_06")]
#[case::i07("indented_07")]
#[case::i08("indented_08")]
#[case::i09("indented_09")]
#[case::q00("quoted_00")]
#[case::q01("quoted_01")]
#[case::q02("quoted_02")]
fn ok_literal_block(#[case] test_case: &str) {
    rst_vs_yaml!("literal_blocks/ok", test_case);
}

#[rstest]
#[case::i03("indented_03", PE::LiteralBlockMissingBlankLine)]
#[case::i04("indented_04", PE::MissingFinalBlankLineError)]
#[case::i10("indented_10", PE::LiteralBlockInvalidQuoteMarker)]
#[case::i12("indented_12", PE::LiteralBlockMissingBlankLine)]
#[case::q03("quoted_03", PE::LiteralBlockInvalidQuoteMarker)]
#[case::q04("quoted_04", PE::LiteralBlockInvalidQuoteMarker)]
#[case::q05("quoted_05", PE::LiteralBlockInconsistentQuoteMarker)]
#[case::q01("quoted_block_01", PE::LiteralBlockInvalidQuoteMarker)]
#[case::q02("quoted_block_02", PE::LiteralBlockInconsistentQuoteMarker)]
fn err_literal_block(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("literal_blocks/err", test_case, expected_error);
}

#[test]
fn ok_consistent_quoted_literal_markers() {
    for marker in "!#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars() {
        let source = format!("A paragraph::\n\n{marker}first\n{marker}second\n\n");
        assert!(
            parse(&source).is_ok(),
            "marker {marker:?} should be accepted"
        );
    }
}
