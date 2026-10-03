# umath: Rust port of the deterministic Unicode-math converter

A pure-Rust port of `py/frozen/umath_v7.py` (sha1 `f40c0d4a…`), the primary
target. The same crate also reproduces `umath_v6.py` (`d7ad5797…`),
`umath_v5.py` (`12dfd65e…`), `umath_v4.py` (`d6a6b4fc…`) and `umath_v3.py`
(`b63113d3…`), selected per call.

*Hashes updated on 2026-10-03 by the spiker. The frozen Python files had examples from non-public sources scrubbed from their comments only (spike `notes/LOG.md` §29; `py/check_comment_only.py`: identical code tokens and ASTs). The earlier hashes were `99eb92dc` (v7), `a8f1dcf5` (v6), `39ee2679` (v5), `a9080099` (v4) and `eb5faf14` (v3). Every differential result below was measured against those, and it holds unchanged.* The library has no dependencies; `serde_json` is used only by
the JSONL bin.

```rust
let (text, spans) = umath::convert(input)?;   // v7
umath::convert_ver(input, umath::Ver::V6)?;    // or convert_v6 / convert_v5 / convert_v4 / convert_v3
// spans: Vec<Span { start, end, latex: Option<String>, conf }>
// start/end are CHAR offsets into `input`; latex None = a candidate it declined
```

## Result

**0 differences between Rust and the Python reference, for each of v3, v4,
v5, v6 and v7, on every input set below.** A "difference" is any mismatch in output
text, in the span list (start, end, LaTeX, confidence; declined candidates
included), or in whether the call raised. A site absent from both sides'
`--changed-only` output was unchanged and span-free on both.

| input set | inputs | v7 changed or spanned | differences (v3 / v4 / v5 / v6 / v7) |
|---|---|---|---|
| gold items (`all-items`, `C-items`, `D-items`) | 1,390 | 1,190 | 0 / 0 / 0 / 0 / 0 |
| estate prose sites (`sites.jsonl`, `display: false`) | 1,293,042 | 16,910 | 0 / 0 / 0 / 0 / 0 |
| math-free external sites (`mathfree-sites.jsonl`) | 2,473,218 | 72 | 0 / 0 / 0 / 0 / 0 |
| fuzz, seed 1 (`tools/fuzz_inputs.py`) | 264,773 | 88,251 | 0 / 0 / 0 / 0 / 0 |
| fuzz, seed 2 | 564,773 | 199,519 | 0 / 0 / 0 / 0 / 0 |
| deep-nesting probes (depth 100–10,000; v6/v7 add `⋃` chains) | 35 / 36 | — | 0 (v5) / 0 (v6) / 0 (v7) |

Version deltas, in output text, the same in both languages:
- v6 against v5: 1 gold item, 1 estate site, 0 math-free sites, and 1,813 /
  4,474 fuzz inputs. The fuzz is dense in exactly what v6 fixed: 𝟊,
  `†`/`*` followed by a letter superscript, and `⋃`.
- v7 against v6: 3 gold items, 43 estate sites (39 distinct bodies), 0
  math-free sites, and 138 / 393 fuzz records.

The corpus alone doesn't exercise everything, so the fuzz sets add three
families: random strings over the converter's special alphabet; mutations of
real math-bearing estate sites; and every BMP codepoint plus every
math-alphanumeric dropped into math contexts, which tests the generated
Unicode tables character by character. The reference raises on 295 to 775 of
the fuzz inputs per version (v3–v5), always `KeyError` on 𝟊 (bug 1 below).
Rust returns `Err` on exactly the same inputs. v6 and v7 raise on none.

**Idempotence** (relevant to md-press `--check` after a write): re-converting
v7's own output changes 0 of 16,910 estate outputs and 0 of 199,519 fuzz
outputs (v6 and v5: also 0 and 0). v3 is not idempotent: 134 of 16,978 estate outputs change on a second
pass. v4's fixed-point wrapper is the reason for the difference.

**Timing** (1,293,042 estate sites, Apple M4 Max):

| | wall | CPU |
|---|---|---|
| Rust v6, 1 thread | 11.0 s | 10.9 s |
| Rust v6, 12 threads | 1.4 s | 13.9 s |
| Python v6, 12 processes | 23.7 s | 279 s |
| Python v5, 1 process | 214 s | 213 s |

About 8.5 µs per site in Rust, roughly 20× less CPU than Python. v7 runs at
the same speed as v6: on a shared machine at load average 46, both took
13.0–13.3 s on 1 thread, and the absolute figures above are from an unloaded
run. The reference
has superlinear paths on very long lines; the port does not (see deviations
below). The longest estate site, 318k chars, is included in the timings above.

## Build and run

```sh
cd rs/umath && cargo build --release && cargo test --release
# JSONL in {"id","text"} -> JSONL out {"id","out"} (in input order)
target/release/umath [--v3|--v4|--v5|--v6|--v7] [--spans] [--changed-only] [--threads N] < in.jsonl   # default v7
# full differential, from the spike root (needs data/bulk/ and data/gold/):
sh rs/umath/tools/differential.sh v7 v6 v5 v4 v3
```

- `tools/make_inputs.py` builds the corpus inputs into `scratch/`.
- `tools/fuzz_inputs.py SEED N` writes the seeded fuzz inputs.
- `tools/ref_jsonl.py` runs a frozen Python file with the same JSONL contract.
- `tools/compare.py` compares two result files.
- `scratch/` (about 1 GB of inputs and outputs) is gitignored and regenerable;
  deleting it is safe.

## Layout (each file maps to named reference functions)

| file | reference functions |
|---|---|
| `src/lex.rs` | `protected_ranges`, `lex`, `_precomposed_math`, `_greek_numeral_after`, `mark_emphasis`, `SNAKE` |
| `src/term.rs` | `parse_script_arg`, `parse_term`, `render_tokens`, `join_latex`, `brace`, `tex_word_sub`, `_add_sup` and the trailing-script regexes |
| `src/spans.rs` | `units_of`, `find_spans` (its closures as methods on `FS`), `trim_span` |
| `src/convert.rs` | `render_span`, `tex_ok`, `dollar_hazard`, `normalize_paren_math`, `symbols_in`, `only_house_labels`, `code_math_interleave`, `link_bracket_positions`, `convert`/`convert_once`, `_spans_vs` |
| `src/consts.rs` | `GREEK`, `OPS`, `OPEN`/`CLOSE`/`PAIR`, `COMBINING`, `FUNCS`, `LABEL_NOUNS`, `REL_LATEX`, … |
| `src/uni.rs` + `src/tables.rs` | Python `str` predicates and `unicodedata` |

The structure follows the Python closely: the same names, the same control
flow, and index loops kept as index loops. That keeps the two readable side by
side. Restructuring is possible, but the differential is the check that any
restructuring kept behavior. Every regex is hand-ported with Python semantics,
including `$` matching before a final `\n`. No regex crate is used.

### Unicode tables

`src/tables.rs` (105 KB) is generated by `tools/gen_tables.py` from the
reference's own interpreter, CPython 3.11.14 with Unicode 14.0. It contains:
`str.isalpha/isdecimal/isdigit/isalnum/isspace/isupper/islower` and category
Mn/Lt as range flags; the reference's own `mathalpha_latex()` and
`subsup_of()` outputs for every codepoint; the NFD shape that
`_precomposed_math` tests; the house-label lexicon; and v6's
`KNOWN_COMMANDS`, taken from v6's own `_known_commands()` (192 commands). The
generator asserts that v6's `mathalpha_latex`/`subsup_of` equal v5's except
for 𝟊. Rust's `char` methods
are different predicates over a newer Unicode version (`is_alphabetic` is not
`isalpha`, `is_whitespace` is not `isspace`), so nothing is taken from them
except where ASCII-exact. Regenerate only together with a new reference; the
tables pin the measured behavior.

**Versions in one crate.** v6 changes the OPS table (`⋃`, `⋂`), and the
lexer consults OPS deep inside; v7 adds rules inside span growth and
rendering. Rather than thread a version parameter
through every function, `run()` sets a thread-local `Ver` (next to the
recursion counter), and the few version-dependent points read it. Each is
commented with the version that introduced it.

## Deviations from the reference (none change outputs)

1. **Errors are values.** Where the reference raises, Rust returns `Err`:
   `MathalphaKeyError` (bug 1; v3–v5 only), `Recursion` (point 2), and `IndexError`, which
   mirrors one Python `IndexError` path in `render_span`/`tex_ok` that no input
   has reached.
2. **Recursion limit.** Python raises `RecursionError` on deep nesting: about
   500 nested brackets inside a script argument, or about 1,000 chained prefix
   operators or `√(` groups. Without a guard, Rust would overflow its stack and
   abort the process. Each recursive function counts one level (`Deep` guard,
   `lib.rs`), and beyond 1,000 levels, Python's frame limit, `convert` returns
   `Err(Recursion)`. Python's frames are a superset of the counted calls, so
   Rust never fails where the reference succeeds. The reverse can't be matched
   exactly, because Python's threshold depends on how deep its caller's stack
   already is (a `Pool` worker sits deeper than a direct call). In principle,
   inputs within a few dozen nesting levels of the threshold can still differ.
   The probes across the threshold (35 for v5, 36 for v6 including `⋃`
   chains), including 450 (both succeed) and 500 (both raise), all agreed. Nothing in the estate comes within two orders of
   magnitude of this depth. 1,000 levels fit in a default 2 MB thread stack
   (tested).
3. **Bracket matching is precomputed.** `find_spans`' `group_close` /
   `group_open` rescan to the end of the line for every bracket, so 20,000
   unmatched `(` took 3.1 s. A one-pass stack match per bracket type gives the
   same partners, because for a single bracket type the scan's "first close
   where depth returns to 0" is exactly the stack partner. That case now takes
   0.2 s.
4. **`us.index(x)`** (a linear scan per relation operator, quadratic on long
   lines in Python) is replaced by the index already in hand. Units are unique
   by token range, so the result is the same.

## The reference does X here and probably shouldn't

Found during the port, against v3–v5. **v6 fixed 1, 2, 3 and the `⋃`/`⋂`
part of 5**: 𝟊 is no longer a math alphanumeric; `_add_sup` inserts a space
(`\ast T`, `\dagger i`); a new `commands_known` gate rejects any `\command`
that is neither in `KNOWN_COMMANDS` nor present in the input; the
quoted-mention rule needs a real closing quote; and `⋃`/`⋂` are
`\bigcup`/`\bigcap` prefix operators. 4 and the rest of 5 remain in v6 and
don't change outputs. The port reproduces each version as frozen, bugs
included. `tests/reference.rs` pins the bugs on v5 and the fixes on v6.

v7 then fixed a bug class found by an independent verifier:
- a glued hyphen before an operator-name word is a word compound, not a minus
  (`$n$-dim` was `$n - \dim$`, `γ-sign` was `\gamma - \operatorname{sign}`);
- grow-left stops at `func-` compounds (`log-det/λ`, `arg-max`);
- `\text{near}-\text{boundary}` renders as one `\text{near-boundary}`;
- a `$` inside inline code makes the whole site abstain, because md-press's
  `edit_pairs` does not mask code.

The port reproduces v7 exactly, and `tests/reference.rs` pins these changes.
Examples below are v5 outputs; 1–4 hold for v3 as well.

1. **𝟊 (U+1D7CA, MATHEMATICAL BOLD CAPITAL DIGAMMA) raises `KeyError`.**
   `mathalpha_latex` builds `GREEK CAPITAL LETTER DIGAMMA`, but Unicode names
   the letter `GREEK LETTER DIGAMMA`. Any text containing an unprotected 𝟊
   makes `convert` raise: `convert('let 𝟊 = 1')`. It is the only codepoint
   that raises; 𝟋 (small) is fine.
2. **A superscript glued onto a `\command` gives an undefined control
   sequence, and `tex_ok` lets it through.**
   - `A†ⁱ = B` → `$A^{\daggeri} = B$`
   - `Ξ*^T = 1` → `$\Xi^{\astT} = 1$` (a starred variable, then a transpose
     written `^T`: a plausible real input)
   - `x†ᵀ³ = y` → `$x^{\daggerT3} = y$`

   `_add_sup` merges the new superscript into the existing one with no
   separator. Its intended separator,
   `('' if extra.startswith('\\') and inner[-1:].isalpha() is False else '')`,
   is `''` on both branches, so it is dead code. `tex_ok` checks structure, not
   command names. None of these appear in the estate's 4,551 KaTeX-validated
   spans; the fuzz found them. A fix is a space (or braces) when `inner` ends in
   a command and `extra` starts with a letter, or an allowlist of command names
   in `tex_ok`.
3. **An opening quote with nothing after the span declines the span.**
   `the value "κ_t` is left unchanged, while `the value "κ_t is` converts.
   `s[end:end + 1] in '"”`'` is `True` for the empty string at the end of the
   text, so the quoted-mention rule fires without a closing quote.
4. **Superlinear time on long lines** (the behavior is correct). On one
   38k-char line Python takes 0.98 s; 4× the length takes about 9× the time.
   md-press's unwrapped paragraphs can be long. The port avoids this
   (deviations 3–4).
5. **Dead code**, harmless but misleading to a reader:
   - `wrap_next` in `render_span` is never set `True`;
   - `_bar_close`, `is_greek`, `QUOTES`, `Unit.mathy`, `Term.why`, `Term.a` and
     the `debug` parameter are unused;
   - `plausible_variable` tests a `'quote'` token kind that the lexer never
     produces;
   - `LARGE = set('∑∏∫∮∇∂⋃⋂') | {'Σ','Π'} - {'Σ','Π'}` evaluates to just the
     first set, because `-` binds before `|`;
   - `⋃`/`⋂` are in `LARGE` but not in `OPS`, so they lex as `other` and the
     large-operator rules never see them (`⋃_i A_i` does not parse as a large
     operator).
6. **The reference's `protected_ranges` uses Python whitespace** (`\s`,
   `isspace`); md-press's uses `char::is_whitespace`. The two disagree on a few
   control characters (U+001C–U+001F are Python whitespace, Rust is not). This
   is irrelevant while the converter uses its own (ported) ranges. It matters
   only if someone swaps in md-press's.

## How it should plug into md-press: my view, not measured

The integration plan already argues gates and trigger scope; I agree with it
and won't repeat it. What the port showed me in addition:

- **Integrate v7** (`umath::convert`). It raises on nothing in any set here
  and is idempotent. (The held-out scores were measured on earlier versions, D on v3/v4; the
  coordinator reports v7 departs from v6 on 3 of 1,390 gold items.)
- **Call it on the whole site body, not on md-press's pieces.** In
  `promote_site`, take `split_structure`'s body, call `umath::convert(body)`,
  and reattach `pre`/`suf`. Skip `split_at_prose_separators`/`split_sentences`
  on this path. The converter makes the prose-role decision for `→ · ×`
  itself, the splitter is what cut live math (LOG §4), and the estate
  measurements were taken on site bodies (`sites.jsonl` `body`).
  `display: true` sites were not converted in any measurement.
- **Let the converter own `\(…\)` normalization.** v4+ checks currency and
  `$` hazards on the raw text before normalizing. If md-press's
  `normalize_paren_math` runs first, those checks see different text than was
  measured. Pass the raw body; md-press's own normalize becomes redundant on
  this path.
- **`Err` means leave the text unchanged and flag it.** In v6 and v7 only
  adversarial nesting (about 500+ levels) produces one.
- **Offsets are chars.** md-press indexes bytes; convert via `char_indices`
  if the spans are used (the output text alone needs nothing).
- **Packaging.** Either a path or workspace dependency, or copy `src/` into
  md-press as a module. `tables.rs` is generated data, like `model/model.json`
  in the classifier's precedent. The functions are pure; the recursion counter
  is thread-local, so parallel use is safe.
- **Speed is no longer a reason for the trigger.** The whole estate converts
  in 1.4 s on 12 threads. Whether to keep the trigger is purely a scope and
  precision decision, which the plan's (a)–(c) sequencing addresses.
- **Keep the differential as the regression test.** Any v8 should go through
  `tools/differential.sh` plus the fuzz before its numbers are cited for the
  Rust side.
