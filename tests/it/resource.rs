use std::collections::BTreeSet;

use rlimit::Resource;

/// The generated file is the source of truth for both the resource constants
/// and the `FromStr` mapping; reading it keeps the test in sync with codegen.
const GENERATED: &str = include_str!("../../src/resource/generated.rs");

fn declared_resources() -> BTreeSet<&'static str> {
    GENERATED
        .lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix("pub const ")?;
            rest.split(':').next()
        })
        .collect()
}

fn parsed_resources() -> BTreeSet<&'static str> {
    GENERATED
        .lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix('"')?;
            let (name, _) = rest.split_once("\" => Ok(Self::")?;
            Some(name)
        })
        .collect()
}

#[test]
fn resource_round_trip() {
    let declared: BTreeSet<String> = declared_resources()
        .into_iter()
        .map(|name| format!("RLIMIT_{name}"))
        .collect();
    let parsed = parsed_resources();

    assert!(
        !parsed.is_empty(),
        "no resources found in the generated file"
    );
    assert_eq!(
        declared,
        parsed
            .iter()
            .map(|name| name.to_string())
            .collect::<BTreeSet<_>>(),
        "the constants and the FromStr mapping disagree",
    );

    for name in &parsed {
        let resource: Resource = name.parse().unwrap();
        assert_eq!(resource.as_name(), *name, "round trip failed for {name}");
    }
}

#[test]
fn unknown_resource_is_rejected() {
    assert!("RLIMIT_UNKNOWN".parse::<Resource>().is_err());
}
