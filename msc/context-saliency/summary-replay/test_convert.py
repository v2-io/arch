import json
import unittest

from convert import (
    build_create_body,
    conversation_to_input_items,
    last_user_prompt_text,
    replace_last_user_text,
)


def req(**kwargs):
    base = {
        "model": "grok-4.6",
        "chat_history": [
            {"type": "system", "content": "sys"},
            {
                "type": "user",
                "content": [{"type": "text", "text": "hello"}],
            },
            {
                "type": "reasoning",
                "id": "rs_1",
                "summary": [{"type": "summary_text", "text": "think"}],
                "encrypted_content": "blob",
                "status": "completed",
            },
            {
                "type": "assistant",
                "content": "I'll look.",
                "tool_calls": [
                    {
                        "id": "call-1",
                        "name": "run_terminal_command",
                        "arguments": '{"command": "ls"}',
                    }
                ],
            },
            {
                "type": "tool_result",
                "tool_call_id": "call-1",
                "content": "ok",
            },
            {
                "type": "user",
                "content": [{"type": "text", "text": "SUMMARIZE THIS"}],
            },
        ],
        "tools": [
            {
                "name": "run_terminal_command",
                "description": "run",
                "parameters": {"type": "object"},
            }
        ],
    }
    base.update(kwargs)
    return base


class ConvertTests(unittest.TestCase):
    def test_last_prompt(self):
        self.assertEqual(last_user_prompt_text(req()["chat_history"]), "SUMMARIZE THIS")

    def test_replace_last_prompt(self):
        h = replace_last_user_text(req()["chat_history"], "NEW")
        self.assertEqual(last_user_prompt_text(h), "NEW")
        self.assertEqual(_user_text(h[1]), "hello")

    def test_reasoning_drops_status(self):
        items = conversation_to_input_items(req()["chat_history"])
        r = [i for i in items if i.get("type") == "reasoning"][0]
        self.assertNotIn("status", r)
        self.assertEqual(r["encrypted_content"], "blob")
        self.assertEqual(r["id"], "rs_1")

    def test_assistant_splits_message_and_function_call(self):
        items = conversation_to_input_items(req()["chat_history"])
        kinds = [(i.get("type"), i.get("role")) for i in items]
        self.assertIn(("message", "assistant"), kinds)
        fc = [i for i in items if i.get("type") == "function_call"][0]
        self.assertEqual(fc["call_id"], "call-1")
        self.assertEqual(fc["name"], "run_terminal_command")
        out = [i for i in items if i.get("type") == "function_call_output"][0]
        self.assertEqual(out["output"], "ok")

    def test_bad_tool_args_become_empty_object(self):
        r = req()
        r["chat_history"][3]["tool_calls"][0]["arguments"] = "not-json"
        items = conversation_to_input_items(r["chat_history"])
        fc = [i for i in items if i.get("type") == "function_call"][0]
        self.assertEqual(fc["arguments"], "{}")

    def test_single_text_user_is_string_content(self):
        items = conversation_to_input_items(req()["chat_history"])
        user = [i for i in items if i.get("role") == "user"][0]
        self.assertEqual(user["content"], "hello")

    def test_image_becomes_input_image(self):
        history = [
            {
                "type": "user",
                "content": [
                    {"type": "text", "text": "see"},
                    {"type": "image", "url": "data:image/png;base64,abc"},
                ],
            }
        ]
        items = conversation_to_input_items(history)
        content = items[0]["content"]
        self.assertEqual(content[0]["type"], "input_text")
        self.assertEqual(content[1]["type"], "input_image")
        self.assertEqual(content[1]["image_url"], "data:image/png;base64,abc")

    def test_build_body_swaps_prompt_and_sets_tool_choice_none(self):
        body = build_create_body(req(), prompt="ALT")
        self.assertEqual(body["model"], "grok-4.6")
        self.assertEqual(body["tool_choice"], "none")
        self.assertEqual(body["tools"][0]["name"], "run_terminal_command")
        self.assertFalse(body["store"])
        last = body["input"][-1]
        self.assertEqual(last["role"], "user")
        self.assertEqual(last["content"], "ALT")

    def test_drop_images(self):
        r = req()
        r["chat_history"][1]["content"] = [
            {"type": "text", "text": "see"},
            {"type": "image", "url": "data:image/png;base64,abc"},
        ]
        body = build_create_body(r, drop_images=True)
        user = [i for i in body["input"] if i.get("role") == "user"][0]
        self.assertIsInstance(user["content"], str)


def _user_text(item):
    c = item["content"]
    if isinstance(c, list):
        return c[0]["text"]
    return c


if __name__ == "__main__":
    unittest.main()
