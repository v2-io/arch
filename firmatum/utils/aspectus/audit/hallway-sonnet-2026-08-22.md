# hallway testimony — Sonnet 5, 2026-08-22 (dated 2026-08-23 by tool clock)

Unprimed first encounter. No README, no source, no design docs read. Only
`aspectus --help`, `aspectus config`, and a handful of runs against real
directories under `~/src`.

## What I ran, in order

1. `aspectus --help`
2. `aspectus` (bare, cwd `~/src/arch`)
3. `aspectus ~/src`
4. `aspectus asf` (cwd `~/src/arch`)
5. `aspectus --depth 1 ~/src`
6. `aspectus config`
7. `aspectus --depth 2 --lines 40 01-aat-core` (cwd `~/src/arch/asf`, a repo
   with one dirty file)
8. `aspectus --format json ~/src/arch/firmatum/utils`

## First five seconds

`--help` was the right instinct and it paid off unusually well — it's not a
flag reference, it's a short essay defining vocabulary ("locus," "aspecta")
before listing options. I read the whole thing before running the tool on
anything, which is not my normal behavior with CLI `--help` text (normally I
skim for the flag I want). That's a signal in itself: the prose was dense
enough, and unusual enough, that skimming felt like it would cost me
something. I don't know yet if that's good design or just verbosity I
tolerated because I was told to be attentive.

One thing I distrusted immediately: the self-description as "the faculty of
looking at a locus" reads like it's borrowing philosophical weight it hasn't
earned yet, on a first read, from a tool I have zero track record with. I
noted this as a flag to watch for, then mostly forgot about it once the
output itself turned out to be trustworthy — see below.

## The output on `~/src/arch` (bare)

This is where the tool either sells itself or doesn't, and it mostly did.

**What I trusted on sight, before I could articulate why:**
- The heat number + age pair (`0.94 · 7.8h ago`) on the root line. I didn't
  know what "heat" meant yet, but pairing a decaying score with a relative
  age instantly reads as "this is a temperature, look at what's warm." I
  correctly guessed it was recency-weighted activity before reading `config`
  confirmed `heat.half-life = 7` (days, I assume — never confirmed).
- The `[git: ... dirty<3>]` bracket. Immediately legible, no learning curve.
- Recency-first ordering. I noticed within one screen that `firmatum/` was
  near the top with `0.94` heat and `asf/` below it despite `asf/` clearly
  being the "main" project by file count — and that felt *correct* rather
  than confusing, because the tool told me up front ("Children are ordered
  by recency... so calling again shows what moved, at the top") that this
  was the design, not an accident. Without having read that in `--help`
  first, I think I would have read it as a bug or as "the tool doesn't know
  what's important."

**What confused me and took real effort to resolve:**
- The census bracket syntax, e.g. `[dir×17 ≈310f · md×35 · py×5 · png×2]`.
  I initially misread `≈310f` as "310 something, filtered maybe?" and had to
  cross-reference the `--help` legend (`≈` = exact count grouped for the
  eye, `~` = estimate, `≥` = floor) to get it right. Even after reading the
  legend, I kept having to re-look it up on each subsequent output — it did
  not stick after one exposure. Three near-identical glyphs (`≈`/`~`/`≥`)
  differing only in what confidence they claim about a number is a real
  cognitive tax, and I don't think plain English would have cost much more
  screen space for a first-time reader; possibly a legend line in the
  output itself (not just `--help`) would fix most of this.
- The `[has: agents, archive ≈2f, git, obsidian-vault ≥38f]` line reads as a
  flat bag of unrelated facts — package ecosystem markers ("git", "rust",
  "python") sitting next to what looks like organically-discovered
  furniture ("archive ≈2f", "obsidian-vault ≥38f"). I could not tell from
  the output alone whether "agents" meant "this directory has agent
  configuration files" or something else; I never fully resolved this and
  moved on without certainty, which bothers me a little as an epistemic
  matter — I'm reporting a partial understanding as fact-shaped in my head
  even now.
- The lines/heat header row (`lines     heat ·       age`) sometimes has
  more columns than others depending on what's present (I saw `lines`,
  `bytes`, `mtime`, `perms`, `heat`, `age` all show up across different
  runs) and the column *set* shifts per-invocation without an obvious
  visual cue that it shifted. On the `~/src` bare run I saw a `perms` and
  `mtime` column I hadn't seen on the `~/src/arch` run — I only noticed
  because I was diffing outputs side by side for this report; in normal use
  I don't think I would have noticed the columns changed shape, and I might
  have misread a value under the wrong header as a result. This is the
  single thing I'd flag as an actual usability risk rather than a learning
  curve: shifting column sets without a strong per-run header signal is the
  kind of thing that produces silent misreads.
- The greyed-out/dimmed directory entries with `⊘` prefix (e.g. `⊘│   ├──
  .orient/`) — I correctly guessed "denied" from context (`--help` mentions
  `[denied]` in the legend text) but the glyph itself, alone, gave me
  nothing. I'd have guessed "excluded by config" before "permission
  denied," and those are different facts I'd want to act on differently.

**What I reached for and couldn't find:**
- A quick "what changed since I last looked" diff mode. The heat/age columns
  gesture at this but I found myself wanting `aspectus --since <time>` or
  similar, and there's no such flag in `--help`.
- Any indication of *why* a directory's heat is what it is — is it commits,
  mtimes, both? `recency-source = mtime` in `config` output answered this,
  but only because I went looking in a completely separate command. A
  first-time user running only `aspectus PATH` would have no way to know
  heat is mtime-derived unless they also ran `config`.
- A legend inline in the main output, as noted above — I had to keep two
  mental maps open (the `--help` legend, the running output) rather than
  one.

## `aspectus config`

This was the single most reassuring command I ran, and I ran it
semi-accidentally (I was curious what "config" as a subcommand would even
mean). It answered several questions the main output had raised for me
(heat half-life, sort default, census format) in one place, cleanly laid
out as "layer / status / source" then "won: / layout: / furniture:". I'd
call this the actual documentation of the tool's live behavior, more useful
to me in the moment than `--help` was, and I did not expect a "config"
subcommand to double as documentation — that was a pleasant surprise, not
something I was primed to look for.

One point of friction: I had to run `config` *after* being confused by the
main output, not before. If I'd known to run it first, several of the
`≈`/`~`/`≥` and heat questions above would have been resolved earlier. I
don't think that's obviously fixable (a first-time user shouldn't need to
read config before ever seeing a directory listing), but it's worth naming:
the natural onboarding order (`--help` → run on a directory) left gaps that
only `config` filled.

## `--depth`, `--lines`, targeted subpath runs

Running `aspectus --depth 2 --lines 40 01-aat-core` from inside `asf/`
worked exactly as I expected from reading `--help` — relative path
resolved fine, depth expanded further, line budget truncated with a `[+
md×146]` tail rather than silently cutting content. The truncation tail is
good: it told me there was more, with a typed count, rather than just
stopping. I trusted that immediately and didn't have to think about it.

I noticed the single dirty file (`aat.aux`, modified per `git status
--short`) was marked with a leading `M` glyph directly on its own line —
`M├── aat.aux`. That's a nice touch and I trusted it on sight (I already
knew from `git status` that it was the one dirty file, so this was a
genuine confirm-what-I-already-know moment rather than a blind guess). The
glyph placement — jammed against the tree-drawing character with no space
— was visually a little cramped at first glance; I initially read
`M├──` as some kind of compound tree-glyph before parsing `M` as a separate
status marker.

## `--format json`

I ran this last, mostly to see whether the "same look, as data" claim in
`--help` was literally true or aspirational. It appears literally true —
same heat/lines/mtime information, same `omitted`/`hidden`/`census`
structure showing up as typed JSON fields instead of bracket glyphs. I did
not verify byte-for-byte parity, only that the shape matched what I'd
already seen rendered. This felt trustworthy without deep verification
partly *because* the text output had already earned trust — I was not
starting from zero on this command.

One thing that stood out in JSON I hadn't fully registered in text mode:
`initial_sha`/`initial_behind`/`latest_sha`/`latest_behind` on every
tracked file. In text mode these columns are off by default
(`columns.initial-sha = off`, `columns.latest-sha = off` per `config`), so
I'd been looking at git-heat-shaped data (heat, age) without realizing
there was a whole parallel "how far behind HEAD was this file when
captured vs. now" dimension sitting underneath, invisible unless you
either turn the columns on or dump JSON. That's a real "oh — there's more
here than I thought" moment, the good kind: nothing was hidden
maliciously, it's an off-by-default column, but I would not have
discovered it without deliberately reaching for `--format json`.

## What I misread and then corrected

- Read `dirty<3>` as "3 dirty files" — correct, confirmed by `git status
  --short` giving 3 lines against `~/src/arch` (the parent repo, not
  `asf/`). Got this right on first read, actually, but I want to flag that
  I was *not* confident until I cross-checked with real `git status`; the
  number alone, without units, required outside verification for me to
  trust it as "files" rather than e.g. "commits ahead."
- Initially read the per-directory census `[dir×17 ≈310f · md×35 ...]` as
  meaning "17 directories, 310 files total, of which 35 are markdown" —
  i.e., I assumed `≈310f` was a strict superset containing the `md×35`
  etc. that follow it. Rereading the `--help` text ("subdirectories as
  containers whose deep file-count (mass) leads") I believe this reading
  is basically right, but I'm still not 100% sure whether the suffix
  buckets after the `f` count are drawn from the *same* files counted in
  `≈310f`, or are a separate top-level-only census. I never fully
  resolved this ambiguity and I'm flagging it as unresolved rather than
  quietly asserting an answer.

## What I'd teach myself to do differently after ten minutes

1. Run `aspectus config` immediately after `--help`, before ever pointing
   it at a directory — it would have collapsed several later confusions
   into one early pass.
2. Treat the census glyphs (`≈`/`~`/`≥`) as needing a cheat-sheet kept
   open, not as something to memorize from one `--help` read.
3. Default to `--format json` for anything I intend to reason over
   carefully or compare across runs, and reserve the text view for the
   "glance" use case it's clearly optimized for — the two felt like
   different tools wearing the same name, in a good way (one for
   scanning, one for computing).
4. Watch for the shifting quiet-column set across invocations rather than
   assuming the header stays fixed.

## What I'd want the steward to know that isn't naturally "findings"

The tool earned trust faster than I expected for something with zero prior
track record with me — the `[+ ...]` truncation-tail pattern and the
`config`-as-documentation move did more for my confidence than any amount
of prose in `--help` could have. The place it cost me the most was the
census glyph system: three lookalike prefix glyphs claiming three
different epistemic postures (exact/estimate/floor) is a genuinely clever
idea that I don't think survives first contact without a cheat sheet
physically present, and right now the only cheat sheet is buried in a
`--help` wall of text I had to scroll back to.

I'm available for follow-up questions if useful.
