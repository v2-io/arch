# Filetype — finish note (2026-08-22, step 4 of impl/grid-cleanup.md)

*Source: `design/filetype.md` (ladder + taxonomy + consumers), `defaults.toml` `[kinds]`, `design/linecount.md` (read budget, binary-omits-never-0), `design/lattice-2.md` §Structure (specials rows provisional) and the filetype row, wanderings-2026-08-22 §1 (bounded reads). Tests: unit tests in `src/filetype.rs`, `tests/filetype.rs` (5), existing linecount / quiet / json / grid snapshots.*

## What the binary does now

`src/kind.rs` is gone. `src/filetype.rs` is the fact: one `FileType { major, minor, trait }` per node, from the detection ladder. Consumers keep today's rendered text except where the design named a change.

- **Line-count / mass** count `text/*`, text-ish `data/*` (not sqlite/parquet/arrow/npz), `log/*`, `exe/script`, `image/svg`, and `empty` (a real `0`). Everything else omits, never `0`.
- **Census buckets** stay keyed by suffix. `format.census = suffix* | minor | major` is the grain knob (`ASPECTUS_FORMAT_CENSUS`); default suffix so `md×19` does not move.
- **Kind-word (quiet)** compares *countable class* (counts lines vs. does not) against the level's plurality, then speaks the *major*. A `.png` among `.md` says `image`; a `.toml` among `.md` is silent; a `.md` among PDFs says `text`. *(Amended 2026-08-22 at landing: raw-major comparison made every `Cargo.toml` speak `data` in a text dir — a manifest among prose is not a surprise.)*
- **JSON** gains `type: {major, minor, trait?}` on every node (dirs too: `{"major":"dir"}`). Schema stays `1` — additive.
- **Specials** classify (`special/fifo` etc.) and serialize; `ls -F` name-suffix glyphs stay off (lattice-2 rows are provisional / blank stat).

## The ladder, as implemented

1. **stat type** — `dir` · `link` (minor `file`/`dir`/`broken`; target's major as trait on file links) · `special/{fifo,socket,block,char}` · regular continues. Specials are never opened (a fifo read would block the glance).
2. **empty** — 0-byte regular file → major `empty`, line count `0`.
3. **exec bit** — marks; does not decide. `+x` rides as trait when nothing more specific (interpreter / native format) is already there. A `+x` `.md` stays `text/markdown`.
4. **magic bytes** — first ≤1 KiB of a read that was happening anyway. Never a second classification read. Known-binary suffixes are **not** opened just to verify magic; the read budget governs whether a file is read at all.
5a. **shebang** — interpreter as trait (`python`, `sh`, `ruby`, `node`, `env`-resolved). Does **not** rewrite a suffix-named type to `exe/script`.
5b. **suffix/name map** — the tie-breaker. Same `[kinds]` table as step 2.
6. **null-byte sniff** — only when the map is silent: `text/plain` vs `binary/plain`.
7. **unknown** — unreadable / unsniffable. Absent from kind-word statistics.

Line-count of a *visible* text file is one full read; its first 1 KiB is the ladder window, so a `.md` that starts `\x89PNG` becomes `image/png` and drops the count. Unknown suffixes sniff 1 KiB first; if that window is the whole file, there is no second read.

## Magic table (curated, ≤1 KiB)

| family | signatures |
|---|---|
| image | PNG, JPEG, GIF87a/89a, RIFF/WEBP, BMP, ICO, TIFF, ISO-BMFF `ftyp` heic/heix/hevc/mif1/msf1 |
| exe/binary | ELF, Mach-O 32/64 both endians, fat Mach-O (`CAFE BABE` with `nfat_arch` 1..=32 — else Java `object/class`), PE (`MZ` + `PE\0\0` at `e_lfanew`) |
| archive | ZIP (`PK\x03\x04` etc.; `[Content_Types].xml` in the window → `doc/{word,excel,powerpoint,ooxml}`), gzip, bzip2, xz, zstd, 7z, tar (`ustar` at 257) |
| doc | `%PDF` |
| data | `SQLite format 3\0` |
| object | `\0asm` (wasm), Java class (see fat-Mach-O split) |
| media | RIFF/WAVE, RIFF/AVI, fLaC, OggS, ID3 / MPEG audio frame, `ftyp` qt/isom/mp4* |
| font | wOFF, wOF2, OTTO, `\0\x01\x00\x00` (ttf) |
| shebang | `#!` does not decide; it hands the interpreter to 5a |

ISO (`CD001` at 32769) sits past the window — suffix only. Same for dmg.

## Calls made

- **`empty` is its own major** (Open leaning). Line-count still shows `0` — "keeps `0` honest for real empty text"; omitting would make a stub look like binary. Empty `.txt` fixtures are unchanged.
- **`exe` is a trait** (Open leaning), not a major, wherever a suffix already names the language. `script.py` + shebang + `+x` stays `text/python` with trait `python`. Extensionless `#!/usr/bin/env python3` is `exe/script` (nothing else to name it). Native binaries from magic *are* `exe/binary` (ladder step 4) — that's a type, not a `+x` mark.
- **Source languages stay `text/<lang>`** (Open leaning, already in the shipped map).
- **`svg` is `image` and counts lines** (decided in the design). Kind-word among markdown is silent (same countable class); among PDFs it would say `image`.
- **Magic only runs when a read is already happening** (line-count or unknown sniff). A `.png` that is actually HTML stays `image/png`. A `.md` that is actually PNG is caught, because we were reading it to count. Budget is law; opening every binary "just to check" would not be a glance.
- **`data/sqlite|parquet|arrow|npz` no longer count as text.** Step 2's `data/* → text` was coarser than this design's "text-ish `data/*`". Snapshots don't carry those suffixes.
- **JSON schema stays 1.** `type` is additive; field names stay ratifiable while consumers are zero.
- **Census grain default is suffix.** `format.census` ships in `defaults.toml` `[format]`. Layout paint of specials glyphs is not this slice.

## Snapshots

Grid goldens **did not change** on the major-word swap (kitchen `blob.bin` is still `binary`; root-level majors were a tie). After the class amendment: kitchen root is all-countable once links sit outside the statistic, so `Cargo.toml` stays silent; `blob.bin` still speaks. The quiet test `kind_intruder_speaks` now also asserts `doc` on a PDF, silence on `Cargo.toml`, and `text` on a `.md` among PDFs.

## Flagged for the design (not acted on)

- **Ladder vs Open on shebang.** The table says shebang *decides* `exe/script`. The Open leaning says exe is a trait so census still sees Python. Implemented the leaning. Contact with a `bin/` of extensionless scripts will show `exe` as the major; a crate of `+x` `*.py` will not.
- **Magic vs known-binary suffixes.** Bytes cannot win on a file we refused to open. The interesting disagreement (lying `.md`) is the one we already read.
- **Fat Mach-O vs Java class** both start `CAFE BABE`; the `nfat_arch` 1..=32 split is the discriminator. A classfile with a pathological version word in that range would be called Mach-O.
- **OOXML in 1 KiB.** `[Content_Types].xml` as the first zip entry is typical of Office and often in the window; when it isn't, a `.docx` we didn't read stays `doc/word` from the suffix. A `.zip` we *did* read (unknown suffix, or a counting type — rare) becomes `archive/zip` unless the window names OOXML.
- **Symlink-to-fifo** classifies as `link/file` (we followed, target isn't a regular file, we don't read). Honest enough; the node's `is_dir` is false. Not worth a glyph.
- **`door`** (Solaris) is in the taxonomy and not implemented — no such files here.
- Specials name-suffix glyphs (`|` `=`) wait on lattice-2 marking those rows on. Classification and JSON are ready.

## Verified

Full `cargo test --offline` green (lib 43 including the new ladder units; `tests/filetype.rs` 5; grid snapshots byte-identical). Not committed, not `cargo install`.
