# md-press

Canonicalizes markdown to the house standards used across `~/src/` — removes manual word-wrapping without touching anything else, and promotes Unicode math to `$LaTeX$` that renders in GitHub, Obsidian, and LaTeX alike (on by default; `--no-math` skips it).

It exists because the general-purpose formatters get this wrong in ways that cost more than they save: every one tested either normalizes *all* soft breaks (destroying deliberately-chunked prose), mangles tables, escapes wikilinks, reserializes YAML frontmatter (eating inline comments), or doesn't know `$…$` math exists. The research behind that verdict is in [`research/`](research/); the problem statement is [`PROBLEM.md`](PROBLEM.md), the design decisions [`PLAN.md`](PLAN.md), and current capability [`STATUS.md`](STATUS.md).

**Where things stand:** start at [`STATUS.md`](STATUS.md) → *Where it stands, and how to pick it back up*. It covers what is built, whether it is installed, the decisions waiting, and the next work. Investigations, including the 2026-10-02 deterministic Unicode-math converter that may replace the model in the math pass, are indexed in [`spikes/`](spikes/README.md).

## Build and install

Source lives at `firmatum/utils/md-press/` in the Archema programme tree (moved from `utils/md-press/` 2026-08-01). Install is still plain cargo — the global binary does not depend on a fixed source path after install.

```sh
# from arch/ (programme root):
cargo install --path firmatum/utils/md-press

# or from this directory:
cargo install --path .     # build release + install to ~/.cargo/bin/md-press
cargo test                 # invariants, ground truth, math gates, regressions (offline)
MD_PRESS_OLLAMA=1 cargo test live_model    # also exercise the real model
cargo build --release      # build only; binary at target/release/md-press
```

`cargo install --path …` is the whole install story — re-run it to upgrade after pulls or path moves, `cargo uninstall md-press` to remove. It requires `~/.cargo/bin` on your PATH; that line was added to `~/.zshrc` on 2026-07-22 (it had been missing, which had left a few earlier `cargo install`s unreachable).

The math pass (on by default) needs nothing installed: since 0.3.0 it is a deterministic converter built into md-press (`umath/`). Only `--math=MODEL`, the older LLM path, needs a local [ollama](https://ollama.com) with the model pulled (`ollama pull llama3.2:3b`); without it, md-press says so once per run and does everything else.

## Usage

**By default md-press only reports.** It writes nothing unless you pass `--fix` (changed 2026-10-05; before that, naming a file edited it). The report shows every decision it would make, each with a short code that flips it.

```sh
md-press FILE.md ...                    # report: what would change, with codes; write nothing
md-press --fix FILE.md ...              # write the plan as reported
md-press --toggle k7q2x,m3xd7 FILE.md   # report the plan with those decisions flipped
md-press --fix --toggle k7q2x FILE.md   # write it that way
md-press -q $(git ls-files '*.md')      # just the files that would change, e.g. a pre-commit check
md-press - < FILE.md                    # report on stdin (unguarded — see below)
md-press --no-math FILE.md              # unwrap only
md-press --math=MODEL FILE.md           # math with a local ollama model instead of the converter
md-press --version                      # 0.2.0 and later report by default; 0.3.0 remembers declined math
```

`--check` is still accepted and means the default. Exit codes: `0` nothing would change (report) or done (`--fix`); `1` the report found files that would change; `2` an error, a code that matches nothing, or a file left untouched by the safety check.

**The report.** For each file that would change, first what md-press is confident about, then what it is unsure about:

```
ref.md: would change — 2 line breaks joined, 7 breaks kept and marked
  confident:
    [5yj4z] L6  keep + mark (p 1.00): "**Target Claim:**" ⏎ "Adversarial interaction is the same coup…"
  unsure:
    [x04cp] L70  join (p 0.47; flip: keep + mark): "…ty for Stabilization.* IEEE TIT 52:3369." ⏎ "*Significance:* The cleanest necessity-a…"
    [p7cpf] L72  keep + mark (p 0.71; flip: join): "…g Degradation in Decision-making Teams.*" ⏎ "*Significance:* The strongest direct anc…"
```

A paragraph's confident joins share one code. Each other decision has its own: a break to keep and mark, a math passage to convert (shown as `«before» → «after»` with a little context), a blank line to add around `$$`. `L` is the line in the file as it is now. `-v` also lists each break inside the joined paragraphs.

**What flipping does, and how the file remembers it.** Flipping a join keeps that break and ends the line with two spaces, so it renders as a line break, and md-press never asks about a marked break again. Flipping a keep joins the lines, and once joined there is nothing left to decide. Flipping a math conversion puts the original text back and writes `<!--md-press:not-math:TEXT-->` at the end of that line or table cell (TEXT is the passage as written); renderers hide it, and later runs leave that text on that line alone. Either way the file itself carries the decision, so once a file's decisions are all taken or flipped, md-press reports nothing for it again. Two exceptions: a declined blank line around `$$` (that declines the house rule itself), and declines under `--math=MODEL`, are not remembered. The marker's shape is a contract other tools rely on (vsect strips any one-line `<!--md-press:…-->` comment): keep it. A math flip you find yourself repeating is either a bug or a house convention (a label that looks like a variable), and is worth a `FEEDBACK-*.md` here.

**Codes are bound to what they describe.** Each is derived from the file's exact contents, its path, this build of md-press, and the options that change the plan (math model and prompt, classifier on or off). Any change makes old codes match nothing, and an unmatched or ambiguous code is an error that writes nothing, so a code always means what the report showed. Without `--toggle`, `--fix` writes the default plan, exactly what the report listed. When it does, it says how many unsure decisions it took at their defaults.

**The usage log.** Each run that reports or writes changes appends one line per file to `$XDG_DATA_HOME/md-press/usage.jsonl` (default `~/.local/share`). Each line holds the unsure decisions with their context and classifier features, which codes were flipped, and counts of the confident joins. It stays on this machine and exists to improve md-press: a flip on a reviewed run is a label for retraining the break classifier, and a repeated math flip points at a bug. `--no-log` or `MD_PRESS_LOG=off` skips it, and deleting the file is always safe. (Joseph's decision, 2026-10-05: stored locally, never sent anywhere, used only to improve the tool.)

**Messages.** On stderr, one-line summaries: files skipped by exclusion, math passages whose proposals failed a check (with `--fix`; `-v` lists them), and a missing model. `-q` reduces the report to a list of files.

`MD_PRESS_MATH=off` (or `0`/`no`) turns the math pass off for every call that doesn't pass `--math`; `MD_PRESS_MATH=MODEL` picks the model. Flags win over the environment — the caller tunes the tool, never the tree it runs on.

**What it changes.** Each prose paragraph split across several lines becomes one long line — including paragraphs inside list items, blockquotes, and footnotes. Which newlines *can* be joined is decided by the CommonMark parse, not by line shapes. Where a joinable break might still be deliberate (a label stack, a header block), a trained classifier decides: keeps are marked with two trailing spaces so they become permanent, and the report shows the probability for the ones it is unsure about. Every break gets a decision, so there is never a pile of files left for you to fix by hand; the codes only let you overrule one. The unwrap stage is deterministic. The break after a callout marker (`> [!note]`) is always kept, since text on that line is the callout's title.

**What it leaves alone.** Tables, code (fenced and inline), YAML frontmatter (byte-for-byte, inline comments included), math spans, wikilinks, HTML, and every line break that carries meaning: CommonMark hard breaks (trailing two spaces or backslash), link and footnote definitions, standalone equation tags like `*[Definition (slug)]*`, whole-line math, display math (`$$` lines and the interior of `$$` blocks), lines that look like table rows even where CommonMark sees a paragraph, and the equation-after-a-colon idiom.

**What it will not touch.** A `.md-pressignore` file (gitignore syntax) at or above a file excludes it, and is honoured *even when the file is named explicitly* — the realistic accident is an agent running `md-press --fix $(find . -name '*.md')` across verbatim material. `--force` overrides. Use it for raw transcripts, provenanced copies, and frozen archaeology: reformatting those is render-equivalent and still destructive, so no automatic check can defend them. This protection has to be declared rather than inferred. (The legacy name `.fmt-mdignore`, from before the 2026-08-06 rename, is honored equally and indefinitely — an exclusion must never expire because the tool changed its name.)

**What it will not touch, by extension: `.udon`.** UDON is not markdown, and the render-equality gate below cannot tell, so `.udon` files are skipped by default even when named explicitly. Three mechanisms, each reproduced rather than argued (`UDON-ASSESSMENT-2026-07-29.md`): UDON's *text law* makes the newline literal text content rather than collapsible whitespace, so joining two prose lines edits the reconstructed value; a bare attribute value runs to end of line, so joining `:author X` upward silently swallows every following `:key` into one attribute holding garbage; and `!:lang:` verbatim blocks are invisible to comrak, which reads them as ordinary paragraphs and flattens working source into one line. All three pass the render check, because a mangled UDON line renders as unremarkable markdown text — which is why the guard sits in front of the gate instead of relying on it. `--allow-udon` proceeds anyway; `--force` deliberately does *not*, so overriding verbatim exclusions cannot silently disable this too. If UDON ever wants automated reflow, the natural home is a tool built on UDON's own recognizer, not this one.

**Stdin mode is unguarded.** Reading stdin there is no filename, so neither a `.md-pressignore` exclusion nor the `.udon` guard can be consulted. (The render-equality gate does run, since 2026-10-05.) So `md-press --fix - < FILE` answers *"what would the engine do to these bytes"*, not *"what would md-press write to this file"*. A file that file mode would skip comes back fully reformatted through stdin. Concretely, piping a `.udon` file through it once reflowed 371 lines to 302 on `udon/v2/theory/OUTLINE.udon`, splicing comment blocks into unrelated attribute rows: the exact damage the guard exists to prevent. To see what would happen to a file, report on it by name. That runs the guards, and (since 2026-10-05) shows the edits, so the old `md-press - < FILE | diff FILE -` preview is no longer needed.

**Why unwrapping is safe.** The source text changes, but the rendered document must not. Before writing anything, md-press re-parses its own output and compares the rendered result against the original; if they differ at all — which would mean a bug in md-press — that file is left exactly as it was and the problem is reported on stderr. Running it again on its own output changes nothing, and files that are already canonical are not touched at all.

**Why `--math` is governed differently.** Promoting math is *meant* to change the rendered document — that is the entire point, since a literal `η` renders as a glyph while `$\eta$` renders as a typeset symbol. Render-equality would forbid the improvement, so it does not apply to that stage, which runs after the gate above and carries its own narrower guarantee instead. See below.

## The math pass (on by default; `--no-math` skips it)

Promotes Unicode math in prose (`η`, `‖δ‖ ≤ R`, `M_τ⁺`) into `$…$` LaTeX, fixes raw `<`/`>` inside inline `$…$` (as `\lt`/`\gt`, `\leq`, `\ll`, …: GitHub reads a raw angle in math as HTML), normalizes `\(…\)` to `$…$`, and adds blank lines around `$$` display math. Since 0.3.0 the engine is a deterministic converter (`umath/`, v7 of the spike in `spikes/spike-unicode-math-2026-10-02/`): no model, the same answer every run, and on the spike's clean held-out set it got 84.5% of triggered lines right against llama3.2's 37.5% (STATUS.md, 0.3.0 section, has the provenance). Each changed passage is its own decision with its own code.

The engine only ever sees prose. Per line or table cell, md-press removes the markdown structure first (indentation, `>` and list markers, task boxes, heading `#`s, definition labels, the trailing hard break) and reattaches it verbatim afterward; code, link destinations, URLs, wikilinks, HTML, and existing math are protected.

**What triggers it.** A strong math glyph (`‖ ∈ ⊂ ∑ ∂ ∞ …`), an isolated Greek letter (not inside a Greek word, not a unit like `μs`), or one of the glyphs this estate also uses as punctuation — `→ · × ≈ ≤ ≥ ≠ ± ≡ √` — in an *operator* role: math-looking on one side, and math, a number, or nothing on the other (`x → ∞`, `n × k`, `H ≤ 5`). Between words, code, links, or labels those glyphs are prose (`asf → logos`, `a · b · c`, `35× faster`, `≈70%`). Bare ASCII (`x`, `M_t`) does not trigger: on untriggered sites the converter is 20–26% wrong, so widening waits on a fresh labeled set. The angle fix applies on every site.

Since render-equality cannot govern this stage, narrower gates apply: **every character outside the new `$…$` spans must be the original's own, in order**, and no code delimiter, `**`/`__`, or HTML comment may move into math, and every `*` must survive (as `*` or `\ast`). A refused passage is left byte-identical and counted in the end-of-run summary (`-v` lists each with its reason).

**The older LLM path.** `--math=MODEL` (or `MD_PRESS_MATH=MODEL`) sends sentence-sized pieces to a local ollama model instead, under the stricter LLM-era gates (prose words survive; nothing inside a new span that doesn't trace back to the original). Its proposals are cached under `$XDG_CACHE_HOME/md-press/math`, keyed by model, prompt file and text; `MD_PRESS_CACHE=off` disables the cache. A different model, version or prompt can propose differently, and its declines are not remembered. To teach it a new case, append an `Input:` / `Output:` pair to [`prompts/unicode-math.txt`](prompts/unicode-math.txt) — no rebuild needed. (That file is `.txt` on purpose: its line breaks are data.)

## Scope

Everything here is honest about being partial. Not yet implemented: the full math-rule registry (the GitHub render-compat checks `asf/bin/lint-md` ran — raw `|`/`*` inside existing `$…$` (raw `<`/`>` is fixed since 0.3.0), emphasis-vulnerable `_` across spans, LaTeX outside math, `#` in math; see `FEEDBACK-2026-08-22.md` §2), blank-line normalization beyond display math, and a repo-wide recursive mode — **there is deliberately no recursive mode yet**: directories like `_obs/`, `old-*`, and provenanced verbatim copies must not be reformatted, and until the tool can be told that, pointing it at a whole tree is your job to scope, not its.
