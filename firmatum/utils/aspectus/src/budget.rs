//! Sibling-share line budget. A cutoff is not a summary.

use crate::ir::Node;

/// How a parent's remaining lines are split among children.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alloc {
    /// Lines the parent line itself occupies (0 or 1).
    pub self_lines: usize,
    /// Per-child subtree budget. `0` means the child is only named in the
    /// aggregate remainder — it is not listed as its own line.
    pub shares: Vec<usize>,
    /// Indices of children with share 0, in listing order.
    pub aggregated: Vec<usize>,
}

impl Alloc {
    pub fn max_share(&self) -> usize {
        self.shares.iter().copied().max().unwrap_or(0)
    }
}

/// Weight: directories outrank files; witnesses and role-dirs a bit more.
/// Absorbed names are not children, so they never get a share.
pub fn weight(node: &Node) -> u32 {
    let mut w = match node.kind {
        crate::ir::NodeKind::Dir => 4,
        crate::ir::NodeKind::Symlink => 2,
        crate::ir::NodeKind::File => 1,
        crate::ir::NodeKind::Aggregate => 1,
    };
    if node.annotations.role.is_some() {
        w += 1;
    }
    if !node.annotations.kinds.is_empty() {
        w += 1;
    }
    w
}

/// Allocate `budget` lines to this node *including* its own line.
///
/// When every child can have a line, leftover lines go to the heaviest
/// directories (round-robin among ties) — never all to the first sibling.
/// When they cannot, one line is reserved for an aggregate that names the
/// omitted children. That is the honesty bar: no silent drop, no
/// first-child height-fill.
pub fn allocate_shares(children: &[Node], budget: usize) -> Alloc {
    let n = children.len();
    if budget == 0 {
        return Alloc {
            self_lines: 0,
            shares: vec![0; n],
            aggregated: (0..n).collect(),
        };
    }
    if n == 0 {
        return Alloc {
            self_lines: 1,
            shares: Vec::new(),
            aggregated: Vec::new(),
        };
    }
    let remain = budget - 1;
    let weights: Vec<u32> = children.iter().map(weight).collect();

    if remain >= n {
        let extra = remain - n;
        let mut shares = vec![1usize; n];
        if extra > 0 {
            let mut order: Vec<usize> = (0..n).collect();
            order.sort_by(|&a, &b| {
                weights[b]
                    .cmp(&weights[a])
                    .then_with(|| children[a].name.cmp(&children[b].name))
            });
            // Prefer giving extra to directories (they can expand).
            let dirs: Vec<usize> = order
                .iter()
                .copied()
                .filter(|&i| children[i].is_dir())
                .collect();
            let cycle = if dirs.is_empty() { &order } else { &dirs };
            for k in 0..extra {
                shares[cycle[k % cycle.len()]] += 1;
            }
        }
        Alloc {
            self_lines: 1,
            shares,
            aggregated: Vec::new(),
        }
    } else {
        // Cannot list everyone. Reserve one line for the aggregate when we
        // can; if remain == 0 we already returned at budget==1 with remain 0
        // — wait, budget>=1 here so remain can be 0 when budget==1.
        if remain == 0 {
            return Alloc {
                self_lines: 1,
                shares: vec![0; n],
                aggregated: (0..n).collect(),
            };
        }
        let list_n = remain.saturating_sub(1);
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| {
            weights[b]
                .cmp(&weights[a])
                .then_with(|| children[a].name.cmp(&children[b].name))
        });
        let mut shares = vec![0usize; n];
        for &i in order.iter().take(list_n) {
            shares[i] = 1;
        }
        let aggregated: Vec<usize> = (0..n).filter(|&i| shares[i] == 0).collect();
        Alloc {
            self_lines: 1,
            shares,
            aggregated,
        }
    }
}

/// Human-readable shares-and-why, for `--explain-budget` (stderr).
pub fn explain(children: &[Node], alloc: &Alloc, budget: usize) -> String {
    let mut out = String::new();
    out.push_str(&format!("budget {budget}\n"));
    out.push_str(&format!("self {}\n", alloc.self_lines));
    out.push_str(&format!(
        "remain {}\n",
        budget.saturating_sub(alloc.self_lines)
    ));
    for (i, child) in children.iter().enumerate() {
        let share = alloc.shares.get(i).copied().unwrap_or(0);
        let why = if share == 0 {
            "aggregate"
        } else if child.is_dir() {
            "dir"
        } else {
            "file"
        };
        let mark = if child.is_dir() { "/" } else { "" };
        out.push_str(&format!(
            "  {}{mark}  share={share}  weight={}  {why}\n",
            child.name,
            weight(child)
        ));
    }
    out
}
