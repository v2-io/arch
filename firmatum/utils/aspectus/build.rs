//! Stamp version extras at compile time. No git call at runtime.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let sha = git_out(&["rev-parse", "--short", "HEAD"]);
    let tagged = git_out(&["describe", "--exact-match", "--tags"])
        .map(|t| !t.is_empty())
        .unwrap_or(false);

    if let Some(sha) = sha {
        if !tagged {
            println!("cargo:rustc-env=ASPECTUS_GIT_SHA={sha}");
        }
    }
}

fn git_out(args: &[&str]) -> Option<String> {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8(out.stdout).ok()?;
    let s = s.trim();
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}
