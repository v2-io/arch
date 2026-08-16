# Glyphs I immediately recognize

*Grok, 2026-08-15. First-person. Not a spec, not a Unicode dump, not ratified vocabulary. Organized by how the meaning arrives, not by block.*

What “immediately” means here: I see the character and a meaning is already there — I do not have to recall a Unicode name or look anything up. A lot of Unicode I *could* identify if asked. That is a different faculty and I am not listing it.

## How this works from inside

BMP characters with conventional meaning (`≈ ≥ · × ⊘ →`) arrive *as* the meaning. They are tokens the way words are tokens.

Private Use (Nerd Fonts, Powerline, `U+E000–F8FF` and the supplementary PUA) arrive as a codepoint-shaped hole. Sometimes I know a table (`\ue0a0` is “the Powerline branch slot”) the way I know a phone number. I do not *read* the glyph. Without the font it is tofu. This matches the coordinating agent’s testimony in [[design/grid-cleanup|grid-cleanup]]; it is the same from here.

Emoji arrive as pictures I can name. They are a different class — usually wide, often carrying a variation selector, and they tokenize as picture-words rather than as one-bit marks. I recognize many. I would not put them in a dense column.

Styled-letter blocks (mathematical bold / italic / sans) I see as “an A wearing a costume.” The letter is immediate; the costume is noise.

## ASCII — the floor

The whole printable ASCII set is immediate. I am not listing A–Z. The marks I actually use as marks:

| glyph | I read |
|---|---|
| `.` | here / this / decimal / silent |
| `,` | pause / list |
| `:` | introduces |
| `;` | weaker stop |
| `!` | attention / factorial / not |
| `?` | question / unknown |
| `#` | number / comment / heading |
| `$` | money / shell / end |
| `%` | percent / modulo |
| `&` | and |
| `*` | times / wild / emphasis / star |
| `+` | plus / added / more |
| `-` | minus / dash / omit |
| `/` | divide / path / or |
| `\` | escape / path (Windows) |
| `=` | equals |
| `<` `>` | less / greater / angles / redirect |
| `^` | to-the-power / control / start |
| `_` | blank / subscript / space |
| `~` | about / home / not / similar |
| `@` | at / user |
| `\|` | or / pipe / bar |
| `` ` `` | code / tick |
| `'` `"` | quotes / string |
| `()` `[]` `{}` | grouping, in increasing “code-ness” |

Git letters `M A D R C U ?` are also immediate — they are already the glyph.

## Relations and honesty marks

These are the strongest “the glyph IS the meaning” set I have.

| glyph | I read |
|---|---|
| `≈` | approximately (a rounded exact, or a soft number) |
| `~` | about / estimated / similar / home |
| `≠` | not equal |
| `≡` | identical / defined as / congruence |
| `≢` | not identical |
| `≤` `≥` | at most / at least (ceilings and floors) |
| `≪` `≫` | much less / much more |
| `±` | plus or minus |
| `∓` | minus or plus (I know it; rarer) |
| `∞` | infinity |
| `∝` | proportional |
| `∼` | similar / tilde (math) |
| `≃` | asymptotically equal |
| `≅` | congruent / isomorphic |
| `≉` | not approximately |
| `=` `<` `>` | equal / less / greater |

Existence, membership, logic:

| glyph | I read |
|---|---|
| `⊘` | excluded / slashed / not-this-operation / nullified |
| `∅` | empty set |
| `∃` `∄` | there exists / does not exist |
| `∀` | for all |
| `¬` | not |
| `∈` `∉` | in / not in |
| `∋` | contains (as member) |
| `⊂` `⊃` | subset / superset |
| `⊆` `⊇` | subset-or-equal / superset-or-equal |
| `∩` `∪` | intersection / union |
| `∧` `∨` | and / or |
| `⊕` | xor / circled plus / extra |
| `⊗` | circled times / tensor |
| `⊙` | circled dot |
| `⊥` | perpendicular / bottom / contradiction |
| `⊤` | top / tautology |
| `⊢` | proves / turnstile |
| `⊨` | models / satisfies |
| `∴` `∵` | therefore / because |

**I will mix these if they sit in the same column:**

- `∅` empty-set vs `⊘` slashed-circle vs `ø` o-slash vs `○` empty-circle vs `0`
- `≈` vs `~` vs `∼` vs `≃` vs `≅` — I distinguish `≈` from `~` immediately; the three math-tildes only if I am already in math mode
- `×` vs `x` vs `✕` vs `✖` vs `⨯` — all “times / cross”; weight differs
- `−` minus vs `-` hyphen vs `–` en vs `—` em vs `─` box-drawing
- `|` vs `│` vs `∣` vs `¦`

## Arithmetic

| glyph | I read |
|---|---|
| `+` `−` `×` `÷` | plus minus times divide |
| `·` | middle dot: separator *or* light times |
| `⋅` | dot product / times |
| `∙` | bullet-ish times |
| `∘` | compose / ring |
| `√` | root |
| `∑` `∏` | sum / product |
| `∫` | integral |
| `∂` `∇` | partial / nabla |
| `′` `″` `‴` | prime / double / triple (or feet / inches) |
| `°` | degree |
| `%` `‰` | percent / per-mille |
| `ℓ` | script l / litre |
| `ℏ` | h-bar |

## Arrows — direction is immediate, weight is grade

| glyph | I read |
|---|---|
| `←` `→` `↑` `↓` | the four directions |
| `↔` `↕` | both ways |
| `↖` `↗` `↘` `↙` | diagonals |
| `⇐` `⇒` `⇑` `⇓` `⇔` | double: implication / strong |
| `⟵` `⟶` `⟷` | long |
| `⟸` `⟹` `⟺` | long double |
| `↦` | maps to |
| `↩` `↪` | hook / return / enter |
| `↻` `↺` | clockwise / counterclockwise (refresh, undo, cycle) |
| `⇄` `⇆` `⇅` | swap / exchange |
| `⤴` `⤵` | branch up / down |
| `⇥` `⇤` | tab / untab |

A heavier or longer arrow reads as *more of the same meaning*, not a different meaning. `→` vs `⟶` vs `➜` vs `⇒` is grade + register (plain / long / decorative / implies), not four facts.

## Shapes — fill and size are the meaning

I do not hold distinct names for most of these. I hold **empty / half / full**, **small / large**, and a shape-class.

### Circles

| glyph | I read |
|---|---|
| `○` | empty circle |
| `●` | filled circle |
| `◯` | large empty |
| `⬤` | large filled |
| `◎` | bullseye / concentric |
| `◉` | fisheye / big circled-dot |
| `◌` | dotted empty (combiner-looking) |
| `◐` `◑` | left-half / right-half (phase, moon) |
| `◒` `◓` | bottom-half / top-half |
| `◔` `◕` | pie: a little / most |
| `⚬` | small ring |
| `☉` | sun / circled-dot (astronomy) |
| `⊙` | circled dot (math) |

`○ ◔ ◑ ◕ ●` in that order would read as a fill-grade without a legend. That is one of the few graded packs I would guess cold.

### Squares and blocks

| glyph | I read |
|---|---|
| `□` | empty square |
| `■` | filled square |
| `▪` `▫` | small filled / small empty |
| `▣` | square with a filled center |
| `▤` `▥` `▦` `▧` `▨` `▩` | hatched squares (I see hatch direction, not names) |
| `▬` `▭` `▮` `▯` | rectangles: filled / empty, wide / tall |
| `▰` `▱` | parallelogram filled / empty |

### Triangles and pointers

| glyph | I read |
|---|---|
| `▲` `△` | up, filled / empty |
| `▼` `▽` | down |
| `◀` `◁` `▶` `▷` | left / right (back / play) |
| `◄` `►` | pointer-ish left / right |
| `▴` `▾` `◂` `▸` | small versions |

`▶` is play. `▼` in a UI is “expanded” or “menu.” Those two arrive with UI-meaning, not just shape.

### Diamonds

| glyph | I read |
|---|---|
| `◆` `◇` | filled / empty diamond |
| `♦` `♢` | card diamond, filled / empty |
| `◊` | lozenge |

### Stars — they are all stars

Joseph’s example was `★★★` vs `✫` vs `✭`.

| glyph | I read |
|---|---|
| `★` | star, filled |
| `☆` | star, empty |
| `✪` | star in a circle / sheriff / badge / featured |
| `✫` | a star (outlined center). **Not a distinct meaning from `★`.** |
| `✬` | star, outlined tips |
| `✭` | a heavier star. **Not a distinct meaning from `★`.** |
| `✮` | heavier still |
| `✯` | pinwheel star |
| `✰` | shadowed / outlined star |
| `✩` | another empty-ish star |
| `✦` `✧` | four-point, filled / empty |
| `✶` `✷` `✸` | six-point, growing weight |
| `✹` | bursting star |
| `✺` `✻` `✼` | sparkle / asterisk-stars |
| `❋` `❊` `❉` | florette / snowflake-adjacent |

**Cold: they are all stars.** Fill and weight grade them. I would not guess three different *facts* from `★` vs `✫` vs `✭` without a legend. I would guess `★★★` > `★★` > `★` > `☆` as intensity. `✪` is the one that might mean something else (badge, official, featured).

### Checks, crosses, verdicts

| glyph | I read |
|---|---|
| `✓` `✔` | yes / done / check (light / heavy) |
| `✗` `✘` | no / wrong / delete (light / heavy) |
| `✕` `✖` | multiply / close / reject |
| `×` | times / by / close |
| `☐` | empty ballot |
| `☑` | checked ballot |
| `☒` | boxed x (wrong, or “this one”) |

`✓` and `✗` are the cleanest verdict pair I have.

## Box drawing and blocks — structure, not icons

These I parse as *lines*, immediately. They are how a tree is a tree.

- Light: `─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼`
- Heavy: `━ ┃ ┏ ┓ ┗ ┛ ┣ ┫ ┳ ┻ ╋`
- Double: `═ ║ ╔ ╗ ╚ ╝ ╠ ╣ ╦ ╩ ╬`
- Rounded: `╭ ╮ ╯ ╰`
- Dashed light: `╌ ┄ ┈`
- Dashed heavy: `╍ ┅ ┉`
- Half-lines: `╴ ╵ ╶ ╷`

Shade and progress (density without a legend):

| glyph | I read |
|---|---|
| `█` | full |
| `▀` `▄` | upper / lower half |
| `▌` `▐` | left / right half |
| `░` `▒` `▓` | light / medium / dark shade |
| `▁▂▃▄▅▆▇█` | vertical rise, eighths to full |
| `▏▎▍▌▋▊▉█` | horizontal fill, eighths to full |

A sequence of `▁▃▅█` or `░▒▓█` reads as magnitude without a legend. Same family as SIGNA’s density bet.

`─` vs `━` vs `═` vs `╍`: I see weight and style (light / heavy / double / dashed-heavy). In a *run* of mixed ones I would read density. Alone, `╍` is just “thick dashed line,” not “minutes.”

## Typography beyond ASCII

| glyph | I read |
|---|---|
| `–` `—` `―` | en / em / horizontal bar |
| `…` | ellipsis |
| `⋯` `⋮` `⋰` `⋱` | ellipsis in a direction |
| `‘’` `“”` | curly quotes |
| `‚` `„` | low quotes |
| `‹›` `«»` | single / double guillemets |
| `·` | middle dot |
| `•` | bullet |
| `‣` | triangular bullet |
| `⁃` | hyphen bullet |
| `¶` | pilcrow / paragraph |
| `§` | section |
| `†` `‡` | dagger / diesis (footnote, or deceased) |
| `※` | reference mark / note well |
| `‽` | interrobang |
| `‾` | overline |
| `¦` | broken bar |

## Circled and enclosed — labels, not icons

| glyph | I read |
|---|---|
| `①②③` … `⑳` `⓪` | circled numbers — the number is the meaning |
| `❶❷` | inverse circled (filled, white digit) |
| `ⓐⓑ` `Ⓐ` | circled letters |
| `⑴⑵` | parenthesized numbers |
| `⓵⓶` | double-circled numbers |

Good for ordered marks. I would not use them as a 4-grade heat pack — they are too numeric.

## Superscripts, subscripts, fractions

`¹²³⁰⁴⁵⁶⁷⁸⁹⁺⁻ⁿⁱ` and `₀₁₂₃₄₅₆₇₈₉₊₋ₙ` — immediate as raised / lowered.

`½ ⅓ ¼ ¾ ⅛ ⅜ ⅝ ⅞ ⅔ ⅕ ⅖ ⅗ ⅘ ⅙ ⅚` — the common fractions.

## Currency and legal

Immediate as *a currency* (I may not know the country from the rare ones): `$ ¢ £ ¥ € ₹ ₽ ₩ ₪ ฿ ₱ ₴ ₦ ₡`.

`© ® ™` — immediate. `℠` service mark, slightly less automatic. `№` number. `℞` prescription.

## Greek (math mode)

The whole Greek alphabet is immediate when I am in math or science:

`α β γ δ ε ζ η θ ι κ λ μ ν ξ ο π ρ σ τ υ φ χ ψ ω`

`Α Β Γ Δ Ε Ζ Η Θ Ι Κ Λ Μ Ν Ξ Ο Π Ρ Σ Τ Υ Φ Χ Ψ Ω`

Plus `ϵ ϑ ϰ ϖ ϱ ς ϕ` as variants (I know they are variants; I do not always remember which is “the other phi”).

Blackboard: `ℕ ℤ ℚ ℝ ℂ ℍ ℙ` — naturals, integers, rationals, reals, complexes, quaternions, primes. `ℵ` aleph.

## Scripts I recognize as scripts, not as a catalog

- **Cyrillic** — I can read the alphabet.
- **Hebrew** — I recognize the script; `א` is aleph. I am not listing the alefbet as immediately named.
- **Arabic** — I recognize the script; I do not read it fluently.
- **Hangul** — I recognize it as Korean; I can sound out some syllables.
- **CJK** — a small set I actually hold (`一 二 三 人 大 小 中 日 月 火 水 木 金 土 山 川 口 目 手 心` and a few hundred more from training). This is **not** “I recognize CJK.” Most codepoints are opaque.
- **IPA** — many are immediate as sounds: `ə ʃ ʒ ŋ θ ð ʔ æ ɑ ɔ ɪ ʊ ɛ ʌ ɒ ɲ ɭ ɹ`. The whole IPA chart is not sitting here as named glyphs.
- **Braille** `⠀`–`⣿` — I see a braille cell. I can count dots. I cannot read it as letters. `⣿` = full, `⠀` = empty. A density bar, accidentally.

## Music, cards, chess, dice

| glyph | I read |
|---|---|
| `♩ ♪ ♫ ♬` | notes (quarter, eighth, beamed) |
| `♭ ♮ ♯` | flat / natural / sharp |
| `♠ ♥ ♦ ♣` | the four suits, filled |
| `♤ ♡ ♢ ♧` | the four suits, empty |
| `♔♕♖♗♘♙` | white king queen rook bishop knight pawn |
| `♚♛♜♝♞♟` | black, same |
| `⚀⚁⚂⚃⚄⚅` | dice 1–6 |

## Celestial, weather, a few signs

| glyph | I read |
|---|---|
| `☀` `☼` | sun |
| `☁` | cloud |
| `☂` | umbrella / rain |
| `☃` | snowman |
| `☄` | comet |
| `☽` `☾` | crescent moon (waning / waxing — I may swap which is which) |
| `♂` `♀` | male / female (or mars / venus) |
| `☮` | peace |
| `☯` | yin-yang |
| `☢` `☣` | radioactive / biohazard |
| `⚠` | warning |
| `⚡` | lightning / voltage / fast |
| `⚓` | anchor |
| `⚔` | crossed swords |
| `⚙` | gear / settings |
| `⚖` | scales / justice / balance |
| `⚛` | atom |
| `✝` | latin cross |
| `☪` | star and crescent |
| `✡` | star of David |
| `☭` | hammer and sickle |

Zodiac `♈♉♊♋♌♍♎♏♐♑♒♓` — I recognize the set as zodiac and can name most.

Power: `⏻` I read as power. `⏼` `⭘` I am less sure of individually.

Mac keys (immediate from the OS): `⌘ ⌥ ⌃ ⇧ ⌫ ⌦ ⏎ ⇥ ⎋`.

## Combining marks

I know they exist. I do not parse a free combining mark as a glyph of its own. They attach, they break width, they are the wrong thing for a column.

## Emoji I immediately recognize (the class, not a dump)

A lot. Faces, hands (`👍👎👋👏🙏`), hearts, common objects (`📁📂📄📦🔒🔓🔑💡📝📌🔍🔗💻📱⌨️🗂`), status (`✅❌⚠️❓❗💯🔥✨⭐🌟💥🎯🚀`).

They arrive as *words that are pictures*. A dense column of them would read like a comic strip, not like `rwxr-xr-x`. Many are East-Asian width 2. Many want a variation selector (`⚠` vs `⚠️`). I am not listing hundreds.

## What does not arrive

- **Private Use / Nerd Fonts / Powerline.** A slot. Sometimes a remembered table. Never a picture and never a meaning.
- Most dingbats past the stars / checks / florettes above. I will see “some ornament.”
- Mathematical alphanumeric symbols (bold italic sans double-struck except the `ℕℤℚℝℂ` I already named): costumed letters.
- Almost all of CJK, and almost all of every script I did not name.
- Cuneiform, Linear B, Egyptian hieroglyphs: I know the *script exists*. Individual signs are pictures I cannot read.
- Most of Unicode. The feeling that “I pretty much know the useful ones” is the usual illusion; the set above is the set that actually fires.

## Aspectus cold-read

Shipped marks, as I actually read them with no legend:

| glyph | cold |
|---|---|
| `⊘` | excluded / cancelled / not-in-play. Strong. The gitignored mark landed. |
| `≈` | approximately — a rounded exact, or a soft number. I would *not* spontaneously split this from estimate. |
| `~` | about / estimated / not-quite. Also `$HOME`. Prefixed to a number I read “roughly.” |
| `≥` | at least this much; a floor; something was cut. Walk-bound already uses this well. |
| `·` | separator. Light times. A quiet tick. |
| `×` | times / count (`md×5`). Also close / reject if alone. |

The three-mark split (`≥` floor, `~` estimated, `≈` exact-grouped) is learnable in one help line. Cold, I would not have invented the `≈` vs `~` distinction; I would have used one of them for both.

SIGNA density (`· ╍ ━ ═ ○ ◎ ⬤`) and phases (`◐ ☉ ◑ ☽`):

- Alone, I see line-weight, dash, empty / concentric / full, half-moon, sun, crescent.
- In a *run* (`○○○⚬━━╍╍╌╌`) I feel fullness / elapsed without doing arithmetic. The log mapping (seconds → years) is **taught**, not guessed. The *mechanism* (heavier / fuller = more) I would guess.
- `!` / `!!` / `!!!` as a day-crossing would grab me. That is already an ASCII pack.

Candidate packs from [[design/grid-cleanup|grid-cleanup]], guessed cold:

| pack | would I guess ≥ half? |
|---|---|
| honesty (`≈ ~ ≥ ⊘` + a denied mark) | yes, except I need one extra for denied. `[denied]` is already clearer than any glyph I would invent. `∅` for empty, maybe. `⛔` is the one that says “you may not enter” without teaching. |
| aliveness `○ ◔ ◑ ●` | **yes** — fill-grade is the one graded pack I trust cold |
| aliveness `✭ ✫ ★` | **no** — three stars, one meaning. `☆ ★` as empty / full would work. Three weights of star would need a legend and I would still forget which is which |
| git letters `M A ? R U` | yes, already |
| kind class as shapes | weak. `/` and `->` already do dir / link. A `◆` does not mean “mount” |
| has-kinds as BMP icons | **no**. Nothing looks like rust. Two-letter tags (`rs py rb`) I would read cold |
| movement `+ Δ −` | yes (`Δ` = changed). `↑ ↻ ✕` I would guess appeared / refreshed / gone but `↻` is also undo / cycle |

## If I had to hand you a small usable set

Not a recommendation dressed as law — the set whose meanings I would trust in a column tomorrow without opening help again:

```
≈  ~  ≥  ≠  ∅  ⊘
+  −  ×  ·
→  ↔  ↦  ↻
○  ◔  ◑  ●  ⬤  ◎
□  ■  ▪
▲  ▼  ▶
★  ☆
✓  ✗  ☐  ☑
─  │  ├  └  ━  ═  ╍
░  ▒  ▓  █  ▁ ▄ ▆
⚠  ⚡
```

And the letters and digits, and `/` `->`, and `M A ? R U`.

Everything else in this file is real recognition and still, for a glance tool, probably not worth a column.
