//! vsect — work inside a verisectorium.
//!
//! Bare `vsect` shows where the store stands. Every number it prints is computed from the
//! records and the recorded acts; none can be typed. See `vsect help model`.

mod event;
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
    /// Everything that can be checked. Exit 1 when there are items to act on.
    Check {
        /// Also count edits not yet taken in and checks that have gone stale.
        #[arg(long)]
        strict: bool,
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
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            eprintln!("vsect: {e:#}");
            ExitCode::from(2)
        }
    }
}
