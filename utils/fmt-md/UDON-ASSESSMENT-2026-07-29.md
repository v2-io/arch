# fmt-md against UDON — assessment (2026-07-29)

**Question asked:** does fmt-md have any obvious deficiencies against UDON
documents? If not, extend it to accept `.udon` as markdown.

**Answer:** yes — obvious, severe, and structural. fmt-md should **not** be
pointed at `.udon` files, and I did not make the extension-acceptance change.
Every claim below is marked by how I know it: **[ran]** = observed from an
actual `fmt-md` invocation in this session; **[spec]** = read directly from
`v2/current-0.9.1-spec/` at the point of use; **[code]** = read directly from
`fmt-md`'s own source. Nothing here is inferred-and-unverified — where I
started from a hunch (see the brief I was given) I chased it to a primary
source or a reproduction before writing it down as a finding.

## Two independent failure mechanisms, both load-bearing

### 1. The soundness gap is real: fmt-md's safety gate cannot see UDON meaning

**[code]** fmt-md's entire safety argument is: parse the input as CommonMark,
join prose lines, re-parse the output as CommonMark, and refuse to write
unless the two renders are HTML-equivalent (`render_fingerprint`,
`src/lib.rs`). It has no model of UDON at all — it doesn't know `.udon` is a
different language, doesn't parse it, and the gate is scoped to *Markdown*
rendering only.

**[spec]** UDON has no concept for this gate to even be checking. Its own
normative invariant is the opposite of Markdown's soft-wrap collapse:

> "The document's text material reconstructs by **pure in-order
> concatenation** of every Text … no fabricated join characters, no
> re-consultation of the source." — `MODEL.md` §6, "the text law"

and, spelled out for line terminators specifically:

> "Each text line's terminator is part of its text." — `CORE.md` §7.2

So where Markdown treats a soft line break as insignificant whitespace
(collapses to nothing or a space, depending on renderer — which is exactly
why fmt-md's HTML-render-equality check works for Markdown), **UDON treats
the newline itself as literal text content.** Replacing a `\n` with a space
across two prose lines is not render-neutral in UDON — it is a direct edit to
the reconstructed value. The tool's central safety claim ("the source text
changes, but the rendered document must not") doesn't have a UDON referent to
be true or false about; the premise doesn't transfer.

### 2. The join logic corrupts structure, not just prose — verified two ways

**[spec]** UDON attribute lines are line-scanned, and a *bare* (unquoted,
non-self-terminating) value commits a flow value that **runs to end of
line**, silently swallowing anything after it — including subsequent `:key`
attributes. CORE.md §6.4 gives this exact shape as a worked example:

> `|el :n value |{em x} :a 1` → `n = the flow value ⟨"value " |{em x} " :a
> 1"⟩ — no :a attribute exists`

**[ran]** I reproduced this directly. Given:

```udon
|article[intro].featured
  :author Joseph Wecker
  :date 2025-12-22
  :tags [udon notation design]
```

`fmt-md --no-classify - < file` (deterministic mode — the automatic fallback
whenever the trained classifier model isn't loadable, `src/main.rs:149-156`)
produced:

```
|article[intro].featured :author Joseph Wecker :date 2025-12-22 :tags [udon notation design]
```

That is not a formatting change. `Joseph` is a bare token; `Wecker` is plain
text, not a guard-confirmed marker, so per §6.4 the flow value commits and
runs to end of line — meaning `author` now owns everything after it,
including the literal text `":date 2025-12-22 :tags [udon notation design]"`.
Under UDON's own attribute-ownership rule this is no longer three attributes;
it's one attribute holding garbage. `date` and `tags` cease to exist. Nothing
in fmt-md's render-equality gate can see this, because comrak has no idea
what a UDON attribute is — the check compares HTML, and the joined line
"renders" as ordinary Markdown text, unremarkably.

**[ran]** The default (classifier-loaded) path doesn't do this specific
merge — it happened to keep those three lines separate — but it isn't safe
either: it silently appended two trailing spaces (a CommonMark hard-break
marker) to `|article[intro].featured`, `:author Joseph Wecker`, and `:date
2025-12-22`, none of which had them, with **no stderr note** (the classifier
scored these ≥0.85, the silent-keep band). That's unrequested byte mutation
of structural lines the tool claims to "leave alone." Which of the two
failure modes you hit is a function of whether the classifier model happens
to be present and how it happens to score a given break — not something a
user or CI step controls.

### 3. Verbatim blocks are not recognized at all, and get flattened

**[spec]** `!:lang:` block verbatim (§10.1) and `!:quote:` are meant to carry
their content byte-for-byte — code, quoted transcripts, anything where line
structure *is* the content.

**[ran]**

```udon
|note
  :body !:elixir:
    defmodule Hello do
      def world, do: IO.puts("Hello from UDON")
    end
```

becomes, under `fmt-md --no-classify`:

```
|note :body !:elixir: defmodule Hello do def world, do: IO.puts("Hello from UDON") end
```

fmt-md has no fenced-code recognition for `!:lang:` (comrak only knows
triple-backtick fences), so it reads the whole verbatim block as an ordinary
paragraph and joins it into one line — turning working Elixir source into a
syntax error, and doing the equivalent to a `!:quote:` block of prose.

### 4. Real-world scale: a live project file, not a synthetic case

**[ran]** I ran `fmt-md` (default, classifier-loaded — the mode a real
invocation would use) against `~/src/udon/v2/theory/OUTLINE.udon`, a live
35-line file with `;`-comment blocks, `|element`/`:attribute` rows, and
prose. It collapsed to 17 lines. The result mixes comment continuation lines
into unrelated structural rows and merges multiple UDON comments and
attribute declarations onto shared physical lines — e.g. one output line
reads `; …Becomes `draft` when a file exists. ; :from where the material
already sits…`, splicing a comment line straight into an unrelated
element's attribute row. This is not a case I constructed to fail; it is
fmt-md's actual behavior on the nearest thing to "typical UDON" this repo
has, run with no special flags.

## Why this isn't fixable by tightening fmt-md's Markdown-side rules

The failures above aren't edge cases in fmt-md's join heuristics (the kind
`--explain` or classifier-threshold tuning would touch). They follow directly
from the fact that fmt-md's decision procedure — "ask comrak what's a
paragraph, join it, check the CommonMark render didn't change" — has no
purchase on a language where (a) the newline is literal text content rather
than collapsible whitespace, (b) column position and line boundaries decide
attribute ownership, not paragraph shape, and (c) verbatim blocks are marked
by syntax comrak doesn't parse at all. Making this safe for UDON isn't a
rule addition to the existing engine; it would be a second, UDON-aware
engine using UDON's own recognizer — a different project, not a flag.

## What I did **not** find deficient (for completeness)

To be fair to the tool: its safety argument is entirely coherent and does
hold **for actual Markdown** — the gate, the exclusion mechanism, and the
math-pass's separate per-line guarantees are all well-reasoned for the
language they target (I read `src/lib.rs`, `src/math.rs`, and `STATUS.md` in
full; nothing there looked wrong for the Markdown case). The deficiency is
entirely in the premise that UDON is "markdown enough" for the same argument
to carry over — it isn't, for reasons the language's own spec states
explicitly (the text law exists precisely because UDON was designed *not* to
have Markdown's kind of insignificant whitespace).

## What I'd recommend, offered as a recommendation rather than something I did

- **Don't extend fmt-md's accepted extensions to `.udon`.** The evidence
  above is that doing so would cause real, silent document corruption, not a
  cosmetic mismatch.
- **Defensive step worth considering, separate from this question:** nothing
  currently stops `fmt-md $(find . -name '*.udon')` — the tool checks no
  extension at all (`src/main.rs`), so any `.udon` file handed to it directly
  gets processed exactly as shown above. `~/src/udon/.fmt-mdignore` today
  excludes only three specific paths (raw transcripts, provenanced copies, a
  malformed-fence archive); it does not exclude `*.udon` as a class. I didn't
  add that line myself since it wasn't asked for and touches a file outside
  this tool's own repo, but it seems like cheap insurance given what's above.
- If UDON ever wants automated reflow, the natural home is `udon-core`
  itself (or a sibling tool built on the UDON recognizer), not fmt-md — the
  Nesting Rule and the text law are exactly the things such a tool would need
  to honor, and fmt-md's engine has neither.

## What I didn't test

I did not run this against `core/generator/*.descent.udon` (the largest real
specimen, ~4,180 lines) or the `design/examples/` corpus — the failure modes
above are general enough (they follow from spec sections that apply to every
`.udon` file, not file-specific quirks) that I don't think a larger run would
change the conclusion, but I haven't looked at what it does to those files
specifically. I also didn't test `--math`, since it's irrelevant to whether
`.udon` should be an accepted extension. No code changes were made to
`fmt-md`.
