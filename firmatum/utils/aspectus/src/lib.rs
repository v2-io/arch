//! aspectus — the look of a locus.
//!
//! The faculty is **aspectus**. A single snapshot it produces is an
//! **aspecta** (the seen-things).

pub mod absorb;
pub mod budget;
pub mod ir;
pub mod render;
pub mod walk;

pub use absorb::{fate, partition, Fate};
pub use budget::{allocate_shares, explain, Alloc};
pub use ir::{broot_failure_fixture, Annotation, Aspecta, Node, NodeKind};
pub use render::render_text;
pub use walk::{same_filesystem, walk, WalkOptions, WalkResult};
