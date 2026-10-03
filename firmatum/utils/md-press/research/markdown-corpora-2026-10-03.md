# Markdown corpora for md-press training and testing — sources survey

*2026-10-03. Written for whoever picks sources for the unwrap classifier and the Unicode-math to LaTeX converter. Joseph's ask: look online for bodies of markdown that are easy to download or clone, and check `~/src-ext/`. The need: (1) markdown with conformant `$…$` / `$$…$$` LaTeX (to invert into Unicode training pairs), (2) markdown where people type math as Unicode or bare ASCII, (3) naturally paired typeset/plain text, (4) hard-wrapped prose.*

## How to read the marks

- **[V]** I did it myself: cloned, downloaded, opened, counted, or called the API. The number is mine.
- **[C]** A web page or API field claimed it and I did not check it independently.
- Counts come from one throwaway script (`m.py`, in the session scratchpad, `corpora-research/`). Fenced code is stripped first. "Inline `$`" is a loose regex (it admits some currency like `$5 … $10`), so read it as an upper bound; "texish" counts only spans containing `\`, `_` or `^`. "Wrapped" is the share of prose lines (>40 chars, not list/table/quote/heading) that are 68–82 columns long. On un-wrapped prose it sits near 0.1–0.2 by chance, so only values around 0.5 or higher mean anything. The Rust RFC histogram below is the real check.
- The clones are in `/private/tmp/claude-505/-Users-josephwecker-v2-src-arch-firmatum-utils-md-press/9953cca8-a575-46f5-9a98-69ff023a25de/scratchpad/corpora-research/` (about 2 GB, session-scoped). Anyone who wants them should re-clone; the commands are trivial and the shallow-clone date is below.

## Licence in one paragraph, because it gates everything

md-press commits fixtures to a public repo. Permissive licences (MIT, Apache-2.0, BSD, CC0, CC-BY) allow that with attribution. **CC BY-SA** also allows it, but any adapted fixture (for example a text with its math converted to Unicode) inherits BY-SA, so it needs its own licence note and cannot sit silently under the repo's licence. **CC BY-ND** (Stan docs) forbids derivatives, and a converted fixture is a derivative. **No licence at all** (nLab, Obsidian help, QuantEcon repo files) means the right is not granted, so use privately for training and evaluation and commit nothing. **Dataset licences like ODC-By** cover the compilation and say nothing about the copyright in each web page inside it.

## Ranking

### Tier 1: large, real, mostly usable right now

**1. Stack Exchange data dump, per-site, `PostHistory.xml`** — real user-typed markdown, the only large source I found that also contains hand-typed Unicode and bare-ASCII math in the same files as `$` LaTeX.
- Shape: one 7z per site on archive.org (`https://archive.org/download/stackexchange/<site>.7z`), 386 files in the item [V listing]. Sizes [V]: `math.stackexchange.com.7z` 3.6 GB, `stats` 597 MB, `mathoverflow.net` 510 MB, `physics` 729 MB, `cs.stackexchange.com` 131 MB, `mathematica` 292 MB.
- Licence: the item description says all user content is CC BY-SA 4.0 and requires attribution to the Stack Exchange network and per-post author links [V, read the archive.org description]. Fixtures are therefore allowed but BY-SA, with an attribution burden (post id plus author is available in the XML).
- **Use `PostHistory.xml` `PostHistoryTypeId="2"` rows, not `Posts.xml`.** `Posts.xml` `Body` is rendered HTML (`<p>…$O(n \lg n)$…</p>`). `PostHistory` `Text` is the original markdown source. [V on cs.stackexchange, dump dated 2024-04-06]
  - 105,378 initial-body rows, 95 MB of text; 52,063 (49%) contain `$`; 3,143 contain Unicode math symbols (`≤ ≥ α-ω ∈ ∑ ∀ ∃ →`); 2,089 contain `x_i` / `^2`-style ASCII math with no `$` anywhere in the post. Proportions on the math sites will differ; I only downloaded cs.
  - Paragraphs are one line (the web editor soft-wraps). Of 46,048 posts with ≥4 prose lines only 377 look hard-wrapped. So this is a source for the math side and for hard negatives (user-chosen line breaks), not for hard-wrap positives.
  - Extraction needs a 7z reader. `bsdtar -xf` on macOS handles it [V].
- Caveat: SE markdown is SE's flavor (MathJax `$`, `\\` quirks, HTML inline), so the fixtures are real but not CommonMark-pure.

**2. nLab (`ncatlab/nlab-content`)** — by far the densest conformant-LaTeX plus Unicode-in-prose source.
- Shape: git repo, 20,766 `pages/*/*/*/*/<id>/content.md` files, 97 MB, no history needed (`--depth 1` is about 270 MB on disk) [V, cloned 2026-10-03]. Format is "Markdown+itex2MML" per the repo description.
- Counts [V, loose regex]: 12,990 files with dollar math, about 466k inline spans (279k texish), about 117k display blocks, 26k Unicode math characters, 122k ASCII sub/superscript hits. Prose lines only about 10% in the 68–82 band, so effectively not hard-wrapped.
- Dialect to know about: `[[wikilinks]]`, `[[!redirects …]]`, `&lbrack;`/`&rbrack;` entities, `{#anchor}` item tags, `__bold__`. This is closer to Joseph's own estate (wikilinks) than anything else here, which is a plus for realism and a minus for generality.
- **Licence: none.** The nLab home page (fetched) says verbatim "There is currently no consensus on a more formal license statement, but if it matters check if relevant individual contributors state such on their nLab homepages." and "Using and distributing content obtained from the nLab is free and encouraged if you acknowledge the source, as usual in academia." The GitHub repo reports no licence. Use for private training and evaluation; do not commit page text as fixtures without asking the nLab maintainers or restricting to pages whose authors state a licence.

**3. Dive into Deep Learning (`d2l-ai/d2l-en`)** — clean, math-heavy, textbook markdown.
- 209 files, 3.2 MB; 154 files with dollar math, 6,908 inline spans (4,426 texish), 1,313 display blocks [V]. Almost no Unicode math (17 characters), which is itself informative: the authors write LaTeX.
- Important and verified by reading it: **d2l breaks lines at clause or sentence boundaries** ("For a polygon with $n$ vertices,⏎we obtain $n$ triangles."). That makes it a very good hard-negative set for the unwrap classifier (deliberate breaks that should usually be preserved), not a positive-wrap set.
- Licence: text CC BY-SA 4.0 (the repo's `LICENSE`, `LICENSE-SUMMARY`), sample code a modified MIT [V]. GitHub reports `NOASSERTION`. Fixtures are allowed but are BY-SA. Last pushed 2024-08.

**4. QuantEcon lectures (`QuantEcon/lecture-python-intro`, also `lecture-python.myst`, `lecture-python-programming.myst`)** — MyST markdown, economics math.
- Intro repo: 59 files, 1.3 MB, 50 with dollar math, 4,751 inline spans (2,753 texish), 1,569 display blocks [V]. Wrapped-fraction 0.32 suggests some files are partly wrapped, but I did not run the file-level histogram on it.
- Licence: the repo has no LICENSE file [V], but the published lecture site footer says "This work is licensed under a Creative Commons Attribution-ShareAlike 4.0 International" [V, fetched `intro.quantecon.org`]. Treat as BY-SA 4.0 with the licence stated on the site rather than in the repo.

**5. PyMC examples (`pymc-devs/pymc-examples`)** — MIT.
- 158 `.md` (147 are `*.myst.md` notebooks) plus 146 `.ipynb`, 3.3 MB of md; 126 files with dollar math, 3,117 inline (1,793 texish), 858 display blocks [V]. MIT, so the cleanest licence for committing among the math-heavy sets. Markdown cells in the `.ipynb` are a second copy of the same prose in JSON form.

### Tier 2: smaller or narrower, but licence-clean and useful for specific gaps

**6. Rust RFCs (`rust-lang/rfcs`, `text/`)** — the best hard-wrap positives I found.
- 657 `.md`, 9.3 MB, Apache-2.0 per the GitHub API [C: also MIT/Apache dual in the repo README, not checked]. Prose-line length histogram [V] on `text/`: 25,824 lines in 70–79, 13,862 in 60–69, 5,150 in 80–89, a sharp falloff after 80. By a rough file-level test (max prose line ≤90 and ≥30% of lines within 12 columns of the max) 178 of 649 files look hard-wrapped at about 72–80. The rest are long-line or semantic-break.
- Almost no math (85 loose `$` hits, 8 texish, 189 Unicode symbols, mostly arrows and typography). Pure unwrap data.

**7. Ethereum EIPs (`ethereum/EIPs`)** — CC0-1.0 [V `LICENSE.md`, GitHub API].
- 1,013 `.md`, 7.9 MB; 27 files with `$` math, 702 loose inline (367 texish), 491 Unicode math characters, 576 ASCII sub/superscript hits [V]. Mostly one-line paragraphs (5 of 592 prose-heavy files look hard-wrapped). Useful mainly as a CC0 source of mixed Unicode and `$` text and for the blockchain/crypto vocabulary; low volume.

**8. Julia manual (`JuliaLang/julia`, `doc/src/**.md`)** — MIT.
- 182 files, 2.5 MB. Math appears as double-backtick spans (``` ``x^2`` ```) and ```` ```math ```` fences, not `$`; only 11 loose `$` spans [V]. 124 Unicode math characters in prose plus much more in code blocks (`n ≤ 2 ? …`, `.≈`, `π`). Julia's Unicode-first style makes it the best licensed place to see Unicode operators in prose-adjacent text, but the math delimiter convention is Documenter's, so it needs its own handling.

**9. Stan docs (`stan-dev/docs`)** — large and math-dense but **BY-ND**.
- 132 `.qmd` + a few md (Quarto), 2.7 MB; 99 files with dollar math, 4,253 inline (2,706 texish), 1,005 display blocks [V]. Wrapped-fraction 0.34; plausibly partly hard-wrapped (I did not histogram it).
- The `LICENSE` says "The text and images are distributed under the CC BY-ND 4.0 license" and the code is BSD-3 [V]. ND forbids adaptations, so a Unicode-converted fixture is a derivative. Private use only.

**10. Apache Spark docs (`apache/spark`, `docs/`)** — Apache-2.0.
- 254 `.md`, 3.5 MB; 28 files with dollar math (433 loose inline, 262 texish, 33 display). 27 of 249 files look hard-wrapped. A modest ML-doc supplement.

**11. Hugging Face transformers docs (`huggingface/transformers`, `docs/source/en`)** — Apache-2.0.
- 760 files, 5.3 MB; 9 files with dollar math (164 inline, 98 texish, 51 display). MDX-flavored (HTML components, `<Tip>`); probably more useful as a test of tolerance than as math data.

**12. Lean Mathlib docstrings (`leanprover-community/mathlib4`)** — Apache-2.0; markdown embedded in Lean comments, so it needs extraction.
- I cloned only `Analysis/Convex`, `Topology/MetricSpace` and `docs/` (sparse, 220 `.lean`, about 2.6k docstring-like openers). Docstrings combine Markdown with Unicode and backticked math, for example "`‖f + g‖_{L^p} ≤ C * (‖f‖_{L^p} + ‖g‖_{L^p})`" and "for `1 < p` and `-1 ≤ s`, we have `1 + p * s < (1 + s) ^ p`" [V]. The whole library is about 500 MB with tens of thousands of such docstrings [C: from GitHub size; not counted]. This is a very good natural source for how formal-math writers type Unicode inside Markdown, and the `‖…‖ ≤` pattern is the one Joseph's `‖δ‖ ≤ R` example uses. The catch is that it is fenced-in-backticks math, so a conversion pipeline has to decide whether backtick-delimited math is in scope.

**13. GitHub docs, jupyter-book, mystmd, MathJax-docs, SciMLDocs, pml-book** — small or math-light; recorded so nobody re-checks.
- `github/docs` content (CC-BY-4.0, MIT code [C via API]): 3,749 files, 20 MB, only 32 with `$` and 1 display block. One file documents the math syntax; not a math source.
- `executablebooks/mystmd` (MIT): 423 files, 28 with math (117 inline). `executablebooks/jupyter-book` (BSD-3): 42 files, 1 with math. `mathjax/MathJax-docs`: effectively one md file. `SciML/SciMLDocs` (MIT): 44 files, 52 inline. `probml/pml-book`: 27 md, no math in them (the book is in notebooks/PDF). All [V].

### Tier 3: datasets (huge, math-dense, but licence-limited for fixtures)

All fit "download for private training/distribution studies"; none is good for public fixtures because the underlying copyright is on the original authors.

- **OpenWebMath** (`open-web-math/open-web-math` on Hugging Face). ODC-By on the compilation [V, README front-matter]; 6.3M documents, 14.7B tokens, 114 parquet shards [V from the API; shard sizes not checked, `download_size` says about 16 GB]. I pulled 30 rows through the datasets-server API [V]: 222 KB of text, 519 loose `$…$` spans, 24 Unicode math characters. Text is web-extracted: LaTeX is preserved as `$…$`, but the surrounding page chrome ("Entering edit mode 10.4 years ago…") is included and the original page copyright is not covered by the dataset licence. Also exposed as the `open-web-math` subset of `EleutherAI/proof-pile-2` (the HF API reported no licence field for either; the proof-pile-2 README defers to RedPajama and OpenWebMath).
- **FineMath** (`HuggingFaceTB/finemath`, `finemath-3plus`, 128 shards listed in the API). ODC-By [V]. A 30-row sample: 148 KB, 76 loose `$` spans, 180 Unicode math characters [V]. Competitive-programming pages with Unicode and ASCII-typed math (the first row I saw was a Codeforces statement with "n cans", "ai" subscript mangling) are a real, ugly distribution of un-LaTeXed math, but with extraction noise (a mojibake `ai��` showed up).
- **NuminaMath-1.5** (`AI-MO/NuminaMath-1.5`). Apache-2.0 [V via API]; three parquet shards. 30-row sample: 9.4 KB, 197 `$` spans, no Unicode math [V]. Competition problems and solutions with clean `$` LaTeX; short text, not markdown-document-shaped, but the cleanest LaTeX-to-Unicode inversion material in this tier.
- **MATH** (`hendrycks/competition_math`). MIT [V via API]. Problems and solutions in LaTeX with `$…$`; one `MATH.zip`. Same character as NuminaMath: licence-clean, short, LaTeX-only.
- **AutoMathText** (`math-ai/AutoMathText`). cc-by-sa-4.0 on the dataset [V via API] over mixed web/arXiv/code sources, so the dataset licence cannot be taken at face value for the underlying text.
- **Proof-Pile / NaturalProofs**: `hoskinson-center/proof-pile` is apache-2.0, `wellecks/naturalproofs-gen` is MIT [V via API]; neither opened. NaturalProofs draws on ProofWiki (CC BY-SA), Stacks Project and textbooks [C].
- **Nemotron-CC-Math** (`nvidia/Nemotron-CC-Math-v1`): licence "other", gated "auto" [V via API]; not opened.

### Tier 4: natural pairs, and where typed Unicode math shows up

**arXiv abstracts vs publisher abstracts of the same paper** — the one natural pairing I could demonstrate.
- arXiv abstract metadata is plain text with `$…$` LaTeX mixed with raw Unicode in the same abstract. Example from the live API [V]: "…starting from $x_1 \leq x_2 \leq ... \leq x_N$ with drift coefficients $ν_j, 1 \leq j \leq N$…" (LaTeX and a Unicode `ν` in one span). The Kaggle copy `Cornell-University/arxiv` is licensed **CC0** and is 5.56 GB, last updated 2026-09-26 [V via the Kaggle API]; arXiv's own page offers OAI-PMH (preferred), the Atom API, S3 and Kaggle [V]. The CC0 is for the metadata; arXiv says it cannot grant redistribution rights for the papers themselves [V, `info.arxiv.org/help/bulk_data.html`].
- The same paper's published abstract is available from Crossref by DOI. In a 60-record arXiv sample [V], 8 had a DOI in the arXiv record and 5 of those had a Crossref abstract. The one I printed (`10.1088/1361-6420/ac3f82`) renders the arXiv `$\ell^p$` as `<jats:italic>ℓ</jats:italic><jats:sup><jats:italic>p</jats:italic></jats:sup>` and `$\mathbb{R}^{\mathbb{N}}$` as MathML `<mml:math>` with Unicode double-struck letters. So the pairs exist: LaTeX on one side, JATS/MathML (renderable to Unicode) on the other. Yield is low in that sample (about 8% of arXiv records had the DOI link at all), so a real pairing run would join on arXiv id via the arXiv Kaggle metadata `doi` field and Crossref bulk, not on search results.
- OpenAlex abstracts did not help in my one test: the abstract for that DOI was empty (OpenAlex deleted many publisher abstracts) [V].
- Licence for the Crossref abstracts is the publishers', not CC0; Crossref metadata terms say the facts are open, abstracts are deposited by publishers [C]. Fine for private training pairs, not for public fixtures.

**Wikipedia** — not markdown. I checked "Cauchy–Schwarz inequality" wikitext [V]: 188 `<math>` blocks, 11 `{{math|…}}` templates, 0 Unicode math characters in the raw source. Wikipedia's Unicode math is in `{{math}}`-template text and rendered HTML. CC BY-SA. Useful as a text source with a conversion step; I did not build one.

**PubMed abstracts** — Unicode in biomedical prose (`α`, `β`, `≥`, `±`, `×`), no LaTeX. 3 abstracts fetched through E-utilities [V]: 8.7 KB, 3 matching characters, so it is thin per abstract. Bulk baseline files exist at NLM; licence is per-abstract publisher copyright with NLM terms. Not downloaded.

**Project Gutenberg and IETF RFCs as hard-wrap analogs** — plain text hard-wrapped at 65–72 columns: I checked one Gutenberg book [V, *Pride and Prejudice*, line lengths pile up at 65–74] and `rfc9110.txt` [V, 7,552 lines over 30 chars, max line length 72]. They are not markdown, but they supply real hard-wrapped English prose at scale, public domain (Gutenberg, US) and IETF-Trust-licensed (RFCs). Useful if the classifier's break decisions are what matter rather than the markdown structure around them. Neither was bulk-downloaded.

## What `~/src-ext/` holds

About 25 clones, mostly coding-agent tools. Using the same scripts [V], excluding `sapientia.snapshot-backup` and `zoetica.snapshot-backup` (Joseph's private copies; they hold the most math in the tree, about 380 and 91 math lines respectively, but they are his estate, not external data) and the 26 non-English `obsidian-help` translations (6 identical-structure files each):

- **Thin, as Joseph suspected.** Loose dollar-math counts per repo: `aisi/inspect_evals` 72 (14 files, MIT), `aisi/unexploitable-search` 38 (1 file, Apache-2.0), `tangent` 32 (6 files, Apache-2.0), `qwen-code` 18, `aisi/vllm-control-arena` 12, `llama.cpp` 7, `obsidian-help` 6 in English (the other 100+ matches are translations of the same math section), the rest under 10.
- **Worth a look:** `aisi/unexploitable-search/docs/cosatisfiability_methodology.md` (38 math lines, real prose math: "let $P = \{p_1, \ldots, p_K\}$ be the set of properties Charlie may identify…", wrapped at about 80 columns [V]). `aisi/inspect_evals/src/inspect_evals/mask/appendix.md` (30 math lines) and `…/mlrc_bench/README.md` (12, `\frac{s_{\text{agent}} - …}`). `tangent/test-files/TestWorkspace/Markdown Compatibility/Math Notation.md` and `…/Obsidian Compatibility/Math Rendering.md` are display-math stress files (Apache-2.0), useful as unit-test fixtures but not for distribution.
- **Unicode symbol hits** in the repos are mostly arrows, `×`, `·` and typography, not math (my first-pass count was 2,574 in `kilocode`, 2,335 in `warp`, and nearly all of it was `→`/`×`). Heuristically the only real Unicode-math signal in `~/src-ext` is `llama.cpp` (100 ASCII sub/superscript hits, 605 symbol hits, some real) and `aisi/inspect_ai`, `inspect_rl`, `optstop` in small amounts.
- **Hard-wrap candidates:** `agno` docs (`regime-census/agno`: 718 files, 0.68 band-fraction, not histogrammed), `aisi/inspect_ai` (93 files, 0.55) and `live_docs` (427 files, 0.47) exceed the 0.5 or near-0.5 mark, so each is plausibly wrapped; I did not verify any as hard-wrapped by file. `qwen-code` is 0.44. Licences vary per repo (`inspect_ai` MIT; `live_docs` has a `LICENSE` that I did not read; `agno` not checked).
- **`obsidian-help`:** 6,287 `.md` files including 26+ translations; English is only about 578 files. No LICENSE file locally and GitHub reports none, so treat as unlicensed for fixtures. The advanced-formatting page is the one math page, repeated in every language.
- **`tex/`** holds a TeX Live install (10 `.md`, no math). `regime-census/` is a census of about 45 agent-framework repos (5,295 md, 32 with math, mostly stray). Nothing else carried math.

## What I looked for and did not find

- **GitHub code search for non-ASCII math** (`α ≤ ∈ ∀`, `‖ ≤ δ`, `.md`) returned empty results through `gh search code`, and repo search by topic returned generic repos (freeCodeCamp, KaTeX). I could not use it to find hand-typed-Unicode-math READMEs. A different route is the `codeparrot/github-code` dataset (licence "other", ungated; per-file licences are in the rows, Markdown is among its languages [C]) or `bigcode/the-stack*` (gated `auto`; I did not request access). I did not verify that either has usable Unicode-math markdown at scale.
- **A public corpus of Obsidian/Notion vaults with math.** I found none with a clear licence; individual student-notes repos exist but nothing I could vet cheaply.
- **Lil'Log (`lilianweng/lilianweng.github.io`) as markdown.** The repo is the built site (187 `.html`, 0 `.md`), so no markdown source there.
- **Naturally paired Unicode/LaTeX text in markdown.** The only real pairs I could demonstrate are arXiv-vs-Crossref abstracts. Not found: a markdown corpus that ships the same passage both ways. Mathlib docstrings paired with the corresponding `.tex` or paper statements is plausible but unverified.
- **Wrapped-fraction was only checked by file histogram** for Rust RFCs, EIPs, Spark and the Stack Exchange cs dump. Everything else rests on the cruder band fraction and is flagged.
- **Lean Zulip archive, Wikibooks, ProofWiki dumps, Stacks Project** (LaTeX, GFDL-style), **unarXive, S2ORC, NaturalProofs content** were not examined beyond a licence field.
- I did not download the 3.6 GB `math.stackexchange.com.7z`, the arXiv Kaggle dump, or any OpenWebMath shard. The cs.stackexchange numbers above are real, but math.SE is the site that would matter most and its ratios will differ.

## If I were choosing (suggestion only)

- For the LaTeX → Unicode training pairs at scale: Stack Exchange math/stats/physics `PostHistory` (CC BY-SA, real typed noise) plus d2l, PyMC examples and QuantEcon as the clean tail. Keep nLab as a private evaluation set because it is the most Joseph-like in dialect.
- For how people really type Unicode and bare-ASCII math: SE posts that contain Unicode or ASCII math without `$` (cs: 3,143 and 2,089 of 105k), Mathlib docstrings (`‖ ≤ ∀`), Julia docs and EIPs. This is the thinnest need and the least well served.
- For unwrap positives: Rust RFCs (verified, licensed), then Gutenberg/RFC text if non-markdown is acceptable. d2l and EIPs as hard negatives.
- For public fixtures: restrict to MIT/Apache/CC0 (PyMC examples, Rust RFCs, EIPs, Julia, Mathlib, Spark/HF docs) or accept BY-SA with a separate fixture licence note (SE, d2l, QuantEcon).

## Feedback on the request

- The four categories were a good frame. The most useful thing I learned is that category 2 (people typing Unicode math) is the one the public web serves worst; the big math corpora are almost all LaTeX-first, because a markdown author with a MathJax-enabled renderer uses `$`. Joseph's estate may be unusually rich in category 2, which is worth keeping in mind when judging how well any public source stands in for it.
- A wanted distinction I could not make from outside: whether the unwrap side needs hard-wrapped *markdown with its structure* or just hard-wrapped prose. If prose is enough, Gutenberg and RFCs are far larger and cleaner than anything markdown I found.
- I did not check licences at the per-file level inside any repo (for example vendored third-party content in `docs/`), only at repo or site level.
