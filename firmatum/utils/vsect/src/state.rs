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
            Verdict::BasisChanged => "stale: what it rests on was re-declared".into(),
            Verdict::CannotEvaluate(why) => format!("cannot evaluate: {why}"),
        }
    }
    pub fn holds(&self) -> bool {
        matches!(self, Verdict::Holds)
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

#[derive(Debug, Default, Clone)]
pub struct RecordState {
    pub exists: bool,
    pub atom: Option<String>,
    pub last_lifecycle: Option<Event>,
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
    pub fn unreconciled(&self) -> Option<&'static str> {
        match (self.exists, &self.last_version, &self.current_version) {
            (true, None, _) => Some("new: not yet minted"),
            (true, Some(l), Some(c)) if l != c => Some("edited since last taken in"),
            (false, _, _)
                if !self.retired
                    && self.superseded_by.is_none()
                    && (self.last_version.is_some() || self.seen_version.is_some()) =>
            {
                Some("deleted: fate not recorded")
            }
            _ => None,
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
    /// Reverse of every declared edge: target id -> (edge, source id)
    pub reverse: BTreeMap<String, Vec<(String, String)>>,
    /// Declared targets that resolve to no record: (source id, edge, target)
    pub dangling: Vec<(String, String, String)>,
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

fn verdict(store: &Store, scan: &Scan, about: &str, recorded: &[BasisEntry]) -> Verdict {
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
        if cur != e.version {
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
        Verdict::StaleUpstream(upstream)
    } else {
        Verdict::Holds
    }
}

const SECURING: &[&str] = &["rederive", "instantiate", "run", "read-check", "assert", "decline"];
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
            }
            if ev.act == "refute" {
                rs.refuted = Some(ev.clone());
            }
            if ev.act == "contest" {
                // a contested refutation shows both; kept on the refute event's side
            }
        }
        // securing events with verdicts
        for ev in &trail.events {
            if !SECURING.contains(&ev.act.as_str()) {
                continue;
            }
            let Some(about) = &ev.about else { continue };
            let rid = record_of(about);
            let recorded = basis_from_yaml(ev.get("basis"));
            let v = verdict(store, &scan, &rid, &recorded);
            records.entry(rid).or_default().securing.push((ev.clone(), v));
        }
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
        State {
            scan,
            trail,
            records,
            findings_open,
            findings_closed,
            questions_open,
            questions_closed,
            reverse,
            dangling,
        }
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
                if !verdict.holds() && ev.act != "decline" {
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
