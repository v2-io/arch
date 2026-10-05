//! The call log: one JSON object per line, appended. Every record carries the
//! server session id (one per process), the parent pid (the harness that
//! launched the server), the operator's tag, and a timestamp. Tool calls record
//! the exact arguments, the harness's `_meta` (Claude Code puts its tool-use id
//! there, which joins the record to the transcript), and the exact text the
//! model was given back.
//!
//! Logging failures never affect the protocol: they are reported once on
//! stderr and logging stops.

use serde_json::{json, Map, Value};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub enum Target {
    Default,
    None,
    Stderr,
    File(PathBuf),
}

enum Sink {
    None,
    Stderr,
    File(File),
}

pub struct Log {
    sink: Sink,
    session: String,
    tag: Option<String>,
    client: Value,
    seq: u64,
    calls: u64,
}

impl Log {
    pub fn open(target: Target, tag: Option<String>) -> Log {
        let path = match target {
            Target::None => None,
            Target::Stderr => return Log::new(Sink::Stderr, tag),
            Target::File(p) => Some(p),
            Target::Default => match std::env::var_os("UTF_MCP_LOG") {
                Some(p) if p == "-" => return Log::new(Sink::Stderr, tag),
                Some(p) if p.is_empty() || p == "none" => None,
                Some(p) => Some(PathBuf::from(p)),
                None => default_path(),
            },
        };
        let sink = match path {
            None => Sink::None,
            Some(p) => {
                if let Some(dir) = p.parent() {
                    let _ = std::fs::create_dir_all(dir);
                }
                match OpenOptions::new().create(true).append(true).open(&p) {
                    Ok(f) => Sink::File(f),
                    Err(e) => {
                        eprintln!("utf-mcp: cannot open call log {}: {e}; continuing without a log", p.display());
                        Sink::None
                    }
                }
            }
        };
        Log::new(sink, tag)
    }

    fn new(sink: Sink, tag: Option<String>) -> Log {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
        let session = format!("{:x}-{:x}", std::process::id(), (nanos & 0xFFFF_FFFF_FFFF) as u64);
        Log { sink, session, tag, client: Value::Null, seq: 0, calls: 0 }
    }

    fn emit(&mut self, event: &str, fields: Value) {
        if matches!(self.sink, Sink::None) {
            return;
        }
        self.seq += 1;
        let mut m = Map::new();
        m.insert("ts".into(), json!(timestamp()));
        m.insert("event".into(), json!(event));
        m.insert("session".into(), json!(self.session));
        m.insert("seq".into(), json!(self.seq));
        if let Some(t) = &self.tag {
            m.insert("tag".into(), json!(t));
        }
        if let Value::Object(f) = fields {
            m.extend(f);
        }
        let mut line = Value::Object(m).to_string();
        line.push('\n');
        let failed = match &mut self.sink {
            Sink::File(f) => f.write_all(line.as_bytes()).is_err(),
            Sink::Stderr => std::io::stderr().write_all(line.as_bytes()).is_err(),
            Sink::None => false,
        };
        if failed {
            eprintln!("utf-mcp: writing the call log failed; logging stops");
            self.sink = Sink::None;
        }
    }

    pub fn start(&mut self, load_ms: u128) {
        let cwd = std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default();
        self.emit(
            "start",
            json!({
                "pid": std::process::id(),
                "ppid": std::os::unix::process::parent_id(),
                "parent_command": parent_command(),
                "cwd": cwd,
                "server": format!("utf-mcp {}", env!("CARGO_PKG_VERSION")),
                "unicode": crate::ucd::UCD_VERSION,
                "load_ms": load_ms,
            }),
        );
    }

    pub fn initialize(&mut self, params: &Value, proto: &str) {
        self.client = params.get("clientInfo").cloned().unwrap_or(Value::Null);
        self.emit(
            "initialize",
            json!({
                "client": self.client,
                "protocol_requested": params.get("protocolVersion"),
                "protocol": proto,
                "client_capabilities": params.get("capabilities"),
            }),
        );
    }

    pub fn notification(&mut self, method: &str) {
        self.emit("notification", json!({"method": method}));
    }

    /// `params` is recorded only for methods the server does not implement,
    /// so harness probes are visible without logging routine traffic.
    pub fn request(&mut self, method: &str, ok: bool, params: &Value) {
        if ok {
            self.emit("request", json!({"method": method, "ok": true}));
        } else {
            let p = params.to_string();
            let p: Value = if p.len() > 4000 {
                let mut cut = 4000;
                while !p.is_char_boundary(cut) {
                    cut -= 1;
                }
                json!(format!("{}…", &p[..cut]))
            } else {
                params.clone()
            };
            self.emit("request", json!({"method": method, "ok": false, "params": p}));
        }
    }

    pub fn call(&mut self, tool: &str, args: &Value, meta: Option<&Value>, result: &str, is_error: bool, micros: u128) {
        self.calls += 1;
        let client = self.client.get("name").cloned().unwrap_or(Value::Null);
        self.emit(
            "call",
            json!({
                "client": client,
                "tool": tool,
                "arguments": args,
                "meta": meta,
                "is_error": is_error,
                "micros": micros,
                "result_chars": result.chars().count(),
                "result": result,
            }),
        );
    }

    pub fn call_rejected(&mut self, params: &Value, msg: &str) {
        self.emit("call_rejected", json!({"params": params, "error": msg}));
    }

    /// One PreToolUse decision made by `utf-mcp agy-hook` (a separate process per decision).
    pub fn hook_decision(&mut self, payload: &Value, tool: &str, server: Option<&str>, decision: &str) {
        self.emit(
            "hook",
            json!({
                "harness": "agy",
                "decision": decision,
                "tool": tool,
                "server": server,
                "tool_call": payload.get("toolCall"),
                "conversation": payload.get("conversationId"),
                "step": payload.get("stepIdx"),
                "model": payload.get("modelName"),
            }),
        );
    }

    pub fn note(&mut self, msg: &str) {
        self.emit("note", json!({"message": msg}));
    }

    pub fn exit(&mut self) {
        let calls = self.calls;
        self.emit("exit", json!({"calls": calls}));
    }
}

fn default_path() -> Option<PathBuf> {
    let base = match std::env::var_os("XDG_STATE_HOME") {
        Some(p) if !p.is_empty() => PathBuf::from(p),
        _ => PathBuf::from(std::env::var_os("HOME")?).join(".local/state"),
    };
    Some(base.join("utf-mcp/calls.jsonl"))
}

/// The launching process's command line, via `ps` (best effort; macOS/Linux).
fn parent_command() -> Value {
    let ppid = std::os::unix::process::parent_id();
    std::process::Command::new("ps")
        .args(["-o", "command=", "-p", &ppid.to_string()])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().chars().take(400).collect::<String>())
        .filter(|s| !s.is_empty())
        .map(Value::String)
        .unwrap_or(Value::Null)
}

/// RFC 3339 UTC timestamp with milliseconds, without external crates.
fn timestamp() -> String {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = d.as_secs() as i64;
    let ms = d.subsec_millis();
    let days = secs.div_euclid(86_400);
    let sod = secs.rem_euclid(86_400);
    // civil_from_days (Howard Hinnant)
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { y + 1 } else { y };
    format!("{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{ms:03}Z", sod / 3600, sod % 3600 / 60, sod % 60)
}
