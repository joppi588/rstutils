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
// #[case::inline_targets("inline_targets_00")]
// #[case::inline_targets("inline_targets_01")]
// #[case::inline_targets("inline_targets_02")]
// #[case::inline_targets("inline_targets_03")]
// #[case::markup_recognition_rules("markup_recognition_rules_00")]
// #[case::markup_recognition_rules("markup_recognition_rules_01")]
// #[case::markup_recognition_rules("markup_recognition_rules_02")]
// #[case::markup_recognition_rules("markup_recognition_rules_03")]
// #[case::markup_recognition_rules("markup_recognition_rules_04")]
// #[case::markup_recognition_rules("markup_recognition_rules_05")]
// #[case::markup_recognition_rules("markup_recognition_rules_06")]
// #[case::markup_recognition_rules("markup_recognition_rules_07")]
// #[case::phrase_references("phrase_references_00")]
// #[case::phrase_references("phrase_references_01")]
// #[case::phrase_references("phrase_references_02")]
// #[case::phrase_references("phrase_references_03")]
// #[case::phrase_references("phrase_references_04")]
// #[case::phrase_references("phrase_references_05")]
// #[case::phrase_references("phrase_references_06")]
// #[case::phrase_references("phrase_references_07")]
// #[case::phrase_references("phrase_references_08")]
// #[case::phrase_references("phrase_references_09")]
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
#[case::emphasis("emphasis_00")]
#[case::emphasis("emphasis_01")]
#[case::emphasis("emphasis_02")]
#[case::emphasis("emphasis_05")]
#[case::emphasis("emphasis_06")]
#[case::footnote_reference("footnote_reference_00")]
#[case::footnote_reference("footnote_reference_01")]
#[case::footnote_reference("footnote_reference_02")]
#[case::footnote_reference("footnote_reference_03")]
#[case::strong("strong_00")]
#[case::strong("strong_01")]
#[case::strong("strong_02")]
// #[case::strong("strong_04")] TODO: #439
fn ok_inline_markup(#[case] test_case: &str) {
    rst_vs_yaml!("inline_markup", test_case)
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
#[case::ea08("embedded_aliases_08")]
fn ok_embedded_alias(#[case] test_case: &str) {
    rst_vs_yaml!("inline_markup", test_case)
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
    rst_vs_yaml!("inline_markup", test_case)
}

#[rstest]
#[case::l00("literal_00")]
#[case::l01("literal_01")]
#[case::l02("literal_02")]
// #[case::l03("literal_03")] // TODO: #464
#[case::l04("literal_04")]
#[case::l05("literal_05")]
#[case::l06("literal_06")]
#[case::l08("literal_08")]
#[case::l10("literal_10")]
#[case::l11("literal_11")]
#[case::l12("literal_12")]
#[case::l13("literal_13")]
fn ok_literal(#[case] test_case: &str) {
    rst_vs_yaml!("inline_markup", test_case)
}

#[rstest]
#[case::l07("literal_07", PE::InlineMissingClosing)]
#[case::l09("literal_09", PE::InlineMissingClosing)]
fn err_literal(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("inline_markup", test_case, expected_error);
}
#[rstest]
#[case::sh00("standalone_hyperlink_00")]
#[case::sh01("standalone_hyperlink_01")]
#[case::sh02("standalone_hyperlink_02")]
#[case::sh03("standalone_hyperlink_03")]
#[case::sh04("standalone_hyperlink_04")]
#[case::sh05("standalone_hyperlink_05")]
fn ok_standalone_hyperlink(#[case] test_case: &str) {
    rst_vs_yaml!("inline_markup", test_case)
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
    rst_vs_yaml!("inline_markup", test_case)
}

// TODO: Err cases
// #[case::embedded_aliases("embedded_aliases_09")]
// #[case::embedded_uris("embedded_uris_11")]
// #[case::emphasis("emphasis_03")]
// #[case::emphasis("emphasis_04")]
// #[case::footnote_reference("footnote_reference_04")]
// #[case::inline_targets("inline_targets_04")]
// #[case::inline_targets("inline_targets_05")]
// #[case::phrase_references("phrase_references_10")]
// #[case::phrase_references("phrase_references_11")]
// #[case::phrase_references("phrase_references_12")]
// #[case::phrase_references("phrase_references_13")]
// #[case::strong("strong_03")]
// #[case::strong("strong_05")]
