# Mass — the size of the unseen

*The short-term point of the whole tool, named. Seeded 2026-08-14.*

Joseph:

> "Right now all agents are very subpar at situational awareness about where they are. They spot and they focus and they discount and never realize where the holes in their understanding are. … I can't tell you how many times I've heard — 'Oh, I didn't realize AAT alone has over 300 segments…' after reading 4 or 5 and reporting that they understand AAT. So the biggest surprisal we are trying to fix in the short term is 'SURPRISE! This "directory" is far more than you thought it was!'"

[[dir-census|Dir census]] is one level deep: `01-aat-core/ [7: 4 dir, 2 .md, 1 other]` tells an agent almost nothing about what those 4 dirs hold. Mass is the cumulative weight of an unexpanded subtree, carried on its line:

```
├── 01-aat-core/  [7: 4 dir, …]  ≈340 files, ≈61k lines
```

so a glance at `asf/` *calibrates the agent's sense of how much it has not seen* before it spends a single read. An agent that says "oriented" after 5 of 300 segments failed because nothing in its glance carried the 300.

## Shape (to design when the row opens)

- **What counts as weight:** descendant file count first; total lines of non-binary files when [[linecount|Line counts]] exist; maybe bytes. Which of these prints, and when (always on dirs at a cutoff? quiet when small?), is lattice work — `size`'s "which number" nuance applies.
- **Honesty under bounds:** mass wants a deep walk; the [[walk-bound|walk bound]] refuses one. A mass computed under a bound prints `≥`; an unbounded mass is exact until the tree changes. This is the row where [[cache|Cache]] pays first — deep mass is the expensive derived fact that makes revisits cheap.
- **Furniture does not count:** `target/`'s 10k build objects are not the mass of a crate. Mass counts what an agent could be expected to comprehend; hidden/omitted furniture and gitignored bodies stay out (or print separately), else every Rust crate looks like a mountain of `.fingerprint`.
- **Not quietable at cutoffs.** Like [[denied|Denied]], mass at an unexpanded dir is existence-information, governed by [[summarization|Summarization]]'s law, not by Quiet.
