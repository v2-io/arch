//! The seen-things: an `Aspecta` is a snapshot of one locus.

use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Dir,
    File,
    Symlink,
    Aggregate,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Annotation {
    /// Kind claims on this node (`git`, `rust`, `obsidian-vault`, …).
    pub kinds: Vec<String>,
    /// Role tag for child role-dirs (`archive`, `trash`).
    pub role: Option<String>,
    /// Visit-budget (or similar) stopped expansion.
    pub truncated: bool,
    /// Names we know exist but did not expand — must appear in the render.
    pub omitted: Vec<String>,
    /// Symlink target, if this node is a symlink.
    pub symlink_target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub kind: NodeKind,
    pub annotations: Annotation,
    pub children: Vec<Node>,
}

impl Node {
    pub fn dir(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: NodeKind::Dir,
            annotations: Annotation::default(),
            children: Vec::new(),
        }
    }

    pub fn file(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: NodeKind::File,
            annotations: Annotation::default(),
            children: Vec::new(),
        }
    }

    pub fn symlink(name: impl Into<String>, target: impl Into<String>) -> Self {
        let mut n = Self {
            name: name.into(),
            kind: NodeKind::Symlink,
            annotations: Annotation::default(),
            children: Vec::new(),
        };
        n.annotations.symlink_target = Some(target.into());
        n
    }

    pub fn with_kinds(mut self, kinds: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.annotations.kinds = kinds.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.annotations.role = Some(role.into());
        self
    }

    pub fn with_omitted(mut self, names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.annotations.omitted = names.into_iter().map(Into::into).collect();
        self
    }

    pub fn truncated(mut self) -> Self {
        self.annotations.truncated = true;
        self
    }

    pub fn with_children(mut self, children: Vec<Node>) -> Self {
        self.children = children;
        self
    }

    pub fn is_dir(&self) -> bool {
        self.kind == NodeKind::Dir
    }
}

/// A snapshot of one locus: the seen-things under a line budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Aspecta {
    pub root: PathBuf,
    pub node: Node,
}

impl Aspecta {
    pub fn new(root: impl Into<PathBuf>, node: Node) -> Self {
        Self {
            root: root.into(),
            node,
        }
    }
}

/// The broot-failure shape: a git repo whose `.git` must not appear as a
/// child, and an unexpanded `01-aat-core` that must still mention `src/`.
pub fn broot_failure_fixture() -> Aspecta {
    let aat = Node::dir("01-aat-core").with_omitted(["src/"]);
    let asf = Node::dir("asf")
        .with_kinds(["git"])
        .with_children(vec![aat, Node::file("README.md")]);
    let root = Node::dir("arch")
        .with_kinds(["git"])
        .with_children(vec![
            asf,
            Node::file("CHARTER-DRAFT.md"),
            Node::file("AGENTIC-DELEGATION.md"),
        ]);
    Aspecta::new("/example/arch", root)
}
