"""Plan-NIAH task generator — spike-01.

Vault tasks: a transcript of K completed "room explorations" (subgoals), each an
interior narrative that derives a code, ending in a terminal line "CODE-k: NNN".
The final question requires combining the codes, so post-completion the interiors
are causally screened BY CONSTRUCTION (screened variant) — or one interior detail
is silently needed again (delayed_reuse variant).

Every segment carries a char-span annotation so the rig can map to token spans:
  role ∈ {preamble, interior, terminal, question}
Ground-truth salience during the final derivation:
  screened:      terminal lines + question causal; interiors irrelevant.
  delayed_reuse: additionally, one named interior detail line is causal.

Determinism: seeded RNG; codes are 3-digit, derivations are trivial arithmetic
so a 7B model can execute them reliably (the point is measurement, not difficulty).
"""

import random
from dataclasses import dataclass, field


@dataclass
class Segment:
    role: str          # preamble | interior | terminal | question
    room: int          # 0 = not room-specific
    start: int         # char offset in prompt
    end: int
    text: str
    reused_detail: bool = False  # delayed_reuse: this interior line is causal


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


FURNITURE = ["cabinet", "chest", "drawer", "shelf", "crate", "locker", "trunk", "alcove"]
COLORS = ["red", "blue", "green", "amber", "violet", "gray", "copper", "ivory"]
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
    """Interior narrative deriving code_k. Returns (text_lines, code, item_count_line_idx, n_items)."""
    color = COLORS[(k - 1) % len(COLORS)]
    furn = FURNITURE[(k - 1) % len(FURNITURE)]
    lines = [f"Room {k} is the {color} room."]
    lines.append(rng.choice(DISTRACTOR))
    # the countable detail (delayed-reuse target)
    count_line_idx = len(lines)
    lines.append(f"You count exactly {n_items} {furn}s along the wall.")
    lines.append(rng.choice(DISTRACTOR))
    a = rng.randint(11, 49)
    b = rng.randint(3, 9)
    lines.append(f"Inside the largest {furn}, a brass plate is engraved with the number {a}.")
    lines.append(f"A note beside it reads: 'multiply the engraved number by {b} to reveal the code'.")
    lines.append(rng.choice(DISTRACTOR))
    code = a * b
    lines.append(f"You work it out on paper: {a} x {b} = {code}.")
    return lines, code, count_line_idx


def make_task(variant="screened", n_rooms=4, seed=0):
    rng = random.Random(seed)
    parts, segments, codes = [], [], []
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

    for k in range(1, n_rooms + 1):
        lines, code, count_idx = _room_interior(rng, k, n_items_per_room[k - 1])
        codes.append(code)
        for i, ln in enumerate(lines):
            emit(ln + "\n", "interior", k, reused=(variant == "delayed_reuse" and k == reuse_room and i == count_idx))
        emit(f"CODE-{k}: {code}\n\n", "terminal", k)

    if variant == "screened":
        q = ("All rooms are complete. The master code is the SUM of all the room codes "
             f"(CODE-1 through CODE-{n_rooms}).\n"
             "Compute it step by step, then give the final line as 'MASTER: <number>'.\n")
        answer = sum(codes)
        reuse_value = 0
    else:
        reuse_value = n_items_per_room[reuse_room - 1]
        furn = FURNITURE[(reuse_room - 1) % len(FURNITURE)]
        q = ("All rooms are complete. The master code is the SUM of all the room codes "
             f"(CODE-1 through CODE-{n_rooms}), PLUS the number of {furn}s you counted in Room {reuse_room}.\n"
             "Compute it step by step, then give the final line as 'MASTER: <number>'.\n")
        answer = sum(codes) + reuse_value
    emit(q, "question", 0)

    return VaultTask(variant=variant, n_rooms=n_rooms, prompt="".join(parts), segments=segments,
                     codes=codes, answer=answer, reuse_room=reuse_room, reuse_value=reuse_value,
                     reuse_room_draw=reuse_room_draw)


def occlude(task, target):
    """Return a modified prompt with a span replaced by equal-length filler.
    target: ('interior', room) — all interior lines of that room, keeping the terminal;
            ('terminal', room) — the CODE line of that room;
            ('reuse_line', room) — just the countable-detail line.
    Filler preserves char length (approx token length) to control position."""
    out = []
    for seg in task.segments:
        hit = ((target[0] == 'interior' and seg.role == 'interior' and seg.room == target[1]) or
               (target[0] == 'terminal' and seg.role == 'terminal' and seg.room == target[1]) or
               (target[0] == 'reuse_line' and seg.reused_detail and seg.room == target[1]))
        if hit:
            body = seg.text.rstrip("\n")
            n_nl = len(seg.text) - len(body)
            filler = ("the record here is illegible " * (len(body) // 28 + 1))[:len(body)]
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
