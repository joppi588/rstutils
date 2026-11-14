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
#[case::t00("targets_00")]
#[case::t01("targets_01")]
#[case::t02("targets_02")]
#[case::t03("targets_03")]
#[case::t04("targets_04")]
#[case::t06("targets_06")]
#[case::t07("targets_07")]
#[case::t08("targets_08")]
#[case::t18("targets_18")]
#[case::a00("anonymous_targets_00")]
#[case::a01("anonymous_targets_01")]
#[case::a02("anonymous_targets_02")]
#[case::a03("anonymous_targets_03")]
#[case::a04("anonymous_targets_04")]
#[case::a06("anonymous_targets_06")]
#[case::l08("legacy auto-id prefix_08")]
#[case::l10("legacy auto-id prefix_10")]
#[case::l11("legacy auto-id prefix_11")]
#[ignore]
fn ok_targets(#[case] test_case: &str) {
    rst_vs_yaml!("targets", test_case);
}

#[rstest]
#[case::t05("targets_05", PE::UnexpectedToken)]
#[case::t09("targets_09", PE::UnexpectedToken)]
#[case::t10("targets_10", PE::UnexpectedToken)]
#[case::t11("targets_11", PE::UnexpectedToken)]
#[case::t12("targets_12", PE::UnexpectedToken)]
#[case::t13("targets_13", PE::UnexpectedToken)]
#[case::t14("targets_14", PE::UnexpectedToken)]
#[case::t15("targets_15", PE::UnexpectedToken)]
#[case::t16("targets_16", PE::UnexpectedToken)]
#[case::t17("targets_17", PE::UnexpectedToken)]
#[case::t19("targets_19", PE::UnexpectedToken)]
#[case::t20("targets_20", PE::UnexpectedToken)]
#[case::l00("legacy-ids_00", PE::UnexpectedToken)]
#[case::l01("legacy-ids_01", PE::UnexpectedToken)]
#[case::l02("legacy-ids_02", PE::UnexpectedToken)]
#[case::l03("legacy-ids_03", PE::UnexpectedToken)]
#[case::l04("legacy-ids_04", PE::UnexpectedToken)]
#[case::l05("legacy-ids_05", PE::UnexpectedToken)]
#[case::a05("anonymous_targets_05", PE::UnexpectedToken)]
#[case::l00("legacy auto-id prefix_00", PE::UnexpectedToken)]
#[case::l01("legacy auto-id prefix_01", PE::UnexpectedToken)]
#[case::l02("legacy auto-id prefix_02", PE::UnexpectedToken)]
#[case::l03("legacy auto-id prefix_03", PE::UnexpectedToken)]
#[case::l04("legacy auto-id prefix_04", PE::UnexpectedToken)]
#[case::l05("legacy auto-id prefix_05", PE::UnexpectedToken)]
#[case::l06("legacy auto-id prefix_06", PE::UnexpectedToken)]
#[case::l07("legacy auto-id prefix_07", PE::UnexpectedToken)]
#[case::l09("legacy auto-id prefix_09", PE::UnexpectedToken)]
#[ignore]
fn err_targets(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("targets", test_case, expected_error);
}
