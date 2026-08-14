//! GitHub as specialized furniture (design/furniture/github.md).
//!
//! `.github/` is not a child of the look; the parent can say there is
//! GitHub metadata here — the workflow count when there are workflows,
//! or just the `github` kind (the map row carries that). No network,
//! no fetch, no `(private)` decision.

use std::fs;
use std::path::Path;

/// `github: N workflows` for a directory whose children include `.github`,
/// or `None` when there are no workflows — the kind alone then carries it.
pub fn facet(dir: &Path) -> Option<String> {
    let rd = fs::read_dir(dir.join(".github").join("workflows")).ok()?;
    let n = rd
        .filter_map(Result::ok)
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name.ends_with(".yml") || name.ends_with(".yaml")
        })
        .count();
    if n == 0 {
        return None;
    }
    let s = if n == 1 { "" } else { "s" };
    Some(format!("github: {n} workflow{s}"))
}
