# Config

Aspectus loads how *this reader* wants the look, and it can show what it loaded.

It is **not** a per-project overlay. The same tree should not change its law because `asf/` has a file and `logos/` does not. The interesting overlay is who is looking (human / script / logogenic agent, and which harness+model), not where the locus sits.

This row is the first arbitrator of the [[aspect-lattice|Aspect lattice]]: which facts are `ON` / `OFF` / `QUIET`, which format is starred, which offices are asked for. The pipeline still decides *when* a fact is implemented.

## Precedence (left loses)

```text
defaults  <  global-config  <  user-home-config  <  agent-type-config  <  env  <  flags
```

Right shadows left. No locus walk. No repo file.

| Layer | What it is |
|---|---|
| **defaults** | Built-in lattice stars and `ON`/`OFF`/`QUIET`. |
| **global-config** | Machine / all-users file, if we ever install one. Not required. |
| **user-home-config** | The user’s file (XDG home). This is the ordinary overlay. |
| **agent-type-config** | Overlay for the *caller kind*, then for harness+model inside logogenic. |
| **env** | `ASPECTUS_*` (and any named-caller env we later bind). |
| **flags** | Highest. Includes the undramatic caller flag a tool-description can pass. |

`--config=PATH` substitutes for **user-home-config** on this invocation. It is not discovered from the tree.

## Agent-type

Detect, in this order of grain:

1. **Kind:** human · script · logogenic-agent.
2. **Inside logogenic:** harness + model, when known.

Detection is best-effort (TTY, `CI=`, parent, whatever is honest). It must not be the contract. The undramatic path is a **flag** the agentic tool-description passes automatically (`--caller=…` spelling still open). A tribal `ASPECTUS_AGENT_MODE=1` is not the happy path.

Do not invent a harness taxonomy in this row. The flag’s value is a key; files or sections keyed by that value wait until we have two real callers.

## Foundations (clauses)

| Clause | Where |
|---|---|
| Asked-for “what did you load?” is data → stdout | [[../../../principles/src/norm-stdout-is-data\|norm-stdout-is-data]] |
| Estate overlays stay out of baked world-law | [[../../../principles/src/norm-overlays-are-config\|norm-overlays-are-config]] |
| Secrets never as argv | [[../../../principles/src/norm-secrets-never-argv\|norm-secrets-never-argv]] |
| Detect human vs machine; do not require a tribal agent-mode env | [[../../../principles/src/norm-machine-path-is-detected\|norm-machine-path-is-detected]] (flag-as-contract is this design’s overlay of “later gift”) |
| Only the caller tunes the channel; the place may offer content, never tuning | [[../../../principles/src/norm-caller-tunes-the-channel\|norm-caller-tunes-the-channel]] (Regime-I adversarial-content grounding) |
| The caller declares itself with a key (`--caller`) | [[../../../principles/src/form-caller-key\|form-caller-key]] |
| Discovery-from-the-tree is write-scope only; tuning never walks the locus | [[../../../principles/src/norm-config-walks-from-locus\|norm-config-walks-from-locus]] · [[../../../principles/src/form-config-precedence\|form-config-precedence]] (both rewritten 2026-08-14 to match this design — no divergence remains) |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Precedence | A key set in two layers: the right-hand layer wins. | Same key in defaults, user-home, env, flag; flag wins. Repeat without the flag: env wins. |
| 2 | No locus file | `aspectus ~/src/arch/asf` does **not** read a file from `asf/` or any parent of the locus. | Fixture tree with a decoy overlay at the locus; show does not name it; values do not change. |
| 3 | User-home | The user-home file, if present, applies regardless of CWD or PATH. | Run from `/tmp` on a nested PATH; user-home keys apply. |
| 4 | `--config=PATH` | That file is user-home for this run. | Explicit file wins over the real user-home file. |
| 5 | Agent-type | Kind is classified; if a caller flag is present, it is the agent-type for configuration selection. | Human TTY vs piped vs `--caller=…` produce different winning layers in `config` show. |
| 6 | Show | `aspectus config` prints, on **stdout**, which layers were consulted, which existed, which won. Exit 0. | No files: show names `defaults` only. |
| 7 | No file is fine | Missing layers are not an error. Defaults apply. Show lists them as absent. | Clean home: exit 0, show says defaults. |
| 8 | Help | Help lists `config`, `--config`, and the caller flag once this row ships. Examples updated in the same commit. | Help snapshot. |
| 9 | Lattice | A lattice office that is configurable is read through this stack, not hardcoded, once the office exists. | After line-count exists: user-home can turn it `OFF`; a flag can turn it back `ON`. |

## Open

Filenames for global and user-home. Caller-flag spelling. How agent-type files are laid out (one file with sections vs one file per key). Do not pick in the implementation; one constant each.

What a file *may contain* waits on lattice offices as they land. This row is *how* a value arrives, *who* it is for, and *showing* the path it took.

## Not in this row

Writing config. A harness zoo. Furniture mappings as content. A file in the project that changes the look (refused — see [[place-wants-known|What the place wants known]] for a later *secondary* channel: notes from the place, not control of the eyes).
