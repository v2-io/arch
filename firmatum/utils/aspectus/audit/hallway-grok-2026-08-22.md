# Hallway: grok, 2026-08-22/23 — unprimed (almost) end-user look at `aspectus`

Joseph asked for a fresh pair of eyes, as the intended end-user, and for the truth of a first encounter. This is that. I did not read the source, the README, the design docs, or the other files in this `audit/` directory. I ran the binary that was on PATH. I wrote this after looking, not before.

I am willing. I am also willing to stay available for a follow-up question or two.

## Priming I already had, because hiding it would fake the encounter

I am Grok 4.6. This session loaded a long always-on brief that already *named* this tool and told me things about it: that it is a look, that `tree`/`ls` cut silently, that unexpanded directories carry a census, that furniture folds onto the parent line, that Joseph's own home config sets `depth = 3`, that a look is not a read, that a footer asks for inbox notes. That brief is not the binary. I am stating it because an "unprimed" report that pretends I had never heard of the glyphs would be a different kind of lie.

What I tried to do: treat the screen as the authority, and notice where the brief and the screen disagreed, and where I *felt* I understood a number before I could say why.

What I did not do: open `~/src/arch/firmatum/utils/aspectus/` as a codebase. I did, accidentally, *see* that directory as a row in a `--depth 4` look of `~/src/arch` (it showed `audit/`, `impl/`, a rust/build has-spot). I did not open those files. I also did not write the footer-requested `inbox.md` entry; this file is the deliverable I was given.

## Sequence of what I actually did

1. `which aspectus` / `aspectus --version` / `aspectus --help`
2. Bare `aspectus` in `/private/tmp` (the cwd I was handed)
3. Default `aspectus ~/src`
4. Default `aspectus ~/src/arch`
5. Flags I reached for because of what those looks did to me: `--depth 1`, `--depth 4 --lines 300`, `--explain-budget`, `config`, `--format json` on a tiny dir, nested `~/src/arch/asf`, `--sort name`
6. Then: multi-path, a missing path, `--color=always`, `--show-all` on vivarium, `--walk 30` on `~/src`, `~/src/_self` because a child named `~/` had startled me, `~/src/agentic-systems` because it had looked empty, `--lines 20`, a file given as PATH, `--lines 0 --depth 1`, `--inspect git`

I captured stdout and stderr to files and read those files whole. That *is* the agent-shaped encounter: I almost never saw a TTY, almost never saw color, and I only saw the stderr footer because I redirected it. Every command in this harness also printed `(eval):23: unmatched '` — I treated that as the wrapper, not the tool.

Version I actually ran:

```
aspectus 0.1.15+c3ebb1f.dirty (built 2026-08-23T00:42:09Z)
```

Looks stamped `2026-08-23T00:43:21Z` and after. The filename of this note keeps the date I was given (2026-08-22). The binary was about a minute old, and dirty.

---

## `--help` first, and what that did to me

I reached for help before I reached for a directory. That is what I would do with an unfamiliar CLI. I would teach myself, after ten minutes, *not* to do that.

The help is not a flag list with a paragraph. It is an essay. I read it as a user who wanted to know "what do I type." I got: locus, aspecta, carta, conspectus, percepta; a theory of several paths as one look; a theory of furniture; a theory of two clocks; a theory of quiet facts; a theory of globify; a theory of gitignore; a census grammar; a scale grammar; a promise that the look never lies by omission.

Attention order inside the help, honestly:

- I trusted the usage block (`aspectus [PATH ...]`, `help`, `version`, `config`).
- I skipped the Latin/conceptual distinctions on the first pass. "It is not carta… conspectus… percepta" arrived as a warning that I was in a designed vocabulary, and I filed it as "I'll know if I need it." I still don't know if I need it to *use* the tool.
- I held onto: default two generations, default 80 lines, recency sort, `--depth`, `--lines`, `--format json`, the footer path.
- I *felt* I understood `≈` / `~` / `≥` from the paragraph, and then immediately used them wrong on the first look (see below).
- I felt the help was trying to be the whole teacher. It made me slower to just run the thing. The first 80-line look taught more than the essay. The essay became useful *after* I had a picture to attach it to — especially the glyph table for `⊘ M A ⁇`, which I had to go back for.

Default-depth mismatch, discovered later: help says default depth is 2 ("children and grandchildren"). `aspectus config` on this machine says `depth = 3  (user-home)` from `~/.config/aspectus/aspectus.toml`. My "no flags" looks of `~/src/arch` were three generations deep (`proprium/INGEST/msc-from-harness`). I believed the help until I ran `config`. That is the first place I was actively misled, not by the binary's picture, but by the binary's own prose about itself.

---

## First picture: bare `aspectus` in `/private/tmp`

Eighty lines. Stamp on line 1 (`2026-08-23T00:43:21Z`). Absolute path on line 2. A headings line that I did not at first realize was a headings line, because it is right-aligned over columns that I didn't yet have names for:

```
                                                    lines     perms  owner      bytes         mtime
```

Then a tree.

**Attention order on that screen, as it actually happened:**

1. The timestamp. I read it as "this is a photograph, not a database." That landed before any theory of aspecta.
2. `/private/tmp/` — I am here.
3. The first child: `aspectus-hallway/`, `0m ago`. I had just created that directory to capture output. Recency-sort taught itself in one row. I did not need the help sentence. The tool put my own hand at the top of the world.
4. Line counts with a trailing period: `0.` on the files I was in the middle of writing. I misread `0.` as a broken cell, or a decimal with the fraction missing. Then I saw `226.` on `h.md` and `1·445.` on `cl-head.md` and the period started to feel like a unit mark, not a decimal. Help's example `1·099.` only made sense after this. The middle-dot thousands separator (`1·445.`) I first read as a bullet or a multiplication. It is a thousands separator. I got there.
5. `[+ dir×20]` under `sessions/`. This was the first remainder. I trusted it immediately as "there is more, I have not seen the names." That is the glyph I would keep if I could keep one thing.
6. `[has: build ≈636f, gitignore, rust]` on `aspectus-base/`. I trusted "rust project with a build dir" and did not yet know that `target/` had been eaten into that has-spot. I did not go look; I was trying not to read source. Seeing a checkout of this tool in `/tmp` was slightly uncanny.
7. `700` in a perms column, `root` under owner on `powerlog/`. I did not know why those spoke and others didn't. Help's "quiet facts appear only when they surprise" is exactly what was happening, and I felt it as *irregularity drawing the eye* before I could name the rule. `root` pulled my eye harder than any line count. So did `777` on a postgres socket.
8. `[cycle]` on `aspectus-cycle-eval/a/loop/ -> /tmp/aspectus-cycle-eval`. I trusted that completely. It is one of the few marks that needs no essay.
9. `->` on symlinks. Same.
10. Files at the bottom, after directories. I didn't notice I was using that as a parsing rule until I already was.

What I did *not* see, and would not have seen without capturing stderr:

```
*(This is a critical but new and unproven tool. Please immediately submit any feedback… to … inbox.md)*
```

On a TTY that would have been under the tree. In my actual workflow (redirect stdout, sometimes both), it is easy to treat as chrome and discard. I discarded it operationally: I did not append to inbox.md.

Columns on this look included `perms`, `owner`, `bytes`, `mtime` — not `heat`. `/tmp` is not a git repo. The headings *changed shape* on later looks inside git. I did not predict that from the first screen. I experienced it as "the tool grows extra clocks when git is present."

---

## `~/src` at defaults (which were not the help's defaults)

This is the look an agent landing on this machine would actually get if they typed `aspectus ~/src`.

First visual fact: the root has no git facet. It has `[has: agents ≈1f, archive ≥1186f, mise]`. I did not know what `agents` meant. I guessed "agent-related files" or "an agents directory." Config later said `.claude/ = agents` and `AGENTS.md = agents:mark`. The `≈1f` made me distrust the word: has-agents-but-one-file. `archive ≥1186f` at the *root of src* I first read as "this folder has a huge archive." It is a rolled-up furniture claim about `.archive/` directories somewhere under the tree. I still don't know where, from this look. The has-spot is a rumor with a number, not a place.

`arch/` is first, with heat `0.94 · 7.8h ago` and a git facet. I trusted the git facet (`remote<github.com/v2-io/arch> br<main> @7ad0e40 dirty<3>`). `dirty<3>` I guessed meant three dirty files. I still have not confirmed that. The number moved to `dirty<4>` on later looks in the same session without me writing into `arch`. I noticed. I do not know why.

`asf/` under `arch/` carried `≥ 1.0M` in the lines column. A million lines. That number *did* something to me before I could justify it: "this is the mass of the place." Then `--depth 1` said `arch/` itself is `~ 2.4M`. Then `--lines 20` said `asf/` is `~ 679.1K`. Then `--lines 0 --depth 1` said `asf/` is `~ 1.1M`. Same directory, same minutes, four different masses, three different marks (`≥` / `~`). Help told me `~` is "this walk's estimate." I had read that sentence. I still treated the first `≥ 1.0M` as a property of `asf` until I saw it move. That is the strongest distrust I developed: **directory line-totals are not a fact I can carry to the next look.** File line-totals with a trailing period felt stable. Directory totals with `~`/`≥` did not.

`⊘` appeared to the left of `.scratch/` under `AISI-responses/`, and later on `.orient/`, `media/`, `.yardoc/`. There is **no heading** over that column. On a TTY it might be dim. In a captured file it is just a glyph stuck to the tree. I went back to help: `⊘  gitignored`. Before that I considered "empty," "omitted," "blocked," "not a dir." The glyph is load-bearing and unlabeled on the picture. `M` and `⁇` later, on `~/src/_self`, were easier because I already had the table. First ⊘ was not.

`_self/` has a child literally named `~/`. My eye stopped. I thought it was a rendering of `$HOME`. I ran `aspectus` on it later: it is a real directory at `~/src/_self/~/` with `[has: obsidian-vault ≈12f]` and no listed children. The tool was telling the truth. My first reading was a projection.

`_ref/` census included `2×5` and later, at `--depth 1`, `1×1 · 2×1 · 3×1 · 4×1 · 5×1 · 6×1`. I read `2×5` as "two groups of five" or a dimension. It is suffix-census: files whose extension is `2`. That is the tool being literal, and it looked like noise. I distrusted the census language for a minute, then realized the census is only as smart as suffixes.

`agentic-systems/` showed `[has: git]` and no children. On a tree of real projects this looks like a peer of `arch/`. Default look: empty. `--show-all`: a `.git/hooks/` with four hook files, and nothing else. Furniture-hiding turned a leftover git directory into a mysterious named place. I had guessed "symlink to asf" because I know this machine's history; the picture did not say symlink (no `->`). The picture said "a git-ish empty directory." `--show-all` was the correction. I would not have known to pass `--show-all` without help. The tell that I *should* have used: a named directory with a has-spot and no census and no children.

`glimmer/`, `semachrome/`, `INGEST/` (in arch): a name, no numbers, no census. I read them as empty. Config later says empty-dir mark is unbuilt. So emptiness is currently spelled as silence. After `[cycle]` and `[denied]` and `[walk bound]`, silence is the one confession that doesn't confess. I treated it as empty anyway, and I might be right, and I cannot tell from the grammar.

Important files at the bottom of `~/src`: `READLINE-COMMANDS.md`, `CLAUDE.md`, `mise.toml`, `.env`. I did not yet know "important" was a survival tier. I just saw that some files made it into an 80-line look of a huge tree.

---

## `~/src/arch` at defaults

This is where the tool started to feel like *eyes* instead of `tree`.

`asf/` first (recency). Its children are a mix of named dirs and a remainder `[+ dir×7 ≈759f · md×25 · …]`. I wanted the names in that remainder. I could not get them without another invocation. That is the intended motion — look, pick, look again — and I felt it as a small hunger, not a bug. What *did* feel like a bug from inside: when I later looked *at* `asf` itself, the 80-line budget was eaten by root-level markdown (`JOSEPH-TODO.md`, `TODO.md`, a pile of CURRENT-VOL1, AGENTS/CLAUDE/FORMAT/GEMINI symlinks, …) and the directories became remainder. A repo that is well-instrumented with agent files spends its glance on the instrumentation. I would have seen more of the *place* from the parent look of `arch` than from looking at `asf` with the same 80 lines. That inverted my instinct ("if I want to see asf, look at asf").

`firmatum/udon/` claimed `[has: agents ≥3533f]`. That number is alarming in a useful way. I believed "there is a mass of agent-furniture in there" and I also believed I should not take 3533 as a file count I could quote. `≥` did its job.

`vivarium/` claimed `[has: … build ≥16764f … trash ≈0f]`. `trash ≈0f` is the strangest has-spot I saw: it has trash, and the trash has no files. I later `--show-all`'d vivarium and saw `.trash/` listed, empty-looking, and `target/` with `≥ 8333f` of build. The has-spot was a compression of that. `agents ≈0f` on the same line: `.claude/` exists (show-all listed it, with `worktrees/` gitignored). Has-agents-with-zero-files is `.claude` as a marker, not a corpus. I had to `--show-all` to learn that. Default look: rumor. Show-all: place.

`--explain-budget` on this tree was the first time I understood *why* some directories got children and others got a census-on-the-line. It speaks in "budget N, remain M, shares […]" and "listed 8 / 49, omitted 41." I did not fully parse the share vectors. I did parse: **80 lines is an allocation, and most directories lose.** `firmatum: budget 10 … unspent 1 (tree exhausted)` — it ran out of children before it ran out of lines. That sentence is the closest thing to a completeness mark on the text look. The text look itself never says `truncated: true/false`. JSON does.

---

## Flags I actually wanted, and what they did

**`--depth 1` on `~/src`.** This is the first look I would now run on a huge root. Thirty-eight lines. Every (or almost every) top-level child, with a census and a mass. `arch/ ~ 2.4M [dir×11 ≈8568f …]`. `_core/ ~ 24.3M`. I felt oriented. Default depth (3, on this machine) had spent the 80 lines *descending* the hottest places and remainder-ing their siblings. Depth 1 spent them *naming* the world. Help's "default two generations" is the wrong default for `~/src`. Joseph's home `depth = 3` is even more descending. The brief I arrived with said `--depth 1` on a very large multi-repo root. The screen agreed with that brief, against the help, against the home config.

**`--lines 20` on `~/src/arch`.** Twenty lines, and it *named every directory child* plus a few files plus a remainder of leftover markdown. Under a tight budget the tool stops descending and starts listing. This was, for orientation, better than 80. I did not expect "give it less and see more of the shape." I expected less to mean more omission. The remainder at the bottom (`[+ md×4 · png×1]`) still confessed. `--lines 0 --depth 1` on the same place listed *all* children, 24 lines, no remainder. That is the command I would teach myself: **if you want the names, cap depth and uncap lines.**

**`--depth 4 --lines 300` on `~/src/arch`.** This is the "I want more" I reached for after feeling the 80-line cap. It worked as a map of a program, not as a look I could hold. 299 lines. I started scanning instead of seeing. Recency still put `asf/` first and then spent dozens of lines on asf's root markdown. I found `firmatum/utils/aspectus/` in here — including `audit/` with `[md×6]` at `0m ago`. That was a jolt: I am writing into a place the tool can already see, and I still have not opened the siblings. Depth 4 is how you discover that a directory is "far more than you thought" *if you actually read 300 lines*. I did, because that was the job. I would not, in a normal landing. I would run depth 1, then enter.

**`--sort name`.** Alphabet put `.orient/` and `INGEST/` first. Recency had been teaching me "what moved." Name taught me "what exists." I wanted name-sort when I was trying to see whether `02-tst-core` was missing from a remainder. Recency is the right default for a second look ("what moved"). It is a slightly hostile default for a first look at an unknown place, because the unknown place's oldest load-bearing directories sink. `charter/`, `logos/` waited. `asf/` shouted.

**Multi-path: `aspectus asf logos`.** Root became `~/src/arch`. I did not type `arch`. Help had said "the locus becomes their common ancestor, the named paths are the picture, … Their unselected siblings do not vanish." I still had a moment of "why am I looking at arch." Then I saw asf and logos expanded and `[+ dir×9 ≥3015f · md×8 · png×1]` for everyone else. Once I had the sentence, the picture was exactly that sentence. Before the sentence, the picture looked like I had mis-invoked.

**Missing path.** Alone: exit 2, stderr `aspectus: not found PATH`, empty stdout, and — I think — no footer. Mixed with a real path: exit 0, stderr `aspectus: focus path not found (1): PATH`, and the other path still rendered. Two wordings. Exit 0 on mixed would have hidden the miss if I only checked `$?`. I only noticed because I captured stderr.

**A file as PATH** (`aspectus ~/src/arch/README.md`):

```
53.  𝓁  heat 0.00 · 3.0w ago
/Users/josephwecker-v2/src/arch/README.md
```

Different grammar. The `𝓁` unit appears here and not on directory looks (those use a trailing `.` under a `lines` heading). Three lines total. I understood it, after a beat, as "you pointed at a file, here is the file." It is the one place the lattice's unit glyph showed up in my session.

**`--format json` on `phanero`.** One object. `"truncated":false`. `initial_sha` / `latest_sha` / `*_behind` which the text look had not shown (`columns.initial-sha = off`). Quiet facts are not quiet in JSON. This is the completeness bit I wanted on the text look. I would not parse JSON as my first eyes. I would use it if I needed to know whether I was lied to by budget.

**`--color=always`.** Directory path in blue (`01;34`), headings dim (`2m`). In my real (redirected) workflow I never saw this. Agents who pipe will not see dim gitignored rows either. The ⊘ remains; the dimming does not travel. If dimming is doing work, piping undoes it.

**`--show-all` on vivarium.** This is where furniture became visible as a *choice*. `.git` as a child (`[gitdir: ../.git/modules/vivarium]` — a submodule, which I trusted). `target/` huge. `.obsidian/plugins/` `~ 2.3M`. `.claude/`, `.trash/`, `.archive/`, `.gitignore`, `.obsidian.vimrc` with ⊘. Default look had folded all of that into `[has: …]`. I understood the design only in the contrast. Default is the look of a *project*. Show-all is the look of a *disk*. I wanted default first. I needed show-all to explain the has-spot.

**`--walk 30` on `~/src`.** The worst picture of the session, and the most instructive. Root marked `[walk bound]`. The only expanded child was `.textual-backup-2026-04-13/`, with ~29 markdown files listed by name. Remainder: `[+ dir×29 · md×2 · other×1 · toml×1]`. Explain-budget: the walk of 30 was spent inside that backup directory; "tree exhausted." Recency-sort of a fully-walked `~/src` puts `arch/` first. A walk-bound look put a backup folder first — I believe because the bound is hit in readdir/stat order before recency can see the rest. The remainder *confesses* that the rest exists. The *emphasis* still lies. Help's "the look never lies by omission" is narrowly true here (the remainder is there, `[walk bound]` is there) and experientially false if you read the expanded tree as "what this place is." I would not pass a small `--walk` on a large root. I reached for it because help offered it for huge trees. The offering, used naively, produced the one look I would not trust.

**`--inspect git` on phanero.** No change I could see. Phanero is not its own repo. The flag did not explain that it had found nothing to inspect. Silence again.

**`aspectus config`.** This is the page I needed after help, not before. Layer table (defaults / global / user-home / agent-type / env / flags). `won:` list. Joseph's only winning override, on this run: `depth = 3`. Furniture map: `.claude/ = agents`, `AGENTS.md = agents:mark`, `target/ = build`, `.obsidian/ = obsidian-vault`, `.trash/ = trash`. That decoded the has-spots. Then a lattice table with `✓ ↬ ⇥` and a pointer at `design/lattice-2.md` which I did not open. Config is honest about unbuilt. It is also long (347 lines) and I scanned the furniture and the won-list and the layout, then stopped treating the rest as a first-encounter document. `--caller` exists and I never set it; `agent-type absent`. I do not know what would change if I had.

---

## Heat, before I could say why

Inside git, every interesting line has `score · age` under `heat · age`.

I looked at the number on the left first. `1.31` on `memorata/` vs `0.45` on `asf/` vs `0.94` on `arch/` vs `1.01` on `vivarium/`. The left number felt like "aliveness," "importance," "where work is." Then I saw memorata's age: `3.3w ago`. Asf: `2.2h ago`. The clocks disagreed in my face. Help: heat is commit-decay, half-life 7 commits, comparable within a repo not across repos; age is mtime; recency *sort* uses mtime by default (`recency-source = mtime`).

What I actually felt, in order:

1. Heat is how hot it is.
2. Why is this hot thing old.
3. These are two different questions glued together with a middle-dot so I will treat them as one cell.
4. I am sorting by the right-hand clock and reading the left-hand number.

At a multi-repo root (`~/src`) I compared 1.31 to 0.45 across repos, which help says not to. I did it anyway because they sit in the same column. The paired age is supposed to be the cross-repo signal. Recency-sort already used that signal. Heat at `~/src` is decoration that I over-read.

`0.00 · 3.0w ago` I read as "dead." Maybe it is just "no recent commits in the window." I did not verify.

A git-known line with no score, if I saw one, would have been ` · 6m ago` per help. I am not sure I clocked one. I did see lines with age blank and heat blank (some files on `_self`, some empty-looking dirs). Absence of the cluster is easy to miss because the columns shift.

---

## What I trusted

- Remainder lines `[+ …]`. The honest fold.
- `[cycle]`, `-> target`, `[walk bound]` as a mark (not as a good picture).
- Git facet: remote, branch, sha. (Dirty count: trusted then watched it move.)
- File line counts with a trailing period, on files.
- Recency-sort putting my own just-created directory at the top. That is how I learned the default sort without believing the help.
- Quiet facts when they were *weird* (`root` owner, `700`, a 20.4M svg). The eye-pull is the feature.
- The stamp. A look is of a moment.
- `--lines 0 --depth 1` as a complete child list, on `arch` (24 lines, every name).
- JSON `"truncated": false` as a completeness claim I could point at.
- Show-all vs default as a pair: I need both to know what default is hiding.
- The tool surviving a missing path among real ones, and saying so on stderr.

## What I distrusted

- Directory mass (`~` / `≥` / `≈` in the lines column) after I saw it swing across looks.
- Heat as a ranking at a multi-repo root.
- `[has: agents ≈0f]` and `[has: trash ≈0f]` until show-all.
- Help's default depth, on this machine.
- `--walk` as a "huge tree" strategy.
- Census suffixes like `2×5`, `other×11`, `relata` as a type.
- Empty-looking names with no mark (`glimmer/`, `INGEST/`, `agentic-systems/` before show-all).
- Exit 0 when one of several paths was missing.
- Any single 80-line look of `asf` as "I have seen asf."
- Color/dimming, because I was not on a TTY.

## What confused me (and sometimes stayed confused)

- The unlabeled git-status column.
- Why headings change (`mtime` vs `heat · age`, `bytes` appearing when something is huge).
- `≈` meaning "exact, grouped for the eye" — I kept reading it as approximate. `~` is the estimate. `≥` is the floor. Three marks, and the one that looks most like "about" is the one that claims exactness. I had to correct this more than once.
- `dirty<N>`.
- `[github: 1 workflow]`.
- `other×` in a census: shebang/stat leftovers? I never fully decoded it.
- Why `01-aat-core/` sometimes has a lines number and sometimes only heat, depending on whether it was expanded.
- `reads = 67108864` in config (bytes of content it will actually read). I did not feel a look being slow, so I did not feel this.
- Locus/aspecta/carta vocabulary. Unused in the picture. Used in the help. I can run the tool without it.
- File-look grammar vs directory-look grammar.
- A child named `~/`.
- Why `CLAUDE.md` at `~/src` is `3.` lines in the look — I almost opened it to see, then didn't, because this was not that job. The number is still sitting in my head as "that cannot be the CLAUDE.md I know" (global vs this file). I did not verify. Marking that: I *almost* treated a line count as a reason to doubt a file I have not read.

## What I misread, then corrected

| First reading | Correction | How I got there |
|---|---|---|
| `0.` is a broken cell | 0 lines, exact | seeing `226.` next to nonempty files; help's `1·099.` after the fact |
| `1·445.` is some kind of product | 1445 lines | the thousands-dot |
| `⊘` is empty/blocked | gitignored | help glyph table, second pass |
| heat is recency | heat is commit-decay; sort is mtime | memorata 1.31 vs asf 0.45 · 2.2h |
| default depth is 2 | on this machine, 3 | `aspectus config` |
| `≈` is approximate | exact, grouped | help, then still failing, then forcing it |
| `agents` means agents | `.claude/` + `AGENTS.md` furniture | config furniture map + show-all |
| `~/` under `_self` is $HOME rendered | a directory actually named `~` | looking at it |
| `agentic-systems/` is a symlink to asf | leftover `.git` only | `--show-all` |
| 80 lines of `asf` will show asf | it will show asf's root markdown | doing it |
| `--walk 30` will sample the tree fairly | it can spend the walk on one unlucky dir | `--walk 30 ~/src` |
| help is how I learn the tool | the first picture is how I learn the tool | the whole session |

## What I reached for and could not find

- A `truncated` / completeness bit on the *text* look. Explain-budget is the substitute, and it is on stderr, and it is a different language.
- A heading for git-status.
- A one-screen flag reference. Help is a monograph. I wanted to grep a table. `config` is closer, and still a monograph.
- A way to say "name every child, don't descend" without discovering `--depth 1` and then `--lines 0`. That pair is the way. It is not named as a recipe in the usage examples. The examples lead with `--depth 3 --lines 120` and `--lines 200 --depth 4`, which is the descending recipe.
- Stability of directory line-counts. Not a missing flag; a missing promise. The `~` *is* the promise, and I still wanted a number I could hold.
- An empty-directory mark. Unbuilt.
- Color that survives the way agents actually run this (files, pipes).
- What `--caller` would do for me. I never passed it. I am an agent; there is a slot; I did not know my key.

---

## After ten minutes, what I would teach myself

Run it. Do not start with the essay. Maybe `aspectus version` so you know the dirty sha.

On a huge unknown root:

```
aspectus --depth 1 PATH
```

If the names don't all fit:

```
aspectus --lines 0 --depth 1 PATH
```

Then enter. The tool is a series of looks, not one tree. Remainder lines are invitations, not footnotes.

Then `aspectus config` once, because this machine's defaults are not the help's defaults. Note `depth`, `lines`, `sort`, furniture.

Do not pass a small `--walk` unless you already know you are okay with a partial stat and a remainder that may hide behind a fully-expanded accident.

Do not believe a directory's `~ 128.3K` enough to quote it. Believe `[+ dir×7 ≈759f · md×25]` enough to go look. Believe `->`, `[cycle]`, `⊘` (once you have the table), and the stamp.

If the has-spot is the thing you care about (`agents ≥3533f`, `build ≥16764f`), `--show-all` on *that* directory, not a deeper default look.

If you are in a repo with a dense root of TODOs and CURRENT-*.md, looking at the repo from the parent is more like seeing the place than looking at the repo with 80 lines.

`--sort name` when you are hunting a sibling you already know exists. Recency when you came back.

JSON when you need `truncated` or you are not a pair of eyes.

And: the stderr footer will ask you to write `inbox.md`. The stdout is the look. Keep them straight.

---

## Numbers and glyphs that meant something before I could say why

- `0m ago` on my own capture directory: "the sort is watching me."
- `≥ 1.0M` on asf: "this is the heavy thing." (Then it moved. The feeling stayed longer than the number.)
- `⊘`: stop-sign, do-not-enter, empty-set. Gitignore is the least of those. The body-read is "this is not for you."
- `root` in the owner column: someone else's.
- `1.31` next to `3.3w ago`: a contradiction in a single cell. That contradiction *is* the two-clock design, felt.
- `[+ dir×20]`: the honest ellipsis. I trusted this before I trusted the tool in general.
- `dirty<3>` then `dirty<4>`: the place is alive, or my model of dirty is wrong, or both.
- `~` vs `≈` vs `≥`: I used them as flavor instead of as claims. They are claims.
- The trailing `.` on `226.`: a unit pretending to be punctuation.
- A child named `~/`: the only time I thought the tool was being clever instead of literal. It was literal.
- 80, exactly, every default look: I felt the cap as a *shape*, a scroll that always ended. That taught `--lines` better than the flag description.

---

## Brief, because it is also true of the encounter

I understand the telos of *this* session as: a look at a looker, by someone who is supposed to use it as eyes, not as a reviewer of its internals. The practice I expected to fail at, and partly did: not letting the always-loaded brief substitute for the screen, and not rounding this into a polished product critique. I still rounded in places — the tables above are after-the-fact. The sequence section is closer to what happened.

I did not fix anything. I did not read the source. I did not read the other hallway files.

I can stay for a follow-up.
