import json
import unittest
from pathlib import Path
from tempfile import NamedTemporaryFile

from transcript import (
    build_tool_record,
    convert,
    format_print,
    format_stat,
    hunk_arrow,
    iso,
    line_diff_stat,
    parse_commit_message,
    relpath,
    strip_eval_unmatched,
)


def ev(ts, **update):
    return (ts, update)


class Unit(unittest.TestCase):
    def test_print_layout(self):
        text = format_print(
            [
                {"type": "user", "ts": "2026-08-30T15:17:32Z", "text": "Hey there, this is a long message " * 10},
                {"type": "assistant", "ts": "2026-08-30T15:17:42Z", "text": "Its response 1"},
                {
                    "type": "tool",
                    "ts": "2026-08-30T15:17:42Z",
                    "name": "read",
                    "path": "~/src/foo/projects",
                    "span": "full",
                },
                {"type": "user", "ts": "2026-08-30T15:18:00Z", "text": "next turn"},
            ]
        )
        lines = text.splitlines()
        self.assertEqual(lines[0], "")
        self.assertTrue(lines[1].startswith('   0.00 | user | "Hey there'))
        self.assertTrue(lines[2].startswith("  10.00 | agnt |"))
        self.assertTrue(lines[3].startswith("   0.00 | tool |"))
        self.assertEqual(lines[4], "")
        self.assertTrue(lines[5].startswith('  18.00 | user | "next turn"'))
        self.assertTrue(
            lines[3].startswith("   0.00 | tool | read | ~/src/foo/projects | full")
        )
        quoted = lines[1].split(" | ", 2)[2]
        self.assertTrue(quoted.startswith('"') and quoted.endswith('"'))
        inner = quoted[1:-1]
        self.assertTrue(inner.endswith("…"))
        self.assertEqual(len(inner), 151)

    def test_print_verbose_keeps_dialog_elides_tools(self):
        long_user = "Hey there, this is a long message " * 10
        long_tool = "x" * 200
        text = format_print(
            [
                {"type": "user", "ts": "2026-08-30T15:17:32Z", "text": long_user},
                {"type": "assistant", "ts": "2026-08-30T15:17:42Z", "text": "Its response 1"},
                {
                    "type": "tool",
                    "ts": "2026-08-30T15:17:42Z",
                    "name": "read",
                    "path": "~/src/foo/projects",
                    "span": "full",
                    "response": long_tool,
                },
            ],
            verbose=1,
        )
        lines = [ln for ln in text.splitlines() if ln]
        user_body = lines[0].split(" | ", 2)[2]
        self.assertEqual(user_body, f'"{ " ".join(long_user.split()) }"')
        self.assertFalse(user_body.endswith("…\""))
        self.assertIn('| agnt | "Its response 1"', lines[1])
        tool_body = lines[2].split(" | ", 2)[2]
        self.assertTrue(tool_body.endswith("…"))
        self.assertEqual(len(tool_body), 151)

    def test_strip_eval_unmatched(self):
        raw = "exit: 0\n(eval):23: unmatched '\nhello\n"
        self.assertEqual(
            " ".join(strip_eval_unmatched(raw).split()),
            "exit: 0 hello",
        )

    def test_write_result_is_content_not_stat(self):
        rec = build_tool_record(
            "write",
            {"file_path": "/tmp/x.py", "content": "alpha\nbeta\ngamma\n"},
            "completed",
            "The file /tmp/x.py has been updated successfully.",
        )
        self.assertEqual(rec["name"], "write")
        self.assertEqual(rec["summary"], "write (3 lines)")
        self.assertTrue(rec["response"].startswith("alpha beta gamma"))
        self.assertNotIn("insertions", rec["response"])
        view = format_print(
            [{"type": "tool", "ts": "2026-08-30T15:17:32Z", **{k: rec[k] for k in rec if k != "type"}}]
        )
        line = [ln for ln in view.splitlines() if ln][0]
        self.assertIn("write (3 lines)", line)
        self.assertIn("alpha beta gamma", line)

    def test_print_verbose_keeps_commit_message(self):
        msg = "Add the thing " + ("word " * 40)
        recs = [
            {
                "type": "tool",
                "name": "commit",
                "ts": "2026-08-30T15:17:32Z",
                "sha": "abcdef0",
                "message": msg,
                "stat": "1 file changed, 3 insertions(+)",
            }
        ]
        short = format_print(recs)
        long = format_print(recs, verbose=1)
        short_line = [ln for ln in short.splitlines() if ln][0]
        long_line = [ln for ln in long.splitlines() if ln][0]
        self.assertTrue(short_line.endswith("…"))
        self.assertIn(msg.strip(), long_line.replace("\n", " "))
        self.assertFalse(long_line.rstrip().endswith("…"))

    def test_hunk_arrow(self):
        self.assertEqual(
            hunk_arrow("JET_30 = _jet_hexes(30)", "JET_N = 60\nJET = _jet_hexes(JET_N)"),
            "JET_30 = _jet_hexes(30) → JET_N = 60 JET = _jet_hexes(JET_N)",
        )

    def test_iso(self):
        self.assertEqual(iso(1788103077), "2026-08-30T15:17:57.000Z")

    def test_relpath(self):
        p = str(Path.home() / "src/arch/foo")
        self.assertEqual(relpath(p), "~/src/arch/foo")

    def test_commit_heredoc(self):
        cmd = '''git commit -m "$(cat <<'EOF'
heatmap: fade recent idle

Head runs of zeros.
EOF
)"'''
        self.assertEqual(
            parse_commit_message(cmd),
            "heatmap: fade recent idle\n\nHead runs of zeros.",
        )

    def test_stat(self):
        a, r, block = line_diff_stat("foo.py", "a\nb\nc\n", "a\nB\nc\nD\n")
        self.assertEqual(a, 2)
        self.assertEqual(r, 1)
        self.assertIn("foo.py | 3", block)
        self.assertIn("2 insertions(+)", block)
        self.assertIn("1 deletion(-)", block)


class Convert(unittest.TestCase):
    def test_dialog_skips_thoughts_and_concatenates_assistant(self):
        recs = [
            r
            for r in convert(
            [
                ev(
                    1,
                    sessionUpdate="user_message_chunk",
                    content={"type": "text", "text": "hello"},
                    _meta={"promptIndex": 0},
                ),
                ev(
                    2,
                    sessionUpdate="agent_thought_chunk",
                    content={"type": "text", "text": "secret"},
                ),
                ev(
                    3,
                    sessionUpdate="agent_message_chunk",
                    content={"type": "text", "text": "Hi. "},
                ),
                ev(
                    4,
                    sessionUpdate="agent_message_chunk",
                    content={"type": "text", "text": "There."},
                ),
                ev(5, sessionUpdate="turn_completed", stop_reason="end_turn"),
            ]
            )
            if r["type"] != "meta"
        ]
        self.assertEqual([r["type"] for r in recs], ["user", "assistant"])
        self.assertEqual(recs[0]["text"], "hello")
        self.assertEqual(recs[1]["text"], "Hi. There.")
        self.assertNotIn("secret", json.dumps(recs))

    def test_tools_interleave_and_windowed_read_is_not_known_well(self):
        recs = [
            r
            for r in convert(
            [
                ev(
                    1,
                    sessionUpdate="tool_call",
                    toolCallId="a",
                    title="read_file",
                    rawInput={"target_file": str(Path.home() / "x.py"), "offset": 10, "limit": 20},
                ),
                ev(
                    2,
                    sessionUpdate="tool_call_update",
                    toolCallId="a",
                    status="completed",
                    content=[{"type": "content", "content": {"type": "text", "text": "partial body"}}],
                ),
                ev(
                    3,
                    sessionUpdate="tool_call",
                    toolCallId="b",
                    title="read_file",
                    rawInput={"target_file": str(Path.home() / "x.py")},
                ),
                ev(
                    4,
                    sessionUpdate="tool_call_update",
                    toolCallId="b",
                    status="completed",
                    content=[{"type": "content", "content": {"type": "text", "text": "whole file here"}}],
                ),
            ]
            )
        ]
        tools = [r for r in recs if r.get("type") == "tool" and r.get("name") == "read"]
        self.assertEqual(tools[0]["span"], "lim=20")
        self.assertEqual(tools[0]["response"], "partial body")
        self.assertNotIn("known_well", tools[0])
        self.assertEqual(tools[1]["span"], "full")
        self.assertEqual(tools[1]["response"], "whole file here")
        self.assertEqual(tools[0]["path"], tools[1]["path"])
        self.assertTrue(tools[0]["path"].startswith("<f"))
        self.assertEqual(tools[1]["known_well"], [tools[1]["path"]])

    def test_commit_record_gets_full_message(self):
        cmd = '''cd /tmp
git commit -m "$(cat <<'EOF'
Add the thing

Body here.
EOF
)"
'''
        recs = [
            r
            for r in convert(
            [
                ev(
                    1,
                    sessionUpdate="tool_call",
                    toolCallId="c",
                    title="run_terminal_command",
                    rawInput={"command": cmd, "description": "commit"},
                ),
                ev(
                    2,
                    sessionUpdate="tool_call_update",
                    toolCallId="c",
                    status="completed",
                    content=[
                        {
                            "type": "content",
                            "content": {
                                "type": "text",
                                "text": " file | 3 +++\n 1 file changed, 3 insertions(+)\n[main abcdef0] Add the thing\n",
                            },
                        }
                    ],
                ),
            ]
            )
            if r["type"] != "meta"
        ]
        self.assertEqual(recs[0]["type"], "tool")
        self.assertEqual(recs[0]["name"], "commit")
        self.assertEqual(recs[0]["message"], "Add the thing\n\nBody here.")
        self.assertEqual(recs[0]["sha"], "abcdef0")
        self.assertIn("1 file changed", recs[0]["stat"])

    def test_dir_aliases_and_meta_rows(self):
        home = str(Path.home())
        recs = convert(
            [
                ev(
                    1,
                    sessionUpdate="tool_call",
                    toolCallId="a",
                    title="read_file",
                    rawInput={"target_file": f"{home}/src/arch/foo/bar.py"},
                ),
                ev(
                    2,
                    sessionUpdate="tool_call_update",
                    toolCallId="a",
                    status="completed",
                    content=[{"type": "content", "content": {"type": "text", "text": "hi"}}],
                ),
                ev(
                    3,
                    sessionUpdate="tool_call",
                    toolCallId="b",
                    title="read_file",
                    rawInput={"target_file": f"{home}/.config/zsh/x.zsh"},
                ),
                ev(
                    4,
                    sessionUpdate="tool_call_update",
                    toolCallId="b",
                    status="completed",
                    content=[{"type": "content", "content": {"type": "text", "text": "yo"}}],
                ),
            ]
        )
        kinds = [(r.get("type"), r.get("kind"), r.get("alias")) for r in recs]
        self.assertEqual(kinds[0][0], "meta")
        self.assertEqual(kinds[0][1], "ts")
        self.assertEqual(kinds[1][0], "meta")
        self.assertEqual(kinds[1][1], "cwd")
        dir_rows = [r for r in recs if r.get("kind") == "dir"]
        self.assertEqual(len(dir_rows), 2)
        self.assertEqual(dir_rows[0]["alias"], "<d1>")
        self.assertEqual(dir_rows[1]["alias"], "<d2>")
        self.assertTrue("src/arch/foo/" in dir_rows[0]["value"])
        tools = [r for r in recs if r.get("type") == "tool"]
        self.assertTrue(tools[0]["path"].startswith("<d1>"))
        self.assertTrue(tools[1]["path"].startswith("<d2>"))
        view = format_print(recs)
        self.assertIn("meta | dir | <d1> =", view)
        self.assertIn("<d1>bar.py", view)

    def test_file_alias_only_when_reused(self):
        home = str(Path.home())
        recs = convert(
            [
                ev(
                    1,
                    sessionUpdate="tool_call",
                    toolCallId="a",
                    title="read_file",
                    rawInput={"target_file": f"{home}/src/arch/foo/projects"},
                ),
                ev(
                    2,
                    sessionUpdate="tool_call_update",
                    toolCallId="a",
                    status="completed",
                    content=[{"type": "content", "content": {"type": "text", "text": "a"}}],
                ),
                ev(
                    3,
                    sessionUpdate="tool_call",
                    toolCallId="b",
                    title="search_replace",
                    rawInput={
                        "file_path": f"{home}/src/arch/foo/projects",
                        "old_string": "a",
                        "new_string": "b",
                    },
                ),
                ev(
                    4,
                    sessionUpdate="tool_call_update",
                    toolCallId="b",
                    status="completed",
                    content=[{"type": "content", "content": {"type": "text", "text": "ok"}}],
                ),
                ev(
                    5,
                    sessionUpdate="tool_call",
                    toolCallId="c",
                    title="read_file",
                    rawInput={"target_file": f"{home}/src/arch/foo/once.py"},
                ),
                ev(
                    6,
                    sessionUpdate="tool_call_update",
                    toolCallId="c",
                    status="completed",
                    content=[{"type": "content", "content": {"type": "text", "text": "z"}}],
                ),
            ]
        )
        files = [r for r in recs if r.get("kind") == "file"]
        self.assertEqual(len(files), 1)
        self.assertEqual(files[0]["alias"], "<f1>")
        self.assertIn("projects", files[0]["value"])
        tools = [r for r in recs if r.get("type") == "tool"]
        self.assertEqual(tools[0]["path"], "<f1>")
        self.assertEqual(tools[1]["path"], "<f1>")
        self.assertTrue(tools[2]["path"].endswith("once.py"))
        self.assertNotEqual(tools[2]["path"], "<f1>")
        view = format_print(recs)
        self.assertIn('meta | file | <f1> =', view)


if __name__ == "__main__":
    unittest.main()
