//! `utf-mcp agy-hook`: a PreToolUse hook for Antigravity CLI (`agy`) that lets a
//! model use the Unicode MCP server and nothing else.
//!
//! agy print mode auto-denies MCP calls unless an allow-rule exists in the
//! user's settings. A hook in a workspace plugin can decide instead, so the
//! policy lives beside the server rather than in `~/.gemini`. agy pipes each
//! pending tool call to the hook as JSON on stdin and reads a decision from
//! stdout:
//!
//!   MCP call to an allowed server  -> allow
//!   agy's own turn-control steps   -> allow   (finish, notify_user, ...)
//!   anything else                  -> deny, with a reason the model sees
//!
//! Every decision is written to the call log, so attempts to reach other tools
//! (shell, file reads, web) are recorded next to the lookups.

use crate::log;
use serde_json::{json, Value};
use std::io::Read;

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
    } else if let Some(s) = &server {
        if servers.iter().any(|a| a == s) {
            ("allow", String::new())
        } else {
            ("deny", format!("MCP server {s:?} is not available in this session; only Unicode lookup tools are."))
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
    // In print mode an "allow" decision alone does not satisfy agy's separate
    // permission check for MCP calls (observed 2026-10-05: still auto-denied).
    // permissionOverrides are the hook contract's temporary grants; they are
    // scoped to this server only.
    if decision == "allow" {
        out["allowTool"] = json!(true);
        if let Some(s) = &server {
            let mut grants = vec![format!("mcp({s}/*)")];
            if let Some(t) = tool_name {
                grants.push(format!("mcp({s}/{t})"));
            }
            if let Some((_, plain)) = s.split_once('_') {
                grants.push(format!("mcp({plain}/*)"));
            }
            out["permissionOverrides"] = json!(grants);
        }
    }
    println!("{out}");
}
