//! Hand-built Aspecta + text render. No read_dir.

use aspectus::{broot_failure_fixture, render_text, Node, NodeKind};

#[test]
fn broot_failure_git_is_not_a_child_and_aat_mentions_src() {
    let a = broot_failure_fixture();
    assert_eq!(a.node.name, "arch");
    assert!(a.node.annotations.kinds.iter().any(|k| k == "git"));
    assert!(
        a.node.children.iter().all(|c| c.name != ".git"),
        ".git must be parent state, not a child"
    );
    assert_eq!(a.node.children[0].kind, NodeKind::Dir);

    let text = render_text(&a.node, 80);
    assert!(
        !text.lines().any(|l| l.contains(".git/") || l.trim_start().starts_with(".git")),
        "render must not list .git as a child:\n{text}"
    );
    assert!(
        text.contains("01-aat-core") && text.contains("src/"),
        "unexpanded 01-aat-core must still mention src/:\n{text}"
    );
}

#[test]
fn node_kinds_cover_dir_file_symlink_aggregate() {
    let n = Node::dir("d")
        .with_children(vec![
            Node::file("f"),
            Node::symlink("l", "t"),
            Node {
                name: "[agg]".into(),
                kind: NodeKind::Aggregate,
                annotations: Default::default(),
                children: vec![],
            },
        ]);
    assert_eq!(n.kind, NodeKind::Dir);
    assert_eq!(n.children[0].kind, NodeKind::File);
    assert_eq!(n.children[1].kind, NodeKind::Symlink);
    assert_eq!(n.children[2].kind, NodeKind::Aggregate);
}
