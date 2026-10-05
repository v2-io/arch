# md-press spikes

Each directory is one investigation, with its own `README.md` as the entry point. Where md-press stands overall, and the decisions these spikes feed, is in `../STATUS.md` → *Where it stands, and how to pick it back up*.

| spike | question | outcome | state |
|---|---|---|---|
| [`spike-unicode-math-2026-10-02/`](spike-unicode-math-2026-10-02/README.md) | Can a deterministic or trained (non-LLM) converter turn Unicode math in prose into LaTeX, deciding where each expression starts and stops? | Yes. Converter v7 has a Python reference and a Rust port that matches it on ~4.6M inputs. On 200 held-out lines it got 84.5% right against llama3.2's 37.5%, with no model, but made about 3× the wrong edits in absolute terms. Its remaining errors are mostly house convention. The scores are against labels written by AI agents. | Spiked, independently verified (`de-novo-feedback-1.md`), corrections answered (`de-novo-response-1.md`). Integration awaits Joseph's decisions; plan in `proposed-integration-plan.md`. v7 has no clean held-out score yet. |
| [`spike-math-model-comparison-2026-10-02/`](spike-math-model-comparison-2026-10-02/README.md) | Is Muse Glimmer 30B a better model for md-press's math pass than llama3.2:3b? | Yes: on 240 real pieces, 138 conversions with 1 minor slip against 88 with 5 wrong ones accepted, at ~6× the time per call. It produced the `operands_survive` gate, and holds the harness and the estate-wide census of what triggers the model. | Set aside in favor of the converter above. |

**Data that came from non-public repos** is kept out of git from 2026-10-03 (Joseph's decision, going forward only): each spike holds it in gitignored files, and its README lists what is local and why. On a fresh clone, those spikes' numbers are reproducible only from the tables they record.
