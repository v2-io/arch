# Archema member-repo migration — concerns, plan, and the `mv-src-repo` utility spec

*2026-07-10, drafted with Joseph the morning after the program repo was founded. Status: **decisions made (§1), execution pending** — delegable per §1b. The general utility (§5) is motivated by this migration and the already-queued second one (archema-io → archema).*

## 0. Target shape

```
~/src/archema-io/                     # the program repo (→ ~/src/archema later, §4)
  CHARTER-DRAFT.md, charter/, _program-seed/, MIGRATION.md
  asf/        ← submodule = v2-io/agentic-systems   (remote name unchanged)
  <form?>/    ← submodule = v2-io/<renamed synthese-paper>   (§1.2 names)
  vivarium/   ← submodule = v2-io/vivarium          (name parity, unchanged)
```

Members remain fully independent repos with their own laws (charter §1); the parent adds one thing of real epistemic value: **pinned program states** — a parent commit records exact member commits, so "the program as ratified/audited/cited on date X" is one hash (the content-addressed / in-vivia pattern applied to the program itself).

## 1. Decision points — DECIDED (Joseph, 2026-07-10)

1. ✅ **Submodules**, operated loosely (pointer bumps = deliberate program-snapshot commits).
2. ✅ **`logos`** — submodule path `archema-io/logos/`, GitHub repo renamed `v2-io/synthese-paper` → `v2-io/logos`. (The `form-*` prefix collision argument carried.)
3. ✅ **Snapshot (bundles + ignored/untracked tars) + mv + old-path symlinks** — no parked clones in `_ref/`.
4. ✅ **Safe-to-edit set as proposed** (member repos, ops, rowan, archema-io, `~/.claude/`); protected: `_core/`, `eli/`, `firmatum/`, all history layers. *(Standing unless Joseph amends.)*
5. ✅ **Utility naming**: plain-spoken — **`mv-src-repo`** in `archema-io/utils/`, not a Latin name. Rule recorded: Latin names are reserved for fundamental things that deserve a new name (memorata was a special case — itself a PROPRIUM memory-mechanism experiment); system tools say what they do. §5 updated accordingly.
6. ✅ **Rowan reflections INTERNED (not moved) 2026-07-10** (§6 item 1 resolved): the five 2025-12-18 consciousness-infrastructure reflections copied to `agentic-systems/msc/reflections-rowan-2025-12/` with provenance README; **originals remain in rowan (their proper home — Joseph)**; the canon citation in `def-death-as-factor-loss` re-pointed at the interned copy (lint clean). General rule harvested for the plan: anything ASF references or could reference gets interned into ASF, since we can.

<details><summary>Original decision-point analysis (for the record)</summary>

1. **Structure: submodules (recommended) vs plain nested clones vs symlinks.** Submodules give recursive clone + pinned states; their frictions (detached HEAD after parent checkout; two-step commit-then-bump) are mitigated below (§3.8) and mostly vanish for a single-operator setup where you simply *work inside the member as always* and bump pointers when you want a snapshot. Nested-ignored clones avoid all friction but lose pinning and recursive clone. Symlinks-only loses the unified tree. Recommendation: **submodules, operated loosely** (pointer bumps are deliberate "program snapshot" commits, not per-change chores).
2. **The philosophy member's name.** `form` works but collides with AAT's `form-*` segment prefix (`form-agent-model`, `form-objective-functional` — "the form segment" would become ambiguous in program-wide speech). Alternatives, same size and register: **`logos`** (the argued/word-facing register; the founding verse makes it nearly a dedication; adjacency to logogenic/logozoetic is resonance to record in the collision ledger, not a conflict), **`argumenta`** (the things argued — exact, -a family), **`disputata`** (quaestiones disputatae — the genre of rigorous argued questions; -ata family with memorata/relata/vestigia). Weak preference: **logos**. Joseph decides.
3. **Rollback substrate: parked clones in `_ref/` vs snapshot + old-path symlinks.** Recommended: **both halves of the safety, neither via parked clones** — (a) pre-move snapshot (rsync copy to `~/src/_ref/pre-archema-migration-2026-07-10/` *or* `git bundle` per repo + tar of untracked/ignored — bundle+tar is far smaller), then (b) **`mv` the live repos into place** (preserving uncommitted work, ignored content, hooks, reflog — a fresh submodule clone preserves none of that), then (c) **symlinks at the old paths** (`~/src/agentic-systems → ~/src/archema-io/asf`, etc.) so every stale reference, muscle memory, and not-yet-swept doc keeps working during the soak period. Delete symlinks after §3.10's soak; delete the snapshot after that.
4. **The safe-to-edit set for path sweeps.** Bulk find/replace is safe in: the three member repos, `ops`, `rowan`, `archema-io` itself, `~/.claude/` memory + CLAUDE.md. It should **not** run unsupervised in: `~/src/_core/**`, `~/src/eli/**`, `~/src/firmatum/` (cohort-sensitive, curated, partly historical record — stale paths there are *history*; the old-path symlinks carry them during transition, and a tombstone note at the end is better than rewriting the record). Joseph confirms the exact set; the utility takes it as config, not code.

</details>

## 1b. Delegation readiness (Joseph's question, answered 2026-07-10)

**Yes — delegable, with a specific shape.** The plan is self-sufficient for a careful executor *now that §1 is decided*, but three disciplines govern the delegation (per the standing sub-agent lessons: agents with Bash exceed analysis-only mandates; stage destructive steps):

- **One session, not a fleet.** This is sequential, stateful, journal-keeping work; parallel agents would race on shared paths. A single fresh session (ideally strong-substrate) with this file as its brief.
- **Phase gates, reported back.** Delegate as three checkpointed phases, each ending in a report before the next begins: **(A)** steps 1–3 (inventory + snapshot — nothing destructive; the report surfaces every uncommitted/unpushed/worktree finding for explicit carry-approval); **(B)** steps 4–6 (GitHub rename, mv+adopt, symlinks — the point of no easy return; gate on A's approval); **(C)** steps 7 & 9 (memory renames + sweeps, per-repo commits, grep-zero verification). The executor keeps the journal *in this file* (append a `## Journal` section, one line per action with its inverse).
- **Two steps stay with Joseph.** Step 8 (memorata/relata DB surgery — Joseph said he'll drive, and it doubles as the archema/rowan disambiguation pass, which needs human judgment on which "archema" is which) and the global `~/.claude/CLAUDE.md` project-map rewrite (judgment prose, not sed). The executor prepares both as proposals (the exact UPDATE statements; a drafted project-map diff) and stops.

Rollback stays live through phase C via the snapshot + journal. If Joseph prefers not to spend a strong session on it: phases A and C are safely delegable to a lesser substrate; phase B is twenty minutes of commands best run by whoever holds the most context or by Joseph directly from the journal's script.

## 2. The full concerns list

**Git / structure**
- [ ] Per-repo pre-flight inventory: `git status` (uncommitted + untracked), `git stash list`, unpushed branches (`git log @{u}..` on all branches), `git worktree list` (linked worktrees break on move — none expected, must check), submodules-within-members (none expected, check).
- [ ] Commit-or-carry everything movable first. Known uncommitted right now: agentic-systems `msc/reflections/29,30`; synthese-paper `revision-dossier-2026-07-09.md`, `supplementary-letter-draft-2026-07-09.md`, FEEDBACK.md pointer edit.
- [ ] GitHub: rename `v2-io/synthese-paper` → chosen name (GitHub redirects old URLs; update remote anyway). No GitHub changes for agentic-systems / vivarium.
- [ ] Move + adopt: `mv` repo into `archema-io/<name>/`; `git submodule add <url> <name>` (git adopts the existing clone when the dir is already a clone of that URL); `git submodule absorbgitdirs` (relocates `.git` into the parent's `.git/modules/`, preserving config/hooks/reflog); set `submodule.<name>.branch = main` and `git config submodule.recurse true`.
- [ ] Old-path symlinks (decision 3). Keep `~/src/vivarium` longest — most-referenced.
- [ ] `.gitignore`d content rides along with `mv` automatically — but diff the snapshot inventory anyway (ops-style gitignored payloads like `linkedin-archive-*` are the pattern to fear; synthese `.build/`, `refs.bak`; agentic-systems `env/`, `__pycache__`; vivarium `target/`, caches in `~/.cache/vivarium` are path-independent, unaffected).
- [ ] Parent pin commit after adoption ("program snapshot: members adopted at <shas>").

**Claude-side**
- [ ] Memory dirs rename (carries memory *and* session transcripts): `~/.claude/projects/-Users-josephwecker-v2-src-agentic-systems` → `…-src-archema-io-asf`; same for synthese-paper → new name; vivarium → `…-archema-io-vivarium`. (Old `.claude.bak.*` dirs: leave as history.)
- [ ] Memory *content* sweep: path references inside `~/.claude/CLAUDE.md` (many), `~/.claude/memory/**`, all project memory dirs (ops memory alone references all three repos repeatedly; vivarium memory references agentic-systems; the new archema-program memory entries).
- [ ] Repo-local `.claude/` settings move with the repos (fine); check global `~/.claude/settings.json` for path-keyed permission entries.
- [ ] Global CLAUDE.md **Project map table** needs manual (not sed) rewriting — it describes the world, and the world changed.

**Tooling / indexes**
- [ ] **memorata**: path surgery in the DB (Joseph will drive; helps archema and rowan at once) — inspect schema first, UPDATE path columns for the four mappings, then verify with searches that previously hit each repo; check memorata config/index roots too, and whether the indexer needs a re-run vs pure path-rewrite.
- [ ] **relata**: cross-project — check its config/DB for absolute bib-root/project paths (`relata --help` first per its own memory note); synthese-paper and agentic-systems are both wired.
- [ ] **vestigia / other tools**: grep configs for `~/src/(agentic-systems|synthese-paper|vivarium|archema)\b`.
- [ ] Obsidian: Joseph consolidates — remove three repo-level `.obsidian/` dirs, keep the global archema-io vault; also remove stale vault entries from Obsidian's own vault registry (app-level config).
- [ ] Editor/shell: VS Code workspace files, shell aliases, mise config, any cron/launchd — grep the dotfiles.

**Path sweeps (the four mappings)**
- [ ] `~/src/agentic-systems` → `~/src/archema-io/asf`
- [ ] `~/src/synthese-paper` → `~/src/archema-io/<name>`
- [ ] `~/src/vivarium` → `~/src/archema-io/vivarium`
- [ ] **`~/src/archema` → `~/src/rowan`** — already stale since yesterday, and *canon cites it*: e.g. `asf 04-eli-core/src/def-death-as-factor-loss.md` Working Notes references `~/src/archema/docs/msc/reflections/2025-12-18-consciousness-infrastructure.md` (a consciousness-infrastructure reflection that lives in what is now the *Ruby port* — decide whether that document should also *move* to a better home, e.g. firmatum or archema-io, rather than merely re-pointing).
- Sweep order: per-repo, one commit per repo with a standard message; verify-by-grep-zero afterward within the safe set; cohort dirs excluded (decision 4).
- [ ] Don't forget the *new* repo's own files: charter, substrate notes, concept-matrix, seed docs are full of `~/src/agentic-systems/...` etc. — archema-io is itself in the sweep set. (Its `~/src/archema-io/` self-references become mapping #5 during the later rename, §4.)

**Rowan (remaining tasks, tracked here so the archema rename can gate on them)**
- [ ] Gem/module rename: gemspec name, `lib/archema*` → `lib/rowan*`, module `Archema` → `Rowan`, binstubs, `require` lines, README body beyond the banner.
- [ ] Dependent internal projects: grep `~/src` for `gem "archema"` / `require "archema"` / `Archema::` and update Gemfiles + code (paths already changed to `~/src/rowan` in the banner's example; the *name* change is the second step).
- [ ] Claim `rowan` on RubyGems when first publishing (verified free 2026-07-08).
- [ ] Claude memories / memorata entries that mean the *schema project* when they say "archema" — re-point to rowan (part of mapping #4 sweep, plus a human pass over hits since some "archema" hits will mean the *program*).

**The second move (gated)**
- [ ] `archema-io` → `archema`: only after (a) all rowan tasks above are done, (b) zero `~/src/archema`-meaning-the-gem references remain in the safe set, (c) the soak period passed. Then: GitHub rename `v2-io/archema-io` → `v2-io/archema`, dir rename, memory-dir rename, memorata pass, sweep mapping `~/src/archema-io` → `~/src/archema`, update domains/README notes. **This is the utility's second run** — its existence is half the reason to build the tool.

## 3. Execution order (one sitting for §§1–8; the rest is soak)

1. Decisions §1 confirmed. 2. Pre-flight inventory (everything committed/stashed; worktrees none; disk space). 3. Snapshot (bundles + ignored/untracked tars, or rsync copy) to `_ref/pre-archema-migration-<date>/`. 4. GitHub rename of synthese-paper. 5. `mv` + submodule adopt + absorbgitdirs, ×3; parent pin commit. 6. Old-path symlinks. 7. Claude memory-dir renames; memory-content sweep; global CLAUDE.md project-map rewrite (manual). 8. memorata + relata surgery + verification queries. 9. Per-repo path sweeps (four mappings), one commit each; grep-zero verification. 10. **Soak (a few days):** work normally; log anything that breaks into this file. 11. Remove symlinks; after more days, delete snapshot. 12. Rowan tasks on their own clock → then §4 second move via the utility.

Rollback at any point ≤ 9: restore from snapshot + `mv` back + rename memory dirs back + revert sweep commits (they're isolated per-repo commits) + memorata inverse UPDATE. This is exactly the journal the utility automates.

## 4. Verification checklist (post-migration)

- `git -C ~/src/archema-io submodule status` shows three clean members at expected SHAs; `git fsck` clean in all four.
- A Claude session started in `archema-io/asf` sees the migrated agentic-systems memory (index loads; a spot-check memory file reads correctly).
- `memorata-search` returns hits with new paths for one known document per member; `relata --help`-verified operation from inside the moved synthese member.
- Grep for the four old paths across the safe set = zero (excluding history layers: CHANGELOG/LOG/_obs/audits/spikes/.integrated — history keeps its paths, per the same rule that keeps "AAD" literal in frozen archaeology; the symlinks + eventual tombstone cover those).
- Builds: `bin/lint-md` runs in asf; `cargo check` in vivarium; papers `bin/build` in the renamed member.

## 5. `mv-src-repo` — the general src-project mover (utility spec)

*Plain name by rule (§1.5): utilities say what they do. Lives in `utils/`; Ruby.*

**Shape.** A small CLI (Ruby, per the script-language convention), living at `archema-io/utils/mv-src-repo` (see `utils/README.md`). Config-driven, not hardcoded: a system-description file (checked into archema-io) declares the bespoke knowledge —

```yaml
# mv-src-repo.yaml (sketch)
memory_root: ~/.claude/projects
memory_globals: [~/.claude/CLAUDE.md, ~/.claude/memory]
safe_sweep_repos: [archema-io, asf, vivarium, <form>, ops, rowan]   # bulk-editable
protected_paths: [~/src/_core, ~/src/eli, ~/src/firmatum, "**/CHANGELOG.md", "**/LOG.md", "**/_obs/**", "**/audits/**"]  # never bulk-edit
indexes:
  memorata: {db: <path>, table/columns: <discovered>, post: verify-queries}
  relata:   {config: <path>}
symlink_old_path: true
snapshot: bundle+tar   # or rsync
```

**Verbs.** `mv-src-repo plan SRC DST` (dry-run: full ordered action list + everything it *would* touch, including per-file sweep hit counts); `mv-src-repo move SRC DST [--submodule-into PARENT] [--github-rename NEW]`; `mv-src-repo verify`; `mv-src-repo rollback <journal>`.

**Behavior requirements.** (1) Pre-flight gate: refuses on dirty status/stashes/worktrees/unpushed unless `--carry` acknowledges each finding. (2) Snapshot before any mutation. (3) **Journal**: every action appended to a journal file with its inverse; `rollback` replays inverses in reverse; every step idempotent so a crashed run resumes. (4) Sweeps are *word-boundary path* replacements, per-repo, each producing one commit with a standard message and a hit-count report; protected paths never touched; a "human-review" bucket for ambiguous hits (e.g. `archema` the program vs the gem). (5) Memory-dir rename derives the slug mechanically from the path (the `-Users-…-src-…` encoding). (6) Index surgery behind per-index adapters (memorata/relata), each with a built-in verification query. (7) `verify` runs the §4 checklist. (8) Old-path symlink creation + a dated reminder entry (in the journal) to remove them.

**Build order.** Don't build it *before* this migration — do this one semi-manually with the journal kept by hand (this file), harvesting each step into the spec; build `mv-src-repo` from the harvested reality; its first full run is the archema-io → archema rename (§2 last block), which is small, gated, and the perfect shakedown.

## 6. Open items appendix

- Where should the misfiled consciousness-infrastructure reflection currently at `~/src/rowan/docs/msc/reflections/2025-12-18-…` actually live? (It predates the split; it is not about the Ruby port.)
- Does ops' `papers/` tracking or `v2.io` reference `~/src/synthese-paper` paths anywhere public-facing? (Sweep will catch private; check public separately.)
- Whether vivarium's `spikes/worldview` env-var docs or any Rust code embed absolute paths (unlikely; one grep).
- Charter §9/§10 gain a line once the structure lands: how member-repo sessions relate to the parent (pointer-bump snapshots; where program-level docs live).
