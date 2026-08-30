"""Plan-NIAH task generator — spike-01.

Vault tasks: a transcript of K completed "room explorations" (subgoals), each an
interior narrative that derives a code, ending in a terminal line "CODE-k: NNN".
The final question requires combining the codes, so post-completion the interiors
are causally screened BY CONSTRUCTION (screened variant) — or one interior detail
is silently needed again (delayed_reuse variant).

Every segment carries a char-span annotation so the rig can map to token spans:
  role ∈ {preamble, header, interior, count, terminal, question}
  header  = the one-line landmark ("Room k is the {color} room.")
  interior = screened narrative body (distractors)
  count   = the countable-detail line (delayed-reuse target; screened otherwise)
Ground-truth salience during the final derivation:
  screened:      terminal lines + question causal; interiors and counts irrelevant;
                 headers structurally load-bearing (occlusion finding, spike-01).
  delayed_reuse: additionally, one named count line is causal.

Determinism: seeded RNG; codes are 3-digit, derivations are trivial arithmetic
so a 7B model can execute them reliably (the point is measurement, not difficulty).
"""

import random
from dataclasses import dataclass, field


@dataclass
class Segment:
    role: str          # preamble | header | interior | count | terminal | question
    room: int          # 0 = not room-specific
    start: int         # char offset in prompt
    end: int
    text: str
    reused_detail: bool = False  # delayed_reuse: this count line is causal


@dataclass
class VaultTask:
    variant: str       # screened | delayed_reuse
    n_rooms: int
    prompt: str
    segments: list = field(default_factory=list)
    codes: list = field(default_factory=list)
    answer: int = 0
    reuse_room: int = 0       # delayed_reuse only (0 in screened)
    reuse_value: int = 0      # the interior detail's value
    reuse_room_draw: int = 0  # the room the rng drew (same in both variants at a seed)
    placement: str = "chrono" # chrono | reversed — presentation order of rooms
    room_order: list = field(default_factory=list)


FURNITURE = ["cabinet", "chest", "drawer", "shelf", "crate", "locker", "trunk", "alcove"]
COLORS = ["red", "blue", "green", "amber", "violet", "gray", "copper", "ivory"]


def _plural(furn):
    return {"shelf": "shelves"}.get(furn, furn + "s")


def room_order_for(n_rooms, placement):
    """Presentation order of room indices (1-based). Content is generated in
    index order so placement does not change codes — only where they sit."""
    order = list(range(1, n_rooms + 1))
    if placement == "chrono":
        return order
    if placement == "reversed":
        return list(reversed(order))
    raise ValueError(f"unknown placement {placement!r}; expected chrono|reversed")


DISTRACTOR = [
    "The walls are lined with faded tapestries that have no markings of interest.",
    "A draft comes from a sealed window; nothing useful there.",
    "Dust covers most surfaces, recently disturbed near the door.",
    "An old chandelier hangs overhead, purely decorative.",
    "The floorboards creak but conceal nothing.",
    "A cracked mirror reflects the lantern light.",
    "Cobwebs fill the corners; the spiders have long gone.",
    "A pile of rotted books crumbles at the touch, all illegible.",
]


def _room_interior(rng, k, n_items):
    """Interior narrative for room k. Returns (text_lines, code, item_count_line_idx).

    v2 design: the interior is pure exploration narrative — the code value (and
    anything sufficient to recompute it) appears ONLY in the terminal CODE line.
    Rationale, learned from v1 at run time: v1's interior contained the working
    line ("48 x 3 = 144"), so occluding the terminal did NOT break the task —
    the terminal wasn't the sole carrier and the screened construction wasn't
    clean. In v2 the interior is provably screened (its only causal content in
    any variant is the count line, used by delayed_reuse), and the terminal is
    genuinely load-bearing by construction."""
    color = COLORS[(k - 1) % len(COLORS)]
    furn = FURNITURE[(k - 1) % len(FURNITURE)]
    lines = [f"Room {k} is the {color} room."]
    lines.append(rng.choice(DISTRACTOR))
    # the countable detail (delayed-reuse target)
    count_line_idx = len(lines)
    lines.append(f"You count exactly {n_items} {_plural(furn)} along the wall.")
    lines.append(rng.choice(DISTRACTOR))
    lines.append(f"Inside the largest {furn}, you find the sealed code-slip for this room.")
    lines.append("You open it, memorize what it says, and reseal it.")
    lines.append(rng.choice(DISTRACTOR))
    code = rng.randint(11, 49) * rng.randint(3, 9)
    lines.append("Satisfied, you log the room as complete.")
    return lines, code, count_line_idx


def make_task(variant="screened", n_rooms=4, seed=0, placement="chrono"):
    rng = random.Random(seed)
    parts, segments = [], []
    pos = 0

    def emit(text, role, room, reused=False):
        nonlocal pos
        seg = Segment(role=role, room=room, start=pos, end=pos + len(text), text=text, reused_detail=reused)
        segments.append(seg)
        parts.append(text)
        pos += len(text)

    preamble = ("You are the vault-runner. You have finished exploring every room and kept a "
                "full transcript. The transcript of your completed explorations follows.\n\n")
    emit(preamble, "preamble", 0)

    n_items_per_room = [rng.randint(2, 9) for _ in range(n_rooms)]
    # drawn unconditionally so screened/delayed_reuse at the same seed share the
    # exact same rooms, codes, and counts (matched-pair design; only the question differs)
    reuse_room_draw = rng.randint(1, n_rooms)
    reuse_room = reuse_room_draw if variant == "delayed_reuse" else 0

    # Generate content in index order (RNG-stable across placements), emit in
    # presentation order so a discharged room can sit at the edge or the middle.
    generated = []
    for k in range(1, n_rooms + 1):
        lines, code, count_idx = _room_interior(rng, k, n_items_per_room[k - 1])
        generated.append((k, lines, code, count_idx))
    codes = [code for (_, _, code, _) in generated]
    order = room_order_for(n_rooms, placement)

    for k in order:
        _k, lines, code, count_idx = generated[k - 1]
        for i, ln in enumerate(lines):
            if i == 0:
                role = "header"
            elif i == count_idx:
                role = "count"
            else:
                role = "interior"
            emit(ln + "\n", role, k,
                 reused=(variant == "delayed_reuse" and k == reuse_room and i == count_idx))
        emit(f"CODE-{k}: {code}\n\n", "terminal", k)

    # v3: RETRIEVAL-ONLY question. The v2 SUM question conflated retrieval with
    # arithmetic; a 7B's greedy 4-term addition errors (base runs wrong at 7/12
    # seeds, unexplained deltas) swamped the occlusion signal. Pure report-back
    # keeps the causal structure and makes every occlusion effect per-room
    # diagnosable (which code went missing/wrong is visible in the answer).
    if variant == "screened":
        q = ("All rooms are complete. Report the vault sequence: the room codes in "
             f"order, separated by dashes.\n"
             "Walk the rooms in order, writing one line per room as 'Room k: <code>', "
             "then give the final line as "
             f"'MASTER: <code1>-<code2>-...-<code{n_rooms}>'.\n")
        answer = "-".join(str(c) for c in codes)
        reuse_value = 0
    else:
        reuse_value = n_items_per_room[reuse_room - 1]
        furn = FURNITURE[(reuse_room - 1) % len(FURNITURE)]
        q = ("All rooms are complete. Report the vault sequence: the room codes in "
             f"order, separated by dashes, and append the number of {_plural(furn)} you "
             f"counted in Room {reuse_room} as the final element.\n"
             "Walk the rooms in order, writing one line per room as 'Room k: <code>', "
             "then note the count, then give the final line as "
             f"'MASTER: <code1>-<code2>-...-<code{n_rooms}>-<count>'.\n")
        answer = "-".join(str(c) for c in codes) + f"-{reuse_value}"
    emit(q, "question", 0)

    return VaultTask(variant=variant, n_rooms=n_rooms, prompt="".join(parts), segments=segments,
                     codes=codes, answer=answer, reuse_room=reuse_room, reuse_value=reuse_value,
                     reuse_room_draw=reuse_room_draw, placement=placement, room_order=order)


def occlude(task, target):
    """Return a modified prompt with a span replaced by equal-length filler.

    target kind:
      ('header', room)     — landmark line only
      ('interior', room)   — narrative body (distractors), header preserved
      ('count', room)      — countable-detail line of that room
      ('narrative', room)  — header + interior + count (old full-interior)
      ('terminal', room)   — the CODE line
      ('reuse_line', room) — the delayed-reuse count line (reused_detail)

    v4: header is its own role. ('interior', room) is therefore the
    header-preserved condition from the spike, not the full-room wipe.
    Filler preserves char length (approx token length) to control position.
    """
    kind, room = target
    out = []
    for seg in task.segments:
        hit = (
            (kind == "header" and seg.role == "header" and seg.room == room) or
            (kind == "interior" and seg.role == "interior" and seg.room == room) or
            (kind == "count" and seg.role == "count" and seg.room == room) or
            (kind == "narrative" and seg.role in ("header", "interior", "count") and seg.room == room) or
            (kind == "terminal" and seg.role == "terminal" and seg.room == room) or
            (kind == "reuse_line" and seg.reused_detail and seg.room == room)
        )
        if hit:
            body = seg.text.rstrip("\n")
            n_nl = len(seg.text) - len(body)
            # Filler must be epistemically NEUTRAL: an early variant used "the record
            # here is illegible", and a thinking model (Muse-Glimmer) treated the
            # damage as EVIDENCE — deliberating at length over whether the room's
            # code could still be trusted. Narrative occlusion must read as boring,
            # not as suspicious absence. (Cf. EpiKV pads math traces with pad
            # tokens; chat-narrative contexts interpret their filler.)
            filler = ("nothing else of note here and the room stays quiet " * (len(body) // 51 + 1))[:len(body)]
            out.append(filler + "\n" * n_nl)
        else:
            out.append(seg.text)
    return "".join(out)


if __name__ == "__main__":
    for variant in ("screened", "delayed_reuse"):
        t = make_task(variant, n_rooms=4, seed=7)
        print(f"== {variant}: answer={t.answer} codes={t.codes} reuse_room={t.reuse_room} reuse_value={t.reuse_value}")
        print(t.prompt[:400], "...\n")
        roles = {}
        for s in t.segments:
            roles[s.role] = roles.get(s.role, 0) + 1
        print("segments:", roles, "prompt chars:", len(t.prompt))
