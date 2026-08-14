//! Stamp version extras at compile time. No git call at runtime.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Re-stamp when the repo state moves — the old script only reran when
    // build.rs changed, so `+sha` froze across commits and dirty rebuilds
    // and a stale install was invisible (hardening 2026-08-14).
    if let Some(gitdir) = git_out(&["rev-parse", "--absolute-git-dir"]) {
        println!("cargo:rerun-if-changed={gitdir}/HEAD");
        println!("cargo:rerun-if-changed={gitdir}/index");
    }

    let sha = git_out(&["rev-parse", "--short", "HEAD"]);
    let tagged = git_out(&["describe", "--exact-match", "--tags"])
        .map(|t| !t.is_empty())
        .unwrap_or(false);
    let dirty = git_out(&["status", "--porcelain"])
        .map(|s| !s.is_empty())
        .unwrap_or(false);

    if let Some(sha) = sha {
        if !tagged {
            let d = if dirty { ".dirty" } else { "" };
            println!("cargo:rustc-env=ASPECTUS_GIT_SHA={sha}{d}");
        }
    }
    // Build stamp (UTC seconds since epoch, formatted at runtime) so
    // `aspectus version` identifies the actual build.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    println!("cargo:rustc-env=ASPECTUS_BUILD_EPOCH={now}");
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
