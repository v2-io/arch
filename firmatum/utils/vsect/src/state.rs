//! The computed view of a store: every displayed value comes from here, and nothing here is typed.

use crate::event::{self, Event, Trail};
use crate::record::{self, Record, Scan};
use crate::store::Store;
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// The basis the check read is the basis now.
    Holds,
    /// The statement's own text changed since the check.
    StaleOwn,
    /// Something the statement rests on changed meaning since the check (which ids).
    StaleUpstream(Vec<String>),
    /// Something in the basis changed, but the check declared what it read and read none of it.
    /// Lighter than stale: visible, counted apart, still open to a confirmer's ending it.
    StaleUpstreamUnread(Vec<String>),
    /// An edge that shapes the basis was added or removed.
    BasisChanged,
    /// A record in the basis is deleted or unreadable now.
    CannotEvaluate(String),
}

impl Verdict {
    pub fn word(&self) -> String {
        match self {
            Verdict::Holds => "holds".into(),
            Verdict::StaleOwn => "stale: own text changed".into(),
            Verdict::StaleUpstream(ids) => format!("stale: upstream changed ({})", ids.join(", ")),
            Verdict::StaleUpstreamUnread(ids) => format!("upstream changed, not read by this check ({})", ids.join(", ")),
            Verdict::BasisChanged => "stale: what it rests on was re-declared".into(),
            Verdict::CannotEvaluate(why) => format!("cannot evaluate: {why}"),
        }
    }
    pub fn holds(&self) -> bool {
        matches!(self, Verdict::Holds)
    }
    /// The lighter state: the check declared what it read, and nothing it read changed.
    pub fn unread(&self) -> bool {
        matches!(self, Verdict::StaleUpstreamUnread(_))
    }
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub item: String, // review:slug#F1
    pub target: String,
    pub at: String,
    pub by: String,
    pub text: String,
    pub placed: Event,
}

#[derive(Debug, Clone)]
pub struct Question {
    pub id: String,
    pub closer: String,
    pub asked: String,
    pub by: String,
    pub title: String,
    pub concerns: Vec<String>,
    pub blocks: Vec<String>,
}

/// Work owed, with an owner, closed by the commit that did it. An imported item carries its
/// tracker line and no authority until a mind re-checks it against HEAD.
#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub concerns: Vec<String>,
    pub owner: String,
    pub opened: String,
    pub by: String,
    pub source: String,
    pub section: String,
    /// The re-checker's restatement of a held item, when the tracker's frame was wrong.
    pub restated: Option<String>,
    /// true until a `recheck` with outcome `holds` (or for items opened with `note`)
    pub unchecked: bool,
}

#[derive(Debug, Default, Clone)]
pub struct RecordState {
    pub exists: bool,
    pub atom: Option<String>,
    pub last_lifecycle: Option<Event>,
    /// Every lifecycle event on the record, oldest first (mint, revise, …).
    pub lifecycle: Vec<Event>,
    pub last_version: Option<String>,
    pub current_version: Option<String>,
    /// The last version any event cited for this record (own-version or target-version), so a
    /// record that was never minted but was checked or refuted at a past revision is still known.
    pub seen_version: Option<String>,
    pub refuted: Option<Event>,
    pub superseded_by: Option<String>,
    pub retired: bool,
    /// Ids of refuted statements this one necessarily rests on.
    pub broken_by: Vec<String>,
    pub securing: Vec<(Event, Verdict)>,
}

impl RecordState {
    /// A record the trail speaks about that is gone from the tree with no fate recorded. (A
    /// record's existence is the tree's to say; only its disappearance needs an event.)
    pub fn unreconciled(&self) -> Option<&'static str> {
        if !self.exists && !self.retired && self.superseded_by.is_none() && self.seen_version.is_some() {
            Some("deleted: fate not recorded")
        } else {
            None
        }
    }
    /// The version to cite for a record that is gone: its last lifecycle version, else the last one seen.
    pub fn last_known_version(&self) -> Option<String> {
        self.last_version.clone().or_else(|| self.seen_version.clone())
    }
    pub fn live(&self) -> bool {
        self.exists && !self.retired && self.superseded_by.is_none()
    }
}

pub struct State {
    pub scan: Scan,
    pub trail: Trail,
    pub records: BTreeMap<String, RecordState>,
    pub findings_open: Vec<Finding>,
    pub findings_closed: usize,
    pub questions_open: Vec<Question>,
    pub questions_closed: usize,
    pub items_open: Vec<Item>,
    pub items_closed: usize,
    /// Reverse of every declared edge: target id -> (edge, source id)
    pub reverse: BTreeMap<String, Vec<(String, String)>>,
    /// Declared targets that resolve to no record: (source id, edge, target)
    pub dangling: Vec<(String, String, String)>,
    /// Declared targets that resolve to no record but are declared planned (store `planned:`)
    pub planned: Vec<(String, String, String)>,
    /// Confirmed equivalences between versions.
    pub eq: Equivalence,
}

/// A difference between a version a check read and the version now, awaiting a second mind.
#[derive(Debug, Clone)]
pub struct PendingConfirm {
    pub record: String,
    pub from: String,
    pub to: String,
    /// The securing events (id, their target) that would hold again if the pair were confirmed.
    pub checks: Vec<(String, String)>,
    /// Checks on this pair that declared a read set which does not include the record (lighter).
    pub unread: Vec<String>,
}

/// An entry in a check's basis as recorded on the event.
#[derive(Debug, Clone)]
pub struct BasisEntry {
    pub id: String,
    pub version: String, // or "unwritten"
}

pub fn basis_of(store: &Store, scan: &Scan, id: &str) -> Vec<BasisEntry> {
    let meaning: BTreeSet<&str> = store
        .kinds
        .edges
        .iter()
        .filter(|(_, d)| d.meaning)
        .map(|(k, _)| k.as_str())
        .collect();
    let in_basis: BTreeSet<&str> = store
        .kinds
        .edges
        .iter()
        .filter(|(_, d)| d.in_basis)
        .map(|(k, _)| k.as_str())
        .collect();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut out = vec![];
    let mut queue: VecDeque<(String, bool)> = VecDeque::new(); // (id, follow further?)
    queue.push_back((id.to_string(), true));
    while let Some((cur, follow)) = queue.pop_front() {
        if !seen.insert(cur.clone()) {
            continue;
        }
        let rec = scan.records.get(&cur);
        out.push(BasisEntry {
            id: cur.clone(),
            version: rec.map(|r| r.own_version.clone()).unwrap_or_else(|| "unwritten".into()),
        });
        if !follow {
            continue;
        }
        if let Some(r) = rec {
            for (edge, targets) in &r.edges {
                let is_meaning = meaning.contains(edge.as_str());
                let is_basis = in_basis.contains(edge.as_str());
                if !(is_meaning || is_basis) {
                    continue;
                }
                for t in targets {
                    let tid = resolve_target(store, &r.id.kind, t);
                    queue.push_back((tid, is_meaning));
                }
            }
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

/// A bare slug in an edge field names a record of the same kind as the source.
pub fn resolve_target(store: &Store, source_kind: &str, t: &str) -> String {
    let t = t.trim().trim_start_matches('#');
    if let Some((k, s)) = t.split_once(':') {
        let k = store.kinds.canonical_kind(k).unwrap_or_else(|| k.to_string());
        format!("{k}:{s}")
    } else {
        format!("{source_kind}:{t}")
    }
}

pub fn basis_to_yaml(b: &[BasisEntry]) -> Value {
    let mut m = serde_yaml::Mapping::new();
    for e in b {
        m.insert(Value::String(e.id.clone()), Value::String(e.version.clone()));
    }
    Value::Mapping(m)
}

pub fn basis_from_yaml(v: Option<&Value>) -> Vec<BasisEntry> {
    let mut out = vec![];
    if let Some(Value::Mapping(m)) = v {
        for (k, v) in m {
            if let (Some(k), Some(v)) = (k.as_str(), v.as_str()) {
                out.push(BasisEntry { id: k.to_string(), version: v.to_string() });
            }
        }
    }
    out
}

pub fn effective_version(b: &[BasisEntry]) -> String {
    let s: Vec<String> = b.iter().map(|e| format!("{}={}", e.id, e.version)).collect();
    crate::norm::sha256_hex(s.join("\n").as_bytes())
}

/// Confirmed equivalences between versions of one statement (core.md §3.6): the smallest
/// equivalence relation containing every unconditional `confirm-rewording … equivalent` pair,
/// plus the scoped confirmations, which apply to the named checks alone.
#[derive(Debug, Default, Clone)]
pub struct Equivalence {
    /// record id -> union-find parent map over versions
    classes: BTreeMap<String, BTreeMap<String, String>>,
    /// securing event id -> (record id, from, to) pairs confirmed for that check only
    scoped: BTreeMap<String, Vec<(String, String, String)>>,
    /// (record id, from, to) pairs a confirmer judged not equivalent for every check
    pub refused: Vec<(String, String, String, String)>, // + confirmer
    /// securing event id -> (record id, from, to) pairs judged not equivalent for that check only
    refused_scoped: BTreeMap<String, Vec<(String, String, String)>>,
}

impl Equivalence {
    fn find(m: &mut BTreeMap<String, String>, v: &str) -> String {
        let mut cur = v.to_string();
        loop {
            match m.get(&cur) {
                Some(p) if p != &cur => cur = p.clone(),
                _ => return cur,
            }
        }
    }
    fn union(&mut self, rec: &str, a: &str, b: &str) {
        let m = self.classes.entry(rec.to_string()).or_default();
        m.entry(a.to_string()).or_insert_with(|| a.to_string());
        m.entry(b.to_string()).or_insert_with(|| b.to_string());
        let ra = Self::find(m, a);
        let rb = Self::find(m, b);
        if ra != rb {
            m.insert(ra, rb);
        }
    }
    pub fn same(&self, rec: &str, a: &str, b: &str) -> bool {
        if a == b {
            return true;
        }
        let Some(m) = self.classes.get(rec) else { return false };
        let mut m = m.clone();
        Self::find(&mut m, a) == Self::find(&mut m, b)
    }
    pub fn scoped_covers(&self, event_id: &str, rec: &str, a: &str, b: &str) -> bool {
        self.scoped
            .get(event_id)
            .map(|v| v.iter().any(|(r, f, t)| r == rec && ((f == a && t == b) || (f == b && t == a))))
            .unwrap_or(false)
    }
    pub fn from_trail(trail: &Trail) -> Equivalence {
        let mut eq = Equivalence::default();
        for ev in &trail.events {
            if ev.act != "confirm-rewording" {
                continue;
            }
            let (Some(rec), Some(from), Some(to)) = (ev.about.clone(), ev.str("from-version"), ev.str("to-version")) else { continue };
            let rec = rec.split('#').next().unwrap_or(&rec).to_string();
            let scoped_ids: Option<Vec<String>> = match ev.get("scope") {
                Some(Value::Mapping(m)) => m.get(Value::String("for".into())).and_then(|v| v.as_sequence()).map(|ids| ids.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()),
                _ => None,
            };
            let equivalent = ev.str("outcome") == Some("equivalent");
            match (equivalent, scoped_ids) {
                (true, None) => eq.union(&rec, from, to),
                (true, Some(ids)) => {
                    for id in ids {
                        eq.scoped.entry(id).or_default().push((rec.clone(), from.to_string(), to.to_string()));
                    }
                }
                (false, None) => eq.refused.push((rec, from.to_string(), to.to_string(), ev.actor.name.clone())),
                (false, Some(ids)) => {
                    for id in ids {
                        eq.refused_scoped.entry(id).or_default().push((rec.clone(), from.to_string(), to.to_string()));
                    }
                }
            }
        }
        eq
    }
    /// Has a confirmer ended this check's claim on this pair (not equivalent, for all or for it)?
    pub fn refused_for(&self, event_id: &str, rec: &str, a: &str, b: &str) -> bool {
        let pair = |r: &str, f: &str, t: &str| r == rec && ((f == a && t == b) || (f == b && t == a));
        self.refused.iter().any(|(r, f, t, _)| pair(r, f, t))
            || self.refused_scoped.get(event_id).map(|v| v.iter().any(|(r, f, t)| pair(r, f, t))).unwrap_or(false)
    }
}

fn verdict(store: &Store, scan: &Scan, about: &str, recorded: &[BasisEntry], event_id: &str, eq: &Equivalence, read_set: &[String]) -> Verdict {
    let now = basis_of(store, scan, about);
    let rec_ids: BTreeSet<&str> = recorded.iter().map(|e| e.id.as_str()).collect();
    let now_ids: BTreeSet<&str> = now.iter().map(|e| e.id.as_str()).collect();
    if rec_ids != now_ids {
        // a deleted target shows as a domain change too; name it when it is the subject itself
        if !scan.records.contains_key(about) {
            return Verdict::CannotEvaluate(format!("{about} is not in the store now"));
        }
        return Verdict::BasisChanged;
    }
    let now_map: BTreeMap<&str, &str> = now.iter().map(|e| (e.id.as_str(), e.version.as_str())).collect();
    let mut upstream = vec![];
    let mut own = false;
    for e in recorded {
        let cur = now_map.get(e.id.as_str()).copied().unwrap_or("unwritten");
        if cur == "unwritten" && e.version != "unwritten" {
            return Verdict::CannotEvaluate(format!("{} is not in the store now", e.id));
        }
        if cur != e.version && !eq.same(&e.id, &e.version, cur) && !eq.scoped_covers(event_id, &e.id, &e.version, cur) {
            if e.id == about {
                own = true;
            } else {
                upstream.push(e.id.clone());
            }
        }
    }
    if own {
        Verdict::StaleOwn
    } else if !upstream.is_empty() {
        // a check that declared its read set (core §5.1 `actor.read`) is lapsed strictly only by a
        // change to something it read; a change elsewhere in the basis is the lighter state
        if !read_set.is_empty() && !upstream.iter().any(|u| read_set.iter().any(|r| r == u || r.trim_start_matches('#') == u)) {
            Verdict::StaleUpstreamUnread(upstream)
        } else {
            Verdict::StaleUpstream(upstream)
        }
    } else {
        Verdict::Holds
    }
}

// `read` is accepted when reading the trail for the five events written under that word before
// take normalized acts (2026-10-30); nothing writes it any more.
const SECURING: &[&str] = &["rederive", "instantiate", "run", "read-check", "assert", "decline", "read"];
const LIFECYCLE: &[&str] = &["mint", "revise", "retire", "supersede", "split", "merge", "rename"];

impl State {
    pub fn compute(store: &Store) -> State {
        let scan = record::scan(store);
        let trail = event::read_all(store);
        let mut records: BTreeMap<String, RecordState> = BTreeMap::new();
        for (id, r) in &scan.records {
            records.entry(id.clone()).or_default().exists = true;
            records.get_mut(id).unwrap().current_version = Some(r.own_version.clone());
        }
        // reverse edges and dangling targets
        let mut reverse: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
        let mut dangling = vec![];
        for (id, r) in &scan.records {
            for (edge, targets) in &r.edges {
                for t in targets {
                    let tid = resolve_target(store, &r.id.kind, t);
                    reverse.entry(tid.clone()).or_default().push((edge.clone(), id.clone()));
                    if !scan.records.contains_key(&tid) {
                        dangling.push((id.clone(), edge.clone(), tid));
                    }
                }
            }
        }
        // lifecycle, refutation
        let record_of = |about: &str| about.split('#').next().unwrap_or(about).to_string();
        for ev in &trail.events {
            let Some(about) = &ev.about else { continue };
            let rid = record_of(about);
            let rs = records.entry(rid.clone()).or_default();
            if let Some(v) = ev.str("own-version").or(ev.str("target-version")) {
                rs.seen_version = Some(v.to_string());
            }
            if LIFECYCLE.contains(&ev.act.as_str()) {
                if ev.act == "mint" {
                    rs.atom = Some(ev.id.clone());
                }
                if let Some(v) = ev.str("own-version") {
                    rs.last_version = Some(v.to_string());
                }
                if ev.act == "retire" {
                    rs.retired = true;
                }
                if ev.act == "supersede" {
                    if let Some(into) = ev.str("into") {
                        rs.superseded_by = Some(into.to_string());
                    }
                }
                rs.last_lifecycle = Some(ev.clone());
                rs.lifecycle.push(ev.clone());
            }
            if ev.act == "refute" {
                rs.refuted = Some(ev.clone());
            }
            if ev.act == "contest" {
                // a contested refutation shows both; kept on the refute event's side
            }
        }
        // securing events with verdicts, under the confirmed equivalences
        let eq = Equivalence::from_trail(&trail);
        for ev in &trail.events {
            if !SECURING.contains(&ev.act.as_str()) {
                continue;
            }
            let Some(about) = &ev.about else { continue };
            let rid = record_of(about);
            let recorded = basis_from_yaml(ev.get("basis"));
            let v = verdict(store, &scan, &rid, &recorded, &ev.id, &eq, &ev.actor.read);
            records.entry(rid).or_default().securing.push((ev.clone(), v));
        }
        // planned targets: slugs the store declares as planned (e.g. OUTLINE `missing` rows)
        let planned_slugs = store.planned_slugs();
        let (planned, dangling): (Vec<_>, Vec<_>) = dangling.into_iter().partition(|(_, _, tgt)| {
            let slug = tgt.split_once(':').map(|(_, s)| s).unwrap_or(tgt);
            planned_slugs.contains(slug)
        });
        // broken: a live record whose `broken`-on-refute edge targets a refuted, unreplaced record
        let refuted_live: BTreeSet<String> = records
            .iter()
            .filter(|(_, s)| s.refuted.is_some() && s.superseded_by.is_none() && !s.retired)
            .map(|(id, _)| id.clone())
            .collect();
        let breaking: BTreeSet<&str> = store
            .kinds
            .edges
            .iter()
            .filter(|(_, d)| d.on_refute == "broken")
            .map(|(k, _)| k.as_str())
            .collect();
        let mut broken: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut queue: VecDeque<String> = refuted_live.iter().cloned().collect();
        let mut seen = BTreeSet::new();
        while let Some(t) = queue.pop_front() {
            if !seen.insert(t.clone()) {
                continue;
            }
            if let Some(srcs) = reverse.get(&t) {
                for (edge, src) in srcs {
                    if breaking.contains(edge.as_str()) {
                        broken.entry(src.clone()).or_default().push(t.clone());
                        queue.push_back(src.clone());
                    }
                }
            }
        }
        for (id, by) in broken {
            records.entry(id).or_default().broken_by = by;
        }
        // findings
        let mut placed: BTreeMap<String, Event> = BTreeMap::new();
        let mut closed: BTreeSet<String> = BTreeSet::new();
        for ev in &trail.events {
            let item = ev.str("finding").map(|s| s.to_string());
            match ev.act.as_str() {
                "place-finding" => {
                    if let Some(i) = item {
                        placed.insert(i, ev.clone());
                    }
                }
                "dispose-finding" => {
                    if let Some(i) = item {
                        closed.insert(i);
                    }
                }
                "review-finding" => {
                    if ev.str("outcome") == Some("answered") {
                        if let Some(i) = item {
                            closed.insert(i);
                        }
                    }
                }
                _ => {}
            }
        }
        let findings_closed = placed.keys().filter(|k| closed.contains(*k)).count();
        let mut findings_open: Vec<Finding> = placed
            .iter()
            .filter(|(k, _)| !closed.contains(*k))
            .map(|(k, ev)| Finding {
                item: k.clone(),
                target: ev.str("target").unwrap_or("(unplaced)").to_string(),
                at: ev.at.clone(),
                by: ev.actor.name.clone(),
                text: ev.str("text").unwrap_or("").to_string(),
                placed: ev.clone(),
            })
            .collect();
        findings_open.sort_by(|a, b| a.at.cmp(&b.at));
        // questions
        let answered: BTreeSet<String> = trail
            .events
            .iter()
            .filter(|e| e.act == "answer")
            .filter_map(|e| e.about.clone())
            .collect();
        let mut questions_open = vec![];
        let mut questions_closed = 0;
        for (id, r) in &scan.records {
            if r.id.kind != "question" {
                continue;
            }
            if answered.contains(id) {
                questions_closed += 1;
                continue;
            }
            let fm = &r.frontmatter;
            let s = |k: &str| fm.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let list = |k: &str| match fm.get(k) {
                Some(Value::Sequence(v)) => v.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect(),
                Some(Value::String(s)) => vec![s.clone()],
                _ => vec![],
            };
            let title = r
                .body
                .lines()
                .find(|l| l.starts_with("# "))
                .map(|l| l[2..].trim().to_string())
                .unwrap_or_else(|| r.id.slug.clone());
            questions_open.push(Question {
                id: id.clone(),
                closer: if s("closer").is_empty() { "anyone".into() } else { s("closer") },
                asked: s("asked"),
                by: s("asked-by"),
                title,
                concerns: list("concerns"),
                blocks: list("blocks"),
            });
        }
        questions_open.sort_by(|a, b| a.asked.cmp(&b.asked));
        // items: closed by a `close` event, or by a `recheck` whose outcome was not `holds`
        let mut item_closed: BTreeSet<String> = BTreeSet::new();
        let mut item_held: BTreeSet<String> = BTreeSet::new();
        for ev in &trail.events {
            let Some(about) = &ev.about else { continue };
            match ev.act.as_str() {
                "close" => {
                    item_closed.insert(about.clone());
                }
                "recheck" => {
                    if ev.str("outcome") == Some("holds") {
                        item_held.insert(about.clone());
                    } else {
                        item_closed.insert(about.clone());
                    }
                }
                _ => {}
            }
        }
        let mut items_open = vec![];
        let mut items_closed = 0;
        for (id, r) in &scan.records {
            if r.id.kind != "item" {
                continue;
            }
            if item_closed.contains(id) {
                items_closed += 1;
                continue;
            }
            let fm = &r.frontmatter;
            let s = |k: &str| fm.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let concerns = match fm.get("concerns") {
                Some(Value::Sequence(v)) => v.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect(),
                Some(Value::String(s)) => vec![s.clone()],
                _ => vec![],
            };
            let title = r.body.lines().find(|l| l.starts_with("# ")).map(|l| l[2..].trim().to_string()).unwrap_or_else(|| r.id.slug.clone());
            let imported = s("provenance") == "imported";
            items_open.push(Item {
                id: id.clone(),
                title,
                concerns,
                owner: s("owner"),
                opened: s("opened"),
                by: s("opened-by"),
                source: s("source"),
                section: s("section"),
                restated: fm.get("restated").and_then(|v| v.as_str()).map(|s| s.to_string()),
                unchecked: imported && !item_held.contains(id),
            });
        }
        items_open.sort_by(|a, b| a.opened.cmp(&b.opened).then(a.id.cmp(&b.id)));
        State {
            scan,
            trail,
            records,
            findings_open,
            findings_closed,
            questions_open,
            questions_closed,
            items_open,
            items_closed,
            reverse,
            dangling,
            planned,
            eq,
        }
    }

    /// Every (record, version read, version now) difference that keeps a check from holding,
    /// with the checks it would re-bind. A confirmation of such a pair by someone other than the
    /// reviser is what `vsect confirm` records.
    pub fn pending_confirms(&self) -> Vec<PendingConfirm> {
        let mut by_pair: BTreeMap<(String, String, String), Vec<(String, String)>> = BTreeMap::new();
        for (id, rs) in &self.records {
            if !rs.live() {
                continue;
            }
            for (ev, v) in &rs.securing {
                if ev.act == "decline" || v.holds() {
                    continue;
                }
                if matches!(ev.str("outcome"), Some("broken") | Some("narrowed")) {
                    continue;
                }
                for e in basis_from_yaml(ev.get("basis")) {
                    let Some(cur) = self.scan.records.get(&e.id).map(|r| r.own_version.clone()) else { continue };
                    if cur == e.version || e.version == "unwritten" {
                        continue;
                    }
                    if self.eq.same(&e.id, &e.version, &cur) || self.eq.scoped_covers(&ev.id, &e.id, &e.version, &cur) {
                        continue;
                    }
                    if self.eq.refused_for(&ev.id, &e.id, &e.version, &cur) {
                        continue; // a confirmer ended this check's claim on the pair: it is redone, not re-bound
                    }
                    by_pair.entry((e.id.clone(), e.version.clone(), cur)).or_default().push((ev.id.clone(), id.clone()));
                }
            }
        }
        let read_sets: BTreeMap<String, Vec<String>> = self.trail.events.iter().map(|e| (e.id.clone(), e.actor.read.clone())).collect();
        by_pair
            .into_iter()
            .map(|((record, from, to), checks)| {
                let unread = checks.iter().filter(|(eid, _)| read_sets.get(eid).map(|r| !r.is_empty() && !r.iter().any(|x| x == &record || x.trim_start_matches('#') == record)).unwrap_or(false)).map(|(eid, _)| eid.clone()).collect();
                PendingConfirm { record, from, to, checks, unread }
            })
            .collect()
    }

    /// Lapsed checks that no confirmation can cure: every pair that lapsed them was judged not
    /// equivalent (for all checks, or for this one). These are redone, not re-bound.
    pub fn ended_checks(&self) -> Vec<(String, &Event)> {
        let mut out = vec![];
        for (id, rs) in &self.records {
            if !rs.live() {
                continue;
            }
            for (ev, v) in &rs.securing {
                if ev.act == "decline" || v.holds() || matches!(ev.str("outcome"), Some("broken") | Some("narrowed")) {
                    continue;
                }
                let mut any_pair = false;
                let mut all_refused = true;
                for e in basis_from_yaml(ev.get("basis")) {
                    let Some(cur) = self.scan.records.get(&e.id).map(|r| r.own_version.clone()) else { continue };
                    if cur == e.version || e.version == "unwritten" || self.eq.same(&e.id, &e.version, &cur) || self.eq.scoped_covers(&ev.id, &e.id, &e.version, &cur) {
                        continue;
                    }
                    any_pair = true;
                    if !self.eq.refused_for(&ev.id, &e.id, &e.version, &cur) {
                        all_refused = false;
                    }
                }
                if any_pair && all_refused {
                    out.push((id.clone(), ev));
                }
            }
        }
        out
    }

    /// Rewordings a check rests on that no `revise` event has classified (reworded/redefined, by
    /// whom). These are what `vsect revise` walks; everything else about a record's text is the
    /// tree's to say.
    pub fn unclassified_rewordings(&self) -> Vec<PendingConfirm> {
        self.pending_confirms().into_iter().filter(|p| self.revisers_between(&p.record, &p.from, &p.to).is_empty()).collect()
    }

    /// Who revised `record` between `from` and `to` (actor names), from its lifecycle trail.
    pub fn revisers_between(&self, record: &str, from: &str, to: &str) -> Vec<String> {
        let Some(rs) = self.records.get(record) else { return vec![] };
        // Follow the version chain (from-version → own-version) rather than event order: two
        // revisions in one second sort by id, and order would miss one.
        let mut next: BTreeMap<&str, Vec<(&str, &str)>> = BTreeMap::new(); // from -> [(own, actor)]
        for ev in &rs.lifecycle {
            if ev.act == "revise" {
                if let (Some(f), Some(o)) = (ev.str("from-version"), ev.str("own-version")) {
                    next.entry(f).or_default().push((o, ev.actor.name.as_str()));
                }
            }
        }
        let mut names: Vec<String> = vec![];
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let mut stack: Vec<&str> = vec![from];
        while let Some(cur) = stack.pop() {
            if cur == to || !seen.insert(cur) {
                continue;
            }
            if let Some(outs) = next.get(cur) {
                for (own, actor) in outs {
                    if !names.iter().any(|n| n == actor) {
                        names.push(actor.to_string());
                    }
                    stack.push(own);
                }
            }
        }
        // if `to` was never reached the chain is broken or branched; the names collected are
        // still everyone who revised away from `from`, which is the conservative set to refuse
        names
    }

    pub fn record(&self, id: &str) -> Option<&Record> {
        self.scan.records.get(id)
    }

    pub fn rs(&self, id: &str) -> Option<&RecordState> {
        self.records.get(id)
    }

    /// Live records of a kind (declared, not built-in process kinds).
    pub fn live_of_kind(&self, kind: &str) -> Vec<&Record> {
        self.scan.records.values().filter(|r| r.id.kind == kind).collect()
    }

    pub fn events_about(&self, id: &str) -> Vec<&Event> {
        let rid = id.split('#').next().unwrap_or(id);
        self.trail
            .events
            .iter()
            .filter(|e| e.about.as_deref().map(|a| a.split('#').next().unwrap_or(a) == rid).unwrap_or(false))
            .collect()
    }

    /// Everything that rests on `id`, transitively over declared edges (reverse walk).
    pub fn dependents(&self, id: &str) -> Vec<(String, String, usize)> {
        let mut out = vec![];
        let mut seen = BTreeSet::new();
        let mut queue: VecDeque<(String, usize)> = VecDeque::new();
        queue.push_back((id.to_string(), 0));
        while let Some((cur, depth)) = queue.pop_front() {
            if let Some(srcs) = self.reverse.get(&cur) {
                for (edge, src) in srcs {
                    if seen.insert(src.clone()) {
                        out.push((src.clone(), edge.clone(), depth + 1));
                        queue.push_back((src.clone(), depth + 1));
                    }
                }
            }
        }
        out
    }

    /// Checks that no longer hold, on live records only: a check that broke a statement which has
    /// since been retired or superseded did its work and is not waiting on anyone.
    pub fn lapsed(&self) -> Vec<(String, &Event, &Verdict)> {
        let mut v = vec![];
        for (id, rs) in &self.records {
            if !rs.live() {
                continue;
            }
            for (ev, verdict) in &rs.securing {
                // a check that broke or narrowed the statement did its work; the finding it
                // opened is the waiting item, not the check
                let broke = matches!(ev.str("outcome"), Some("broken") | Some("narrowed"));
                if !verdict.holds() && ev.act != "decline" && !broke {
                    v.push((id.clone(), ev, verdict));
                }
            }
        }
        v
    }

    /// Refutations recorded, and how many of those statements have since been replaced.
    pub fn refutations(&self) -> (usize, usize) {
        let n = self.records.values().filter(|s| s.refuted.is_some()).count();
        let replaced = self.records.values().filter(|s| s.refuted.is_some() && (s.superseded_by.is_some() || s.retired)).count();
        (n, replaced)
    }

    pub fn unreconciled(&self) -> Vec<(String, &'static str)> {
        let mut v: Vec<(String, &'static str)> = self
            .records
            .iter()
            .filter_map(|(id, s)| s.unreconciled().map(|why| (id.clone(), why)))
            .collect();
        v.sort();
        v
    }

    pub fn age_days(&self, at: &str) -> String {
        match chrono::DateTime::parse_from_rfc3339(at) {
            Ok(t) => {
                let d = chrono::Utc::now().signed_duration_since(t.with_timezone(&chrono::Utc));
                let days = d.num_days();
                if days <= 0 {
                    let h = d.num_hours();
                    if h <= 0 { "now".into() } else { format!("{h}h") }
                } else {
                    format!("{days}d")
                }
            }
            Err(_) => "?".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eq_with(scoped: &[(&str, &str, &str, &str)], refused_scoped: &[(&str, &str, &str, &str)], unconditional: &[(&str, &str, &str)]) -> Equivalence {
        let mut eq = Equivalence::default();
        for (ev, rec, a, b) in scoped {
            eq.scoped.entry(ev.to_string()).or_default().push((rec.to_string(), a.to_string(), b.to_string()));
        }
        for (ev, rec, a, b) in refused_scoped {
            eq.refused_scoped.entry(ev.to_string()).or_default().push((rec.to_string(), a.to_string(), b.to_string()));
        }
        for (rec, a, b) in unconditional {
            eq.union(rec, a, b);
        }
        eq
    }

    #[test]
    fn checks_scoping_binds_only_the_named_check() {
        // a `for-checks` on check E1 re-binds E1 and nothing else on the same pair
        let eq = eq_with(&[("E1", "seg:scope", "v1", "v2")], &[], &[]);
        assert!(eq.scoped_covers("E1", "seg:scope", "v1", "v2"));
        assert!(eq.scoped_covers("E1", "seg:scope", "v2", "v1"), "scope is symmetric in direction");
        assert!(!eq.scoped_covers("E2", "seg:scope", "v1", "v2"), "an unnamed check stays pending");
        assert!(!eq.same("seg:scope", "v1", "v2"), "a scoped confirmation is not an unconditional one");
    }

    #[test]
    fn checks_scoping_ends_only_the_named_check() {
        // a `not-equivalent --checks E1` ends E1 on the pair and leaves E2 to its own judgment
        let eq = eq_with(&[("E2", "seg:scope", "v1", "v2")], &[("E1", "seg:scope", "v1", "v2")], &[]);
        assert!(eq.refused_for("E1", "seg:scope", "v1", "v2"));
        assert!(!eq.refused_for("E2", "seg:scope", "v1", "v2"));
        assert!(eq.scoped_covers("E2", "seg:scope", "v1", "v2"));
    }

    #[test]
    fn unconditional_confirmations_chain() {
        let eq = eq_with(&[], &[], &[("seg:a", "v1", "v2"), ("seg:a", "v2", "v3")]);
        assert!(eq.same("seg:a", "v1", "v3"), "a→b and b→c confirmed means a check that read a holds at c");
        assert!(!eq.same("seg:b", "v1", "v3"), "equivalence is per record");
    }
}
