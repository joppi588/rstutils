// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
#[path = "common/mod.rs"]
mod test_parser;
use rstu_parser::parser_errors::ParserErrorKind as PE;
use test_parser::rst_err;
use test_parser::rst_vs_yaml;

#[rstest]
// #[case::references("references_00")]
// #[case::references("references_01")]
// #[case::references("references_02")]
// #[case::references("references_03")]
// #[case::references("references_04")]
// #[case::references("references_05")]
// #[case::references("references_06")]
#[case::c00("citation_reference_00")]
#[case::c01("citation_reference_01")]
#[case::c02("citation_reference_02")]
#[case::footnote_reference("footnote_reference_00")]
#[case::footnote_reference("footnote_reference_01")]
#[case::footnote_reference("footnote_reference_02")]
#[case::footnote_reference("footnote_reference_03")]
fn ok_inline_reference(#[case] test_case: &str) {
    rst_vs_yaml!("inline/reference", test_case)
}

#[rstest]
#[case::ea01("embedded_aliases_00")]
#[case::ea01("embedded_aliases_01")]
#[case::ea02("embedded_aliases_02")]
#[case::ea03("embedded_aliases_03")]
#[case::ea04("embedded_aliases_04")]
#[case::ea05("embedded_aliases_05")]
#[case::ea06("embedded_aliases_06")]
#[case::ea07("embedded_aliases_07")]
fn ok_embedded_alias(#[case] test_case: &str) {
    rst_vs_yaml!("inline/reference", test_case)
}

#[rstest]
#[case::e00("embedded_uris_00")]
#[case::e01("embedded_uris_01")]
#[case::e02("embedded_uris_02")]
#[case::e03("embedded_uris_03")]
#[case::e04("embedded_uris_04")]
#[case::e05("embedded_uris_05")]
#[case::e06("embedded_uris_06")]
#[case::e07("embedded_uris_07")]
#[case::e08("embedded_uris_08")]
#[case::e09("embedded_uris_09")]
#[case::e10("embedded_uris_10")]
fn ok_embedded_uris(#[case] test_case: &str) {
    rst_vs_yaml!("inline/reference", test_case)
}

#[rstest]
#[case::pr00("phrase_references_00")]
#[case::pr01("phrase_references_01")]
#[case::pr02("phrase_references_02")]
#[case::pr03("phrase_references_03")]
#[case::pr04("phrase_references_04")]
#[case::pr05("phrase_references_05")]
#[case::pr06("phrase_references_06")]
#[case::pr07("phrase_references_07")]
#[case::pr08("phrase_references_08")]
#[case::pr09("phrase_references_09")]
fn ok_phrase_reference(#[case] test_case: &str) {
    rst_vs_yaml!("inline/reference", test_case)
}

#[rstest]
#[case::pr10("phrase_references_10", PE::UnexpectedToken)]
#[case::pr11("phrase_references_11", PE::UnexpectedToken)]
#[case::pr12("phrase_references_12", PE::UnexpectedToken)]
#[case::pr13("phrase_references_13", PE::UnexpectedToken)]
fn err_phrase_reference(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("inline/reference", test_case, expected_error);
}

#[rstest]
#[case::sh00("standalone_hyperlink_00")]
#[case::sh01("standalone_hyperlink_01")]
#[case::sh02("standalone_hyperlink_02")]
#[case::sh03("standalone_hyperlink_03")]
#[case::sh04("standalone_hyperlink_04")]
#[case::sh05("standalone_hyperlink_05")]
fn ok_standalone_hyperlink(#[case] test_case: &str) {
    rst_vs_yaml!("inline/reference", test_case)
}

#[rstest]
#[case::s00("substitution_references_00")]
#[case::s01("substitution_references_01")]
#[case::s02("substitution_references_02")]
#[case::s03("substitution_references_03")]
#[case::s04("substitution_references_04")]
#[case::s05("substitution_references_05")]
#[case::s06("substitution_references_06")]
fn ok_substitution_references(#[case] test_case: &str) {
    rst_vs_yaml!("inline/reference", test_case)
}

// TODO: Err cases
// #[case::embedded_aliases("embedded_aliases_09")]
// #[case::embedded_uris("embedded_uris_11")]
// #[case::footnote_reference("footnote_reference_04")]
