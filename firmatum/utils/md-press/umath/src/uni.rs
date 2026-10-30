//! Python `str` character predicates, from the reference interpreter's own
//! Unicode database (generated `tables.rs`). Rust's `char::is_alphabetic`,
//! `is_whitespace`, `is_numeric` … are different predicates over a newer
//! Unicode version; using them would silently change behavior.

use crate::tables::*;

#[inline]
pub fn flags(c: char) -> u16 {
    let o = c as u32;
    if o < 0x80 {
        return ascii_flags(c);
    }
    match FLAG_RANGES.binary_search_by(|&(a, b, _)| {
        if o < a {
            std::cmp::Ordering::Greater
        } else if o > b {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }) {
        Ok(k) => FLAG_RANGES[k].2,
        Err(_) => 0,
    }
}

#[inline]
fn ascii_flags(c: char) -> u16 {
    let mut f = 0;
    if c.is_ascii_alphabetic() {
        f |= ALPHA | ALNUM;
        f |= if c.is_ascii_uppercase() { UPPER } else { LOWER };
    }
    if c.is_ascii_digit() {
        f |= DECIMAL | DIGIT | ALNUM;
    }
    // str.isspace on ASCII: \t \n \v \f \r, 0x1c-0x1f, space
    if matches!(c, '\t' | '\n' | '\x0b' | '\x0c' | '\r' | '\x1c'..='\x1f' | ' ') {
        f |= SPACE;
    }
    f
}

#[inline]
pub fn is_alpha(c: char) -> bool {
    flags(c) & ALPHA != 0
}
#[inline]
pub fn is_decimal(c: char) -> bool {
    flags(c) & DECIMAL != 0
}
#[inline]
pub fn is_digit(c: char) -> bool {
    flags(c) & DIGIT != 0
}
#[inline]
pub fn is_alnum(c: char) -> bool {
    flags(c) & ALNUM != 0
}
#[inline]
pub fn is_space(c: char) -> bool {
    flags(c) & SPACE != 0
}
#[inline]
pub fn is_mn(c: char) -> bool {
    flags(c) & MN != 0
}
/// sre `\w`
#[inline]
pub fn is_word(c: char) -> bool {
    c == '_' || is_alnum(c)
}

/// `str.isupper()` (CPython unicode_isupper_impl).
pub fn py_isupper(s: &str) -> bool {
    let mut it = s.chars();
    if let (Some(c), None) = (it.next(), it.next()) {
        return flags(c) & UPPER != 0;
    }
    let mut cased = false;
    for c in s.chars() {
        let f = flags(c);
        if f & (LOWER | TITLE) != 0 {
            return false;
        }
        if !cased && f & UPPER != 0 {
            cased = true;
        }
    }
    cased
}

/// `str.islower()` (CPython unicode_islower_impl).
pub fn py_islower(s: &str) -> bool {
    let mut it = s.chars();
    if let (Some(c), None) = (it.next(), it.next()) {
        return flags(c) & LOWER != 0;
    }
    let mut cased = false;
    for c in s.chars() {
        let f = flags(c);
        if f & (UPPER | TITLE) != 0 {
            return false;
        }
        if !cased && f & LOWER != 0 {
            cased = true;
        }
    }
    cased
}

/// `str.isalpha()` on a whole string (non-empty, all alpha).
pub fn py_isalpha_str(s: &str) -> bool {
    !s.is_empty() && s.chars().all(is_alpha)
}

/// `str.strip()` (Unicode whitespace).
pub fn py_strip(s: &str) -> &str {
    s.trim_matches(is_space)
}

pub fn subsup_of(c: char) -> Option<(bool, char)> {
    SUBSUP.binary_search_by_key(&c, |&(k, _, _)| k).ok().map(|k| (SUBSUP[k].1, SUBSUP[k].2))
}

pub fn mathalpha_latex(c: char) -> Option<&'static str> {
    MATHALPHA.binary_search_by_key(&c, |&(k, _)| k).ok().map(|k| MATHALPHA[k].1)
}

pub fn mathalpha_raises(c: char) -> bool {
    MATHALPHA_RAISES.contains(&c)
}

pub fn precomposed(c: char) -> Option<(char, char)> {
    PRECOMPOSED.binary_search_by_key(&c, |&(k, _, _)| k).ok().map(|k| (PRECOMPOSED[k].1, PRECOMPOSED[k].2))
}

pub fn house_label(s: &str) -> bool {
    HOUSE_LABELS.contains(&s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ascii_fast_path_matches_table() {
        for o in 0u32..0x80 {
            let c = char::from_u32(o).unwrap();
            let t = FLAG_RANGES.iter().find(|&&(a, b, _)| a <= o && o <= b).map_or(0, |r| r.2);
            assert_eq!(ascii_flags(c), t, "U+{o:04X}");
        }
    }
}
