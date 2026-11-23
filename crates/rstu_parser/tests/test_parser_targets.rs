// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_ast::NodeClass;
use rstu_parser::parse;
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
#[case::t10("targets_10")]
#[case::t18("targets_18")]
#[case::a00("anonymous_targets_00")]
#[case::a01("anonymous_targets_01")]
#[case::a02("anonymous_targets_02")]
#[case::a03("anonymous_targets_03")]
#[case::a04("anonymous_targets_04")]
#[case::a05a("anonymous_targets_05a")]
#[case::a06("anonymous_targets_06")]
fn ok_targets(#[case] test_case: &str) {
    rst_vs_yaml!("targets", test_case);
}

/// GIVEN a document containing explicit named/anonymous and inline targets
/// WHEN the document is parsed through the public entry point
/// THEN all targets have distinct target types and preserve their labels/destinations
#[test]
fn explicit_and_inline_targets_are_distinguishable() {
    let document = parse(
        ".. _named:\n\n.. __: https://anonymous.example\n\n__ https://short.example\n\nSee _`inline`.\n",
    )
    .expect("document parses");
    let children = document.borrow().children.clone();
    let explicit_targets: Vec<_> = children
        .iter()
        .filter(|child| child.borrow().class == NodeClass::Target)
        .cloned()
        .collect();

    assert_eq!(explicit_targets.len(), 3);
    assert_eq!(
        explicit_targets[0]
            .borrow()
            .attributes
            .get_str("target_type")
            .as_deref(),
        Some("explicit")
    );
    assert_eq!(
        explicit_targets[0]
            .borrow()
            .attributes
            .get_str("label")
            .as_deref(),
        Some("named")
    );
    for (target, uri) in explicit_targets[1..]
        .iter()
        .zip(["https://anonymous.example", "https://short.example"])
    {
        assert_eq!(
            target.borrow().attributes.get_str("anonymous").as_deref(),
            Some("true")
        );
        assert_eq!(
            target.borrow().attributes.get_str("refuri").as_deref(),
            Some(uri)
        );
    }

    let paragraph = children
        .iter()
        .find(|child| child.borrow().class == NodeClass::Paragraph)
        .expect("inline target paragraph");
    let paragraph_children = paragraph.borrow().children.clone();
    let inline_target = paragraph_children
        .iter()
        .find(|child| child.borrow().class == NodeClass::Target)
        .expect("inline target");
    assert_eq!(
        inline_target
            .borrow()
            .attributes
            .get_str("target_type")
            .as_deref(),
        Some("inline")
    );
    let plain_text: Vec<_> = paragraph_children
        .iter()
        .filter(|child| child.borrow().class == NodeClass::PlainText)
        .filter_map(|child| child.borrow().attributes.get_str("text"))
        .collect();
    assert_eq!(plain_text, ["See ", ".\n"]);
}

#[rstest]
#[case::a05b("anonymous_targets_05b", PE::UnexpectedToken)]
#[case::t05("targets_05", PE::TargetMissingClosing)]
// #[case::t09("targets_09", PE::UnexpectedToken)]
// #[case::t11("targets_11", PE::UnexpectedToken)]
// #[case::t12("targets_12", PE::UnexpectedToken)]
// #[case::t13("targets_13", PE::UnexpectedToken)]
// #[case::t14("targets_14", PE::UnexpectedToken)]
// #[case::t15("targets_15", PE::UnexpectedToken)]
// #[case::t16("targets_16", PE::UnexpectedToken)]
// #[case::t17("targets_17", PE::UnexpectedToken)]
// #[case::t19("targets_19", PE::UnexpectedToken)]
// #[case::t20("targets_20", PE::UnexpectedToken)]
// #[case::l00("legacy-ids_00", PE::UnexpectedToken)]
// #[case::l01("legacy-ids_01", PE::UnexpectedToken)]
// #[case::l02("legacy-ids_02", PE::UnexpectedToken)]
// #[case::l03("legacy-ids_03", PE::UnexpectedToken)]
// #[case::l04("legacy-ids_04", PE::UnexpectedToken)]
// #[case::l05("legacy-ids_05", PE::UnexpectedToken)]
// #[case::l00("legacy auto-id prefix_00", PE::UnexpectedToken)]
// #[case::l01("legacy auto-id prefix_01", PE::UnexpectedToken)]
// #[case::l02("legacy auto-id prefix_02", PE::UnexpectedToken)]
// #[case::l03("legacy auto-id prefix_03", PE::UnexpectedToken)]
// #[case::l04("legacy auto-id prefix_04", PE::UnexpectedToken)]
// #[case::l05("legacy auto-id prefix_05", PE::UnexpectedToken)]
// #[case::l06("legacy auto-id prefix_06", PE::UnexpectedToken)]
// #[case::l07("legacy auto-id prefix_07", PE::UnexpectedToken)]
// #[case::l09("legacy auto-id prefix_09", PE::UnexpectedToken)]
// #[ignore]
fn err_targets(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("targets", test_case, expected_error);
}

#[rstest]
#[case::inline_targets("inline_targets_00")]
#[case::inline_targets("inline_targets_01")]
#[case::inline_targets("inline_targets_02")]
#[case::inline_targets("inline_targets_03")]
fn ok_inline_target(#[case] test_case: &str) {
    rst_vs_yaml!("targets/internal_target", test_case)
}

#[rstest]
#[case::inline_targets("inline_targets_04", PE::TargetMissingClosing)]
#[case::inline_targets("inline_targets_05", PE::TargetMissingClosing)]
fn err_inline_targets(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("targets/internal_target", test_case, expected_error);
}
