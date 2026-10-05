//! `utf-mcp agy-hook`: a PreToolUse hook for Antigravity CLI (`agy`) that lets a
//! model use the Unicode MCP server and nothing else.
//!
//! agy pipes each pending tool call to the hook as JSON on stdin and reads a
//! decision from stdout. This hook's policy:
//!
//!   `call_mcp_tool` whose ServerName is an allowed server -> allow
//!   agy's own turn-control steps (finish, notify_user, ...) -> allow
//!   any tool named with --allow-tool                       -> allow
//!   anything else                                           -> deny, with a reason the model sees
//!
//! The allow decision keys on the tool's name first. ServerName is a model-written
//! argument, so it is only consulted for agy's MCP tool, never as a pass for
//! other tools that happen to carry it.
//!
//! What it does not do (observed with agy 1.2.17, 2026-10-05): an "allow" here
//! does not satisfy print mode's separate MCP permission check; agy still
//! auto-denies the call unless ~/.gemini/antigravity-cli/settings.json has an
//! allow-rule. The hook's value is the deny side and the record: every decision
//! goes to the call log, so attempts to reach other tools are recorded next to
//! the lookups. Behaviour when the hook times out or fails to start is untested.

use crate::log;
use serde_json::{json, Value};
use std::io::Read;

/// agy's MCP tool (observed in agy's transcript as the step that carries ServerName/ToolName).
const MCP_TOOL: &str = "call_mcp_tool";
/// agy-internal steps that only steer the turn; denying them could break it.
const CONTROL_TOOLS: &[&str] = &["finish", "notify_user", "task_boundary", "wait", "wait_5_seconds"];

pub fn run(rest: &[String], mut lg: log::Log) {
    let mut servers: Vec<String> = Vec::new();
    let mut extra_tools: Vec<String> = Vec::new();
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--allow-server" if i + 1 < rest.len() => {
                servers.push(rest[i + 1].clone());
                i += 1;
            }
            "--allow-tool" if i + 1 < rest.len() => {
                extra_tools.push(rest[i + 1].clone());
                i += 1;
            }
            other => {
                eprintln!("utf-mcp agy-hook: unknown argument {other:?}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    if servers.is_empty() {
        servers.push("unicode_unicode".into());
    }

    let mut input = String::new();
    let _ = std::io::stdin().take(4 << 20).read_to_string(&mut input);
    let payload: Value = serde_json::from_str(&input).unwrap_or(Value::Null);
    let call = payload.get("toolCall").cloned().unwrap_or(Value::Null);
    let name = call.get("name").and_then(Value::as_str).unwrap_or("").to_string();
    let args = call.get("args").cloned().unwrap_or(Value::Null);
    let server = ["ServerName", "serverName", "server_name", "server"]
        .iter()
        .find_map(|k| args.get(*k).and_then(Value::as_str))
        .map(str::to_string);

    let tool_name = ["ToolName", "toolName", "tool_name", "tool"].iter().find_map(|k| args.get(*k).and_then(Value::as_str));
    let (decision, reason) = if payload.is_null() {
        ("deny", "hook could not parse the tool call".to_string())
    } else if name == MCP_TOOL {
        let plain = |t: &str| !t.is_empty() && t.len() <= 64 && t.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
        match &server {
            // ToolName is model-written: refuse anything that is not a plain identifier.
            Some(_) if tool_name.is_some_and(|t| !plain(t)) => ("deny", "MCP tool name is not a plain identifier".to_string()),
            Some(s) if servers.iter().any(|a| a == s) => ("allow", String::new()),
            Some(s) => ("deny", format!("MCP server {s:?} is not available in this session; only Unicode lookup tools are.")),
            None => ("deny", "MCP call without a server name".to_string()),
        }
    } else if CONTROL_TOOLS.contains(&name.as_str()) || extra_tools.iter().any(|t| *t == name) {
        ("allow", String::new())
    } else {
        ("deny", format!("The tool {name:?} is not available in this session; only Unicode lookup tools are."))
    };

    lg.hook_decision(&payload, &name, server.as_deref(), decision);
    let mut out = json!({"decision": decision});
    if !reason.is_empty() {
        out["reason"] = json!(reason);
    }
    // allowTool and permissionOverrides are the hook contract's grants. Neither made
    // agy 1.2.17's print mode accept the MCP call (observed 2026-10-05); they are
    // kept, scoped to this one server, in case a later agy honours them.
    if decision == "allow" {
        out["allowTool"] = json!(true);
        if name == MCP_TOOL {
            // Only the operator-given server name goes into the grant; nothing
            // model-written is interpolated into a permission string.
            if let Some(s) = &server {
                out["permissionOverrides"] = json!([format!("mcp({s}/*)")]);
            }
        }
    }
    println!("{out}");
}
