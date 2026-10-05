//! utf-mcp — a read-only Unicode Character Database lookup served over the
//! Model Context Protocol (stdio transport: newline-delimited JSON-RPC 2.0).
//!
//! The server never touches the file system or network on a model's behalf.
//! Its only side effect is the call log, whose path is fixed by whoever launches
//! it (flag or environment), never by tool arguments.

mod hook;
mod log;
mod tools;
mod ucd;

use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::time::Instant;

const SERVER_NAME: &str = "utf-mcp";
const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Protocol revisions this server speaks (it uses only features common to all).
const PROTOCOLS: &[&str] = &["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"];
const DEFAULT_PROTOCOL: &str = "2025-06-18";
/// Longest accepted request line; longer lines are answered with an error and discarded.
const MAX_LINE: usize = 1 << 20;

const HELP: &str = "utf-mcp — Unicode Character Database lookup over MCP (stdio)

USAGE
  utf-mcp [--log PATH | --log - | --no-log] [--tag TEXT]      serve MCP on stdin/stdout
  utf-mcp query TOOL [JSON-ARGS]                                run one tool, print its result
  utf-mcp tools                                                 print the tool list (JSON)
  utf-mcp dump                                                  TSV of core properties of every code point
  utf-mcp [--log ...] agy-hook [--allow-server NAME]...         agy PreToolUse hook: allow only the
                                                                Unicode MCP server (see README)
  utf-mcp --version | --help

TOOLS  identify, range, search, blocks   (see README.md)

CALL LOG (JSON Lines, one record per request, appended)
  --log PATH     append to PATH (directories are created)
  --log -        write records to stderr instead of a file
  --no-log       no call log
  default        $UTF_MCP_LOG if set, else $XDG_STATE_HOME/utf-mcp/calls.jsonl
                 (~/.local/state/utf-mcp/calls.jsonl)
  --tag TEXT     free text copied into every record (also $UTF_MCP_TAG), e.g. a run id
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut log_target = log::Target::Default;
    let mut tag = std::env::var("UTF_MCP_TAG").ok();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print!("{HELP}");
                return;
            }
            "--version" | "-V" => {
                println!("{SERVER_NAME} {VERSION} (Unicode {})", ucd::UCD_VERSION);
                return;
            }
            "--no-log" => log_target = log::Target::None,
            "--log" => {
                i += 1;
                let Some(p) = args.get(i) else { die("--log needs a path (or -)") };
                log_target = if p == "-" { log::Target::Stderr } else { log::Target::File(p.into()) };
            }
            "--tag" => {
                i += 1;
                let Some(t) = args.get(i) else { die("--tag needs a value") };
                tag = Some(t.clone());
            }
            "query" => return query_mode(&args[i + 1..]),
            "agy-hook" => return hook::run(&args[i + 1..], log::Log::open(log_target, tag)),
            "dump" => return dump(),
            "tools" => {
                println!("{}", serde_json::to_string_pretty(&tools::tool_list()).unwrap());
                return;
            }
            other => die(&format!("unknown argument {other:?} (try --help)")),
        }
        i += 1;
    }
    serve(log::Log::open(log_target, tag));
}

fn die(msg: &str) -> ! {
    eprintln!("utf-mcp: {msg}");
    std::process::exit(2);
}

fn query_mode(rest: &[String]) {
    let Some(tool) = rest.first() else { die("query needs a tool name") };
    let args: Value = match rest.get(1) {
        Some(s) => serde_json::from_str(s).unwrap_or_else(|e| die(&format!("arguments are not JSON: {e}"))),
        None => json!({}),
    };
    let u = ucd::Ucd::load();
    match tools::call(&u, tool, &args) {
        None => die(&format!("no tool named {tool:?}")),
        Some(Ok(v)) => println!("{}", tools::render(&v)),
        Some(Err(e)) => {
            println!("error: {e}");
            std::process::exit(1);
        }
    }
}

/// Tab-separated table of every code point's core properties, for cross-checking
/// against other UCD implementations. Columns: cp, name, gc, ccc, bidi,
/// decomposition (Hangul syllables: algorithmic), numeric_type, numeric_value, mirrored, eaw, age, script, block.
fn dump() {
    let u = ucd::Ucd::load();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());
    let _ = writeln!(out, "cp\tname\tgc\tccc\tbidi\tdecomposition\tnumeric_type\tnumeric_value\tmirrored\teaw\tage\tscript\tblock");
    for cp in 0..=ucd::MAX_CP {
        let d = u.ud(cp);
        let (nt, nv) = u.numeric(cp).unwrap_or(("", ""));
        let _ = writeln!(
            out,
            "{cp:04X}\t{}\t{}\t{}\t{}\t{}\t{nt}\t{nv}\t{}\t{}\t{}\t{}\t{}",
            u.name(cp).unwrap_or_default(),
            u.general_category(cp),
            d.map(|d| d.ccc).unwrap_or("0"),
            d.map(|d| d.bidi).unwrap_or(""),
            d.map(|d| d.decomp)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .or_else(|| ucd::Ucd::hangul_decomposition(cp).map(|v| v.iter().map(|c| format!("{c:04X}")).collect::<Vec<_>>().join(" ")))
                .unwrap_or_default(),
            d.map(|d| if d.mirrored { "Y" } else { "N" }).unwrap_or(""),
            u.east_asian_width(cp),
            u.age(cp),
            u.script(cp),
            u.block(cp).map(|b| b.name).unwrap_or("No_Block"),
        );
    }
}

fn serve(mut log: log::Log) {
    let t0 = Instant::now();
    let u = ucd::Ucd::load();
    log.start(t0.elapsed().as_millis());

    let stdin = io::stdin();
    let mut input = stdin.lock();
    let stdout = io::stdout();
    let mut buf: Vec<u8> = Vec::new();
    loop {
        buf.clear();
        // Read one line, refusing to buffer more than MAX_LINE bytes of it.
        let mut too_long = false;
        loop {
            let chunk = match input.fill_buf() {
                Ok(c) => c,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(_) => break,
            };
            if chunk.is_empty() {
                break;
            }
            let (take, done) = match chunk.iter().position(|&b| b == b'\n') {
                Some(p) => (p + 1, true),
                None => (chunk.len(), false),
            };
            if !too_long {
                if buf.len() + take > MAX_LINE {
                    too_long = true;
                    buf.clear();
                } else {
                    buf.extend_from_slice(&chunk[..take]);
                }
            }
            input.consume(take);
            if done {
                break;
            }
        }
        if buf.is_empty() && !too_long {
            break; // EOF
        }
        let replies: Vec<Value> = if too_long {
            log.note("oversized request discarded");
            vec![error_reply(Value::Null, -32600, &format!("request longer than {MAX_LINE} bytes"))]
        } else {
            let line = String::from_utf8_lossy(&buf);
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            match serde_json::from_str::<Value>(line) {
                Err(e) => {
                    log.note(&format!("unparseable request: {e}"));
                    vec![error_reply(Value::Null, -32700, &format!("parse error: {e}"))]
                }
                Ok(Value::Array(batch)) => batch.into_iter().filter_map(|m| handle(&u, &mut log, m)).collect(),
                Ok(m) => handle(&u, &mut log, m).into_iter().collect(),
            }
        };
        let mut out = stdout.lock();
        for r in replies {
            let _ = writeln!(out, "{r}");
        }
        let _ = out.flush();
    }
    log.exit();
}

fn error_reply(id: Value, code: i64, msg: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": msg}})
}

/// Handle one JSON-RPC message; None for notifications (no reply).
fn handle(u: &ucd::Ucd, log: &mut log::Log, msg: Value) -> Option<Value> {
    let method = msg.get("method").and_then(Value::as_str).map(str::to_owned);
    let id = msg.get("id").cloned();
    let params = msg.get("params").cloned().unwrap_or(Value::Null);
    let Some(method) = method else {
        // A response to a request we never sent, or garbage.
        if let Some(id) = id {
            return Some(error_reply(id, -32600, "invalid request: no method"));
        }
        return None;
    };
    let Some(id) = id else {
        log.notification(&method);
        return None;
    };
    let started = Instant::now();
    let result: Result<Value, (i64, String)> = match method.as_str() {
        "initialize" => {
            let asked = params.get("protocolVersion").and_then(Value::as_str).unwrap_or(DEFAULT_PROTOCOL);
            let proto = if PROTOCOLS.contains(&asked) { asked } else { DEFAULT_PROTOCOL };
            log.initialize(&params, proto);
            Ok(json!({
                "protocolVersion": proto,
                "capabilities": {"tools": {"listChanged": false}},
                "serverInfo": {"name": SERVER_NAME, "title": "Unicode lookup", "version": VERSION},
                "instructions": format!(
                    "Read-only lookup in the Unicode Character Database, version {}. \
                     `identify` answers what given glyphs, code points or names are; \
                     `range` lists a character's code-point neighbours; \
                     `search` finds characters by words in their names; \
                     `blocks` shows Unicode blocks and the headings their code charts are divided into. \
                     Facts come from the UCD files only; nothing here renders or measures glyph shapes.",
                    ucd::UCD_VERSION
                ),
            }))
        }
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({"tools": tools::tool_list()})),
        "resources/list" => Ok(json!({"resources": []})),
        "resources/templates/list" => Ok(json!({"resourceTemplates": []})),
        "prompts/list" => Ok(json!({"prompts": []})),
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            match tools::call(u, name, &args) {
                None => Err((-32602, format!("unknown tool {name:?}"))),
                Some(r) => {
                    let (text, is_error) = match r {
                        Ok(v) => (tools::render(&v), false),
                        Err(e) => (format!("error: {e}"), true),
                    };
                    log.call(name, &args, params.get("_meta"), &text, is_error, started.elapsed().as_micros());
                    return Some(json!({
                        "jsonrpc": "2.0", "id": id,
                        "result": {"content": [{"type": "text", "text": text}], "isError": is_error}
                    }));
                }
            }
        }
        _ => Err((-32601, format!("method not found: {method}"))),
    };
    if method != "tools/call" && method != "initialize" {
        log.request(&method, result.is_ok(), &params);
    }
    if let Err((_, ref m)) = result {
        if method == "tools/call" {
            log.call_rejected(&params, m);
        }
    }
    Some(match result {
        Ok(r) => json!({"jsonrpc": "2.0", "id": id, "result": r}),
        Err((code, m)) => error_reply(id, code, &m),
    })
}
