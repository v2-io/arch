//! Four-fate mapping is data. Drive `fate` / `partition` as shipped.

use aspectus::{fate, partition, Fate};

#[test]
fn furniture_is_absorb() {
    for (name, kind) in [
        (".git", "git"),
        ("target", "build"),
        (".obsidian", "obsidian-vault"),
        (".claude", "agents"),
        ("node_modules", "build"),
    ] {
        assert_eq!(
            fate(name, true),
            Fate::Absorb { kind },
            "{name} should absorb as {kind}"
        );
    }
}

#[test]
fn ds_store_is_omit() {
    assert_eq!(fate(".DS_Store", false), Fate::Omit);
}

#[test]
fn unknown_hidden_remains_child() {
    match fate(".orient", true) {
        Fate::Child { role: None } => {}
        other => panic!(".orient should be an unknown-hidden child, got {other:?}"),
    }
    match fate(".mystery", false) {
        Fate::Child { role: None } => {}
        other => panic!("got {other:?}"),
    }
}

#[test]
fn role_dirs_are_children_with_tag() {
    match fate(".archive", true) {
        Fate::Child { role: Some("archive") } => {}
        other => panic!("got {other:?}"),
    }
    match fate(".trash", true) {
        Fate::Child { role: Some("trash") } => {}
        other => panic!("got {other:?}"),
    }
}

#[test]
fn witnesses_stay_listable() {
    match fate("Cargo.toml", false) {
        Fate::Witness { kind: Some("rust") } => {}
        other => panic!("got {other:?}"),
    }
    match fate("README.md", false) {
        Fate::Witness { kind: None } => {}
        other => panic!("got {other:?}"),
    }
}

#[test]
fn partition_default_hides_absorb_lists_children() {
    let entries = [
        (".git".into(), true),
        ("target".into(), true),
        (".DS_Store".into(), false),
        (".archive".into(), true),
        (".orient".into(), true),
        ("src".into(), true),
        ("Cargo.toml".into(), false),
    ];
    let p = partition(entries, None);
    assert!(p.absorbed.iter().any(|(n, k)| n == ".git" && *k == "git"));
    assert!(p.absorbed.iter().any(|(n, k)| n == "target" && *k == "build"));
    assert!(p.omitted.iter().any(|n| n == ".DS_Store"));
    assert!(p.children.iter().any(|c| c.name == "src"));
    assert!(p.children.iter().any(|c| c.name == ".archive" && c.role == Some("archive")));
    assert!(p.children.iter().any(|c| c.name == ".orient" && c.role.is_none()));
    assert!(p.children.iter().any(|c| c.name == "Cargo.toml"));
    assert!(p.children.iter().all(|c| c.name != ".git" && c.name != "target"));
}

#[test]
fn raw_opens_absorb() {
    let entries = [(".git".into(), true), ("src".into(), true)];
    let p = partition(entries, Some("*"));
    assert!(p.absorbed.is_empty());
    assert!(p.children.iter().any(|c| c.name == ".git"));
}
