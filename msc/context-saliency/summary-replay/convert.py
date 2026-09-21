"""ConversationItem (compaction_requests JSON) → xAI Responses API `input`.

Mirrors grok-build's `build_responses_input` / `conversation_item_to_input_items`
in `xai-grok-sampling-types` so a prompt swap is the only intended variable.
"""

from __future__ import annotations

import json
from typing import Any


def last_user_prompt_text(chat_history: list[dict]) -> str:
    if not chat_history:
        raise ValueError("empty chat_history")
    last = chat_history[-1]
    if last.get("type") != "user":
        raise ValueError(f"last item is {last.get('type')!r}, expected user (summarization prompt)")
    return _user_text(last)


def replace_last_user_text(chat_history: list[dict], prompt: str) -> list[dict]:
    """Return a shallow copy of history with the last user message's text replaced."""
    if not chat_history:
        raise ValueError("empty chat_history")
    last = chat_history[-1]
    if last.get("type") != "user":
        raise ValueError(f"last item is {last.get('type')!r}, expected user")
    new_last = dict(last)
    content = last.get("content")
    if isinstance(content, str):
        new_last["content"] = [{"type": "text", "text": prompt}]
    elif isinstance(content, list):
        parts = []
        replaced = False
        for part in content:
            if isinstance(part, dict) and part.get("type") == "text" and not replaced:
                parts.append({**part, "text": prompt})
                replaced = True
            else:
                parts.append(part)
        if not replaced:
            parts.append({"type": "text", "text": prompt})
        new_last["content"] = parts
    else:
        new_last["content"] = [{"type": "text", "text": prompt}]
    return list(chat_history[:-1]) + [new_last]


def conversation_to_input_items(chat_history: list[dict]) -> list[dict]:
    items: list[dict] = []
    for raw in chat_history:
        items.extend(_item_to_input(raw))
    return items


def tools_to_responses_tools(tools: list[dict]) -> list[dict]:
    out = []
    for t in tools:
        entry: dict[str, Any] = {
            "type": "function",
            "name": t["name"],
            "description": t.get("description") or "",
        }
        if t.get("parameters") is not None:
            entry["parameters"] = t["parameters"]
        out.append(entry)
    return out


def build_create_body(
    request: dict,
    *,
    prompt: str | None = None,
    model: str | None = None,
    include_tools: bool = True,
    drop_images: bool = False,
    reasoning_effort: str = "high",
    temperature: float = 1.0,
    store: bool = False,
) -> dict:
    history = request["chat_history"]
    if prompt is not None:
        history = replace_last_user_text(history, prompt)
    if drop_images:
        history = [_drop_images(it) for it in history]

    body: dict[str, Any] = {
        "model": model or request.get("model") or "grok-4.6",
        "input": conversation_to_input_items(history),
        "temperature": temperature,
        "store": store,
        "reasoning": {"effort": reasoning_effort, "summary": "concise"},
    }
    tools = request.get("tools") or []
    if include_tools and tools:
        body["tools"] = tools_to_responses_tools(tools)
        body["tool_choice"] = "none"
    return body


def input_stats(input_items: list[dict]) -> dict[str, Any]:
    by_type: dict[str, int] = {}
    text_chars = 0
    images = 0
    for it in input_items:
        t = it.get("type") or it.get("role") or "unknown"
        if it.get("role") and it.get("type") in (None, "message"):
            t = f"message:{it.get('role')}"
        by_type[t] = by_type.get(t, 0) + 1
        text_chars += _count_text_chars(it)
        images += _count_images(it)
    return {
        "n_items": len(input_items),
        "by_type": by_type,
        "text_chars": text_chars,
        "images": images,
        "rough_tokens": text_chars // 4,
    }


def _user_text(item: dict) -> str:
    content = item.get("content")
    if isinstance(content, str):
        return content
    if isinstance(content, list):
        parts = []
        for p in content:
            if isinstance(p, dict) and p.get("type") == "text":
                parts.append(p.get("text") or "")
            elif isinstance(p, str):
                parts.append(p)
        return "\n".join(parts)
    return ""


def _item_to_input(item: dict) -> list[dict]:
    kind = item.get("type")
    if kind == "system":
        return [
            {
                "type": "message",
                "role": "system",
                "content": item.get("content") or "",
            }
        ]
    if kind == "user":
        return [
            {
                "type": "message",
                "role": "user",
                "content": _user_content(item.get("content")),
            }
        ]
    if kind == "reasoning":
        out: dict[str, Any] = {"type": "reasoning"}
        if item.get("id"):
            out["id"] = item["id"]
        if item.get("summary") is not None:
            out["summary"] = item["summary"]
        if item.get("encrypted_content"):
            out["encrypted_content"] = item["encrypted_content"]
        # status is output-only; the live sampler strips it before replay
        return [out]
    if kind == "assistant":
        items: list[dict] = []
        content = item.get("content") or ""
        if content:
            items.append(
                {
                    "type": "message",
                    "role": "assistant",
                    "content": content,
                }
            )
        for tc in item.get("tool_calls") or []:
            arguments = tc.get("arguments") or "{}"
            if not _is_json(arguments):
                arguments = "{}"
            items.append(
                {
                    "type": "function_call",
                    "call_id": tc.get("id") or "",
                    "name": tc.get("name") or "",
                    "arguments": arguments,
                }
            )
        return items
    if kind == "tool_result":
        return [
            {
                "type": "function_call_output",
                "call_id": item.get("tool_call_id") or "",
                "output": item.get("content") or "",
            }
        ]
    if kind == "backend_tool_call":
        # Pass through whatever the session stored; none in the first fixture.
        kind_obj = item.get("kind") or item
        return [kind_obj] if isinstance(kind_obj, dict) else []
    raise ValueError(f"unknown ConversationItem type: {kind!r}")


def _user_content(content: Any) -> Any:
    if isinstance(content, str):
        return content
    if not isinstance(content, list):
        return "" if content is None else str(content)
    if len(content) == 1 and isinstance(content[0], dict) and content[0].get("type") == "text":
        return content[0].get("text") or ""
    parts = []
    for p in content:
        if isinstance(p, str):
            parts.append({"type": "input_text", "text": p})
        elif isinstance(p, dict) and p.get("type") == "text":
            parts.append({"type": "input_text", "text": p.get("text") or ""})
        elif isinstance(p, dict) and p.get("type") == "image":
            parts.append(
                {
                    "type": "input_image",
                    "image_url": p.get("url") or p.get("image_url"),
                    "detail": "auto",
                }
            )
        else:
            parts.append(p)
    return parts


def _drop_images(item: dict) -> dict:
    if item.get("type") != "user":
        return item
    content = item.get("content")
    if not isinstance(content, list):
        return item
    kept = [
        p
        for p in content
        if not (isinstance(p, dict) and p.get("type") == "image")
    ]
    if len(kept) == len(content):
        return item
    new_item = dict(item)
    new_item["content"] = kept or [{"type": "text", "text": "[image omitted for replay]"}]
    return new_item


def _is_json(s: str) -> bool:
    try:
        json.loads(s)
        return True
    except (TypeError, ValueError):
        return False


def _count_text_chars(it: dict) -> int:
    n = 0
    c = it.get("content")
    if isinstance(c, str):
        n += len(c)
    elif isinstance(c, list):
        for p in c:
            if isinstance(p, dict):
                n += len(p.get("text") or "")
            elif isinstance(p, str):
                n += len(p)
    n += len(it.get("arguments") or "")
    n += len(it.get("output") or "")
    n += len(it.get("encrypted_content") or "")
    for s in it.get("summary") or []:
        if isinstance(s, dict):
            n += len(s.get("text") or "")
    return n


def _count_images(it: dict) -> int:
    c = it.get("content")
    if not isinstance(c, list):
        return 0
    return sum(
        1
        for p in c
        if isinstance(p, dict) and p.get("type") in ("image", "input_image")
    )
