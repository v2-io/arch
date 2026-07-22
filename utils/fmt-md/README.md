# fmt-md

Canonicalizes markdown to the house standards used across `~/src/` — removes manual word-wrapping without touching anything else, and (optionally) promotes Unicode math to `$LaTeX$` that renders in GitHub, Obsidian, and LaTeX alike.

It exists because the general-purpose formatters get this wrong in ways that cost more than they save: every one tested either normalizes *all* soft breaks (destroying deliberately-chunked prose), mangles tables, escapes wikilinks, reserializes YAML frontmatter (eating inline comments), or doesn't know `$…$` math exists. The research behind that verdict is in [`research/`](research/); the problem statement is [`PROBLEM.md`](PROBLEM.md), the design decisions [`PLAN.md`](PLAN.md), and current capability [`STATUS.md`](STATUS.md).

## Build and install

```sh
cargo install --path .     # build release + install to ~/.cargo/bin/fmt-md
cargo test                 # 9 tests: invariants, ground truth, math gates
cargo build --release      # build only; binary at target/release/fmt-md
```

`cargo install --path .` is the whole install story — re-run it to upgrade, `cargo uninstall fmt-md` to remove. It requires `~/.cargo/bin` on your PATH; that line was added to `~/.zshrc` on 2026-07-22 (it had been missing, which had left a few earlier `cargo install`s unreachable).

Optional: `--math` additionally needs a local [ollama](https://ollama.com) with the chosen model pulled (`ollama pull llama3.2:3b`). Everything else works without it.

## Usage

**Named files are edited in place.** Nothing is written in `--check` mode, and stdin mode writes only to stdout.

```sh
fmt-md FILE.md ...              # edit those files in place
fmt-md --check FILE.md ...      # dry run: print what would change, write nothing
fmt-md - < FILE.md | diff FILE.md -     # dry run showing the actual edits
fmt-md --math FILE.md           # also promote Unicode math (needs ollama)
fmt-md --math=MODEL FILE.md     # ... with a specific local model
fmt-md --check $(git ls-files '*.md')   # e.g. a pre-commit check
```

Exit codes: `0` done (written, or nothing needed changing), `1` `--check` found files that would change, `2` an error or a file left untouched by the safety check.

**What it changes.** Each prose paragraph split across several lines becomes one long line — including paragraphs inside list items, blockquotes, and footnotes. Which newlines get joined is decided by the CommonMark parse, not by line shapes, so it is deterministic: no heuristics, no confidence thresholds, and no pile of files flagged for you to fix by hand.

**What it leaves alone.** Tables, code (fenced and inline), YAML frontmatter (byte-for-byte, inline comments included), math spans, wikilinks, HTML, and every line break that carries meaning: CommonMark hard breaks (trailing two spaces or backslash), link and footnote definitions, standalone equation tags like `*[Definition (slug)]*`, whole-line math, and the equation-after-a-colon idiom.

**What it will not touch.** A `.fmt-mdignore` file (gitignore syntax) at or above a file excludes it, and is honoured *even when the file is named explicitly* — the realistic accident is an agent running `fmt-md $(find . -name '*.md')` across verbatim material. `--force` overrides. Use it for raw transcripts, provenanced copies, and frozen archaeology: reformatting those is render-equivalent and still destructive, so no automatic check can defend them. This is the one protection that has to be declared rather than inferred.

**Why unwrapping is safe.** The source text changes, but the rendered document must not. Before writing anything, fmt-md re-parses its own output and compares the rendered result against the original; if they differ at all — which would mean a bug in fmt-md — that file is left exactly as it was and the problem is reported on stderr. Running it again on its own output changes nothing, and files that are already canonical are not touched at all.

**Why `--math` is governed differently.** Promoting math is *meant* to change the rendered document — that is the entire point, since a literal `η` renders as a glyph while `$\eta$` renders as a typeset symbol. Render-equality would forbid the improvement, so it does not apply to that stage, which runs after the gate above and carries its own narrower guarantee instead. See below.

## The math pass (`--math`)

Optional and off by default. Promotes Unicode/bare math in prose (`η`, `‖δ‖ ≤ R`, `M_t`) into `$…$` LaTeX, and normalizes blank lines around `$$` display math. The default model is `llama3.2:3b` (~2 GB), which proved accurate at the one genuinely hard judgment — where a mathematical expression starts and stops.

The model *only* judges those boundaries. House rules (`\lt`/`\gt`, `\ast`, `\vert`, command spacing) are applied deterministically afterward, so the model never needs to know the standard.

Since render-equality cannot govern this stage, three narrower gates check the model's work per line: delimiters must balance; prose outside the math must survive word for word; and nothing may appear inside a new span that doesn't trace back to the original line (that last gate is not hypothetical — it caught the model importing `\to M_{t+1}` from one of its own few-shot examples into a line containing neither). A proposal failing any gate is discarded and the line reported on stderr, leaving it byte-identical. So the model cannot quietly reword prose or invent mathematics; the worst it can do is decline to help and say so.

To teach it a new case, append an `Input:` / `Output:` pair to [`prompts/unicode-math.txt`](prompts/unicode-math.txt) — no rebuild needed, and each flagged line is a ready-made candidate. Few-shot examples work markedly better than prose rules for a model this size; adding rules as a list made it worse. (That file is `.txt` on purpose: its line breaks are data, so a markdown formatter — this one included — must not treat it as prose. Dogfooding caught exactly that.)

## Scope

Everything here is honest about being partial. Not yet implemented: the full math-rule registry (the checks `asf/bin/lint-md` still owns), blank-line normalization beyond display math, config discovery, and frozen-region exclusions — which means **there is deliberately no repo-wide recursive mode yet**: directories like `_obs/`, `old-*`, and provenanced verbatim copies must not be reformatted, and until the tool can be told that, pointing it at a whole tree is your job to scope, not its.
