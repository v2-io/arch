//! The verbs. Each is something the work already does; recording is a side effect of doing it.

use crate::event::{self, Actor, Event};
use crate::norm;
use crate::record::{self, Id, Record};
use crate::state::{self, BasisEntry, State};
use crate::store::{self, Store};
use crate::{Cli, Cmd};
use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use serde_json::json;
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{IsTerminal, Write};

const EXIT_OK: i32 = 0;
const EXIT_ITEMS: i32 = 1;
const EXIT_NOT_RUN: i32 = 3;

pub fn run(cli: Cli) -> Result<i32> {
    let cwd = std::env::current_dir()?;
    let answers: BTreeMap<String, String> = cli
        .answer
        .iter()
        .filter_map(|a| a.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())))
        .collect();
    let json = cli.format.as_deref() == Some("json");
    let dry = cli.dry_run;
    let actor_args = cli.actor.clone();

    let Some(cmd) = cli.cmd else {
        let store = Store::find(&cwd)?;
        return dashboard(&store, json);
    };
    if let Cmd::Init { name, steward, decider } = &cmd {
        return init(&cwd, name, steward, decider, dry);
    }
    if let Cmd::Help { topic } = &cmd {
        return help(topic.as_deref());
    }
    let store = Store::find(&cwd)?;
    match cmd {
        Cmd::Revise { record } => {
            let actor = actor(&actor_args, "adopter")?;
            revise(&store, record.as_deref(), actor, &answers, dry)
        }
        Cmd::Take { path, check, act, line, criterion, committed, outcome, recheck, at, source, slug, note } => {
            let actor = actor(&actor_args, "verifier")?;
            match (path, check) {
                (Some(p), None) => take(&store, &p, actor, dry),
                (None, Some(target)) => {
                    let criterion = criterion.ok_or_else(|| anyhow!("--criterion: what did the check test?"))?;
                    let outcome = outcome.ok_or_else(|| anyhow!("--outcome: confirmed | broken | narrowed | inconclusive | failed-to-run"))?;
                    take_inline(
                        &store, &target, &act, &line, &criterion, &committed, &outcome, recheck.as_deref(),
                        at.as_deref(), &source, slug.as_deref(), note.as_deref(), actor, dry,
                    )
                }
                _ => bail!("give a review path, or --check <target> with --criterion and --outcome"),
            }
        }
        Cmd::Ask { text, concerns, blocks, closer, why, slug } => {
            let actor = actor(&actor_args, "author")?;
            ask(&store, &text, &concerns, &blocks, &closer, why.as_deref(), slug.as_deref(), actor, dry)
        }
        Cmd::Answer { question, outcome, text, reopen_if, superseded_by } => {
            let actor = actor(&actor_args, "decider")?;
            answer(&store, &question, &outcome, text.as_deref(), reopen_if.as_deref(), superseded_by.as_deref(), actor, dry)
        }
        Cmd::Open { target } => open(&store, target.as_deref(), json),
        Cmd::Findings { dispose, r#as, why, answered, by_revision } => match (dispose, answered) {
            (Some(item), _) => {
                let actor = actor(&actor_args, "integrator")?;
                let disp = r#as.ok_or_else(|| anyhow!("--as rejected | known-defect"))?;
                let why = why.ok_or_else(|| anyhow!("--why: a finding is never closed without a reason"))?;
                dispose_finding(&store, &item, &disp, &why, actor, dry)
            }
            (None, Some(item)) => {
                let actor = actor(&actor_args, "reviewer")?;
                let rev = by_revision.ok_or_else(|| anyhow!("--by-revision: the version, event or record that answers it"))?;
                let why = why.ok_or_else(|| anyhow!("--why: say how the revision answers it"))?;
                review_finding(&store, &item, &rev, &why, actor, dry)
            }
            (None, None) => open(&store, None, json),
        },
        Cmd::Confirm { record, from, to, judgment, criterion } => match record {
            None => confirm_list(&store),
            Some(r) => {
                let actor = actor(&actor_args, "second-mind")?;
                confirm(&store, &r, from.as_deref(), to.as_deref(), judgment.as_deref(), criterion.as_deref(), actor, &answers, dry)
            }
        },
        Cmd::Refute { target, ground, by_item, at } => {
            let actor = actor(&actor_args, "verifier")?;
            refute(&store, &target, &ground, by_item.as_deref(), at.as_deref(), actor, dry)
        }
        Cmd::Check { strict } => check(&store, strict, json),
        Cmd::Read { id, at, bare } => read(&store, &id, at.as_deref(), bare),
        Cmd::Why { id } => why(&store, &id),
        Cmd::Impact { id } => impact(&store, &id),
        Cmd::Kinds => kinds(&store),
        Cmd::Init { .. } | Cmd::Help { .. } => unreachable!(),
    }
}

// ---------- helpers ----------

fn actor(a: &crate::ActorArgs, default_role: &str) -> Result<Actor> {
    let name = a
        .by
        .clone()
        .or_else(|| std::env::var("VSECT_BY").ok())
        .filter(|s| !s.is_empty());
    let Some(name) = name else {
        eprintln!("Who is acting? Give --by <name> (or set VSECT_BY). It is recorded as your claim.");
        std::process::exit(EXIT_NOT_RUN);
    };
    let role = a
        .role
        .clone()
        .or_else(|| std::env::var("VSECT_ROLE").ok())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| default_role.to_string());
    let session = a
        .session
        .clone()
        .or_else(|| std::env::var("VSECT_SESSION").ok())
        .filter(|s| !s.is_empty());
    Ok(Actor { name, role: Some(role), session, read: vec![] })
}

/// Ask a question: answered in advance by `--answer key=…`, at a terminal by a prompt, otherwise
/// not at all (the caller stops with exit 3 and the question printed).
fn ask_q(answers: &BTreeMap<String, String>, key: &str, question: &str, options: &[&str]) -> Option<String> {
    if let Some(a) = answers.get(key) {
        return Some(a.clone());
    }
    if std::io::stdin().is_terminal() {
        print!("{question} [{}] ", options.join(" / "));
        std::io::stdout().flush().ok();
        let mut s = String::new();
        if std::io::stdin().read_line(&mut s).is_ok() {
            let s = s.trim().to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
        return None;
    }
    eprintln!("{question}\n  answer in advance with --answer {key}=<{}>", options.join("|"));
    None
}

fn yaml_str(s: &str) -> Value {
    Value::String(s.to_string())
}

fn yaml_list(v: &[String]) -> Value {
    Value::Sequence(v.iter().map(|s| yaml_str(s)).collect())
}

fn declarations(r: &Record) -> Value {
    let mut m = serde_yaml::Mapping::new();
    for (k, v) in &r.frontmatter {
        if k == "slug" {
            continue;
        }
        m.insert(yaml_str(k), v.clone());
    }
    Value::Mapping(m)
}

fn today() -> String {
    chrono::Utc::now().format("%Y-%m-%d").to_string()
}

fn short(v: &str) -> String {
    norm::short(v)
}

/// Load the records in `id`'s basis as they were at a git revision.
fn basis_at(store: &Store, id: &Id, rev: &str) -> Result<(Record, Vec<BasisEntry>)> {
    let meaning: BTreeSet<String> = store.kinds.edges.iter().filter(|(_, d)| d.meaning || d.in_basis).map(|(k, _)| k.clone()).collect();
    let mut loaded: BTreeMap<String, Record> = BTreeMap::new();
    let mut queue = vec![id.clone()];
    let mut first: Option<Record> = None;
    while let Some(cur) = queue.pop() {
        let rid = cur.record_id();
        if loaded.contains_key(&rid) {
            continue;
        }
        let Some(path) = record::path_at(store, &cur, rev)? else {
            loaded.insert(rid.clone(), Record {
                id: cur.clone(), rel: String::new(), frontmatter: BTreeMap::new(), body: String::new(),
                text: String::new(), own_version: "unwritten".into(), edges: BTreeMap::new(),
            });
            continue;
        };
        let r = record::record_at(store, &cur.kind, &path, rev)?;
        for (edge, targets) in &r.edges {
            if meaning.contains(edge) {
                for t in targets {
                    let tid = state::resolve_target(store, &r.id.kind, t);
                    if let Ok(parsed) = Id::parse(store, &tid) {
                        queue.push(parsed);
                    }
                }
            }
        }
        if first.is_none() {
            first = Some(r.clone());
        }
        loaded.insert(rid, r);
    }
    let first = first.ok_or_else(|| anyhow!("{id} is not at {rev}"))?;
    let mut basis: Vec<BasisEntry> = loaded.values().map(|r| BasisEntry { id: r.id.record_id(), version: r.own_version.clone() }).collect();
    basis.sort_by(|a, b| a.id.cmp(&b.id));
    Ok((first, basis))
}

// ---------- init ----------

fn init(cwd: &std::path::Path, name: &str, steward: &str, decider: &str, dry: bool) -> Result<i32> {
    let dir = cwd.join(".vsect");
    if dir.exists() {
        bail!("a store already exists here ({})", dir.display());
    }
    let text = store::founding_kinds(name, steward, decider);
    if dry {
        println!("would write {}:\n{text}", dir.join("kinds.yaml").display());
        return Ok(EXIT_OK);
    }
    std::fs::create_dir_all(dir.join("events"))?;
    std::fs::create_dir_all(dir.join("objects"))?;
    std::fs::write(dir.join("kinds.yaml"), text)?;
    println!("Founded store \"{name}\".");
    println!("  .vsect/kinds.yaml   name, steward, decider — no kinds of the store's own declared");
    println!("  Declare a record kind there (grain, find globs, edge fields) and run `vsect revise` to take the records in.");
    Ok(EXIT_OK)
}

// ---------- dashboard ----------

fn dashboard(store: &Store, json: bool) -> Result<i32> {
    let st = State::compute(store);
    let moment = store.moment();
    let domain_kinds: Vec<String> = store.kinds.kinds.keys().cloned().collect();
    let n_records: usize = domain_kinds.iter().map(|k| st.live_of_kind(k).len()).sum();

    // known broken
    let mut broken: Vec<(String, String)> = vec![];
    for (id, rs) in &st.records {
        if let Some(ev) = &rs.refuted {
            if rs.superseded_by.is_none() && !rs.retired {
                let g = ev.get("ground").and_then(|g| g.get("kind")).and_then(|v| v.as_str()).unwrap_or("evidence");
                let deps = st.dependents(id).len();
                broken.push((id.clone(), format!("refuted by {g} ({}); {deps} statements rest on it", st.age_days(&ev.at))));
            }
        } else if !rs.broken_by.is_empty() && rs.live() {
            broken.push((id.clone(), format!("broken: rests on refuted {}", rs.broken_by.join(", "))));
        }
    }
    // waiting
    let unrec = st.unreconciled();
    let stale: Vec<(String, String, String)> = st.lapsed().into_iter().map(|(id, ev, v)| (id, ev.at.clone(), v.word())).collect();
    let (n_refuted, n_replaced) = st.refutations();
    let steward_q: Vec<&state::Question> = st.questions_open.iter().filter(|q| q.closer.starts_with("steward")).collect();
    let other_q: Vec<&state::Question> = st.questions_open.iter().filter(|q| !q.closer.starts_with("steward")).collect();

    // evidence
    let mut by_line: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut checked_records: BTreeSet<String> = BTreeSet::new();
    let mut holding_records: BTreeSet<String> = BTreeSet::new();
    for (id, rs) in &st.records {
        for (ev, v) in &rs.securing {
            if ev.act == "decline" {
                continue;
            }
            let line = ev.str("line").unwrap_or("?").to_string();
            let outcome = ev.str("outcome").unwrap_or("?").to_string();
            *by_line.entry(line).or_default().entry(outcome).or_default() += 1;
            checked_records.insert(id.clone());
            if v.holds() {
                holding_records.insert(id.clone());
            }
        }
    }
    // declared status words (shown raw, never computed)
    let mut declared: BTreeMap<String, (usize, usize)> = BTreeMap::new(); // status -> (n, with holding check)
    for k in &domain_kinds {
        for r in st.live_of_kind(k) {
            let s = r.frontmatter.get("status").and_then(|v| v.as_str()).unwrap_or("(none)").to_string();
            let e = declared.entry(s).or_default();
            e.0 += 1;
            if holding_records.contains(&r.id.record_id()) {
                e.1 += 1;
            }
        }
    }
    let oldest = |ats: Vec<&str>| -> String {
        ats.iter().min().map(|a| st.age_days(a)).unwrap_or_default()
    };

    if json {
        let out = json!({
            "schema": "vsect-dashboard/0",
            "store": store.kinds.name, "moment": moment, "records": n_records,
            "broken": broken.iter().map(|(id, why)| json!({"id": id, "why": why})).collect::<Vec<_>>(),
            "waiting": {
                "findings_open": st.findings_open.len(),
                "questions_open": st.questions_open.len(),
                "questions_for_steward": steward_q.len(),
                "edits_not_taken_in": unrec.len(),
                "checks_stale": stale.len(),
            },
            "evidence": { "by_line": by_line, "records_with_a_check": checked_records.len(),
                          "records_with_a_holding_check": holding_records.len(),
                          "declared_status": declared.iter().map(|(k, (n, h))| json!({"status": k, "records": n, "with_holding_check": h})).collect::<Vec<_>>() },
            "events": st.trail.events.len(),
            "trail_problems": st.trail.problems.len(),
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(EXIT_OK);
    }

    let kind_word = if domain_kinds.is_empty() { "no record kinds declared".to_string() } else { domain_kinds.join("+") };
    println!("{} · {} store · {} records · {} events · computed as of {}", store.kinds.name, kind_word, n_records, st.trail.events.len(), moment);
    println!();
    println!("Known broken");
    if broken.is_empty() {
        println!("  (none recorded)");
    }
    for (id, why) in &broken {
        println!("  ⚠ {id} — {why}");
    }
    println!();
    println!("Waiting, oldest first");
    let mut any = false;
    if !st.findings_open.is_empty() {
        any = true;
        println!("  {:>4} findings not yet disposed of     oldest {:<5} → vsect open", st.findings_open.len(), oldest(st.findings_open.iter().map(|f| f.at.as_str()).collect()));
    }
    if !steward_q.is_empty() {
        any = true;
        println!("  {:>4} questions for the steward        oldest {:<5} → vsect open", steward_q.len(), oldest(steward_q.iter().map(|q| q.asked.as_str()).collect()));
    }
    if !other_q.is_empty() {
        any = true;
        println!("  {:>4} questions open                   oldest {:<5} → vsect open", other_q.len(), oldest(other_q.iter().map(|q| q.asked.as_str()).collect()));
    }
    let pending = st.pending_confirms();
    if !pending.is_empty() {
        any = true;
        println!("  {:>4} rewordings awaiting a second mind  ({} checks lapsed) → vsect confirm", pending.len(), stale.len());
    } else if !stale.is_empty() {
        any = true;
        println!("  {:>4} checks lapsed, not curable by confirmation  → vsect check --strict", stale.len());
    }
    if !unrec.is_empty() {
        any = true;
        println!("  {:>4} edits not yet taken in                       → vsect revise", unrec.len());
    }
    if !any {
        println!("  (nothing waiting)");
    }
    println!();
    println!("Evidence recorded (raw — this store has not declared how to compute standing)");
    if by_line.is_empty() {
        println!("  no check recorded on any record");
    }
    for (line, outs) in &by_line {
        let parts: Vec<String> = outs.iter().map(|(o, n)| format!("{o} {n}")).collect();
        println!("  {:<12} {}", line, parts.join(" · "));
    }
    println!("  records with a check recorded {} · holding now {} · without any {}", checked_records.len(), holding_records.len(), n_records.saturating_sub(checked_records.len()));
    if n_refuted > 0 {
        println!("  refutations recorded {n_refuted} · of which replaced or retired {n_replaced}");
    }
    if !declared.is_empty() {
        let parts: Vec<String> = declared.iter().map(|(s, (n, h))| format!("{s} {n} (checked {h})")).collect();
        println!("  declared `status:` words, as typed by their authors — not standing:");
        println!("    {}", parts.join(" · "));
    }
    if !st.trail.problems.is_empty() || !st.dangling.is_empty() || !st.scan.collisions.is_empty() {
        println!();
        println!("Structural: {} trail problems · {} dangling targets · {} collisions → vsect check", st.trail.problems.len(), st.dangling.len(), st.scan.collisions.len());
    }
    println!();
    println!("Read:     vsect read <kind:slug>  ·  vsect why <kind:slug>  ·  vsect open [<kind:slug>]");
    println!("Work:     vsect revise · take · ask · answer · refute · findings · check");
    println!("Help:     vsect help <verb>   ·  vsect help model   ·  vsect kinds");
    Ok(EXIT_OK)
}

// ---------- revise ----------

fn revise(store: &Store, one: Option<&str>, actor: Actor, answers: &BTreeMap<String, String>, dry: bool) -> Result<i32> {
    let st = State::compute(store);
    let only: Option<String> = match one {
        Some(s) => Some(Id::parse(store, s)?.record_id()),
        None => None,
    };
    let todo: Vec<(String, &'static str)> = st.unreconciled().into_iter().filter(|(id, _)| only.as_ref().map(|o| o == id).unwrap_or(true)).collect();
    if todo.is_empty() {
        match &only {
            Some(o) if !st.records.contains_key(o) => bail!("{o} is not in the store"),
            _ => println!("Nothing to take in."),
        }
        return Ok(EXIT_OK);
    }
    let n = todo.len();
    let mut minted = 0usize;
    let mut revised = 0usize;
    let mut settled = 0usize;
    for (id, why) in todo {
        let rs = st.rs(&id).cloned().unwrap_or_default();
        let parsed = Id::parse(store, &id)?;
        match why {
            "new: not yet minted" => {
                let r = st.record(&id).unwrap();
                let form = r.frontmatter.get("form").and_then(|v| v.as_str()).unwrap_or("");
                event::write_object(store, &r.id.kind, form, &r.text, dry)?;
                let mut ev = Event::new(store, "mint", Some(&id), actor.clone())
                    .with("own-version", yaml_str(&r.own_version))
                    .with("record", yaml_str(&r.rel))
                    .with("declarations", declarations(r));
                let p = event::write_event(store, &mut ev, dry)?;
                minted += 1;
                if n <= 12 || dry {
                    println!("  • {id} — minted at v:{} {}", short(&r.own_version), if dry { format!("(would write {})", p.display()) } else { String::new() });
                }
            }
            "edited since last taken in" => {
                let r = st.record(&id).unwrap();
                let form = r.frontmatter.get("form").and_then(|v| v.as_str()).unwrap_or("");
                event::write_object(store, &r.id.kind, form, &r.text, dry)?;
                let from = rs.last_version.clone().unwrap_or_default();
                // does any check rest on this record's version?
                let resting: Vec<String> = st
                    .records
                    .iter()
                    .flat_map(|(rid, s)| s.securing.iter().map(move |(ev, _)| (rid.clone(), ev)))
                    .filter(|(_, ev)| state::basis_from_yaml(ev.get("basis")).iter().any(|b| b.id == id))
                    .map(|(rid, _)| rid)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                let change = if resting.is_empty() {
                    "unclassified".to_string()
                } else {
                    println!("  • {id}  v:{} → v:{}", short(&from), short(&r.own_version));
                    println!("      {} check(s) rest on this version: {}", resting.len(), resting.join(", "));
                    match ask_q(answers, "change", "      Did the meaning change, or only the words?", &["reworded", "redefined"]) {
                        Some(a) if a == "reworded" || a == "redefined" => a,
                        Some(a) => bail!("`{a}` is not an answer: reworded or redefined"),
                        None => return Ok(EXIT_NOT_RUN),
                    }
                };
                let mut ev = Event::new(store, "revise", Some(&id), actor.clone())
                    .with("own-version", yaml_str(&r.own_version))
                    .with("from-version", yaml_str(&from))
                    .with("change", yaml_str(&change))
                    .with("record", yaml_str(&r.rel))
                    .with("declarations", declarations(r));
                if let Some(a) = &rs.atom {
                    ev.atom = Some(a.clone());
                }
                event::write_event(store, &mut ev, dry)?;
                revised += 1;
                if resting.is_empty() {
                    println!("  • {id}  v:{} → v:{}  (no check rests on it; change left unclassified)", short(&from), short(&r.own_version));
                } else if change == "reworded" {
                    println!("      → recorded as your claim. The checks stay with v:{} until a different mind confirms the change didn't touch them.", short(&from));
                } else {
                    println!("      → the checks on it and on everything that rests on it stop counting.");
                }
            }
            "deleted: fate not recorded" => {
                println!("  • {id} — gone from the store. What happened to it?");
                let ans = match ask_q(answers, "fate", "      fate?", &["retired", "superseded:<kind:slug>"]) {
                    Some(a) => a,
                    None => return Ok(EXIT_NOT_RUN),
                };
                let last = rs.last_known_version().unwrap_or_default();
                let mut ev = if ans == "retired" {
                    Event::new(store, "retire", Some(&id), actor.clone()).with("own-version", yaml_str(&last))
                } else if let Some(into) = ans.strip_prefix("superseded:") {
                    let into_id = Id::parse(store, into)?.record_id();
                    if !st.scan.records.contains_key(&into_id) {
                        bail!("{into_id} is not in the store; a successor must exist");
                    }
                    Event::new(store, "supersede", Some(&id), actor.clone())
                        .with("own-version", yaml_str(&last))
                        .with("from", yaml_str(&id))
                        .with("into", yaml_str(&into_id))
                } else {
                    bail!("`{ans}` is not a fate: retired, or superseded:<kind:slug>");
                };
                if let Some(a) = &rs.atom {
                    ev.atom = Some(a.clone());
                }
                event::write_event(store, &mut ev, dry)?;
                settled += 1;
                let citing: Vec<String> = st.dependents(&id).iter().filter(|(_, _, d)| *d == 1).map(|(s, _, _)| s.clone()).collect();
                if !citing.is_empty() {
                    println!("      → {} statements still cite it and should now cite its fate: {}", citing.len(), citing.join(", "));
                }
                let open_here = st.findings_open.iter().filter(|f| f.target == id).count();
                if open_here > 0 {
                    println!("      → {open_here} finding(s) against it stay open until disposed of (vsect findings --dispose)");
                }
            }
            _ => {}
        }
        let _ = parsed;
    }
    println!("{}: {minted} minted · {revised} revised · {settled} fates recorded{}", if dry { "Dry run" } else { "Taken in" }, if dry { " (nothing written)" } else { "" });
    Ok(EXIT_OK)
}

// ---------- take ----------

struct Item {
    id: String,     // "C1"
    kind: String,   // check | finding | equivalence | finding-review
    fields: BTreeMap<String, String>,
    text: String,
}

fn parse_items(body: &str) -> Vec<Item> {
    let re = Regex::new(r"^## ([A-Za-z0-9-]+) \((check|finding|equivalence|finding-review)\)\s*$").unwrap();
    let mut items: Vec<Item> = vec![];
    let mut cur: Option<Item> = None;
    let mut in_fields = false;
    for line in body.lines() {
        if let Some(c) = re.captures(line) {
            if let Some(i) = cur.take() {
                items.push(i);
            }
            cur = Some(Item { id: c[1].to_string(), kind: c[2].to_string(), fields: BTreeMap::new(), text: String::new() });
            in_fields = true;
            continue;
        }
        if line.starts_with("## ") || line.starts_with("# ") {
            if let Some(i) = cur.take() {
                items.push(i);
            }
            continue;
        }
        if let Some(i) = cur.as_mut() {
            if in_fields {
                if line.trim().is_empty() {
                    if !i.fields.is_empty() {
                        in_fields = false;
                    }
                    continue;
                }
                // fields are list items (`- key: value`), so md-press keeps one per line;
                // bare `key: value` lines are accepted too
                let fline = line.trim_start().strip_prefix("- ").unwrap_or(line);
                if let Some((k, v)) = fline.split_once(':') {
                    let k = k.trim();
                    if !k.contains(' ') && !k.is_empty() {
                        i.fields.insert(k.to_string(), v.trim().trim_matches('"').to_string());
                        continue;
                    }
                }
                in_fields = false;
            }
            i.text.push_str(line);
            i.text.push('\n');
        }
    }
    if let Some(i) = cur.take() {
        items.push(i);
    }
    for i in &mut items {
        i.text = i.text.trim().to_string();
    }
    items
}

fn take(store: &Store, path: &str, actor: Actor, dry: bool) -> Result<i32> {
    let abs = if std::path::Path::new(path).is_absolute() { std::path::PathBuf::from(path) } else { std::env::current_dir()?.join(path) };
    let rel = abs.canonicalize().unwrap_or(abs.clone()).strip_prefix(&store.root).map(|p| p.to_string_lossy().to_string()).map_err(|_| anyhow!("{path} is outside the store"))?;
    let content = std::fs::read_to_string(&abs).with_context(|| format!("reading {path}"))?;
    let review = record::load_record(store, "review", &rel, &content)?;
    if review.frontmatter.get("kind").and_then(|v| v.as_str()) != Some("review") {
        bail!("{path} is not a review record (frontmatter `kind: review`)");
    }
    let author: Actor = review
        .frontmatter
        .get("author")
        .map(|v| serde_yaml::from_value(v.clone()))
        .transpose()?
        .unwrap_or_else(|| actor.clone());
    let items = parse_items(&review.body);
    if items.is_empty() {
        bail!("no typed items in {path}: sections like `## C1 (check)` / `## F1 (finding)` are what `take` reads (free-form intake is not built yet)");
    }
    let st = State::compute(store);
    let review_id = review.id.record_id();
    let mut lines = vec![];
    let mut items_yaml = vec![];
    let mut wrote = 0usize;
    let mut skipped = 0usize;
    let mut act_on = 0usize;

    // the review record itself is minted so its version is on record
    if st.rs(&review_id).map(|s| s.last_version.is_none()).unwrap_or(true) {
        event::write_object(store, "review", "", &review.text, dry)?;
        let mut ev = Event::new(store, "mint", Some(&review_id), actor.clone())
            .with("own-version", yaml_str(&review.own_version))
            .with("record", yaml_str(&rel))
            .with("declarations", declarations(&review));
        event::write_event(store, &mut ev, dry)?;
    }

    for item in &items {
        let item_id = format!("{review_id}#{}", item.id);
        items_yaml.push(yaml_str(&format!("{}:{}", item.id, item.kind)));
        let target_s = item.fields.get("target").ok_or_else(|| anyhow!("{item_id}: no `target:`"))?;
        let target = Id::parse(store, target_s)?;
        let tid = target.record_id();
        // which version did it examine?
        let (basis, version_from, target_version) = if let Some(v) = item.fields.get("version") {
            let b = state::basis_of(store, &st.scan, &tid);
            (b, "declared", v.clone())
        } else if let Some(rev) = item.fields.get("at") {
            let (r, b) = basis_at(store, &target, rev)?;
            let form = r.frontmatter.get("form").and_then(|v| v.as_str()).unwrap_or("");
            event::write_object(store, &r.id.kind, form, &r.text, dry)?;
            (b, "git-revision", r.own_version.clone())
        } else {
            let Some(r) = st.record(&tid) else {
                bail!("{item_id}: {tid} is not in the store now; say which version it read with `at: <git rev>` or `version: <own version>`");
            };
            (state::basis_of(store, &st.scan, &tid), "current", r.own_version.clone())
        };
        let about = target.to_string();
        match item.kind.as_str() {
            "check" => {
                let act = item.fields.get("act").or(item.fields.get("check")).cloned().unwrap_or_else(|| "rederive".into());
                let outcome = item.fields.get("outcome").ok_or_else(|| anyhow!("{item_id}: no `outcome:`"))?;
                let dup = st.records.get(&tid).map(|s| s.securing.iter().any(|(e, _)| e.str("from") == Some(item_id.as_str()))).unwrap_or(false);
                if dup {
                    skipped += 1;
                    continue;
                }
                let mut ev = Event::new(store, &act, Some(&about), author.clone())
                    .with("from", yaml_str(&item_id))
                    .with("basis", state::basis_to_yaml(&basis))
                    .with("effective-version", yaml_str(&state::effective_version(&basis)))
                    .with("target-version", yaml_str(&target_version))
                    .with("version-from", yaml_str(version_from))
                    .with("line", yaml_str(item.fields.get("line").map(|s| s.as_str()).unwrap_or("derivation")))
                    .with("criterion", yaml_str(item.fields.get("criterion").map(|s| s.as_str()).unwrap_or("")))
                    .with("committed", yaml_str(item.fields.get("committed").map(|s| s.as_str()).unwrap_or("after")))
                    .with("recheck", yaml_str(item.fields.get("recheck").map(|s| s.as_str()).unwrap_or("")))
                    .with("outcome", yaml_str(outcome));
                if let Some(a) = st.rs(&tid).and_then(|s| s.atom.clone()) {
                    ev.atom = Some(a);
                }
                event::write_event(store, &mut ev, dry)?;
                wrote += 1;
                let now_v = st.record(&tid).map(|r| r.own_version.clone());
                let mut note = String::new();
                if let Some(nv) = &now_v {
                    if nv != &target_version {
                        note = format!("\n      ⚠ the text is now v:{} and the check read v:{}: it stays with the version it read", short(nv), short(&target_version));
                        act_on += 1;
                    }
                } else {
                    note = "\n      (the record is not in the store now; the check binds to the version it read)".into();
                }
                lines.push(format!("  {} → {about} read at v:{} ({version_from}) · {act} on {} · outcome {outcome}{note}", item.id, short(&target_version), item.fields.get("line").map(|s| s.as_str()).unwrap_or("derivation")));
                if outcome == "broken" || outcome == "narrowed" {
                    // a check that broke something opens a finding too
                    let fid = format!("{item_id}/finding");
                    let mut f = Event::new(store, "place-finding", Some(&about), author.clone())
                        .with("finding", yaml_str(&fid))
                        .with("target", yaml_str(&about))
                        .with("basis", state::basis_to_yaml(&basis))
                        .with("version-from", yaml_str(version_from))
                        .with("text", yaml_str(&format!("check {} came back {outcome}: {}", item.id, first_sentence(&item.text))));
                    event::write_event(store, &mut f, dry)?;
                    lines.push(format!("      → opens finding {fid} (something now needs answering)"));
                    act_on += 1;
                }
            }
            "finding" => {
                let dup = st.findings_open.iter().any(|f| f.item == item_id) || st.trail.events.iter().any(|e| e.act == "place-finding" && e.str("finding") == Some(item_id.as_str()));
                if dup {
                    skipped += 1;
                    continue;
                }
                let mut f = Event::new(store, "place-finding", Some(&about), author.clone())
                    .with("finding", yaml_str(&item_id))
                    .with("target", yaml_str(&about))
                    .with("basis", state::basis_to_yaml(&basis))
                    .with("version-from", yaml_str(version_from))
                    .with("text", yaml_str(&first_sentence(&item.text)));
                event::write_event(store, &mut f, dry)?;
                wrote += 1;
                act_on += 1;
                lines.push(format!("  {} → finding on {about} at v:{}: {}", item.id, short(&target_version), first_sentence(&item.text)));
            }
            other => {
                lines.push(format!("  {} ({other}) — not taken: this intake handles check and finding items only", item.id));
            }
        }
    }
    // the take event is written once per review version: a re-take that added nothing adds no event either
    let already_taken = st.trail.events.iter().any(|e| e.act == "take" && e.about.as_deref() == Some(&review_id) && e.str("review-version") == Some(&review.own_version));
    if wrote > 0 || !already_taken {
        let mut tev = Event::new(store, "take", Some(&review_id), actor.clone())
            .with("review", yaml_str(&rel))
            .with("review-version", yaml_str(&review.own_version))
            .with("items", Value::Sequence(items_yaml));
        event::write_event(store, &mut tev, dry)?;
    }
    println!("{} {rel}{}", if dry { "Would take" } else { "Taken" }, if dry { " (dry run, nothing written)" } else { "" });
    for l in lines {
        println!("{l}");
    }
    if skipped > 0 {
        println!("  {skipped} item(s) already taken; nothing added for them");
    }
    let _ = wrote;
    Ok(if act_on > 0 { EXIT_ITEMS } else { EXIT_OK })
}

fn first_sentence(s: &str) -> String {
    let t = s.lines().find(|l| !l.trim().is_empty()).unwrap_or("").trim();
    let t: String = t.chars().take(240).collect();
    t
}

#[allow(clippy::too_many_arguments)]
fn take_inline(
    store: &Store, target: &str, act: &str, line: &str, criterion: &str, committed: &str, outcome: &str,
    recheck: Option<&str>, at: Option<&str>, sources: &[String], slug: Option<&str>, note: Option<&str>,
    actor: Actor, dry: bool,
) -> Result<i32> {
    let target_id = Id::parse(store, target)?;
    let ok_outcomes = ["confirmed", "broken", "narrowed", "inconclusive", "failed-to-run"];
    if !ok_outcomes.contains(&outcome) {
        bail!("outcome `{outcome}` is not one of {}", ok_outcomes.join(", "));
    }
    let slug = slug.map(|s| s.to_string()).unwrap_or_else(|| format!("{}-{}-{}", target_id.slug, act, today()));
    if !record::slug_re().is_match(&slug) {
        bail!("slug `{slug}` must be lowercase kebab-case");
    }
    let dir = store.builtin_dir("review");
    let path = dir.join(format!("{slug}.md"));
    if path.exists() {
        bail!("{} exists; a review's body is never edited after it is taken. Give --slug another name.", path.display());
    }
    let mut fm = String::new();
    fm.push_str("---\nkind: review\n");
    fm.push_str(&format!("author:\n  name: {}\n", yaml_quote(&actor.name)));
    if let Some(r) = &actor.role {
        fm.push_str(&format!("  role: {}\n", yaml_quote(r)));
    }
    if let Some(s) = &actor.session {
        fm.push_str(&format!("  session: {}\n", yaml_quote(s)));
    }
    if !sources.is_empty() {
        fm.push_str("sources:\n");
        for s in sources {
            fm.push_str(&format!("  - {}\n", yaml_quote(s)));
        }
    }
    fm.push_str(&format!("written: {}\n---\n\n", event::now()));
    let mut body = format!("# {}: {} of {}\n\n", today(), act, target_id);
    body.push_str("## C1 (check)\n");
    body.push_str(&format!("- target: {target_id}\n- act: {act}\n- line: {line}\n- criterion: {}\n- committed: {committed}\n- outcome: {outcome}\n", criterion.replace('\n', " ")));
    if let Some(r) = recheck {
        body.push_str(&format!("- recheck: {}\n", r.replace('\n', " ")));
    }
    if let Some(a) = at {
        body.push_str(&format!("- at: {a}\n"));
    }
    body.push('\n');
    if let Some(n) = note {
        body.push_str(n.trim());
        body.push('\n');
    }
    let content = format!("{fm}{body}");
    if dry {
        println!("would write {}:\n{content}", path.display());
    } else {
        std::fs::create_dir_all(&dir)?;
        std::fs::write(&path, content)?;
        println!("Wrote {}", path.strip_prefix(&store.root).unwrap_or(&path).display());
    }
    if dry {
        return Ok(EXIT_OK);
    }
    take(store, path.to_str().unwrap(), actor, dry)
}

fn yaml_quote(s: &str) -> String {
    serde_yaml::to_string(&Value::String(s.to_string())).unwrap_or_default().trim().to_string()
}

// ---------- ask / answer ----------

fn slugify(text: &str) -> String {
    let mut s = String::new();
    let mut last_dash = true;
    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            s.push(c.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            s.push('-');
            last_dash = true;
        }
        if s.len() >= 48 {
            break;
        }
    }
    s.trim_matches('-').to_string()
}

#[allow(clippy::too_many_arguments)]
fn ask(store: &Store, text: &str, concerns: &[String], blocks: &[String], closer: &str, why: Option<&str>, slug: Option<&str>, actor: Actor, dry: bool) -> Result<i32> {
    let ok = closer == "anyone" || closer == "steward-fact" || closer == "steward-purpose" || closer.starts_with("awaiting:");
    if !ok {
        bail!("closer `{closer}` is not one of anyone, awaiting:<work>, steward-fact, steward-purpose");
    }
    if closer.starts_with("steward") && why.is_none() {
        bail!("nothing reaches the steward without a stated reason: give --why");
    }
    for c in concerns.iter().chain(blocks.iter()) {
        Id::parse(store, c)?;
    }
    let slug = slug.map(|s| s.to_string()).unwrap_or_else(|| slugify(text));
    if !record::slug_re().is_match(&slug) {
        bail!("slug `{slug}` must be lowercase kebab-case");
    }
    let dir = store.builtin_dir("question");
    let path = dir.join(format!("{slug}.md"));
    if path.exists() {
        bail!("question:{slug} exists already");
    }
    let mut c = String::new();
    c.push_str("---\nkind: question\n");
    c.push_str(&format!("closer: {closer}\nasked: {}\nasked-by: {}\n", event::now(), yaml_quote(&actor.name)));
    if !concerns.is_empty() {
        c.push_str(&format!("concerns: [{}]\n", concerns.join(", ")));
    }
    if !blocks.is_empty() {
        c.push_str(&format!("blocks: [{}]\n", blocks.join(", ")));
    }
    c.push_str("---\n\n");
    c.push_str(&format!("# {}\n", text.trim()));
    if let Some(w) = why {
        c.push_str(&format!("\n{}\n", w.trim()));
    }
    if dry {
        println!("would write {}:\n{c}", path.display());
        return Ok(EXIT_OK);
    }
    std::fs::create_dir_all(&dir)?;
    std::fs::write(&path, &c)?;
    let rel = path.strip_prefix(&store.root).unwrap().to_string_lossy().to_string();
    let r = record::load_record(store, "question", &rel, &c)?;
    let qid = r.id.record_id();
    event::write_object(store, "question", "", &r.text, dry)?;
    let mut mint = Event::new(store, "mint", Some(&qid), actor.clone())
        .with("own-version", yaml_str(&r.own_version))
        .with("record", yaml_str(&rel))
        .with("declarations", declarations(&r));
    event::write_event(store, &mut mint, dry)?;
    let mut ev = Event::new(store, "ask", Some(&qid), actor)
        .with("closer", yaml_str(closer))
        .with("concerns", yaml_list(concerns))
        .with("blocks", yaml_list(blocks));
    event::write_event(store, &mut ev, dry)?;
    let who = match closer {
        "anyone" => "open to anyone".to_string(),
        c if c.starts_with("steward") => "for the steward".to_string(),
        c => format!("awaiting {}", &c[9..]),
    };
    println!("Recorded {qid}, {who}. ({rel})");
    Ok(EXIT_OK)
}

fn answer(store: &Store, question: &str, outcome: &str, text: Option<&str>, reopen_if: Option<&str>, superseded_by: Option<&str>, actor: Actor, dry: bool) -> Result<i32> {
    let qid = Id::parse(store, question)?.record_id();
    let st = State::compute(store);
    let Some(q) = st.questions_open.iter().find(|q| q.id == qid) else {
        if st.scan.records.contains_key(&qid) {
            bail!("{qid} is already closed (vsect why {qid})");
        }
        bail!("{qid} is not in the store");
    };
    match outcome {
        "answered" => {
            if text.is_none() {
                bail!("--text: the answer itself");
            }
        }
        "rejected" => {
            if reopen_if.is_none() {
                bail!("--reopen-if: a rejection says what would reopen it");
            }
        }
        "superseded" => {
            if superseded_by.is_none() {
                bail!("--superseded-by <question:slug>");
            }
        }
        o => bail!("outcome `{o}` is not answered, rejected or superseded"),
    }
    if q.closer.starts_with("steward") {
        let steward = &store.kinds.steward;
        if !steward.is_empty() && &actor.name != steward {
            bail!("{qid} is for the steward ({steward}); the recorded actor is {}. Refusing at the porcelain — this guards against honest mistakes, not fraud.", actor.name);
        }
    }
    let mut ev = Event::new(store, "answer", Some(&qid), actor).with("outcome", yaml_str(outcome));
    if let Some(t) = text {
        ev = ev.with("text", yaml_str(t));
    }
    if let Some(r) = reopen_if {
        ev = ev.with("reopen-if", yaml_str(r));
    }
    if let Some(s) = superseded_by {
        ev = ev.with("superseded-by", yaml_str(&Id::parse(store, s)?.record_id()));
    }
    if let Some(a) = st.rs(&qid).and_then(|s| s.atom.clone()) {
        ev.atom = Some(a);
    }
    let p = event::write_event(store, &mut ev, dry)?;
    println!("{qid} → {outcome}{}", if dry { format!(" (would write {})", p.display()) } else { String::new() });
    Ok(EXIT_OK)
}

// ---------- open / findings ----------

fn open(store: &Store, target: Option<&str>, json: bool) -> Result<i32> {
    let st = State::compute(store);
    let filter = match target {
        Some(t) => Some(Id::parse(store, t)?.record_id()),
        None => None,
    };
    let matches = |t: &str| filter.as_ref().map(|f| t.split('#').next().unwrap_or(t) == f).unwrap_or(true);
    let mut by_target: BTreeMap<String, Vec<(String, String, String, String, String)>> = BTreeMap::new(); // target -> (at, kind, item, resolver, text)
    for f in &st.findings_open {
        if matches(&f.target) {
            by_target.entry(f.target.clone()).or_default().push((f.at.clone(), "finding".into(), f.item.clone(), format!("anyone but {}", f.by), f.text.chars().take(160).collect()));
        }
    }
    for q in &st.questions_open {
        let targets: Vec<String> = if q.concerns.is_empty() { vec!["(the store)".into()] } else { q.concerns.clone() };
        for t in targets {
            if matches(&t) {
                by_target.entry(t).or_default().push((q.asked.clone(), "question".into(), q.id.clone(), q.closer.clone(), q.title.clone()));
            }
        }
    }
    // lapsed checks and unrecorded fates are open work too
    for (id, ev, v) in st.lapsed() {
        if matches(&id) {
            by_target.entry(id.clone()).or_default().push((ev.at.clone(), "lapsed-check".into(), ev.str("from").unwrap_or(&ev.id).to_string(), "a different mind than the reviser".into(), v.word()));
        }
    }
    for (id, rs) in &st.records {
        if !matches(id) {
            continue;
        }
        if let Some(why) = rs.unreconciled() {
            if why.starts_with("deleted") {
                by_target.entry(id.clone()).or_default().push((ev_at(&rs.last_lifecycle), "deleted".into(), id.clone(), "whoever deleted it".into(), why.into()));
            }
        }
    }
    for v in by_target.values_mut() {
        v.sort();
    }
    if json {
        let out: Vec<_> = by_target.iter().flat_map(|(t, items)| items.iter().map(move |(at, kind, item, resolver, text)| json!({"target": t, "since": at, "kind": kind, "item": item, "resolver": resolver, "text": text}))).collect();
        println!("{}", serde_json::to_string_pretty(&json!({"schema": "vsect-open/0", "items": out}))?);
        return Ok(EXIT_OK);
    }
    if by_target.is_empty() {
        println!("Nothing open{}.", filter.map(|f| format!(" about {f}")).unwrap_or_default());
        return Ok(EXIT_OK);
    }
    let total: usize = by_target.values().map(|v| v.len()).sum();
    println!("{total} open item(s), by what they concern, oldest first (resolver in brackets):");
    for (t, items) in &by_target {
        println!("\n{t}");
        for (at, kind, item, resolver, text) in items {
            println!("  {:<4} {:<12} {item}  [{resolver}]", st.age_days(at), kind);
            if !text.is_empty() {
                println!("       {text}");
            }
        }
    }
    Ok(EXIT_OK)
}

fn ev_at(e: &Option<Event>) -> String {
    e.as_ref().map(|e| e.at.clone()).unwrap_or_default()
}

fn dispose_finding(store: &Store, item: &str, disposition: &str, why: &str, actor: Actor, dry: bool) -> Result<i32> {
    if disposition != "rejected" && disposition != "known-defect" {
        bail!("--as rejected | known-defect");
    }
    let st = State::compute(store);
    let Some(f) = st.findings_open.iter().find(|f| f.item == item) else {
        bail!("{item} is not an open finding (vsect open)");
    };
    let mut ev = Event::new(store, "dispose-finding", Some(&f.target), actor)
        .with("finding", yaml_str(item))
        .with("disposition", yaml_str(disposition))
        .with("reason", yaml_str(why));
    event::write_event(store, &mut ev, dry)?;
    println!("{item} → {disposition}{}", if dry { " (dry run)" } else { "" });
    Ok(EXIT_OK)
}

fn review_finding(store: &Store, item: &str, revision: &str, why: &str, actor: Actor, dry: bool) -> Result<i32> {
    let st = State::compute(store);
    let Some(f) = st.findings_open.iter().find(|f| f.item == item) else {
        bail!("{item} is not an open finding (vsect open)");
    };
    // the finding's placer is the finder; the reviser is whoever last revised the target. Refuse
    // when the confirmer is the reviser (the design's rule); the finder confirming is ideal.
    let rid = f.target.split('#').next().unwrap_or(&f.target).to_string();
    if let Some(rs) = st.rs(&rid) {
        if let Some(ev) = &rs.last_lifecycle {
            if ev.act == "revise" && ev.actor.name == actor.name {
                bail!("{} revised {rid} last; a finding is confirmed answered by someone other than the reviser", actor.name);
            }
        }
    }
    let mut ev = Event::new(store, "review-finding", Some(&f.target), actor)
        .with("finding", yaml_str(item))
        .with("revision", yaml_str(revision))
        .with("outcome", yaml_str("answered"))
        .with("criterion", yaml_str(why));
    event::write_event(store, &mut ev, dry)?;
    println!("{item} → answered by {revision}{}", if dry { " (dry run)" } else { "" });
    Ok(EXIT_OK)
}

// ---------- refute ----------

fn refute(store: &Store, target: &str, ground: &str, by_item: Option<&str>, at: Option<&str>, actor: Actor, dry: bool) -> Result<i32> {
    let tid = Id::parse(store, target)?;
    let (gkind, gtext) = ground.split_once(':').ok_or_else(|| anyhow!("--ground kind:text, kind one of counterexample, broken-derivation, missed-prediction, broken-mapping"))?;
    let kinds = ["counterexample", "broken-derivation", "missed-prediction", "broken-mapping"];
    if !kinds.contains(&gkind) {
        bail!("ground `{gkind}` is not evidence vsect accepts: {}. Authority is not a ground.", kinds.join(", "));
    }
    let st = State::compute(store);
    let rid = tid.record_id();
    let (basis, version) = if let Some(rev) = at {
        let (r, b) = basis_at(store, &tid, rev)?;
        let form = r.frontmatter.get("form").and_then(|v| v.as_str()).unwrap_or("");
        event::write_object(store, &r.id.kind, form, &r.text, dry)?;
        (b, r.own_version)
    } else {
        let Some(r) = st.record(&rid) else {
            bail!("{rid} is not in the store now; give --at <git rev> for the text that was refuted");
        };
        (state::basis_of(store, &st.scan, &rid), r.own_version.clone())
    };
    let mut g = serde_yaml::Mapping::new();
    g.insert(yaml_str("kind"), yaml_str(gkind));
    g.insert(yaml_str("text"), yaml_str(gtext.trim()));
    let mut ev = Event::new(store, "refute", Some(&tid.to_string()), actor)
        .with("basis", state::basis_to_yaml(&basis))
        .with("target-version", yaml_str(&version))
        .with("ground", Value::Mapping(g));
    if let Some(b) = by_item {
        ev = ev.with("by", yaml_str(b));
    }
    if let Some(a) = st.rs(&rid).and_then(|s| s.atom.clone()) {
        ev.atom = Some(a);
    }
    let p = event::write_event(store, &mut ev, dry)?;
    println!("{} {tid} at v:{} by {gkind}, as of {}.{}", if dry { "Would refute" } else { "Refuted" }, short(&version), store.moment(), if dry { format!(" (would write {})", p.display()) } else { String::new() });
    println!("  No mark written into any file in this version: `vsect check` reports a refuted statement without one.");
    let deps = st.dependents(&rid);
    if deps.is_empty() {
        println!("  Nothing in the store rests on it.");
    } else {
        for (d, edge, depth) in deps.iter().filter(|(_, _, depth)| *depth == 1) {
            let what = store.kinds.edges.get(edge).map(|e| e.on_refute.clone()).unwrap_or_else(|| "flag".into());
            println!("  {edge} of {d} → {}{}", what, if *depth > 1 { " (transitively)" } else { "" });
        }
        let further = deps.iter().filter(|(_, _, d)| *d > 1).count();
        if further > 0 {
            println!("  … and {further} more transitively (vsect impact {tid})");
        }
    }
    if st.record(&rid).is_some() && st.rs(&rid).map(|s| s.superseded_by.is_none()).unwrap_or(true) {
        println!("  What replaces it? Revise the record and run `vsect revise`, or record its fate when it is deleted.");
    }
    Ok(EXIT_OK)
}

// ---------- confirm ----------

/// A line diff of two texts (LCS), rendered with `-`/`+` and one line of context.
fn line_diff(a: &str, b: &str) -> String {
    let al: Vec<&str> = a.lines().collect();
    let bl: Vec<&str> = b.lines().collect();
    let (n, m) = (al.len(), bl.len());
    let mut dp = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            dp[i][j] = if al[i] == bl[j] { dp[i + 1][j + 1] + 1 } else { dp[i + 1][j].max(dp[i][j + 1]) };
        }
    }
    let mut ops: Vec<(char, &str)> = vec![];
    let (mut i, mut j) = (0, 0);
    while i < n && j < m {
        if al[i] == bl[j] {
            ops.push((' ', al[i]));
            i += 1;
            j += 1;
        } else if dp[i + 1][j] >= dp[i][j + 1] {
            ops.push(('-', al[i]));
            i += 1;
        } else {
            ops.push(('+', bl[j]));
            j += 1;
        }
    }
    while i < n {
        ops.push(('-', al[i]));
        i += 1;
    }
    while j < m {
        ops.push(('+', bl[j]));
        j += 1;
    }
    let mut out = String::new();
    let mut last_shown: Option<usize> = None;
    for (k, (c, line)) in ops.iter().enumerate() {
        let near_change = (k.saturating_sub(1)..=(k + 1).min(ops.len() - 1)).any(|x| ops[x].0 != ' ');
        if *c == ' ' && !near_change {
            continue;
        }
        if let Some(l) = last_shown {
            if k > l + 1 {
                out.push_str("      …\n");
            }
        }
        let shown: String = line.chars().take(160).collect();
        out.push_str(&format!("    {c} {shown}\n"));
        last_shown = Some(k);
    }
    if out.is_empty() {
        out.push_str("    (the normalized texts are identical)\n");
    }
    out
}

fn confirm_list(store: &Store) -> Result<i32> {
    let st = State::compute(store);
    let pending = st.pending_confirms();
    if pending.is_empty() {
        println!("Nothing awaits a second mind: every recorded check holds, or lapsed for a reason a confirmation cannot cure.");
        return Ok(EXIT_OK);
    }
    println!("{} rewording(s) awaiting a second mind. Each: the record, the version its checks read → the version now, and what a confirmation would re-bind.", pending.len());
    for p in &pending {
        let revisers = st.revisers_between(&p.record, &p.from, &p.to);
        let targets: BTreeSet<&str> = p.checks.iter().map(|(_, t)| t.as_str()).collect();
        println!("\n  {}  v:{} → v:{}   revised by {}", p.record, short(&p.from), short(&p.to), if revisers.is_empty() { "(no revise event)".to_string() } else { revisers.join(", ") });
        println!("    re-binds {} check(s) on: {}", p.checks.len(), targets.into_iter().collect::<Vec<_>>().join(", "));
        println!("    → vsect confirm {} --from {} --to {}", p.record, short(&p.from), short(&p.to));
    }
    Ok(EXIT_OK)
}

#[allow(clippy::too_many_arguments)]
fn confirm(store: &Store, record: &str, from: Option<&str>, to: Option<&str>, judgment: Option<&str>, criterion: Option<&str>, actor: Actor, answers: &BTreeMap<String, String>, dry: bool) -> Result<i32> {
    let rid = Id::parse(store, record)?.record_id();
    let st = State::compute(store);
    let mut pending: Vec<_> = st.pending_confirms().into_iter().filter(|p| p.record == rid).collect();
    if let Some(f) = from {
        pending.retain(|p| p.from.starts_with(f));
    }
    if let Some(t) = to {
        pending.retain(|p| p.to.starts_with(t));
    }
    if pending.is_empty() {
        if !st.scan.records.contains_key(&rid) {
            bail!("{rid} is not in the store");
        }
        println!("No rewording of {rid} awaits a second mind{}.", if from.is_some() || to.is_some() { " for those versions" } else { "" });
        return Ok(EXIT_OK);
    }
    let mut wrote = 0;
    for p in pending {
        // the second mind is not the reviser
        let revisers = st.revisers_between(&p.record, &p.from, &p.to);
        if revisers.iter().any(|r| r == &actor.name) {
            bail!("{} made this change ({} v:{} → v:{}); a rewording is confirmed by someone other than the reviser.", actor.name, p.record, short(&p.from), short(&p.to));
        }
        let old = event::read_object(store, &p.from)?;
        let new = event::read_object(store, &p.to)?;
        println!("{}  v:{} → v:{}  (revised by {})", p.record, short(&p.from), short(&p.to), if revisers.is_empty() { "no recorded reviser".to_string() } else { revisers.join(", ") });
        print!("{}", line_diff(event::object_text(&old), event::object_text(&new)));
        println!("  {} check(s) read v:{} and would hold again:", p.checks.len(), short(&p.from));
        let mut criteria = vec![];
        for (eid, target) in &p.checks {
            let ev = st.trail.events.iter().find(|e| &e.id == eid);
            let crit = ev.and_then(|e| e.str("criterion")).unwrap_or("");
            let from_item = ev.and_then(|e| e.str("from")).unwrap_or(eid);
            println!("    {from_item} on {target}{}", if crit.is_empty() { String::new() } else { format!(": {}", crit.chars().take(140).collect::<String>()) });
            if !crit.is_empty() {
                criteria.push(crit.to_string());
            }
        }
        let j = match judgment {
            Some(j) => j.to_string(),
            None => match ask_q(answers, "judgment", "  Does the new text say what the old one said? [equivalent / for-checks (only as it bears on these checks) / not-equivalent]", &["equivalent", "for-checks", "not-equivalent"]) {
                Some(a) => a,
                None => return Ok(EXIT_NOT_RUN),
            },
        };
        let (outcome, scope): (&str, Value) = match j.as_str() {
            "equivalent" | "yes" => ("equivalent", yaml_str("unconditional")),
            "for-checks" | "checked" => {
                let mut m = serde_yaml::Mapping::new();
                m.insert(yaml_str("for"), Value::Sequence(p.checks.iter().map(|(e, _)| yaml_str(e)).collect()));
                ("equivalent", Value::Mapping(m))
            }
            "not-equivalent" | "no" => ("not-equivalent", yaml_str("unconditional")),
            other => bail!("`{other}` is not a judgment: equivalent, for-checks, or not-equivalent"),
        };
        if j == "for-checks" && criterion.is_none() {
            bail!("a confirmation scoped to particular checks says what it compared: give --criterion");
        }
        let mut ev = Event::new(store, "confirm-rewording", Some(&p.record), actor.clone())
            .with("from-version", yaml_str(&p.from))
            .with("to-version", yaml_str(&p.to))
            .with("outcome", yaml_str(outcome))
            .with("scope", scope);
        if let Some(c) = criterion {
            ev = ev.with("criterion", yaml_str(c));
        }
        if let Some(a) = st.rs(&p.record).and_then(|s| s.atom.clone()) {
            ev.atom = Some(a);
        }
        event::write_event(store, &mut ev, dry)?;
        wrote += 1;
        match outcome {
            "equivalent" if j == "for-checks" => println!("  → recorded: equivalent as it bears on those {} check(s); they hold again at the current text. Other checks through this version still wait.", p.checks.len()),
            "equivalent" => println!("  → recorded: equivalent. Every check that read v:{} of {} holds again, and so does everything downstream that rests on it. Nothing downstream was written.", short(&p.from), p.record),
            _ => println!("  → recorded: not equivalent. The change is a redefinition from here on; the checks stay with v:{} until redone.", short(&p.from)),
        }
    }
    if !dry && wrote > 0 {
        let after = State::compute(store);
        let still = after.pending_confirms().len();
        println!("{} rewording(s) still await a second mind.", still);
    }
    Ok(EXIT_OK)
}

// ---------- check ----------

fn check(store: &Store, strict: bool, json: bool) -> Result<i32> {
    let st = State::compute(store);
    let mut items: Vec<(String, String, String)> = vec![]; // (class, where, what)
    for (rel, why) in &st.scan.unreadable {
        items.push(("unreadable-record".into(), rel.clone(), why.clone()));
    }
    for (id, paths) in &st.scan.collisions {
        items.push(("collision".into(), id.clone(), format!("two files claim one id: {}", paths.join(", "))));
    }
    // A dangling edge target is a notice, not a structural error: ASF's FORMAT allows `depends:`
    // to name a planned segment, and `bin/lint-outline` owns that check against the OUTLINE's
    // `missing` rows. --strict counts them, which is where a stale name gets caught.
    let mut notices: Vec<(String, String, String)> = vec![];
    for (src, edge, tgt) in &st.dangling {
        notices.push(("dangling-target".into(), src.clone(), format!("`{edge}` names {tgt}, which is no record in the store (planned, or a stale name?)")));
    }
    for (rel, why) in &st.trail.problems {
        items.push(("trail".into(), rel.clone(), why.clone()));
    }
    for (rel, why) in event::check_objects(store) {
        items.push(("trail".into(), rel, why));
    }
    // referenced events that do not exist
    let ids: BTreeSet<&str> = st.trail.events.iter().map(|e| e.id.as_str()).collect();
    for ev in &st.trail.events {
        for key in ["corrects", "episode", "contests", "atom"] {
            if let Some(r) = ev.str(key).or(if key == "atom" { ev.atom.as_deref() } else { None }) {
                if !ids.contains(r) {
                    items.push(("trail".into(), ev.rel_path(), format!("`{key}` names event {} which does not exist", short(r))));
                }
            }
        }
    }
    // open items whose target does not resolve: a judgment item (dispose, or confirm answered by the
    // successor), so a notice, not a structural error
    for f in &st.findings_open {
        let t = f.target.split('#').next().unwrap_or(&f.target);
        if !st.scan.records.contains_key(t) {
            let fate = st.rs(t).and_then(|s| s.superseded_by.clone()).map(|s| format!("superseded by {s}")).unwrap_or_else(|| "gone".into());
            notices.push(("open-on-gone-target".into(), f.item.clone(), format!("finding targets {t}, {fate}: confirm it answered (vsect findings --answered) or dispose of it")));
        }
    }
    for q in &st.questions_open {
        for t in q.concerns.iter().chain(q.blocks.iter()) {
            if !st.scan.records.contains_key(t.split('#').next().unwrap_or(t)) {
                notices.push(("open-on-gone-target".into(), q.id.clone(), format!("question concerns {t}, which is not in the store now")));
            }
        }
    }
    // refuted / broken, unmarked. Marks are not written in this version, so every refuted live
    // record is reported; as a notice, since nothing could clear it yet, and the record's own
    // `vsect read` already shows the refutation.
    for (id, rs) in &st.records {
        if rs.exists && rs.refuted.is_some() && rs.superseded_by.is_none() && !rs.retired {
            let r = st.record(id).unwrap();
            if !r.body.contains("> [!vsect refuted]") {
                notices.push(("unmarked".into(), id.clone(), "refuted, and its file carries no mark saying so (mark-writing is not built; replace or retire it)".into()));
            }
        }
        if rs.live() && !rs.broken_by.is_empty() {
            notices.push(("unmarked".into(), id.clone(), format!("rests on refuted {}; its file carries no mark", rs.broken_by.join(", "))));
        }
    }
    // declared-field hygiene for domain kinds
    for k in store.kinds.kinds.keys() {
        for r in st.live_of_kind(k) {
            if let Some(s) = r.frontmatter.get("slug").and_then(|v| v.as_str()) {
                if s != r.id.slug {
                    items.push(("slug".into(), r.rel.clone(), format!("frontmatter slug `{s}` disagrees with the file name")));
                }
            }
        }
    }
    let structural = items.len();
    for (id, why) in st.unreconciled() {
        notices.push(("not-taken-in".into(), id, why.into()));
    }
    for (id, ev, v) in st.lapsed() {
        notices.push(("lapsed-check".into(), id, format!("{} ({}) {}", ev.str("from").unwrap_or(&ev.id), ev.actor.name, v.word())));
    }
    let not_run = if store.root.join(".git").exists() { vec![] } else { vec!["deleted-event detection against git history (no .git here)".to_string()] };
    if json {
        println!("{}", serde_json::to_string_pretty(&json!({
            "schema": "vsect-check/0", "moment": store.moment(), "strict": strict,
            "items": items.iter().map(|(c, w, t)| json!({"class": c, "where": w, "what": t})).collect::<Vec<_>>(),
            "notices": notices.iter().map(|(c, w, t)| json!({"class": c, "where": w, "what": t})).collect::<Vec<_>>(),
            "not_run": not_run,
        }))?);
    } else {
        if items.is_empty() {
            println!("Structure: clean ({} records, {} events, as of {})", st.scan.records.len(), st.trail.events.len(), store.moment());
        } else {
            println!("{} structural item(s):", items.len());
            for (c, w, t) in &items {
                println!("  {c:<20} {w}: {t}");
            }
        }
        if !notices.is_empty() {
            if strict {
                println!("{} notice(s) (counted, --strict):", notices.len());
                for (c, w, t) in &notices {
                    println!("  {c:<20} {w}: {t}");
                }
            } else {
                let mut by_class: BTreeMap<&str, usize> = BTreeMap::new();
                for (c, _, _) in &notices {
                    *by_class.entry(c.as_str()).or_default() += 1;
                }
                let parts: Vec<String> = by_class.iter().map(|(c, n)| format!("{n} {c}")).collect();
                println!("{} notice(s), not counted: {} (--strict lists and counts them)", notices.len(), parts.join(" · "));
            }
        }
        if !st.planned.is_empty() {
            let slugs: BTreeSet<&str> = st.planned.iter().map(|(_, _, t)| t.as_str()).collect();
            println!("{} edge target(s) name planned records (declared by the store's `planned:` rows), not counted: {}", st.planned.len(), slugs.into_iter().collect::<Vec<_>>().join(", "));
        }
        for n in &not_run {
            println!("not run: {n}");
        }
    }
    let count = structural + if strict { notices.len() } else { 0 };
    Ok(if count > 0 { EXIT_ITEMS } else { EXIT_OK })
}

// ---------- read / why / impact / kinds ----------

fn read(store: &Store, id: &str, at: Option<&str>, bare: bool) -> Result<i32> {
    let pid = Id::parse(store, id)?;
    let rid = pid.record_id();
    let st = State::compute(store);
    if let Some(v) = at {
        let full = st.trail.events.iter().filter_map(|e| e.str("own-version").or(e.str("target-version"))).find(|x| x.starts_with(v)).map(|s| s.to_string()).unwrap_or_else(|| v.to_string());
        let canon = event::read_object(store, &full)?;
        print!("{}", event::object_text(&canon));
        return Ok(EXIT_OK);
    }
    let Some(r) = st.record(&rid) else {
        match st.rs(&rid) {
            Some(rs) if rs.last_version.is_some() => {
                bail!("{rid} is not in the store now (last seen at v:{}; `vsect read {rid} --at {}` shows that text; `vsect why {rid}` its history)", short(rs.last_version.as_deref().unwrap_or("")), short(rs.last_version.as_deref().unwrap_or("")));
            }
            _ => bail!("{rid} is not in the store"),
        }
    };
    if bare {
        print!("{}", r.text);
        return Ok(EXIT_OK);
    }
    let rs = st.rs(&rid).cloned().unwrap_or_default();
    println!("{rid} · {} · v:{} · as of {}", r.rel, short(&r.own_version), store.moment());
    let mut marks = vec![];
    if let Some(ev) = &rs.refuted {
        marks.push(format!("⚠ refuted {} by {}", ev.at, ev.get("ground").and_then(|g| g.get("kind")).and_then(|v| v.as_str()).unwrap_or("evidence")));
    }
    if !rs.broken_by.is_empty() {
        marks.push(format!("⚠ broken: rests on refuted {}", rs.broken_by.join(", ")));
    }
    if let Some(why) = rs.unreconciled() {
        marks.push(format!("↻ {why} (vsect revise)"));
    }
    for (ev, v) in &rs.securing {
        let who = format!("{}{}", ev.actor.name, ev.actor.role.as_deref().map(|r| format!(" as {r}")).unwrap_or_default());
        let sym = if v.holds() { "✓" } else { "↻" };
        marks.push(format!("{sym} {} {} by {who} on {} · {} · {}", ev.act, ev.str("outcome").unwrap_or(""), ev.at.get(..10).unwrap_or(""), ev.str("line").unwrap_or(""), v.word()));
        if let Some(c) = ev.str("criterion") {
            if !c.is_empty() {
                marks.push(format!("    criterion: {c}"));
            }
        }
    }
    for f in st.findings_open.iter().filter(|f| f.target.split('#').next() == Some(&rid)) {
        marks.push(format!("⚠ open finding {} ({}, {}): {}", f.item, f.by, st.age_days(&f.at), f.text));
    }
    for q in st.questions_open.iter().filter(|q| q.concerns.iter().any(|c| c == &rid) || q.blocks.iter().any(|c| c == &rid)) {
        marks.push(format!("? {} ({}, {}): {}", q.id, q.closer, st.age_days(&q.asked), q.title));
    }
    let declared: Vec<String> = ["type", "status", "stage", "form"].iter().filter_map(|k| r.frontmatter.get(*k).and_then(|v| v.as_str()).map(|v| format!("{k}: {v}"))).collect();
    if !declared.is_empty() {
        println!("declared (typed by the author, not computed): {}", declared.join(" · "));
    }
    if marks.is_empty() {
        println!("no check recorded · nothing open here");
    }
    for m in marks {
        println!("{m}");
    }
    println!("{}", "─".repeat(60));
    print!("{}", r.body);
    Ok(EXIT_OK)
}

fn why(store: &Store, id: &str) -> Result<i32> {
    let rid = Id::parse(store, id)?.record_id();
    let st = State::compute(store);
    let evs = st.events_about(&rid);
    if evs.is_empty() {
        println!("{rid}: no recorded act. {}", if st.scan.records.contains_key(&rid) { "It is in the store but not yet taken in (vsect revise)." } else { "It is not in the store." });
        return Ok(EXIT_OK);
    }
    println!("{rid} — {} act(s), oldest first", evs.len());
    for ev in evs {
        let who = format!("{}{}", ev.actor.name, ev.actor.role.as_deref().map(|r| format!(" ({r})")).unwrap_or_default());
        let mut detail = vec![];
        for k in ["change", "outcome", "line", "from", "into", "finding", "disposition", "closer", "version-from"] {
            if let Some(v) = ev.str(k) {
                if !v.is_empty() {
                    detail.push(format!("{k} {v}"));
                }
            }
        }
        if let Some(v) = ev.str("own-version").or(ev.str("target-version")) {
            detail.push(format!("v:{}", short(v)));
        }
        if let Some(g) = ev.get("ground") {
            detail.push(format!("ground {}: {}", g.get("kind").and_then(|v| v.as_str()).unwrap_or(""), g.get("text").and_then(|v| v.as_str()).unwrap_or("")));
        }
        if let Some(t) = ev.str("text") {
            detail.push(format!("“{}”", t.chars().take(120).collect::<String>()));
        }
        println!("  {}  {:<16} {:<28} {}", ev.at, ev.act, who, detail.join(" · "));
        if let Some(c) = ev.str("criterion") {
            if !c.is_empty() {
                println!("      criterion: {c}");
            }
        }
        if let Some(r) = ev.str("recheck") {
            if !r.is_empty() {
                println!("      recheck: {r}");
            }
        }
        if let Some(b) = ev.get("basis") {
            let n = b.as_mapping().map(|m| m.len()).unwrap_or(0);
            if n > 0 {
                println!("      basis: {n} statement version(s) · event {}", short(&ev.id));
            }
        }
    }
    Ok(EXIT_OK)
}

fn impact(store: &Store, id: &str) -> Result<i32> {
    let rid = Id::parse(store, id)?.record_id();
    let st = State::compute(store);
    let deps = st.dependents(&rid);
    let basis = state::basis_of(store, &st.scan, &rid);
    println!("{rid} rests on {} statement(s) (its basis) and {} rest on it.", basis.len().saturating_sub(1), deps.len());
    if !deps.is_empty() {
        println!("If it were refuted:");
        for (d, edge, depth) in &deps {
            let what = store.kinds.edges.get(edge).map(|e| e.on_refute.clone()).unwrap_or_else(|| "flag".into());
            println!("  {:<10} {d}  via `{edge}`{}", what, if *depth > 1 { format!(" ({depth} hops)") } else { String::new() });
        }
        println!("If its meaning changed: every check on those {} lapses until confirmed.", deps.len());
    }
    Ok(EXIT_OK)
}

fn kinds(store: &Store) -> Result<i32> {
    let k = &store.kinds;
    println!("{} · steward {} · decider {}", k.name, if k.steward.is_empty() { "(none)" } else { &k.steward }, if k.decider.is_empty() { "(none)" } else { &k.decider });
    println!("\nRecord kinds declared by this store:");
    if k.kinds.is_empty() {
        println!("  (none — kinds are admitted when records show they fail differently from every other kind)");
    }
    for (name, d) in &k.kinds {
        println!("  {name:<12} grain {} · find {} · edges {}{}", d.grain, d.find.join(" "), d.edges.iter().map(|(f, e)| format!("{f}→{e}")).collect::<Vec<_>>().join(", "), if d.note.is_empty() { String::new() } else { format!("\n{:14}{}", "", d.note) });
    }
    println!("\nBuilt in (vsect's verbs write them): question, decision, purpose, review");
    println!("\nEdges:");
    for (name, e) in &k.edges {
        println!("  {name:<12} in basis {} · meaning {} · on refute: {}{}", e.in_basis, e.meaning, e.on_refute, if e.note.is_empty() { String::new() } else { format!(" — {}", e.note) });
    }
    println!("\nLines and rungs (lowest first):");
    for (name, rungs) in &k.lines {
        println!("  {name:<12} {}", rungs.join(" → "));
    }
    Ok(EXIT_OK)
}

// ---------- help ----------

fn help(topic: Option<&str>) -> Result<i32> {
    match topic {
        Some("model") => println!("{}", MODEL),
        Some(v) => println!("{}", verb_help(v)),
        None => println!("{}", USAGE),
    }
    Ok(EXIT_OK)
}

const USAGE: &str = "vsect — work inside a verisectorium. Bare `vsect` is the dashboard.

Read      vsect read <kind:slug> [--at v] [--bare]   the record with marks: ✓ holds · ↻ lapsed · ⚠ broken/open · ? question
          vsect why <kind:slug>                      every recorded act on it
          vsect impact <kind:slug>                   what rests on it
          vsect open [<kind:slug>]                   what is open, keyed by statement, oldest first
Work      vsect revise [<kind:slug>]                 take in edits made outside vsect (mint / revise / fate of a deletion)
          vsect take <review.md>                     take in a review: checks become evidence, findings stay open
          vsect take --check <id> --criterion … --outcome …   record one check inline
          vsect ask \"…\" [--concerns id] [--closer …]  record an open question
          vsect answer <question:slug> --outcome …   close one
          vsect confirm [<kind:slug>]                the second mind: confirm a rewording still says what was checked
          vsect refute <id> --ground kind:text        record a refutation, on evidence
          vsect findings --dispose <item> --as … --why …
          vsect check [--strict]                     everything that can be checked; exit 1 = act on it
Store     vsect init --name … --steward … --decider …   found a store (no kinds)
          vsect kinds                                what the store declares
Every writing verb takes --by <name> --role <role> [--session s] and --dry-run; questions can be answered in
advance with --answer key=value, and without a terminal an unanswered question stops the verb with exit 3.
Exit codes describe the command, not the store: 0 done · 1 done, with items to act on · 2 failed · 3 not run.
";

const MODEL: &str = "The logical model (short form; the design is at ~/src/aat-v3/msc/vsect/model.md)

One fate, one atom.  A statement is the unit that can be right or wrong on its own. Here every record
is one statement (`segment:<slug>`): its text is the body minus Working Notes and generated marks.
Carving records into several statements is later work.

Version.   The SHA-256 of a statement's kind, declared form and normalized text (normalizer vsect-norm/0,
           named on every event). Every version vsect has seen is kept under .vsect/objects/.
Basis.     A statement plus everything it rests on through meaning edges (here: `depends:`), transitively.
           A check is bound to the versions of the whole basis at the moment it was recorded.
Event.     One recorded act, one YAML file under .vsect/events/, named by its content hash, never edited.
           Nobody writes them by hand; the verbs emit them.
Check.     A securing act (rederive, instantiate, run, read-check, assert, decline) by someone, on a line
           (derivation, simulation, instance, imported, measured, testimony), against a criterion, with an
           outcome. It holds while its basis is unchanged; it lapses when the statement's own text changes
           (`stale: own`) or when something it rests on changes (`stale: upstream (which)`).
Standing.  Computed, never typed. Until this store declares how to compute it, vsect shows raw evidence.
           The frontmatter `status:` word is shown as what its author typed, beside the evidence.
Finding.   Something wrong or missing, from a review, open on its statement until someone other than its
           author confirms it answered, or it is disposed of (rejected / known-defect) with a reason.
Question.  An open item with a closer: anyone, awaiting:<work>, steward-fact, steward-purpose.
Refutation. Needs a ground that is evidence, never authority. What rests on the refuted statement through
           a `broken`-on-refute edge is broken too.
Review.    What a verifier, auditor or integrator sent back: a record at reviews/<slug>.md with typed items
           (`## C1 (check)`, `## F1 (finding)`). Its body is never edited after `take`.
";

fn verb_help(v: &str) -> String {
    match v {
        "take" => "vsect take <reviews/x.md>  or  vsect take --check <kind:slug> --criterion \"…\" --outcome confirmed \\
    [--act rederive|instantiate|run|read-check|assert|decline] [--line derivation] [--committed before|after] \\
    [--recheck \"how to re-run\"] [--at <git rev>] [--source path]… [--note \"…\"] [--slug name]

A review record carries typed items. Each `## C1 (check)` section has fields target, act, line, criterion,
committed, outcome, recheck, and optionally `at: <git rev>` or `version: <own version>` for the text it read;
then free text. Each `## F1 (finding)` has target (and optionally at/version), then the finding in words.
A check whose outcome is broken or narrowed also opens a finding. Taking the same review twice adds nothing.
If the text changed after the check, take says so and the check stays with the version it read.".into(),
        "revise" => "vsect revise [<kind:slug>] [--answer change=reworded|redefined] [--answer fate=retired|superseded:<id>]

Walks every edit not yet taken in: new records are minted; edited ones get a revise event (you are asked
whether the meaning changed only when a recorded check rests on the old version); deleted ones need a fate.
A rewording is recorded as your claim; the old checks stay with the old version until a different mind confirms.".into(),
        "check" => "vsect check [--strict] [--format json]

Structural problems, always counted: unreadable records, id collisions, a frontmatter slug disagreeing with
its file name, edited or misnamed event and object files, events naming events that do not exist.
Notices, counted only with --strict: dangling edge targets (a planned segment, or a stale name — lint-outline
owns the planned case), edits not yet taken in, checks that lapsed, open items whose target is gone, refuted
statements whose file carries no mark. Exit 1 when anything counted is present.".into(),
        "refute" => "vsect refute <kind:slug> --ground counterexample|broken-derivation|missed-prediction|broken-mapping:\"…\" \\
    [--by-item review:slug#C2] [--at <git rev>]

Records that a statement was shown false, on evidence, and lists what rests on it. No mark is written into
any file in this version; `vsect check` reports the refuted statement as unmarked until that exists.".into(),
        "confirm" => "vsect confirm                       what awaits a second mind: each record, the version its checks read → now
vsect confirm <kind:slug> [--from v] [--to v] [--judgment equivalent|for-checks|not-equivalent] [--criterion \"…\"]

A check binds to the exact versions it read. When a statement in its basis is reworded, the check
lapses until someone other than the reviser confirms the new text says what the old one said.
`confirm` shows the diff of the two stored versions and the checks it would re-bind, then asks.
  equivalent      — full stop: every check that read the old version holds again, and so does
                    everything downstream, with nothing written downstream (one event at the root).
  for-checks      — only as it bears on the listed checks' criteria (--criterion says what you compared);
                    other checks through this version keep waiting.
  not-equivalent  — the change is a redefinition from here on; the checks stay with the old version.
Refuses the reviser. Confirmations chain: a→b and b→c confirmed means a check that read a holds at c.".into(),
        "ask" => "vsect ask \"question?\" [--concerns kind:slug]… [--blocks kind:slug]… [--closer anyone|awaiting:<work>|steward-fact|steward-purpose] [--why \"…\"]

Writes questions/<slug>.md and records the ask. Nothing reaches the steward without a stated reason.".into(),
        "answer" => "vsect answer question:<slug> --outcome answered --text \"…\"  |  --outcome rejected --reopen-if \"…\"  |  --outcome superseded --superseded-by question:<slug>".into(),
        "open" => "vsect open [<kind:slug>] [--format json]\n\nEvery open item keyed by the statement it concerns: findings, questions, lapsed checks, unrecorded fates.".into(),
        "read" => "vsect read <kind:slug> [--at <version prefix>] [--bare]\n\nThe record with marks above it. --bare prints the statement text alone (no marks, no Working Notes).".into(),
        _ => format!("no help for `{v}` yet; `vsect help` lists the verbs"),
    }
}
