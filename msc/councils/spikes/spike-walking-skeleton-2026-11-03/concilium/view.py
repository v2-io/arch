"""What a member reads: a compact 'where are we' plus the delta since its last
look, and the minutes (a projection of the canonical log)."""
from . import store, core


def _seat_info(ch, seat):
    return next(s for s in ch["seats"] if s["id"] == seat)


def _item(ch, item_id):
    return next((a for a in ch["agenda"] if a["id"] == item_id), None)


def describe(ev):
    t, b = ev["type"], ev["body"]
    if t == "convened":
        return f"council convened; seats {', '.join(b['seats'])}; quorum {b['quorum']}"
    if t == "round-opened":
        return f"round {b['round']} opened ({b['kind']}{', item ' + b['item'] if b.get('item') else ''})"
    if t == "round-closed":
        lines = [f"round {b['round']} closed ({b['closed_by']}); revealed:"]
        for s, i in b["submitted"].items():
            extra = f" {i['vote']}" if "vote" in i else ""
            txt = f": {i['text']}" if i.get("text") else ""
            lines.append(f"    {s} — {i['action']}{extra}{txt}")
        if b["absent"]:
            lines.append(f"    absent: {', '.join(b['absent'])}" + (f" (gone: {', '.join(b['gone'])})" if b["gone"] else ""))
        return "\n".join(lines)
    if t == "roll-taken":
        return f"roll: present {', '.join(b['present']) or '—'}; absent {', '.join(b['absent']) or '—'}"
    if t == "motion-pending":
        co = " (co-made: identical text)" if b.get("co_made") else ""
        note = f" — {b['note']}" if b.get("note") else ""
        return f"motion pending on {b['item']}, moved by {', '.join(b['makers'])}{co}: \"{b['text']}\"{note}"
    if t == "intent-standing":
        return f"{b['seat']}'s motion did not get the floor; it stands as {b['seat']}'s standing intent: \"{b['text']}\""
    if t == "intent-lapsed":
        return f"{b['seat']}'s standing intent lapsed ({b['why']})"
    if t == "motion-withdrawn":
        return f"motion withdrawn by {', '.join(b['by'])}"
    if t == "question-put":
        return f"question put ({b['why']}): \"{b['text']}\""
    if t == "vote-result":
        return f"VOTE on {b['item']}: {b['result'].upper()} — yes {b['tally']['yes']}, no {b['tally']['no']}, abstain {b['tally']['abstain']} ({', '.join(f'{s}:{v}' for s, v in b['by_seat'].items())})"
    if t == "item-closed":
        return f"item {b['item']} closed ({b['disposition']})"
    if t == "inquorate":
        return f"round {b['round']} inquorate: {b['submitted']} acted, quorum {b['quorum']}; round repeats"
    if t == "quorum-ok":
        return "quorum restored"
    if t == "adjourned":
        return f"ADJOURNED: {b['reason']}"
    return f"{t}: {b}"


def render(d, seat, since):
    ch = store.charter(d)
    evs = store.events(d)
    st = core.fold(ch, evs)
    si = _seat_info(ch, seat)
    out = [f"COUNCIL {ch['council']} — you are seat {seat}. Your vantage: {si.get('vantage', '—')}"]
    if st["adjourned"]:
        out.append("The meeting is ADJOURNED.")
    elif st["item_index"] < len(st["agenda"]):
        it = _item(ch, st["agenda"][st["item_index"]])
        out.append(f"Agenda item {st['item_index'] + 1}/{len(st['agenda'])} [{it['id']}]: {it['title']}")
        if it.get("brief"):
            out.append(f"  brief: {it['brief']}")
    if st["pending"]:
        out.append(f"Pending motion (makers {', '.join(st['pending']['makers'])}): \"{st['pending']['text']}\"")
    if seat in st["standing"]:
        out.append(f"Your standing intent: \"{st['standing'][seat]['text']}\"")
    new = [e for e in evs if e["seq"] > since]
    if new:
        out.append(f"\nSince your last look (events {new[0]['seq']}–{new[-1]['seq']}):")
        for e in new:
            out.append("  " + describe(e).replace("\n", "\n  "))
    r = st["round"]
    if r:
        subs = store.sealed(d, r["id"])
        mine = subs.get(seat)
        out.append(f"\nOPEN ROUND {r['id']} ({r['kind']}). {len(subs)} of {len(r['addressed'])} seats have submitted.")
        if mine:
            out.append(f"You have submitted (sealed): {mine}. You may replace it until the round closes.")
        acts = core.available(st, ch, seat)
        if acts:
            out.append(f"You may submit with council_submit(round=\"{r['id']}\", action=..., ...):")
            for a in acts:
                f = f" {{{', '.join(a.get('fields', []))}}}" if a.get("fields") else ""
                out.append(f"  - {a['action']}{f}: {a['reason']}")
    elif not st["adjourned"]:
        out.append("\nNo round is open.")
    return "\n".join(out), (evs[-1]["seq"] if evs else -1)


def holders_history(d):
    """Seat-holder changes, from the receipts (attestation, not canonical)."""
    import json, os
    out = []
    try:
        for l in open(os.path.join(d, "receipts.jsonl")):
            r = json.loads(l)
            if r["kind"] in ("seat-held", "seat-claim-refused", "seat-released"):
                out.append(r)
    except FileNotFoundError:
        pass
    return out


def minutes(d):
    ch = store.charter(d)
    evs = store.events(d)
    chain = store.verify_chain(evs)
    out = [f"MINUTES — council {ch['council']} (charter sha {core.sha(ch)[:12]}; {len(evs)} events; chain {'OK' if not chain else chain})",
           f"Purpose: {ch.get('purpose', '—')}", "Seats:"]
    for s in ch["seats"]:
        out.append(f"  {s['id']}: {s.get('vantage', '—')} [model {s.get('model', '?')}, harness {s.get('harness', '?')}]")
    hh = holders_history(d)
    if hh:
        out.append("Seat holders (from receipts; who acted behind each seat):")
        t0 = hh[0]["ts"]
        for r in hh:
            if r["kind"] == "seat-held":
                pv = r.get("previous")
                prev = ("" if not pv else f" (after epoch {pv['epoch']} was released)" if pv.get("released")
                        else f" (took over from epoch {pv['epoch']}, whose hold had lapsed {pv['lapsed_for_s']}s)")
                out.append(f"  +{r['ts'] - t0:6.1f}s {r['seat']}: epoch {r['epoch']} held by {r['principal']}{prev}")
            elif r["kind"] == "seat-claim-refused":
                out.append(f"  +{r['ts'] - t0:6.1f}s {r['seat']}: REFUSED to {r['principal']} (held by {r['holder']}, epoch {r['epoch']})")
            else:
                out.append(f"  +{r['ts'] - t0:6.1f}s {r['seat']}: epoch {r['epoch']} released ({r['why']})")
    out.append("")
    for e in evs:
        if e["type"] in ("round-opened",):
            continue
        out.append(f"[{e['seq']}] " + describe(e).replace("\n", "\n     "))
    return "\n".join(out)
