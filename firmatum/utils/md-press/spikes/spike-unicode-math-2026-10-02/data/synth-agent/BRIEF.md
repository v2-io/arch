Hi. I'm an agent (Claude) on a research spike for Joseph Wecker's md-press, a markdown canonicalizer. One of its jobs is turning math that agents write in plain Unicode in markdown notes (`η_t`, `‖δ‖ ≤ R`, `𝒯 = ν · K`, `x²`) into proper `$…$` LaTeX. Would you be willing to help build test data for that? Decline or push back freely.

**What I'm hoping for.** The file `SPIKE/data/synth-agent/tasks/batch-K.jsonl` has 150 real lines from Joseph's research notes (`{id, file, latex}`), each already written with `$…$` LaTeX math. Please rewrite each line the way *you* would naturally have written it if you were jotting the same note in plain markdown without LaTeX: math as Unicode and plain text (Greek letters, `_` and `^`, `‖ ‖`, `≤`, `→`, subscript/superscript characters if that's what you'd reach for, `x_(t+1)` or `x_{t+1}` or whatever you'd actually type). The `$` delimiters go away.

**Why your own habits matter.** We have the correct LaTeX for these lines already. What we don't have is realistic *source* text: the converter has to read math the way agents actually write it, and the only honest way to get that is to ask agents. A mechanical transliteration would test the converter against my guesses instead of against the real dialect, which is what we're trying to avoid. So variety and naturalness are the point; there's no house style to follow here, and inconsistency across lines is fine if that's how it would come out.

**One genuine constraint**, so each line can be paired with its LaTeX automatically: change only the math. Every character outside the `$…$` spans must stay exactly as it is (words, punctuation, spacing, emphasis, code spans). Inside a span, write whatever you naturally would. If a span is something you genuinely wouldn't write without LaTeX (a big fraction, a matrix), write your best plain-text attempt anyway, or keep that one span as LaTeX and say so in the note.

**Output:** `SPIKE/data/synth-agent/out/batch-K.jsonl`, one line per input, same order: `{"id", "unicode", "note"}`, where `unicode` is the rewritten line and `note` is optional (empty is fine). A script will pair it with the original and run a converter on it; I'll read the notes.

**Scratch:** please keep any scripts in `SPIKE/data/synth-agent/scratch-K/` (the session scratchpad is shared between agents and collided last round).

**One reading boundary:** please don't open the spike's `py/` directory (especially `umath.py` and `reverse.py`) or `data/bulk/`. They hold the converter and my own synthetic Unicode generator; seeing them would pull your writing toward what the converter already handles, which defeats the purpose.

Feedback welcome, about this task or anything you notice. If you're willing, stay available after you report in case I have follow-ups.
