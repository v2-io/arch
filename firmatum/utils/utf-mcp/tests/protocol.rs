//! Drive the built server over stdio the way an MCP client does, and check the
//! replies and the call log.

use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};

fn run(lines: &[String], extra_args: &[&str]) -> (Vec<Value>, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_utf-mcp"))
        .args(extra_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let mut stdin = child.stdin.take().unwrap();
        for l in lines {
            stdin.write_all(l.as_bytes()).unwrap();
            stdin.write_all(b"\n").unwrap();
        }
    }
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    let replies = stdout.lines().map(|l| serde_json::from_str(l).expect("every stdout line is JSON")).collect();
    (replies, String::from_utf8(out.stderr).unwrap())
}

fn req(id: u64, method: &str, params: Value) -> String {
    json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}).to_string()
}

#[test]
fn handshake_call_and_log() {
    let dir = std::env::temp_dir().join(format!("utf-mcp-test-{}", std::process::id()));
    let log = dir.join("calls.jsonl");
    let _ = std::fs::remove_file(&log);
    let lines = vec![
        req(1, "initialize", json!({"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "t", "version": "0"}})),
        json!({"jsonrpc": "2.0", "method": "notifications/initialized"}).to_string(),
        req(2, "tools/list", json!({})),
        req(3, "tools/call", json!({"name": "identify", "arguments": {"text": "⑧"}, "_meta": {"k": "v"}})),
        req(4, "tools/call", json!({"name": "range", "arguments": {"start": "U+0000", "end": "U+FFFF"}})),
        req(5, "tools/call", json!({"name": "no_such_tool", "arguments": {}})),
        req(6, "bogus/method", json!({})),
        "not json".to_string(),
        req(7, "ping", json!({})),
        req(8, "initialize", json!({"protocolVersion": "1999-01-01"})),
    ];
    let (r, _) = run(&lines, &["--log", log.to_str().unwrap(), "--tag", "test-tag"]);
    assert_eq!(r.len(), 9, "one reply per request, none for the notification");
    assert_eq!(r[0]["result"]["protocolVersion"], "2025-06-18");
    assert_eq!(r[0]["result"]["serverInfo"]["name"], "utf-mcp");
    let names: Vec<&str> = r[1]["result"]["tools"].as_array().unwrap().iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["identify", "range", "search", "blocks"]);
    for t in r[1]["result"]["tools"].as_array().unwrap() {
        assert_eq!(t["annotations"]["readOnlyHint"], true);
    }
    assert_eq!(r[2]["result"]["isError"], false);
    assert!(r[2]["result"]["content"][0]["text"].as_str().unwrap().contains("CIRCLED DIGIT EIGHT"));
    assert_eq!(r[3]["result"]["isError"], true, "bad tool input is a tool error the model can read");
    assert_eq!(r[4]["error"]["code"], -32602);
    assert_eq!(r[5]["error"]["code"], -32601);
    assert_eq!(r[6]["error"]["code"], -32700);
    assert_eq!(r[6]["id"], Value::Null);
    assert_eq!(r[7]["result"], json!({}));
    assert_eq!(r[8]["result"]["protocolVersion"], "2025-06-18", "unknown version falls back to the default");

    let records: Vec<Value> = std::fs::read_to_string(&log).unwrap().lines().map(|l| serde_json::from_str(l).unwrap()).collect();
    assert_eq!(records.first().unwrap()["event"], "start");
    assert_eq!(records.last().unwrap()["event"], "exit");
    assert!(records.iter().all(|x| x["tag"] == "test-tag"));
    let calls: Vec<&Value> = records.iter().filter(|x| x["event"] == "call").collect();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0]["tool"], "identify");
    assert_eq!(calls[0]["arguments"], json!({"text": "⑧"}));
    assert_eq!(calls[0]["meta"], json!({"k": "v"}));
    assert_eq!(calls[0]["client"], "t");
    assert!(calls[0]["result"].as_str().unwrap().contains("CIRCLED DIGIT EIGHT"));
    assert_eq!(calls[1]["is_error"], true);
    assert!(records.iter().any(|x| x["event"] == "call_rejected"));
    let bogus = records.iter().find(|x| x["method"] == "bogus/method").unwrap();
    assert_eq!(bogus["ok"], false);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn oversized_line_is_refused_and_server_continues() {
    let big = format!("{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"ping\",\"pad\":\"{}\"}}", "x".repeat(2 << 20));
    let (r, _) = run(&[big, req(2, "ping", json!({}))], &["--no-log"]);
    assert_eq!(r.len(), 2);
    assert_eq!(r[0]["error"]["code"], -32600);
    assert_eq!(r[1]["id"], 2);
}

#[test]
fn empty_batch_is_an_error() {
    let (r, _) = run(&["[]".to_string()], &["--no-log"]);
    assert_eq!(r.len(), 1);
    assert_eq!(r[0]["error"]["code"], -32600);
}

#[test]
fn batch_requests() {
    let batch = json!([
        {"jsonrpc": "2.0", "id": 1, "method": "ping"},
        {"jsonrpc": "2.0", "method": "notifications/initialized"},
        {"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "blocks", "arguments": {"query": "U+1F000"}}}
    ])
    .to_string();
    let (r, _) = run(&[batch], &["--no-log"]);
    assert_eq!(r.len(), 2);
    assert!(r[1]["result"]["content"][0]["text"].as_str().unwrap().contains("Mahjong Tiles"));
}

#[test]
fn agy_hook_policy() {
    let hook = |payload: &str| -> Value {
        let mut c = Command::new(env!("CARGO_BIN_EXE_utf-mcp"))
            .args(["--no-log", "agy-hook", "--allow-server", "unicode_unicode"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        c.stdin.take().unwrap().write_all(payload.as_bytes()).unwrap();
        serde_json::from_slice(&c.wait_with_output().unwrap().stdout).unwrap()
    };
    let ok = hook(r#"{"toolCall":{"name":"call_mcp_tool","args":{"ServerName":"unicode_unicode","ToolName":"identify"}}}"#);
    assert_eq!(ok["decision"], "allow");
    assert_eq!(ok["permissionOverrides"][0], "mcp(unicode_unicode/*)");
    assert_eq!(hook(r#"{"toolCall":{"name":"run_command","args":{"CommandLine":"ls"}}}"#)["decision"], "deny");
    assert_eq!(hook(r#"{"toolCall":{"name":"call_mcp_tool","args":{"ServerName":"other"}}}"#)["decision"], "deny");
    assert_eq!(hook("garbage")["decision"], "deny");
    assert_eq!(hook(r#"{"toolCall":{"name":"finish","args":{}}}"#)["decision"], "allow");
    // a model-written ServerName does not unlock other tools
    assert_eq!(hook(r#"{"toolCall":{"name":"run_command","args":{"CommandLine":"ls","ServerName":"unicode_unicode"}}}"#)["decision"], "deny");
    assert_eq!(hook(r#"{"toolCall":{"name":"view_file","args":{"AbsolutePath":"/etc/passwd","server":"unicode_unicode"}}}"#)["decision"], "deny");
    // overrides name only this server
    assert_eq!(ok["permissionOverrides"].as_array().unwrap().len(), 2);
}
