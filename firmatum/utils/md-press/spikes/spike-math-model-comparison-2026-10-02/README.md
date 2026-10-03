# Which local model should md-press's math pass use? (set aside 2026-10-03)

*Written by the agent that reworked md-press's math pass on 2026-10-02 (see `../../STATUS.md` → *Math on by default, and what that required*). It is to whoever picks the math pass back up, Joseph first.*

**Where it landed.** Muse Glimmer 30B was clearly better than llama3.2:3b at md-press's math conversions, and about 6× slower per call. Both lost to the deterministic converter that `../spike-unicode-math-2026-10-02/` built and verified the same night. On that spike's held-out set, run through md-press's own gates, llama3.2 got 37.5% right and the converter 84.5% (`de-novo-response-1.md` there). The converter spike also tested an LLM as a fallback where the converter changes nothing, and found it gained nothing on its sets A and B. So this comparison is set aside. What it leaves behind is the measurement, the harness, and three gate fixes it caused.

## The comparison

**Question** (Joseph): is Muse Glimmer 30B, a newer local Meta model, better than llama3.2:3b at md-press's math work?

**Where the data is.** Every data file is split by where its pieces came from. `data/` holds the 190 pieces from public repos and is committed. `data-local/` holds the 50 from private or local-only repos and is gitignored: Joseph's call (2026-10-03) to keep non-public text off GitHub going forward. Its classification comes from `projects -vvv`, with local-only counted as non-public. Record `id`s are shared, so concatenating the two halves restores the full 240. The counts and results below are over all 240, so on a fresh clone the committed half reproduces only the 190 public pieces' share of them. Example id 181 lives in `data-local/`; the other ids cited here are in `data/`.

**Setup.** 240 pieces of text md-press would really send to a model. They were sampled at random from the 8,027 distinct pieces its pipeline produces over the estate's tracked `.md` files (`data/sample.json`: 120 from `/arch/asf/`, 120 from elsewhere; how they were collected is under *Harness*). Both models got md-press's exact prompt (`prompts/unicode-math.txt` + `Input: <piece>` + `Output:`) as a raw completion: temperature 0, stop at newline, length cap `64 + len/2` tokens.
- llama3.2:3b ran via ollama.
- Muse Glimmer 30B ran as `muse-glimmer-30B-kquant-dynamic.gguf` with its `dflash-kquant.gguf` speculative drafter, on llama.cpp b10358 `llama-server -c 8192`, per `~/models/muse-glimmer/notes.md`. The machine is an M4 Max, 48 GB.

Every proposal went through md-press's real gates (`harness/src/bin/judge.rs`), and then I read every accepted conversion by hand, because passing the gates doesn't make a conversion correct.

**Results with md-press's gates as they stand now** (`data/judged-*.jsonl`):

| | converted | left unchanged | refused | median s/call |
|---|---|---|---|---|
| llama3.2:3b | 83 | 61 | 96 | 0.58 |
| Muse Glimmer 30B | 138 | 64 | 38 | 3.34 |

**With the gates as they were before this comparison** (`data/judged-*-first-gates.jsonl`): llama3.2 had 88 converted, 5 of them wrong but accepted:
- id 94: `ρ/R` → `\rho/\rho`
- id 181: `𝒜` → `\mathcal{S}`
- id 105: `||δ||` → `\delta`, the norm dropped
- ids 26 and 130: a full stop swallowed into the span

Muse had 138 converted, with 1 minor slip (id 102, `δ_sat^(1)` → `\delta_{\mathrm{sat}^{(1)}}`, a misplaced brace). The other accepted conversions read as correct to me; one reader's judgment, not independently checked. Muse handled things llama3.2 could not:
- `>>` → `\gg`, where llama3.2 wrote `\gt`, weakening "much greater than" to "greater than"
- `η2p` read as partial eta-squared, `\eta^2_p`
- `X < n_future × Y` → `$X \lt n_{\text{future}} \times Y$`
- a full sea-level equation

When Muse wasn't sure, it mostly returned the line unchanged instead of mangling it. llama3.2 often returned one of its few-shot examples instead ("The gain η exceeds ρ under load.", "Here is the converted line:"), which the gates refuse.

**Chat mode.** Muse is chat- and reasoning-tuned. With its native `Reasoning strength: low` system prompt it was correct on a 3-line smoke test, but took 40–55 s per line on ~2–3K characters of hidden reasoning. This build ignored every per-request way of turning thinking off that I tried: `reasoning_budget: 0`, `chat_template_kwargs.enable_thinking: false`, and a `Reasoning strength: none` system line. Each one spent its whole token budget reasoning and returned an empty answer. Raw completion is the usable mode.

**What adopting Muse would have taken.** About 20 GB resident. md-press would need a llama.cpp backend; md-press only speaks ollama's `/api/generate`. Muse's notes also mention an ollama MLX build, `muse-glimmer:30b-mlx` (~21 GB), not pulled here, which would additionally need md-press to send ollama the prompt `raw`, since it is a chat model. A cold press of the estate would take ~7 h against llama3.2's ~1.5 h; md-press's proposal cache makes re-runs free. The GGUF lives on the T7 Archive drive, linked from `~/models/muse-glimmer/gguf`.

## What this work changed in md-press

- **`operands_survive`**, a gate. Every letter-run, digit-run, Greek letter, script capital, norm or absolute-value bar, and trailing full stop or comma in the text being replaced must reappear in the span that replaced it. Reading llama3.2's five accepted errors above showed the content gate only checked that a span invented nothing *anywhere in the line*. On the stored proposals, the new gate refuses exactly those five and none of Muse's 138.
- **`*` preservation**, added to that gate after the converter spike's independent verifier found a single-`*` emphasis marker deleted and passed. Re-judging this data changes no earlier correct conversion.
- **`\epsilon`, `\frac`, `\max`, subscript words, `\infty`.** These came from the same day's whole-file reviews rather than from this sample, and are recorded in STATUS.

## The detector census (same harness, same day)

`harness/src/census.rs` runs md-press's own math detector at its own parse-identified prose sites and writes one TSV row per firing: path, unwrapped line, `X` if excluded by `.md-pressignore`, the trigger glyphs, and the text truncated to 220 characters. It runs over the estate's tracked `.md` files outside `.moved*`, `_older`, `_ref` and `ruby-community` (12,281 files).
- `census/fires-before.tsv`: run with md-press's detector *before* the 2026-10-02 rework. Commit `8614e44` has that detector, if you want to regenerate it.
- `census/fires-after.tsv`: run with the detector as it stood after the rework.

Both are gitignored: 11 MB and 2.3 MB of estate text, and they regenerate from the recipe below. Their summary numbers are in STATUS: **44,647 → 8,635 would-be model calls**. Two thirds of the old calls came only from prose `→`/`·`. In a uniform sample of 40 remaining firings, about 36 were real math.

## Harness

Everything builds from `harness/` (`cargo build --release`). It depends on md-press by relative path, so it judges with whatever gates md-press has when you build it. The packaged `judge` reproduces `data/judged-*.jsonl` byte for byte as of 2026-10-03.

- `census`: paths on stdin → TSV on stdout. File list recipe: for each git repo found with `find ~/src -maxdepth 4 -name .git -not -path './_ref/*'`, run `git ls-files '*.md'`, then drop paths containing `/.moved`, `/_older/`, `ruby-community` or `/_ref/`.
- `pieces`: paths on stdin → one JSON line per piece md-press would send to a model. It collects them through md-press's `Model::fake` seam, so they are exactly what the pipeline sends. Its full estate output is `../spike-unicode-math-2026-10-02/inputs/pieces.jsonl`, not duplicated here.
- `judge`: `{text, proposal}` JSON lines on stdin → the same records with `outcome`, `reason`, and `result` (what md-press would write).
- `propose.py DIR llama|muse`: collects proposals for `DIR/sample.json`. Its prompt-file path is hard-coded to md-press, and the `muse` run needs `llama-server` running on `127.0.0.1:8080`.
- `analyze.py DIR [diff]`: the outcome summary and a per-piece minimal-edit view. It reads `DIR/judged-{llama,muse}.jsonl`.
- `nmp`, `fp`, `fpdiff`: small debugging probes, for the detector on stdin lines, and for render fingerprints of `format_plain` (whole, and first-difference).

## Machine state this left behind

On 2026-10-02 `ollama pull llama3.2:3b` replaced that model's dangling symlink, which pointed into the then-unmounted T7, with a real 2 GB file on the internal drive. md-press's model therefore no longer needs the drive. The T7's copy of that blob (`sha256-dde5aa3…`) is now unreferenced, and Joseph can delete it whenever he likes.
