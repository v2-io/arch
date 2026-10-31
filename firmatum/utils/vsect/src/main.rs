//! vsect — work inside a verisectorium.
//!
//! Bare `vsect` shows where the store stands. Every number it prints is computed from the
//! records and the recorded acts; none can be typed. See `vsect help model`.

mod event;
mod intent;
mod norm;
mod record;
mod state;
mod store;
mod verbs;

use clap::{Args, Parser, Subcommand};
use std::process::ExitCode;

/// Who is acting. Recorded as a claim, never verified.
#[derive(Args, Debug, Clone, Default)]
pub struct ActorArgs {
    /// Your name as the record will carry it (or set VSECT_BY).
    #[arg(long, global = true)]
    pub by: Option<String>,
    /// Your role in this act: author, verifier, integrator, auditor, adopter, … (or VSECT_ROLE).
    #[arg(long, global = true)]
    pub role: Option<String>,
    /// The harness session, if known (or VSECT_SESSION).
    #[arg(long, global = true)]
    pub session: Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "vsect", version, about = "Work inside a verisectorium. Bare `vsect` is the dashboard.", disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
    #[command(flatten)]
    pub actor: ActorArgs,
    /// Print exactly what would be written, and write nothing.
    #[arg(long, global = true)]
    pub dry_run: bool,
    /// Output for a program: `json`.
    #[arg(long, global = true)]
    pub format: Option<String>,
    /// Answer a question in advance, as key=value (repeatable): change=reworded, fate=retired, fate=superseded:<id>.
    #[arg(long, global = true, value_name = "KEY=VALUE")]
    pub answer: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Found a store here: a name, a steward, a decider, and no kinds of the store's own.
    Init {
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "")]
        steward: String,
        #[arg(long, default_value = "")]
        decider: String,
    },
    /// Take in edits made outside vsect: mint new records, record revisions, settle deletions.
    Revise {
        /// One record (kind:slug). With nothing: every edit in the store not yet taken in.
        record: Option<String>,
    },
    /// Take in a review (checks and findings), or record one check inline.
    Take {
        /// Path to a review record, or omit with --check.
        path: Option<String>,
        /// Record one check inline on this target (kind:slug); vsect writes the review record for you.
        #[arg(long, value_name = "TARGET")]
        check: Option<String>,
        /// What was done: rederive, instantiate, run, read-check, assert, decline.
        #[arg(long, default_value = "rederive")]
        act: String,
        /// The evidence line: derivation, simulation, instance, imported, measured, testimony.
        #[arg(long, default_value = "derivation")]
        line: String,
        /// What the check tested, in one sentence.
        #[arg(long)]
        criterion: Option<String>,
        /// Was the criterion fixed before the outcome was seen? before | after.
        #[arg(long, default_value = "before")]
        committed: String,
        /// confirmed | broken | narrowed | inconclusive | failed-to-run.
        #[arg(long)]
        outcome: Option<String>,
        /// How to re-run the check (a path, a command, or "re-derive from §2").
        #[arg(long)]
        recheck: Option<String>,
        /// The text was read at this git revision (for a target that has since changed or gone).
        #[arg(long)]
        at: Option<String>,
        /// Where the check is written up (paths), repeatable.
        #[arg(long)]
        source: Vec<String>,
        /// Slug for the review record vsect writes (default: derived from target and date).
        #[arg(long)]
        slug: Option<String>,
        /// A sentence or two of what was found, for the review record.
        #[arg(long)]
        note: Option<String>,
        /// The records in the target's basis this check actually read (kind:slug, repeat or comma-separate). Declared: a change elsewhere in the basis is then "upstream changed, not read", not lapsed.
        #[arg(long, value_delimiter = ',')]
        read: Vec<String>,
    },
    /// Record an open question about the work.
    Ask {
        text: String,
        /// Statements it concerns (kind:slug), repeatable.
        #[arg(long)]
        concerns: Vec<String>,
        /// Statements it holds up, repeatable.
        #[arg(long)]
        blocks: Vec<String>,
        /// Who may close it: anyone | awaiting:<work> | steward-fact | steward-purpose.
        #[arg(long, default_value = "anyone")]
        closer: String,
        /// Why it is open, in a sentence or two.
        #[arg(long)]
        why: Option<String>,
        #[arg(long)]
        slug: Option<String>,
    },
    /// Close a question: answered, rejected (say what would reopen it), or superseded.
    Answer {
        question: String,
        #[arg(long)]
        outcome: String,
        #[arg(long)]
        text: Option<String>,
        #[arg(long)]
        reopen_if: Option<String>,
        /// The question that supersedes this one.
        #[arg(long)]
        superseded_by: Option<String>,
    },
    /// Record work owed: an open item with an owner, closed by the commit that does it.
    Note {
        text: String,
        /// Statements it concerns (kind:slug), repeatable.
        #[arg(long)]
        concerns: Vec<String>,
        /// Who resolves it (a name or role).
        #[arg(long, default_value = "anyone")]
        owner: String,
        #[arg(long)]
        why: Option<String>,
        #[arg(long)]
        slug: Option<String>,
        /// The roll-up item this one was split out of (item:slug), kept as provenance.
        #[arg(long)]
        from: Option<String>,
    },
    /// Close an item: done (by which commit), moot, superseded, or duplicate (--of the survivor).
    Close {
        item: String,
        /// done | moot | superseded | duplicate
        #[arg(long)]
        outcome: String,
        /// The commit that did the work (for `done`).
        #[arg(long)]
        by_commit: Option<String>,
        #[arg(long)]
        why: Option<String>,
        /// For `duplicate`: the surviving item (item:slug).
        #[arg(long)]
        of: Option<String>,
    },
    /// Re-check an imported item against HEAD: holds (real and open), landed (say where), stale, superseded, duplicate (--of), or not-an-item.
    Recheck {
        item: String,
        /// holds | landed | stale | superseded | duplicate | not-an-item
        #[arg(long)]
        outcome: String,
        /// What you checked and found, in a sentence.
        #[arg(long)]
        why: String,
        /// Who resolves it, when it holds (a KR id like C.1, or a name).
        #[arg(long)]
        owner: Option<String>,
        /// For `duplicate`: the item it duplicates (item:slug).
        #[arg(long)]
        of: Option<String>,
        /// For `holds`: the item restated in the frame the re-check found true; kept beside the verbatim import.
        #[arg(long)]
        restate: Option<String>,
    },
    /// Bring outside material in. `--tracker` imports a tracker file's items as unchecked items.
    Bring {
        path: String,
        /// Import the file as a tracker of work items.
        #[arg(long)]
        tracker: bool,
        /// The item unit in that file: bullets (top-level, after the first `##`) | checkboxes | sections | rows.
        #[arg(long, default_value = "bullets")]
        unit: String,
        /// Heading level for `--unit sections`.
        #[arg(long, default_value_t = 3)]
        level: usize,
    },
    /// What is open: findings and questions, keyed by the statement they concern, oldest first.
    Open {
        /// Only items about this statement (kind:slug).
        target: Option<String>,
    },
    /// Open findings; or dispose of one that will not be fixed.
    Findings {
        /// Dispose of this item (review:slug#F1).
        #[arg(long)]
        dispose: Option<String>,
        /// rejected | known-defect
        #[arg(long, value_name = "DISPOSITION")]
        r#as: Option<String>,
        #[arg(long)]
        why: Option<String>,
        /// Confirm this finding (review:slug#F1) answered by a revision — by someone other than the reviser.
        #[arg(long)]
        answered: Option<String>,
        /// The revision that answers it: a version prefix, an event id, or the successor record.
        #[arg(long)]
        by_revision: Option<String>,
    },
    /// The second mind: confirm that a reworded statement still says what the checked version said.
    Confirm {
        /// The record whose rewording you judge (kind:slug). With nothing: list what awaits a second mind.
        record: Option<String>,
        /// The version the checks read (prefix). Default: every pending pair on the record.
        #[arg(long)]
        from: Option<String>,
        /// The version now (prefix). Default: the current one.
        #[arg(long)]
        to: Option<String>,
        /// Your judgment, if not asked interactively: equivalent | for-checks | not-equivalent.
        #[arg(long)]
        judgment: Option<String>,
        /// What you compared, in a sentence (required for `for-checks`; welcome otherwise).
        #[arg(long)]
        criterion: Option<String>,
        /// Judge only these checks (review:slug#C1 item refs or event-id prefixes; repeat or comma-separate). The rest stay pending.
        #[arg(long, value_delimiter = ',')]
        checks: Vec<String>,
    },
    /// Record that a statement was shown false, on evidence; shows what rests on it.
    Refute {
        target: String,
        /// kind:text — counterexample, broken-derivation, missed-prediction, broken-mapping.
        #[arg(long)]
        ground: String,
        /// The review item or event that showed it (review:slug#C2).
        #[arg(long)]
        by_item: Option<String>,
        /// The text was read at this git revision (for a target that has since gone).
        #[arg(long)]
        at: Option<String>,
    },
    /// Say what you are about to do to a record, visibly to every agent and worktree now. Refused if another agent holds it (unless --alongside).
    Claim {
        /// Records (kind:slug), one or more.
        records: Vec<String>,
        /// What you intend, in a sentence.
        #[arg(long)]
        intent: String,
        /// edit (exclusive by nature) | record (additive: findings, items) | free text
        #[arg(long, default_value = "edit")]
        kind: String,
        /// How long the claim lives before it expires: 6h, 90m, 1d.
        #[arg(long, default_value = "6h")]
        ttl: String,
        /// Record a co-intent beside another agent's live claim instead of being refused.
        #[arg(long)]
        alongside: bool,
    },
    /// Release your claim on records (or anyone's expired one; --force with --why for a live one).
    Release {
        records: Vec<String>,
        #[arg(long)]
        force: bool,
        #[arg(long)]
        why: Option<String>,
    },
    /// Every live claim: who is at work on what, where, since when, and what they said.
    Intents {
        /// Include expired claims.
        #[arg(long)]
        all: bool,
    },
    /// For hooks: given file paths, report records claimed by someone other than --by. Exit 1 if any.
    Claimed {
        paths: Vec<String>,
    },
    /// Say that an existing item or finding was raised again from another source, without editing it.
    Raise {
        /// The item (item:slug) or finding (review:x#F1).
        target: String,
        /// Where it was raised again (a file, a review, a line).
        #[arg(long)]
        source: String,
        #[arg(long)]
        why: Option<String>,
    },
    /// Correct the record of an act without editing it: a correction event that names the original.
    Annotate {
        /// The event (id prefix) or review item ref (review:x#C1) being corrected.
        event: String,
        /// The correction, in a sentence.
        #[arg(long)]
        why: String,
    },
    /// Everything that can be checked. Exit 1 when there are items to act on.
    Check {
        /// Also count edits not yet taken in and checks that have gone stale.
        #[arg(long)]
        strict: bool,
    },
    /// Drop stored statement versions that the durable ref (origin/main) can reproduce, or that no event cites.
    Gc,
    /// Where every cited version lives: kept as an object, on the durable ref, on another ref only, or lost. --keep seals the "another ref only" ones as objects.
    Anchors {
        #[arg(long)]
        keep: bool,
    },
    /// One-time store migrations.
    Migrate {
        /// Drop creation events (mint, import, ask, note) and revise events no check rests on; events then say only what the tree does not.
        #[arg(long)]
        prune_creation_events: bool,
    },
    /// A record as it stands, with marks for what is known about it.
    Read {
        id: String,
        /// The text at a version vsect has seen.
        #[arg(long)]
        at: Option<String>,
        /// The text alone: no marks, no Working Notes.
        #[arg(long)]
        bare: bool,
    },
    /// The history of a statement: every recorded act on it.
    Why { id: String },
    /// What rests on a statement, and what a refutation would do to each.
    Impact { id: String },
    /// The store's declared kinds, edges and lines.
    Kinds,
    /// Help on a verb, or `help model` for the logical model.
    Help { topic: Option<String> },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match verbs::run(cli) {
        Ok(code) => {
            let written = event::drain_written();
            if !written.is_empty() {
                println!("Wrote {} path(s); commit them by path: {}", written.len(), written.join(" "));
            }
            ExitCode::from(code as u8)
        }
        Err(e) => {
            eprintln!("vsect: {e:#}");
            ExitCode::from(2)
        }
    }
}
