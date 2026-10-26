// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::parser_errors::ParserErrorKind as PE;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_err;
use test_parser::rst_vs_yaml;

#[rstest]
#[case::sh00("section_headers_00")]
#[case::sh01("section_headers_01")]
#[case::sh02("section_headers_02")]
#[case::sh04("section_headers_04")]
#[case::sh05("section_headers_05")]
#[case::sh06("section_headers_06")]
#[case::sh07("section_headers_07")]
#[case::sh11("section_headers_11")]
#[case::sh13("section_headers_13")]
#[case::sh15("section_headers_15")]
#[case::sh16("section_headers_16")]
#[case::sh17("section_headers_17")]
#[case::sh20("section_headers_20")]
#[case::sh21("section_headers_21")]
#[case::sh23("section_headers_23")]
#[case::sh24("section_headers_24")]
#[case::sh25("section_headers_25")]
#[case::sh26("section_headers_26")]
#[case::sh27("section_headers_27")]
#[case::sh28("section_headers_28")]
#[case::sh29("section_headers_29")]
#[case::sh29("section_headers_30")]
#[case::sh31("section_headers_31")]
#[case::sh34("section_headers_34")]
#[case::sh35("section_headers_35")]
#[case::sh36("section_headers_36")]
fn ok_section_headers(#[case] test_case: &str) {
    rst_vs_yaml!("section_headers/ok", test_case);
}

#[rstest]
#[case::sh14("section_headers_14")]
#[case::sh18("section_headers_18")]
#[case::sh19("section_headers_19")]
fn warn_section_headers(#[case] test_case: &str) {
    rst_vs_yaml!("section_headers/warn", test_case);
}

#[rstest]
#[case::sh03("section_headers_03", PE::UnexpectedToken)]
#[case::sh08("section_headers_08", PE::SectionTitleMissingClosingAfterOpening)]
#[case::sh09("section_headers_09", PE::SectionTitleMissingClosingAfterOpening)]
#[case::sh10("section_headers_10", PE::SectionTitleMissingClosingAfterOpening)]
#[case::sh12("section_headers_12", PE::SectionTitleUnbalancedStyle)]
#[case::sh22("section_headers_22", PE::ListEndError)]
#[case::sh29b("section_headers_29b", PE::MissingFinalBlankLineError)]
#[case::sh32("section_headers_32", PE::SectionTitleUnbalancedStyle)]
#[case::sh33("section_headers_33", PE::UnexpectedToken)] // TODO: Correct interpretetation as comment??
fn err_section_headers(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("section_headers/err", test_case, expected_error);
}
