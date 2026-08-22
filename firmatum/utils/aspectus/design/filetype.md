# Filetype — the basic type of a node

*Drafted 2026-08-22 under Joseph's grant ("you're all good to do as you please; you see where we're going"), from his sketch in [[lattice-2|Lattice 2]] §Derived information: Directory, Specials, then "the traditional stuff (or our custom designation of stuff) from `file` — with its stat heuristics, then magic bytes detection, file-suffix heuristics, shebang, execution permission bit, and typical lookup hierarchy… I have no problem doing the file-suffix as a tie-breaker which `file` doesn't." Proposed throughout; his to correct.*

## What filetype is (and is not)

**Filetype** is the basic kind of thing a node is — the fact every other row consumes when it asks "is this text?", "is this a directory?", "which census bucket?", "which `ls -F` suffix?". It is one fact with two levels:

```
major / minor      e.g.  text/markdown · image/svg · exe/script · data/json · dir · link
```

It is **not**: symlink target, cloud hydration, permissions, emptiness, git-ignored-ness, owner, or the *kind of place* a directory is (Obsidian vault, Python repo — that is the claims cluster, [[lattice-2|lattice-2]] §Claims, fed by evidence this row does not gather). Those are separate facts that happen to co-occur on a node.

## The detection ladder (precedence, highest first)

Each step either **decides** or **defers**. The ladder stops at the first decision. This is `file(1)`'s order with one deliberate change at step 5.

| # | step | decides | cost |
|---|---|---|---|
| 1 | **stat type** | `dir` · `link` (then re-stat the target for its own type) · `fifo` · `socket` · `block` · `char` · `door` · regular → continue | free (already paid) |
| 2 | **empty regular file** | `empty` (a real minor of `text`? — see Open) | free |
| 3 | **executable bit** (any `x`) | defers, but *marks*: whatever follows gets `exe/` as its major when the content is a script or a native binary; a `+x` `.md` is still `text/markdown` (exec bit is a hint, not a type) | free |
| 4 | **magic bytes** (first ≤ 1 KiB, one bounded read — the same read the null-byte sniff already does) | native binaries (`exe/binary`: Mach-O, ELF, PE), images (`image/png jpeg gif webp bmp tiff ico heic`), archives (`archive/zip gzip bzip2 xz zstd tar 7z`), documents (`doc/pdf`, OOXML via zip+`[Content_Types].xml`), audio/video (`media/…`), databases (`data/sqlite`), `#!` → step 5a | one read |
| 5a | **shebang** | `exe/script` with the interpreter as a trait (`python`, `sh`, `ruby`, `node`, `env`-resolved) | same read |
| 5b | **suffix map** — **the tie-breaker `file` refuses to use** | everything the bytes can't name: `text/markdown udon html css csv tsv json yaml toml xml tex bib rst org`, source languages (`text/rust python ruby js ts …`), `data/…` for structured formats, `log/…` | free |
| 6 | **content sniff** (null byte in the 1 KiB window → `binary`; else `text/plain`) | the floor — never a *kind* word, only text-vs-binary (today's `kind.rs` law) | same read |
| 7 | **unknown** | `unknown` — absent from statistics, never faked as text | — |

Why suffix *after* magic but *before* sniff: bytes cannot lie about a PNG, but they say "ASCII text" about every `.md`, `.udon`, `.rs` — `file`'s famous unhelpfulness — and the suffix is the author's own declaration of kind. Where bytes and suffix disagree in a way that matters (a `.png` that is actually HTML; a `.md` that starts with `\x89PNG`) the bytes win and the line may carry the kind-word as a *surprise* (the quiet law already speaks for "the binary among the .md").

## The taxonomy (major / minor)

| major | minors (open list; config-extendable) | notes |
|---|---|---|
| `dir` | — | the `/` suffix; the claims cluster says what *kind of place* |
| `link` | `file` · `dir` · `broken` | target's type rides as a trait |
| `special` | `fifo` · `socket` · `block` · `char` · `door` | `ls -F` suffixes where they exist (`\|` `=`) |
| `text` | `plain` · `markdown` · `udon` · `html` · `css` · `csv` · `tsv` · `xml` · `tex` · `bib` · `rst` · `org` · `adoc` · `diff` · … | counts lines; the mass unit |
| `text` (source) | `rust` · `python` · `ruby` · `js` · `ts` · `c` · `cpp` · `go` · `java` · `erlang` · `elixir` · `haskell` · `zig` · `lua` · `shell` · `sql` · … | counts lines; language is the minor so the census can say `rs×19` *or* `rust×19` per format |
| `data` | `json` · `yaml` · `toml` · `ini` · `sqlite` · `parquet` · `arrow` · `npz` · … | structured data; counts lines where text |
| `log` | `log` · `jsonl` · `ndjson` · … | text, but the reader's question is "how big", not "what does it say" |
| `exe` | `script` (trait: interpreter) · `binary` (trait: mach-o/elf/pe) | the exec bit + shebang/magic; `*` suffix per `ls -F` if wanted |
| `image` | `png` · `jpeg` · `gif` · `webp` · `svg` (text!) · `heic` · … | `svg` is `image` by purpose and text by bytes — major wins for the census bucket, text wins for line-count |
| `media` | `mp3` · `mp4` · `mov` · `wav` · `flac` · … | binary |
| `doc` | `pdf` · `docx` · `xlsx` · `pptx` · `pages` · `key` · … | binary; titles later |
| `archive` | `zip` · `gzip` · `tar` · `xz` · `zstd` · `7z` · `dmg` · `iso` · … | binary; "contains things" — a future inside-census |
| `font` | `ttf` · `otf` · `woff` · `woff2` | binary |
| `object` | `o` · `a` · `so` · `dylib` · `rlib` · `rmeta` · `class` · `jar` · `wasm` · `pyc` | build products — usually furniture anyway |
| `model` | `pt` · `onnx` · `gguf` · `safetensors` | binary, huge |
| `binary` | `plain` | sniffed, unnamed |
| `empty` | — | a 0-byte regular file — see Open |
| `unknown` | — | not sniffable (unreadable); absent from stats |

A **trait** rides beside major/minor where it matters: interpreter, binary format, the link target's type, `+x`.

## What consumes it

- **line-count** / **mass**: `text/*`, `data/*` (text ones), `log/*`, `exe/script`, `image/svg` count lines; everything else omits (never `0`).
- **census buckets**: the bucket key is the *suffix* by default (what ships) with a `format` option to bucket by **minor** (`rust×19` vs `rs×19`) or by **major** (`text×22 · image×3`) — the coarser the bucket, the shorter the census; a caller may pick the grain.
- **kind-word** (quiet): speaks when a node's *major* differs from its level's plurality.
- **name-suffix glyph** (`ls -F` family): `/` dir, `|` fifo, `=` socket, `@` link (we use `→ target` instead), `*` executable — which of these ship is lattice-2's call.
- **the `○ ◔ ◑ ●` idea** (Joseph: "can be different types of files"): a one-cell major-class glyph is possible here if ever wanted — text / data / binary / image as four fills — tucked away.
- **JSON**: `type: {major, minor, trait?}` on every node.

## Config

One map, same grammar family as `furniture` / `important`: `kinds = "SUFFIX:MAJOR/MINOR, NAME:MAJOR/MINOR, !SUFFIX"` — superset of today's `SUFFIX:text|binary` (those two words keep working as `text/plain` / `binary/plain`). Magic and shebang tables are code (they are facts about bytes, not preferences); the suffix map and the extensionless-name map are data and ship in the defaults file.

## Open

- **`empty`** — its own major, or `text/plain` with `0` lines? Leaning own major: emptiness is the reader's question ("is this a stub?"), and it keeps `0` honest for real empty text.
- **Source languages as `text/<lang>` vs their own major `source`** — leaning `text` with the language as minor: for mass they *are* text; a `source` major would split the census of a crate into `text×3 · source×19` for no gain.
- **`svg`**: `image` major, text lines — the one double-citizen; decided above, flagged.
- Whether `exe` is a major or a *trait* on `text/<script-lang>` / `binary` — leaning trait (a `+x` Python file is still Python to the census), with `exe/script`/`exe/binary` as the rendered word only where the exec bit is the surprise.
- The minor list is open-ended by design; the defaults file carries the shipped one.
