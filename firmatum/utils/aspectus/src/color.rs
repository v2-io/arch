//! Paint a look. TTY changes presentation, not which stream.

use std::io::{self, IsTerminal};

const DIR: &str = "\x1b[01;34m";
const RESET: &str = "\x1b[0m";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Auto,
    Always,
    Never,
}

impl Mode {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "auto" => Some(Mode::Auto),
            "always" => Some(Mode::Always),
            "never" => Some(Mode::Never),
            _ => None,
        }
    }

    pub fn active(self) -> bool {
        match self {
            Mode::Never => false,
            Mode::Always => true,
            Mode::Auto => io::stdout().is_terminal(),
        }
    }
}

/// Directory names (including the trailing `/`). Files stay unpainted for now.
pub fn dir(name: &str, on: bool) -> String {
    if on {
        format!("{DIR}{name}{RESET}")
    } else {
        name.to_string()
    }
}
