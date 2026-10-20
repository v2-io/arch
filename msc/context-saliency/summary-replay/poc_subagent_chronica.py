#!/usr/bin/env python3
"""
poc_subagent_chronica.py — Deterministic Heuristic Pre-Modeler for Isolated Sub-Agent JSONL Traces.

Maps an append-only Claude Code sub-agent `.jsonl` (under an isolated warrant: worktree,
separate clone, or non-colliding scope, so exogenous rho_wall = 0) into ASF mathematical
structures with ZERO external LLM calls (O(T) single pass):

1. Initial Brief Parser (tau = 1):
   Extracts Isolation Warrant, O_0 (objectives & constraints), and M_0^seed (seeded paths).
2. The "Orient-Decide Dipole" Extractor for every Interventional Loop Datum d_k = (a_k, o_k):
   - Originating Intent sigma_intent(d_k): from tool `description` + Preceding Thinking Suffix (Decide).
   - Causal Footprint Scope(d_k): deterministic reads, mutations, tests, commits.
   - Resulting Model Update Delta_M_k & Mismatch delta_k: from Tool Result signals + Next Thinking Prefix (Orient).
3. Two-Scale AND/OR Strategy DAG (Sigma_T) & Footprint Invalidation Tracker:
   - Level-0 loop attempts typed into OR-gates (retries/failures on same scope) & AND-chains.
   - Level-1 Episode Envelopes K_m bounded by verification checks, commits, or phase transitions.
   - Automatic footprint invalidation when a mutation touches a previously verified file before re-test.
4. Exact ASF Propagation & CMC Middle-Folded CONSPECTUS (W_T):
   - Computes root confidence P_hat_Sigma(T), Jacobian sensitivity J_T, and compression ratio DL(W_T) / DL(C_T).
"""

import json
import math
import os
import re
import sys
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Set, Tuple


@dataclass
class BriefModel:
    session_id: str
    path: str
    total_jsonl_lines: int
    coordinator_turns: int
    isolation_evidence: List[str]
    objectives: List[str]
    constraints: List[str]
    seeded_paths: List[str]
    born_ts: str


@dataclass
class LoopDatum:
    idx: int                  # 1-based loop index d_1, d_2, ...
    line_use: int             # JSONL line index of tool_use
    line_res: int             # JSONL line index of tool_result
    ts_use: str
    ts_res: str
    tool_name: str
    explicit_desc: str        # tool_use.input.description or synthesized
    decide_thought: str       # suffix of preceding thinking block (Originating Intent)
    command_or_target: str
    reads: List[str] = field(default_factory=list)
    mutates: List[str] = field(default_factory=list)
    is_test: bool = False
    is_commit: bool = False
    is_error: bool = False
    result_bytes: int = 0
    result_preview: str = ""
    orient_thought: str = ""  # prefix of following thinking block (Uptake / Delta M_t)
    uptake_class: str = "ROUTINE_STEP"  # VERIFIED_PASS | APORIA_DISCOVERY | EPISTEMIC_HOIST | ERROR_RETRY | ROUTINE_STEP
    point_status_s0: float = 0.8        # s_v(T_0)
    sigma_verified: float = 0.5         # observability sigma_v
    gamma_role: str = "AND"             # AND | OR_BRANCH_FAIL | OR_BRANCH_SURVIVE
    invalidated_by: Optional[int] = None


@dataclass
class EpisodeEnvelope:
    ep_id: str
    title: str
    phase_type: str           # ORIENT_SURVEY | IMPLEMENT_VERIFY | SYNTHESIS_DELIVERY
    start_idx: int
    end_idx: int
    start_line: int
    end_line: int
    ts_start: str
    ts_end: str
    loop_indices: List[int] = field(default_factory=list)
    impetus: str = ""
    hoisted_M: List[Dict[str, str]] = field(default_factory=list)
    or_branches_pruned: List[Dict[str, str]] = field(default_factory=list)
    peak_surprise: Optional[Dict[str, str]] = field(default_factory=list)
    stale_warnings: List[str] = field(default_factory=list)
    end_state: str = ""
    verified_by_env: bool = False
    status_s: float = 0.8
    jacobian_J: float = 0.0
    screened_off: bool = False
    raw_bytes: int = 0
    folded_bytes: int = 0


def split_sentences(text: str) -> List[str]:
    text = re.sub(r"\s+", " ", text.strip())
    if not text:
        return []
    # Split on sentence boundaries while keeping code/file references intact
    parts = re.split(r"(?<=[.!?])\s+(?=[A-Z0-9`\"'])", text)
    return [p.strip() for p in parts if p.strip()]


def extract_paths_from_text(text: str) -> List[str]:
    paths = re.findall(r"(?:~/|/Users/[^\s`\"')]+|[a-zA-Z0-9_.-]+/[a-zA-Z0-9_./-]+(?:\.[a-zA-Z0-9]+)?)", text)
    cleaned = []
    for p in paths:
        p = p.rstrip(".,;:!?)`\"'")
        if len(p) > 3 and not p.startswith("http") and "/" in p:
            if p not in cleaned:
                cleaned.append(p)
    return cleaned[:15]


def parse_brief(lines: List[dict], path: str) -> Tuple[BriefModel, List[Tuple[int, str, str]]]:
    """Extract initial coordinator brief and any subsequent coordinator nudges."""
    coord_turns: List[Tuple[int, str, str]] = []
    session_id = os.path.basename(path).replace(".jsonl", "")
    for i, obj in enumerate(lines):
        if obj.get("type") == "user":
            msg = obj.get("message", {})
            c = msg.get("content", "")
            ts = obj.get("timestamp", "")
            if isinstance(c, str) and c.strip():
                coord_turns.append((i, ts, c))
            elif isinstance(c, list):
                texts = [x.get("text", "") for x in c if isinstance(x, dict) and x.get("type") == "text"]
                joined = "\n".join(t for t in texts if t.strip())
                if joined:
                    coord_turns.append((i, ts, joined))

    first_text = coord_turns[0][2] if coord_turns else ""
    born_ts = coord_turns[0][1] if coord_turns else ""

    # Isolation evidence
    iso_patterns = [
        r"[^.\n]*(?:worktree|clone it in|~/src-ext|do not edit|do not touch|only touch|read-only|isolated|your own|decline|scope)[^.\n]*[.\n]?"
    ]
    iso_ev = []
    for pat in iso_patterns:
        for m in re.finditer(pat, first_text, flags=re.IGNORECASE):
            s = re.sub(r"\s+", " ", m.group(0)).strip()
            if 10 < len(s) < 220 and s not in iso_ev:
                iso_ev.append(s)

    # Objectives & Constraints from markdown bullets or bold headers
    objectives = []
    constraints = []
    for line in first_text.splitlines():
        s = line.strip()
        if not s:
            continue
        low = s.lower()
        if any(k in low for k in ["do not ", "don't ", "never ", "avoid ", "not to treat", "only touch"]):
            if len(s) < 240:
                constraints.append(re.sub(r"^[-*•]\s*", "", s))
        elif s.startswith(("- ", "* ", "1.", "2.", "3.")) or "**Your question" in s or "**What " in s or "**The ask" in s:
            if len(s) < 240:
                objectives.append(re.sub(r"^[-*•]\s*", "", s))

    if not objectives:
        sents = split_sentences(first_text)
        objectives = sents[:3]

    seeded_paths = extract_paths_from_text(first_text)

    return BriefModel(
        session_id=session_id,
        path=path,
        total_jsonl_lines=len(lines),
        coordinator_turns=len(coord_turns),
        isolation_evidence=iso_ev[:4],
        objectives=objectives[:6],
        constraints=constraints[:5],
        seeded_paths=seeded_paths[:10],
        born_ts=born_ts,
    ), coord_turns


def analyze_scope_and_role(tool_name: str, inp: dict, res_text: str, is_err: bool) -> Tuple[List[str], List[str], bool, bool, bool]:
    """Deterministically infer (reads, mutates, is_test, is_commit, effective_error)."""
    reads: List[str] = []
    mutates: List[str] = []
    is_test = False
    is_commit = False

    if tool_name in ("Read", "Grep", "Glob"):
        fp = inp.get("file_path") or inp.get("path") or inp.get("pattern") or ""
        if fp:
            reads.append(os.path.basename(fp) if "/" in fp else fp)
    elif tool_name in ("Edit", "Write", "MultiEdit"):
        fp = inp.get("file_path") or inp.get("path") or ""
        if fp:
            mutates.append(os.path.basename(fp) if "/" in fp else fp)
    elif tool_name == "Bash":
        cmd = inp.get("command", "")
        # Check for test execution
        if re.search(r"\b(cargo\s+test|pytest|npm\s+test|pnpm\s+test|go\s+test|ctest|script/cibuild)\b", cmd):
            is_test = True
        # Check for git commit
        if re.search(r"\bgit\s+commit\b", cmd):
            is_commit = True
        # Check for inline python/perl/sed/tee/cat-redirect file mutations
        for m in re.finditer(r"(?:open\(['\"]([^'\"]+)['\"],\s*['\"]w|p=['\"]([^'\"]+)['\"].*\.write_text|sed\s+-i[^>]*\s+([^\s;]+)|cat\s+>\s*([^\s;<]+)|tee\s+([^\s;|]+))", cmd):
            target = next((g for g in m.groups() if g), None)
            if target:
                mutates.append(os.path.basename(target))
        # Check for python inline read+write pattern: p='src/nodes.rs' ... open(p,'w')
        if "open(" in cmd and "'w'" in cmd or '"w"' in cmd:
            for m in re.finditer(r"['\"]([a-zA-Z0-9_./-]+\.(?:rs|py|ts|js|md|toml|json|html|css))['\"]", cmd):
                mutates.append(os.path.basename(m.group(1)))
        # Check for file reads in bash
        for m in re.finditer(r"\b(?:cat|sed|head|tail|grep|rg)\b[^;|&\n]*?\s+([a-zA-Z0-9_./-]+\.(?:rs|py|ts|js|md|toml|json|svg|sh|c|h))", cmd):
            fname = os.path.basename(m.group(1))
            if fname not in mutates and fname not in reads:
                reads.append(fname)

    # Check if tool output contains test failures or compiler errors even if is_error=False
    effective_err = is_err
    if re.search(r"(?:^error\[E\d+\]|^FAILED\b|test result: FAILED|Traceback \(most recent call last\)|ModuleNotFoundError:|SyntaxError:|panicked at)", res_text, flags=re.MULTILINE):
        effective_err = True

    return list(dict.fromkeys(reads)), list(dict.fromkeys(mutates)), is_test, is_commit, effective_err


def classify_uptake(
    explicit_desc: str,
    decide_thought: str,
    orient_thought: str,
    is_err: bool,
    is_test: bool,
    is_commit: bool,
    mutates: List[str],
) -> Tuple[str, float, float]:
    """
    Deterministically classify the Orient-prefix of the thinking block following a tool_result
    into (uptake_class, point_status_s0, sigma_verified).
    Crucially accounts for Anticipation Polarity (o_hat): if the Originating Intent explicitly
    aimed to confirm a failure or reproduce a bug ("confirm ... fail", "reproduce bug"),
    then observing a test failure matches anticipation (delta = o - o_hat = 0 -> VERIFIED_NEG_CONTROL)!
    """
    intent_low = (explicit_desc + " " + decide_thought).lower()
    anticipates_failure = any(
        p in intent_low
        for p in [
            "confirm new tests fail",
            "confirm reference_definitions tests fail",
            "confirm bug-2 tests fail",
            "fail without the fix",
            "fail without fix",
            "reproduce bug",
            "verify pass with fix, fail without",
        ]
    )
    low = orient_thought.lower()
    if "api error:" in low or "safeguards flagged" in low:
        return "ERROR_RETRY", 0.0, 1.0
    if is_err:
        if anticipates_failure:
            return "VERIFIED_NEG_CONTROL", 1.0, 1.0
        return "ERROR_RETRY", 0.0, 1.0
    if is_commit:
        return "VERIFIED_PASS", 1.0, 1.0
    if is_test:
        if any(w in low for w in ["fail", "broke", "wrong", "error", "mismatch"]) and not anticipates_failure:
            return "ERROR_RETRY", 0.0, 1.0
        return "VERIFIED_PASS", 1.0, 1.0

    # Check verbal verification markers in the opening sentence of next thinking block
    first_sent = split_sentences(orient_thought)[0].lower() if orient_thought else ""
    if any(k in first_sent for k in [
        "all ", "everything checks out", "works", "fixed", "passes", "passed",
        "matches expectations", "clean", "succeeded", "confirmed", "good,", "great,",
        "looks good", "built successfully", "is all green"
    ]) and not any(bad in first_sent for bad in ["not ", "but ", "fail", "wrong"]):
        return "VERIFIED_PASS", 1.0, 1.0

    # Check aporia / surprise / policy or bug discovery markers
    if any(k in low for k in [
        "conflict", "wait", "however", "related bug", "issue is", "problem is",
        "doesn't ", "does not ", "discrepancy", "careful", "tricky", "unexpected",
        "bypass", "wrong start line", "gap in"
    ]):
        return "APORIA_DISCOVERY", 0.9, 1.0

    # Check epistemic hoist markers (structural realization)
    if any(k in low for k in [
        "need to account", "cleanest fix", "root cause", "because ", "this means",
        "is stored", "functions by", "already in place", "shows that", "reveals"
    ]):
        return "EPISTEMIC_HOIST", 0.9, 0.9

    if mutates:
        return "PRAGMATIC_MUTATION", 0.85, 0.7  # unverified until tested or committed

    return "ROUTINE_STEP", 0.9, 0.8


def extract_loop_data(lines: List[dict]) -> Tuple[List[LoopDatum], str]:
    """
    Pair every tool_use with its tool_result and bind:
      - Preceding thinking block suffix -> decide_thought (Originating Intent)
      - Following thinking block prefix -> orient_thought (Model Update Delta M_t)
    Also returns the final assistant reply text.
    """
    # Map tool_use_id -> result info
    results_by_id: Dict[str, Tuple[int, str, bool, str]] = {}
    for i, obj in enumerate(lines):
        if obj.get("type") == "user":
            c = obj.get("message", {}).get("content", [])
            ts = obj.get("timestamp", "")
            if isinstance(c, list):
                for item in c:
                    if isinstance(item, dict) and item.get("type") == "tool_result":
                        tid = item.get("tool_use_id", "")
                        is_err = bool(item.get("is_error", False))
                        raw_content = item.get("content", "")
                        if isinstance(raw_content, list):
                            raw_str = "\n".join(x.get("text", "") for x in raw_content if isinstance(x, dict))
                        else:
                            raw_str = str(raw_content)
                        results_by_id[tid] = (i, ts, is_err, raw_str)

    # Walk assistant events in order, tracking pending thinking text
    loops: List[LoopDatum] = []
    pending_thinking: List[str] = []
    last_loop_awaiting_orient: Optional[LoopDatum] = None
    final_reply_text = ""

    for i, obj in enumerate(lines):
        if obj.get("type") != "assistant":
            continue
        ts = obj.get("timestamp", "")
        content = obj.get("message", {}).get("content", [])
        if not isinstance(content, list):
            continue

        for block in content:
            if not isinstance(block, dict):
                continue
            btype = block.get("type")
            if btype == "thinking":
                th = block.get("thinking", "").strip()
                if th:
                    # If the previous tool call hasn't received its post-tool Orient thought yet,
                    # bind the opening sentences of this thinking block to it!
                    if last_loop_awaiting_orient is not None:
                        sents = split_sentences(th)
                        orient_part = " ".join(sents[:2]) if sents else th[:240]
                        last_loop_awaiting_orient.orient_thought = orient_part
                        u_cls, s0, sig = classify_uptake(
                            last_loop_awaiting_orient.explicit_desc,
                            last_loop_awaiting_orient.decide_thought,
                            orient_part,
                            last_loop_awaiting_orient.is_error,
                            last_loop_awaiting_orient.is_test,
                            last_loop_awaiting_orient.is_commit,
                            last_loop_awaiting_orient.mutates,
                        )
                        last_loop_awaiting_orient.uptake_class = u_cls
                        last_loop_awaiting_orient.point_status_s0 = s0
                        last_loop_awaiting_orient.sigma_verified = sig
                        last_loop_awaiting_orient = None
                    pending_thinking.append(th)

            elif btype == "text":
                tx = block.get("text", "").strip()
                if tx:
                    final_reply_text = tx
                    if last_loop_awaiting_orient is not None and not last_loop_awaiting_orient.orient_thought:
                        sents = split_sentences(tx)
                        orient_part = " ".join(sents[:2]) if sents else tx[:200]
                        last_loop_awaiting_orient.orient_thought = orient_part
                        u_cls, s0, sig = classify_uptake(
                            last_loop_awaiting_orient.explicit_desc,
                            last_loop_awaiting_orient.decide_thought,
                            orient_part,
                            last_loop_awaiting_orient.is_error,
                            last_loop_awaiting_orient.is_test,
                            last_loop_awaiting_orient.is_commit,
                            last_loop_awaiting_orient.mutates,
                        )
                        last_loop_awaiting_orient.uptake_class = u_cls
                        last_loop_awaiting_orient.point_status_s0 = s0
                        last_loop_awaiting_orient.sigma_verified = sig
                        last_loop_awaiting_orient = None

            elif btype == "tool_use":
                tid = block.get("id", "")
                tname = block.get("name", "")
                inp = block.get("input", {}) if isinstance(block.get("input"), dict) else {}

                # Extract explicit low-level intent description
                desc = inp.get("description") or inp.get("toolSummary") or ""
                cmd_target = inp.get("command") or inp.get("file_path") or inp.get("path") or inp.get("pattern") or inp.get("url") or str(inp)[:120]
                if not desc:
                    if tname in ("Read", "Write", "Edit"):
                        desc = f"{tname} {os.path.basename(str(cmd_target))}"
                    elif tname in ("Grep", "Glob"):
                        desc = f"{tname} '{inp.get('pattern', '')}'"
                    else:
                        desc = f"{tname} call"

                # Extract Decide suffix from pending_thinking
                decide_str = ""
                if pending_thinking:
                    last_th = pending_thinking[-1]
                    sents = split_sentences(last_th)
                    decide_str = " ".join(sents[-2:]) if len(sents) >= 2 else last_th[-220:]
                pending_thinking.clear()

                res_line, res_ts, is_err, res_str = results_by_id.get(tid, (i + 1, ts, False, ""))
                reads, mutates, is_test, is_commit, eff_err = analyze_scope_and_role(tname, inp, res_str, is_err)

                ld = LoopDatum(
                    idx=len(loops) + 1,
                    line_use=i,
                    line_res=res_line,
                    ts_use=ts,
                    ts_res=res_ts,
                    tool_name=tname,
                    explicit_desc=desc,
                    decide_thought=decide_str,
                    command_or_target=re.sub(r"\s+", " ", str(cmd_target)).strip()[:140],
                    reads=reads,
                    mutates=mutates,
                    is_test=is_test,
                    is_commit=is_commit,
                    is_error=eff_err,
                    result_bytes=len(res_str.encode("utf-8")),
                    result_preview=re.sub(r"\s+", " ", res_str).strip()[:120],
                )
                # Default classification if no subsequent thinking block overrides it
                u_cls, s0, sig = classify_uptake(desc, decide_str, "", eff_err, is_test, is_commit, mutates)
                ld.uptake_class = u_cls
                ld.point_status_s0 = s0
                ld.sigma_verified = sig

                loops.append(ld)
                last_loop_awaiting_orient = ld

    # Second pass on loops:
    # 1. Identify OR-branches (retries / failed attempts followed by another attempt on overlapping scope or command)
    # 2. Identify footprint invalidations (test/verification at d_j followed by mutation at d_k on same file without later test)
    last_verified_test_by_file: Dict[str, int] = {}
    mutated_files_in_session: Set[str] = set()

    for idx_pos, ld in enumerate(loops):
        if ld.uptake_class == "ERROR_RETRY":
            ld.gamma_role = "OR_BRANCH_FAIL"
            ld.point_status_s0 = 0.0
            if idx_pos + 1 < len(loops):
                loops[idx_pos + 1].gamma_role = "OR_BRANCH_SURVIVE"

        for mf in ld.mutates:
            mutated_files_in_session.add(mf)
            # Invalidate any prior test verification covering this file
            if mf in last_verified_test_by_file:
                prev_test_idx = last_verified_test_by_file[mf]
                loops[prev_test_idx - 1].invalidated_by = ld.idx

        if ld.is_test and ld.uptake_class != "ERROR_RETRY":
            for mf in list(mutated_files_in_session):
                last_verified_test_by_file[mf] = ld.idx
            # Re-validating clears earlier invalidations if they were re-tested
            for prev in loops[:idx_pos]:
                if prev.is_test and prev.invalidated_by is not None:
                    prev.invalidated_by = None

    return loops, final_reply_text


def segment_episodes(loops: List[LoopDatum], lines: List[dict]) -> List[EpisodeEnvelope]:
    """
    Deterministically segment Level-0 loops into Level-1 Episode Envelopes K_m:
      - Boundary triggers:
        1. Commit event (closes an implementation/fix episode)
        2. Verified test suite pass after one or more mutations (closes a build/verify episode)
        3. Transition from pure epistemic survey (first N read-only steps) into first file mutation / branch creation
        4. Bucket cap (~18 loops) during long read-only/write surveys (e.g. surfacing files 50..69)
    """
    if not loops:
        return []

    episodes: List[EpisodeEnvelope] = []
    cur_batch: List[LoopDatum] = []
    seen_mutation_in_batch = False

    def flush_batch(batch: List[LoopDatum], reason: str):
        if not batch:
            return
        ep_num = len(episodes) + 1
        ep_id = f"K{ep_num}"
        first_l, last_l = batch[0], batch[-1]

        has_mut = any(bool(x.mutates) for x in batch)
        has_test = any(x.is_test and x.uptake_class != "ERROR_RETRY" for x in batch)
        has_commit = any(x.is_commit and x.uptake_class != "ERROR_RETRY" for x in batch)
        has_verified_step = any(x.uptake_class in ("VERIFIED_PASS", "VERIFIED_NEG_CONTROL") for x in batch)
        tail_failed_unrecovered = (last_l.uptake_class == "ERROR_RETRY")

        if has_commit or (has_mut and has_test):
            phase = "IMPLEMENT_VERIFY"
        elif has_mut:
            phase = "ARTIFACT_GENERATION"
        else:
            phase = "EPISTEMIC_SURVEY"

        # Impetus: first loop's decide_thought or explicit_desc
        impetus = first_l.decide_thought if first_l.decide_thought else first_l.explicit_desc
        if len(impetus) > 200:
            impetus = impetus[:197] + "..."

        # Hoisted M_t facts (from EPISTEMIC_HOIST, APORIA_DISCOVERY, and VERIFIED_NEG_CONTROL steps)
        hoisted = []
        for x in batch:
            if x.uptake_class in ("EPISTEMIC_HOIST", "APORIA_DISCOVERY", "VERIFIED_NEG_CONTROL") and x.orient_thought:
                scope_list = x.mutates or x.reads or [x.tool_name]
                hoisted.append({
                    "at_loop": f"d{x.idx} (L{x.line_res})",
                    "ts": x.ts_res,
                    "scope": ",".join(scope_list[:3]),
                    "claim": x.orient_thought[:220],
                })

        # Pruned OR-branches
        or_pruned = []
        for x in batch:
            if x.gamma_role == "OR_BRANCH_FAIL":
                or_pruned.append({
                    "at_loop": f"d{x.idx} (L{x.line_use})",
                    "desc": x.explicit_desc,
                    "error_preview": x.result_preview[:140],
                })

        # Peak surprise (largest result or aporia discovery)
        aporia_steps = [x for x in batch if x.uptake_class == "APORIA_DISCOVERY"]
        peak = None
        if aporia_steps:
            p_step = aporia_steps[0]
            peak = {
                "at_loop": f"d{p_step.idx} (L{p_step.line_res})",
                "desc": p_step.explicit_desc,
                "insight": p_step.orient_thought[:220],
            }
        elif or_pruned:
            peak = {
                "at_loop": or_pruned[0]["at_loop"],
                "desc": or_pruned[0]["desc"],
                "insight": f"Failed branch ({or_pruned[0]['error_preview'][:100]}) -> triggered OR-fallback",
            }

        # Stale warnings
        stale = []
        for x in batch:
            if x.invalidated_by is not None:
                stale.append(f"d{x.idx} ({x.explicit_desc}) invalidated by later mutation at d{x.invalidated_by}")

        # End state & verification (any internal OR_BRANCH_FAIL that was followed by a surviving step in the batch is recovered!)
        verified = (
            has_commit
            or has_test
            or has_verified_step
            or (not tail_failed_unrecovered and phase in ("EPISTEMIC_SURVEY", "ARTIFACT_GENERATION"))
        )
        end_desc = f"{last_l.explicit_desc}"
        if last_l.orient_thought:
            end_desc += f" -> {split_sentences(last_l.orient_thought)[0]}"

        # Status propagation inside K_m
        if stale:
            status_s = 0.65
            verified = False
        elif tail_failed_unrecovered:
            status_s = 0.50
            verified = False
        elif has_commit or has_test:
            status_s = 1.0
        elif verified:
            status_s = 0.96
        else:
            status_s = 0.80

        # Raw byte size of the JSONL span [start_line .. end_line]
        raw_b = sum(len(json.dumps(lines[line_idx]).encode("utf-8")) for line_idx in range(first_l.line_use, min(len(lines), last_l.line_res + 1)))

        title = f"{phase}: {first_l.explicit_desc} .. {last_l.explicit_desc}"
        ep = EpisodeEnvelope(
            ep_id=ep_id,
            title=title,
            phase_type=phase,
            start_idx=first_l.idx,
            end_idx=last_l.idx,
            start_line=first_l.line_use,
            end_line=last_l.line_res,
            ts_start=first_l.ts_use,
            ts_end=last_l.ts_res,
            loop_indices=[x.idx for x in batch],
            impetus=impetus,
            hoisted_M=hoisted[:4],
            or_branches_pruned=or_pruned,
            peak_surprise=peak,
            stale_warnings=stale,
            end_state=end_desc[:220],
            verified_by_env=verified,
            status_s=status_s,
            raw_bytes=raw_b,
        )
        # Compute folded stub byte size
        stub_str = render_folded_episode_stub(ep)
        ep.folded_bytes = len(stub_str.encode("utf-8"))
        episodes.append(ep)

    for i, ld in enumerate(loops):
        # Transition from initial read-only orientation to first code branch / edit
        if (
            cur_batch
            and not seen_mutation_in_batch
            and (ld.mutates or "checkout -b" in ld.command_or_target or "worktree" in ld.explicit_desc.lower())
            and len(cur_batch) >= 4
        ):
            flush_batch(cur_batch, "survey_to_mutate")
            cur_batch = []
            seen_mutation_in_batch = False

        cur_batch.append(ld)
        if ld.mutates or "checkout -b" in ld.command_or_target:
            seen_mutation_in_batch = True

        # Never split an episode while an OR-retry branch is still open!
        if ld.uptake_class == "ERROR_RETRY":
            continue

        # Close episode on git commit
        if ld.is_commit and not ld.is_error and len(cur_batch) >= 2:
            flush_batch(cur_batch, "commit_seal")
            cur_batch = []
            seen_mutation_in_batch = False
        # Or close episode on full test suite pass or explicit VERIFIED_PASS after mutations
        elif (
            (ld.is_test or ld.uptake_class == "VERIFIED_PASS")
            and not ld.is_error
            and seen_mutation_in_batch
            and len(cur_batch) >= 6
            and (i + 1 < len(loops) and not loops[i + 1].is_commit)
        ):
            flush_batch(cur_batch, "verify_seal")
            cur_batch = []
            seen_mutation_in_batch = False
        # Or close batch if it grows beyond 14-18 loops in survey/artifact generation
        elif not seen_mutation_in_batch and len(cur_batch) >= 14:
            flush_batch(cur_batch, "survey_chunk")
            cur_batch = []
        elif seen_mutation_in_batch and not any(x.is_test for x in cur_batch) and len(cur_batch) >= 18:
            flush_batch(cur_batch, "artifact_chunk")
            cur_batch = []

    if cur_batch:
        flush_batch(cur_batch, "tail")

    # Compute Root Plan Confidence P_hat_Sigma and Analytical Jacobian J_T across Episode Envelopes (AND-composition)
    prod_all = 1.0
    for ep in episodes:
        prod_all *= ep.status_s
    for idx_e, ep in enumerate(episodes):
        other_prod = 1.0
        for idx_o, other in enumerate(episodes):
            if idx_o != idx_e:
                other_prod *= other.status_s
        ep.jacobian_J = round(other_prod, 4)
        # CMC Screen-Off Rule: Episode K_m is safe to middle-fold if it is verified, has no stale warnings,
        # and is not the active tail episode (or if the session itself has completed).
        ep.screened_off = bool(ep.verified_by_env and not ep.stale_warnings)

    return episodes


def render_folded_episode_stub(ep: EpisodeEnvelope) -> str:
    lines = [
        f'<folded-episode id="{ep.ep_id}" loops="d{ep.start_idx}..d{ep.end_idx}" jsonl_lines="L{ep.start_line}..L{ep.end_line}" '
        f'ts="{ep.ts_start}..{ep.ts_end}" phase="{ep.phase_type}" s="{ep.status_s:.2f}" screened_off="{ep.screened_off}">',
        f"  <impetus>{ep.impetus}</impetus>",
    ]
    for hm in ep.hoisted_M:
        lines.append(f'  <hoisted-M at="{hm["at_loop"]}" ts="{hm["ts"]}" scope="{hm["scope"]}">{hm["claim"]}</hoisted-M>')
    if ep.peak_surprise:
        lines.append(
            f'  <peak-lesson at="{ep.peak_surprise["at_loop"]}" desc="{ep.peak_surprise["desc"]}">'
            f'{ep.peak_surprise["insight"]}</peak-lesson>'
        )
    for ob in ep.or_branches_pruned:
        lines.append(f'  <pruned-OR-branch at="{ob["at_loop"]}" desc="{ob["desc"]}">{ob["error_preview"]}</pruned-OR-branch>')
    for sw in ep.stale_warnings:
        lines.append(f"  <stale-warning>{sw}</stale-warning>")
    lines.append(f"  <end-state>{ep.end_state}</end-state>")
    lines.append("</folded-episode>")
    return "\n".join(lines)


def run_poc_on_file(path: str) -> dict:
    with open(path, "r", errors="ignore") as f:
        lines = [json.loads(line) for line in f]

    raw_file_bytes = os.path.getsize(path)
    brief, coord_turns = parse_brief(lines, path)
    loops, final_reply = extract_loop_data(lines)
    episodes = segment_episodes(loops, lines)

    p_hat = 1.0
    for ep in episodes:
        p_hat *= ep.status_s

    total_ep_raw = sum(ep.raw_bytes for ep in episodes)
    total_ep_folded = sum(ep.folded_bytes if ep.screened_off else ep.raw_bytes for ep in episodes)
    all_folded_bytes = sum(ep.folded_bytes for ep in episodes)

    # Count how many loops had explicit description vs decide_thought vs orient_thought
    n_explicit_desc = sum(1 for x in loops if x.explicit_desc and not x.explicit_desc.endswith(" call"))
    n_decide = sum(1 for x in loops if x.decide_thought)
    n_orient = sum(1 for x in loops if x.orient_thought)
    n_hoist = sum(1 for x in loops if x.uptake_class in ("EPISTEMIC_HOIST", "APORIA_DISCOVERY"))
    n_or_fail = sum(1 for x in loops if x.gamma_role == "OR_BRANCH_FAIL")
    n_verified = sum(1 for x in loops if x.uptake_class == "VERIFIED_PASS")

    return {
        "brief": brief,
        "coord_turns": coord_turns,
        "loops": loops,
        "episodes": episodes,
        "final_reply_bytes": len(final_reply.encode("utf-8")),
        "raw_file_bytes": raw_file_bytes,
        "total_ep_raw_bytes": total_ep_raw,
        "total_ep_folded_bytes": total_ep_folded,
        "all_folded_stubs_bytes": all_folded_bytes,
        "p_hat_sigma": round(p_hat, 4),
        "stats": {
            "total_loops": len(loops),
            "with_explicit_desc": n_explicit_desc,
            "with_decide_thought": n_decide,
            "with_orient_thought": n_orient,
            "epistemic_or_aporia_hoists": n_hoist,
            "or_branches_pruned": n_or_fail,
            "verified_pass_steps": n_verified,
            "total_episodes": len(episodes),
            "screened_off_episodes": sum(1 for e in episodes if e.screened_off),
        },
    }


def main():
    default_paths = [
        # PoC 1: Isolated clone in ~/src-ext/comrak — Rust parser bug diagnosis, 3 fixes, test suite & commits
        "/Users/josephwecker-v2/.claude/projects/-Users-josephwecker-v2-src-arch-firmatum-udon/d28cb106-6ce2-4002-bf8d-098f6a13bb5f/subagents/agent-a77924084bb29df3c.jsonl",
        # PoC 2: Dedicated git worktree in ~/src/limen — macOS .app bundle + SVG->icns packaging & tests
        "/Users/josephwecker-v2/.claude/projects/-Users-josephwecker-v2-src-limen/1d991a6c-9626-42c7-9210-a9453b6d0698/subagents/agent-a05f5016e620c7a5c.jsonl",
        # PoC 3: Scoped UDON history & memorata-search survey (u=2 coordinator turns, incremental file surfacing)
        "/Users/josephwecker-v2/.claude/projects/-Users-josephwecker-v2-src-arch-firmatum-udon/5930da5d-2aed-49ea-ac8b-ed619c1d6a0d/subagents/agent-ae42349504f61ed0c.jsonl",
    ]
    paths = sys.argv[1:] if len(sys.argv) > 1 else default_paths

    for idx, p in enumerate(paths, 1):
        res = run_poc_on_file(p)
        b: BriefModel = res["brief"]
        st = res["stats"]
        print("=" * 92)
        print(f"PoC RUN #{idx}: {b.session_id}  (lines={b.total_jsonl_lines}, u={b.coordinator_turns}, raw={res['raw_file_bytes']/1024:.1f} KB)")
        print("=" * 92)
        print(f"  Born TS:            {b.born_ts}")
        print(f"  Isolation Warrant:  {b.isolation_evidence[0] if b.isolation_evidence else 'Scoped subagent task'}")
        print(f"  Seeded Paths (M_0): {', '.join(b.seeded_paths[:5])}")
        print(f"  Extracted O_0:      {b.objectives[0][:140] if b.objectives else 'N/A'}")
        if b.constraints:
            print(f"  Extracted Bounds:   {b.constraints[0][:140]}")
        print("-" * 92)
        print(
            f"  Loop Dipole Stats:  {st['total_loops']} tool loops | "
            f"explicit_desc={st['with_explicit_desc']}/{st['total_loops']} | "
            f"decide_intent={st['with_decide_thought']}/{st['total_loops']} | "
            f"orient_uptake={st['with_orient_thought']}/{st['total_loops']}"
        )
        print(
            f"  Uptake Breakdown:   hoisted_M/aporia={st['epistemic_or_aporia_hoists']} | "
            f"pruned_OR_branches={st['or_branches_pruned']} | "
            f"verified_checks={st['verified_pass_steps']}"
        )
        print(
            f"  Episode DAG Sigma:  {st['total_episodes']} episodes ({st['screened_off_episodes']} CMC screened-off) | "
            f"P_hat_Sigma={res['p_hat_sigma']} | "
            f"Span Compression: {res['total_ep_raw_bytes']/1024:.1f} KB -> {res['all_folded_stubs_bytes']/1024:.1f} KB "
            f"({100.0 * (1.0 - res['all_folded_stubs_bytes'] / max(1, res['total_ep_raw_bytes'])):.1f}% reduction)"
        )
        print("-" * 92)
        print("  SAMPLE LEVEL-0 ORIENT-DECIDE DIPOLES (Intent -> Tool -> Result -> Delta_M / Uptake):")
        shown = 0
        for ld in res["loops"]:
            if ld.uptake_class in ("APORIA_DISCOVERY", "EPISTEMIC_HOIST", "VERIFIED_PASS", "ERROR_RETRY") and ld.orient_thought:
                print(f"    [d{ld.idx:02d} L{ld.line_use}..L{ld.line_res}] {ld.tool_name} | desc: {ld.explicit_desc!r}")
                if ld.decide_thought:
                    print(f"       ├─ Originating Intent (Decide): {ld.decide_thought[:150]}")
                print(f"       ├─ Tool Outcome ({ld.result_bytes}B, err={ld.is_error}): scope_r={ld.reads[:3]} scope_m={ld.mutates[:3]}")
                print(f"       └─ Post-Tool Uptake [{ld.uptake_class}, s0={ld.point_status_s0}]: {ld.orient_thought[:165]}")
                shown += 1
                if shown >= 5:
                    break
        print("-" * 92)
        print("  CMC MIDDLE-FOLDED EPISODE STUBS (CONSPECTUS W_T):")
        for ep in res["episodes"]:
            print(render_folded_episode_stub(ep))
        print()


if __name__ == "__main__":
    main()
