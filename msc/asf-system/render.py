#!/usr/bin/env python3
"""Draw ASF's system model and check it.

Reads model.yaml beside this file. Writes:
  out/asf-system.dot, out/asf-system.svg   swimlanes; stocks as Petri places, stations as transitions
  out/report.md                            the structural checks and the flow computation, per regime

Needs PyYAML and numpy, and graphviz's `dot` on PATH. Run from anywhere: python3 render.py
"""
import pathlib
import subprocess

import numpy as np
import yaml

HERE = pathlib.Path(__file__).resolve().parent
OUT = HERE / "out"
M = yaml.safe_load((HERE / "model.yaml").read_text())

LANE_COLOR = {
    "outside_in": "#f2f2f2", "influx": "#e8f1fb", "working": "#fdf3e1", "canon": "#e7f5e9",
    "explanation": "#f3eafa", "application": "#fbe9ec", "outside_out": "#f2f2f2", "sinks": "#e6e6e6",
}
ROLE_SHAPE = {"judgment": "box", "decision": "box", "mechanical": "box"}  # decision points: colour, not a diamond (Joseph)
LANE_ORDER = [lane["id"] for lane in M["lanes"]]


def q(s):
    return '"' + str(s).replace('"', '\\"').replace("\n", "\\n") + '"'


def flows_for(regime):
    """Item types, gain matrix A (A[i,j] = items of j per item of i) and influx vector f."""
    items = list(M["flows"]["items"])
    idx = {k: n for n, k in enumerate(items)}
    A = np.zeros((len(items), len(items)))
    used = {}
    for g in M["flows"]["gains"]:
        if g.get("regime") not in (None, regime):
            continue
        A[idx[g["from"]], idx[g["to"]]] = g["value"]
        used[(g["from"], g["to"])] = g
    f = np.zeros(len(items))
    for k, v in M["flows"]["influx"].items():
        f[idx[k]] = v
    return items, A, f, used


def solve(regime):
    items, A, f, used = flows_for(regime)
    rho = max(abs(np.linalg.eigvals(A)))
    x = np.linalg.solve(np.eye(len(items)) - A.T, f) if rho < 1 else None
    return items, A, rho, x, used


def checks():
    st, stations = M["stocks"], M["stations"]
    consumed = {s["consumes"] for s in stations.values()}
    produced = {p for s in stations.values() for p in s.get("produces", [])}
    out = []
    for k, s in st.items():
        if s.get("external"):
            continue
        if k not in consumed:
            out.append(f"- **No exit transition:** `{k}` declares terminal states {s.get('terminal')} but no station consumes it (invariant 8).")
        if k not in produced and not s.get("founding"):
            out.append(f"- **Nothing produces** `{k}`: no station writes it.")
    no_drain = [n for n, s in stations.items() if not s.get("drainer")]
    out.append(f"- **No drainer** ({len(no_drain)} of {len(stations)} stations): " + ", ".join(f"`{n}`" for n in no_drain)
               + ". The drain condition fails at each until someone answers for it.")
    dec = [n for n, s in stations.items() if s["role"] == "decision"]
    out.append("- **Decision points** (where importance and, later, a charter matter): " + ", ".join(f"`{n}` ({stations[n]['slot']})" for n in dec) + ".")
    return out


def dot(regime="founding"):
    items, A, rho, x, used = solve(regime)
    xs = dict(zip(items, x)) if x is not None else {}
    station_x = {M["flows"]["items"][k]["station"]: xs.get(k) for k in items}
    L = ["digraph asf {", 'rankdir=LR; newrank=true; compound=true; nodesep=0.25; ranksep=0.9;',
         'node [fontname="Helvetica", fontsize=10]; edge [fontname="Helvetica", fontsize=8, color="#555555"];',
         f'label={q("ASF system model, iteration 1 (councils-46, 2026-11-04) against KERNEL draft 2. Regime shown: " + regime + f"   ρ = {rho:.2f}")}; labelloc=t; fontsize=14;']
    for lane in M["lanes"]:
        lid = lane["id"]
        L.append(f'subgraph cluster_{lid} {{ label={q(lane["label"])}; style=filled; color="#bbbbbb"; fillcolor="{LANE_COLOR[lid]}"; fontsize=11;')
        L.append(f'  anchor_{lid} [shape=point, style=invis];')
        for k, s in M["stocks"].items():
            if s["lane"] != lid:
                continue
            pen = "2.5" if s.get("kernel") else "1"
            color = "#c0392b" if s.get("kernel_demand") else "black"
            shape = "ellipse" if not s.get("external") else "folder"
            lab = s["label"] + ("\n[kernel]" if s.get("kernel") else "") + ("\n[demand on the kernel]" if s.get("kernel_demand") else "")
            L.append(f'  "{k}" [shape={shape}, penwidth={pen}, color="{color}", label={q(lab)}];')
        for n, s in M["stations"].items():
            if s["lane"] != lid:
                continue
            drain = s.get("drainer") or "⚠ none"
            xv = station_x.get(n)
            lab = f'{s["kind"]}.{s["verb"]}\nslot: {s["slot"]}\ndrainer: {drain}' + (f"\nload ≈ {xv:.1f} per source" if xv is not None else "")
            fill = "#d9d9d9" if s["role"] == "mechanical" else ("#fff2b3" if s["role"] == "decision" else "white")
            border = "#c0392b" if not s.get("drainer") else "black"
            L.append(f'  "st_{n}" [shape={ROLE_SHAPE[s["role"]]}, style="filled,rounded", fillcolor="{fill}", color="{border}", label={q(lab)}];')
        if lid == "sinks":
            for s in M["sinks"]:
                L.append(f'  "sink_{s}" [shape=doublecircle, style=filled, fillcolor="#333333", fontcolor=white, fontsize=8, label={q(s)}];')
        L.append("}")
    for a, b in zip(LANE_ORDER, LANE_ORDER[1:]):
        L.append(f"anchor_{a} -> anchor_{b} [style=invis, weight=50];")
    lane_of = {k: s["lane"] for k, s in M["stocks"].items()}
    for n, s in M["stations"].items():
        L.append(f'"{s["consumes"]}" -> "st_{n}";')
        for p in s.get("produces", []):
            back = LANE_ORDER.index(lane_of[p]) < LANE_ORDER.index(s["lane"])
            style = ', style=dashed, color="#c0392b", constraint=false' if back else ""
            L.append(f'"st_{n}" -> "{p}" [{style.lstrip(", ")}];' if style else f'"st_{n}" -> "{p}";')
        for k in s.get("sinks", []):
            L.append(f'"st_{n}" -> "sink_{k}" [color="#888888", arrowsize=0.6];')
        for e in s.get("exits", []):
            L.append(f'"st_{n}" -> "{e}" [label="set aside", color="#888888"];')
    legend = ("ellipse: a kind of atom (Petri place); bold border: kernel kind; red border: a demand on the kernel\\l"
              "white: a judgment station · yellow: a decision point · grey: mechanical (deterministic)\\l"
              "red station border: no drainer (the drain condition fails) · red dashed edge: feedback upstream\\l"
              "load: items processed per source of influx, from x = (I − Aᵀ)⁻¹ f, no wall time\\l")
    L.append(f'legend [shape=note, fontsize=9, label="{legend}"];')
    L.append("}")
    return "\n".join(L)


def report():
    R = ["# ASF system model: checks and flows", "",
         "*Generated by `render.py` from `model.yaml`. Every gain carries its tag in the model; the arithmetic is illustrative.*", "",
         "## Structural checks", ""] + checks() + ["", "## Flows, per regime", ""]
    for regime in ("sprint", "founding"):
        items, A, rho, x, used = solve(regime)
        R.append(f"### {regime}: spectral radius ρ = {rho:.2f}" + ("  (work grows without bound)" if rho >= 1 else ""))
        R.append("")
        if x is not None:
            R.append("| item | per source of influx |")
            R.append("|---|---|")
            for k, v in zip(items, x):
                R.append(f"| {k} | {v:.2f} |")
        R.append("")
        R.append("| from → to | gain | tag | note |")
        R.append("|---|---|---|---|")
        for (a, b), g in used.items():
            R.append(f"| {a} → {b} | {g['value']} | {g['tag']} | {g.get('note', '')} |")
        R.append("")
    return "\n".join(R)


if __name__ == "__main__":
    OUT.mkdir(exist_ok=True)
    d = dot("founding")
    (OUT / "asf-system.dot").write_text(d)
    subprocess.run(["dot", "-Tsvg", str(OUT / "asf-system.dot"), "-o", str(OUT / "asf-system.svg")], check=True)
    (OUT / "report.md").write_text(report())
    print((OUT / "report.md").read_text())
