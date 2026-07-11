//! Internal, allocation-minimal implementations backing `SimpleMatch`'s `_ci`/`_ci_alphanum`
//! methods. Not part of the public API: `SimpleMatch`'s method names and `bool` return types
//! are the stable surface; this module is free to change.
//!
//! Case-insensitive comparisons use a byte-level ASCII fast path when both the subject and
//! pattern are ASCII (the common case for log lines, filenames, slugs, etc.), falling back to
//! full Unicode-aware case folding (matching `str::to_lowercase()`) otherwise. Either way, only
//! the pattern (typically short) is ever materialized into a small buffer; the subject is
//! scanned lazily and comparisons stop as soon as the outcome is known, so this never allocates
//! or scans proportionally to the *subject's* length the way `.to_lowercase().contains(...)` does.
//!
//! Note: for the extremely rare Unicode characters whose lowercase form expands to more than
//! one character (e.g. Turkish İ), the `_ends_with` functions compare those expansions in
//! per-original-character order rather than fully-expanded order; `_eq`, `_starts_with` and
//! `_contains` are unaffected. This only applies on the (already rare) non-ASCII fallback path.

use alphanumeric::StripCharacters;
use std::collections::VecDeque;

// ---------------------------------------------------------------------------
// Unicode-correct character streams (used when either string has non-ASCII bytes)
// ---------------------------------------------------------------------------

fn lower_chars(s: &str) -> impl Iterator<Item = char> + '_ {
    s.chars().flat_map(char::to_lowercase)
}

// Note: deliberately not a named `fn ... -> impl Iterator + '_` wrapper like `lower_chars`
// above — a free function whose own return type is `impl Trait` and that internally calls
// a trait method whose return type is *also* `impl Trait` (RPITIT, like `alphanumeric_chars`
// below) hits a rustc limitation composing the two opaque types (E0515, "cannot return value
// referencing function parameter"), even though the equivalent expression compiles fine
// inline or passed straight into a generic function. So this stays inlined at each call site.

fn starts_with_iter<I: Iterator<Item = char>>(mut subject: I, needle: &[char]) -> bool {
    for &nc in needle {
        match subject.next() {
            Some(sc) if sc == nc => continue,
            _ => return false,
        }
    }
    true
}

fn ends_with_iter<I: Iterator<Item = char>>(mut subject_rev: I, needle: &[char]) -> bool {
    for &nc in needle.iter().rev() {
        match subject_rev.next() {
            Some(sc) if sc == nc => continue,
            _ => return false,
        }
    }
    true
}

fn eq_iter<I: Iterator<Item = char>>(mut subject: I, needle: &[char]) -> bool {
    for &nc in needle {
        match subject.next() {
            Some(sc) if sc == nc => continue,
            _ => return false,
        }
    }
    subject.next().is_none()
}

fn contains_iter<I: Iterator<Item = char>>(subject: I, needle: &[char]) -> bool {
    if needle.is_empty() {
        return true;
    }
    let first = needle[0];
    let last = needle[needle.len() - 1];
    let mut window: VecDeque<char> = VecDeque::with_capacity(needle.len());
    for c in subject {
        window.push_back(c);
        if window.len() > needle.len() {
            window.pop_front();
        }
        if window.len() == needle.len()
            && window[0] == first
            && window[window.len() - 1] == last
            && window.iter().eq(needle.iter())
        {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// ASCII fast path (raw bytes: no UTF-8 decode, no Unicode case-folding tables)
// ---------------------------------------------------------------------------

fn eq_ascii(subject: &[u8], needle: &[u8]) -> bool {
    subject.eq_ignore_ascii_case(needle)
}

fn starts_with_ascii(subject: &[u8], needle: &[u8]) -> bool {
    needle.len() <= subject.len() && subject[..needle.len()].eq_ignore_ascii_case(needle)
}

fn ends_with_ascii(subject: &[u8], needle: &[u8]) -> bool {
    needle.len() <= subject.len()
        && subject[subject.len() - needle.len()..].eq_ignore_ascii_case(needle)
}

fn contains_ascii(subject: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > subject.len() {
        return false;
    }
    let first = needle[0].to_ascii_lowercase();
    let last = needle[needle.len() - 1].to_ascii_lowercase();
    for start in 0..=(subject.len() - needle.len()) {
        let window = &subject[start..start + needle.len()];
        if window[0].to_ascii_lowercase() == first
            && window[window.len() - 1].to_ascii_lowercase() == last
            && window.eq_ignore_ascii_case(needle)
        {
            return true;
        }
    }
    false
}

fn ascii_alphanum_lower_bytes(bytes: &[u8]) -> impl Iterator<Item = u8> + '_ {
    bytes
        .iter()
        .copied()
        .filter(|b| b.is_ascii_alphanumeric())
        .map(|b| b.to_ascii_lowercase())
}

fn starts_with_byte_iter<I: Iterator<Item = u8>>(mut subject: I, needle: &[u8]) -> bool {
    for &nb in needle {
        match subject.next() {
            Some(sb) if sb == nb => continue,
            _ => return false,
        }
    }
    true
}

fn ends_with_byte_iter<I: Iterator<Item = u8>>(mut subject_rev: I, needle: &[u8]) -> bool {
    for &nb in needle.iter().rev() {
        match subject_rev.next() {
            Some(sb) if sb == nb => continue,
            _ => return false,
        }
    }
    true
}

fn eq_byte_iter<I: Iterator<Item = u8>>(mut subject: I, needle: &[u8]) -> bool {
    for &nb in needle {
        match subject.next() {
            Some(sb) if sb == nb => continue,
            _ => return false,
        }
    }
    subject.next().is_none()
}

fn contains_byte_iter<I: Iterator<Item = u8>>(subject: I, needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    let first = needle[0];
    let last = needle[needle.len() - 1];
    let mut window: VecDeque<u8> = VecDeque::with_capacity(needle.len());
    for b in subject {
        window.push_back(b);
        if window.len() > needle.len() {
            window.pop_front();
        }
        if window.len() == needle.len()
            && window[0] == first
            && window[window.len() - 1] == last
            && window.iter().eq(needle.iter())
        {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Dispatch: ASCII fast path when both strings are ASCII, else Unicode fallback.
// ---------------------------------------------------------------------------

pub(crate) fn ci_eq(subject: &str, pattern: &str) -> bool {
    if subject.is_ascii() && pattern.is_ascii() {
        eq_ascii(subject.as_bytes(), pattern.as_bytes())
    } else {
        let needle: Vec<char> = lower_chars(pattern).collect();
        eq_iter(lower_chars(subject), &needle)
    }
}

pub(crate) fn ci_starts_with(subject: &str, pattern: &str) -> bool {
    if subject.is_ascii() && pattern.is_ascii() {
        starts_with_ascii(subject.as_bytes(), pattern.as_bytes())
    } else {
        let needle: Vec<char> = lower_chars(pattern).collect();
        starts_with_iter(lower_chars(subject), &needle)
    }
}

pub(crate) fn ci_ends_with(subject: &str, pattern: &str) -> bool {
    if subject.is_ascii() && pattern.is_ascii() {
        ends_with_ascii(subject.as_bytes(), pattern.as_bytes())
    } else {
        let needle: Vec<char> = lower_chars(pattern).collect();
        ends_with_iter(subject.chars().rev().flat_map(char::to_lowercase), &needle)
    }
}

pub(crate) fn ci_contains(subject: &str, pattern: &str) -> bool {
    if subject.is_ascii() && pattern.is_ascii() {
        contains_ascii(subject.as_bytes(), pattern.as_bytes())
    } else {
        let needle: Vec<char> = lower_chars(pattern).collect();
        contains_iter(lower_chars(subject), &needle)
    }
}

pub(crate) fn ci_alphanum_eq(subject: &str, pattern: &str) -> bool {
    if subject.is_ascii() && pattern.is_ascii() {
        let needle: Vec<u8> = ascii_alphanum_lower_bytes(pattern.as_bytes()).collect();
        eq_byte_iter(ascii_alphanum_lower_bytes(subject.as_bytes()), &needle)
    } else {
        let needle: Vec<char> = pattern.alphanumeric_chars().flat_map(char::to_lowercase).collect();
        eq_iter(subject.alphanumeric_chars().flat_map(char::to_lowercase), &needle)
    }
}

pub(crate) fn ci_alphanum_starts_with(subject: &str, pattern: &str) -> bool {
    if subject.is_ascii() && pattern.is_ascii() {
        let needle: Vec<u8> = ascii_alphanum_lower_bytes(pattern.as_bytes()).collect();
        starts_with_byte_iter(ascii_alphanum_lower_bytes(subject.as_bytes()), &needle)
    } else {
        let needle: Vec<char> = pattern.alphanumeric_chars().flat_map(char::to_lowercase).collect();
        starts_with_iter(subject.alphanumeric_chars().flat_map(char::to_lowercase), &needle)
    }
}

pub(crate) fn ci_alphanum_ends_with(subject: &str, pattern: &str) -> bool {
    if subject.is_ascii() && pattern.is_ascii() {
        let needle: Vec<u8> = ascii_alphanum_lower_bytes(pattern.as_bytes()).collect();
        ends_with_byte_iter(
            subject
                .as_bytes()
                .iter()
                .rev()
                .copied()
                .filter(|b| b.is_ascii_alphanumeric())
                .map(|b| b.to_ascii_lowercase()),
            &needle,
        )
    } else {
        let needle: Vec<char> = pattern.alphanumeric_chars().flat_map(char::to_lowercase).collect();
        ends_with_iter(
            subject.alphanumeric_chars().rev().flat_map(char::to_lowercase),
            &needle,
        )
    }
}

pub(crate) fn ci_alphanum_contains(subject: &str, pattern: &str) -> bool {
    if subject.is_ascii() && pattern.is_ascii() {
        let needle: Vec<u8> = ascii_alphanum_lower_bytes(pattern.as_bytes()).collect();
        contains_byte_iter(ascii_alphanum_lower_bytes(subject.as_bytes()), &needle)
    } else {
        let needle: Vec<char> = pattern.alphanumeric_chars().flat_map(char::to_lowercase).collect();
        contains_iter(subject.alphanumeric_chars().flat_map(char::to_lowercase), &needle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Oracle: the original allocate-then-compare formula, corrected to strip
    // non-alphanumeric characters from BOTH sides for the alphanum variants
    // (the old code only stripped the subject and left the pattern merely
    // lower-cased, which was an inconsistency with equals_ci_alphanum).
    fn oracle_eq(s: &str, p: &str) -> bool {
        s.to_lowercase() == p.to_lowercase()
    }
    fn oracle_starts_with(s: &str, p: &str) -> bool {
        s.to_lowercase().starts_with(&p.to_lowercase())
    }
    fn oracle_ends_with(s: &str, p: &str) -> bool {
        s.to_lowercase().ends_with(&p.to_lowercase())
    }
    fn oracle_contains(s: &str, p: &str) -> bool {
        s.to_lowercase().contains(&p.to_lowercase())
    }
    fn strip_non_alphanum(s: &str) -> String {
        s.chars().filter(|c| c.is_alphanumeric()).collect()
    }
    fn oracle_alphanum_eq(s: &str, p: &str) -> bool {
        strip_non_alphanum(&s.to_lowercase()) == strip_non_alphanum(&p.to_lowercase())
    }
    fn oracle_alphanum_starts_with(s: &str, p: &str) -> bool {
        strip_non_alphanum(&s.to_lowercase()).starts_with(&strip_non_alphanum(&p.to_lowercase()))
    }
    fn oracle_alphanum_ends_with(s: &str, p: &str) -> bool {
        strip_non_alphanum(&s.to_lowercase()).ends_with(&strip_non_alphanum(&p.to_lowercase()))
    }
    fn oracle_alphanum_contains(s: &str, p: &str) -> bool {
        strip_non_alphanum(&s.to_lowercase()).contains(&strip_non_alphanum(&p.to_lowercase()))
    }

    const PAIRS: &[(&str, &str)] = &[
        ("Hello World", "hello world"),
        ("Hello World", "HELLO"),
        ("Hello World", "world"),
        ("Hello World", "xyz"),
        ("", ""),
        ("", "x"),
        ("x", ""),
        ("Start-Up", "startup"),
        ("hello---world", "HELLOWORLD"),
        ("_Cat-image.JPG", "cat"),
        ("_Cat-image.JPG", "jpg"),
        ("photo-file.JPG!!!", "jpg"),
        ("Café Münster", "CAFÉ MÜNSTER"),
        ("Café Münster", "cafemunster"),
        ("Café", "café"),
        ("naïve", "NAIVE"),
        ("abcabc", "bc"),
        ("aaa", "a"),
        ("floating-point error at line 667", "float"),
        ("[floating-point] error in line 999", "float"),
        ("rounding error at line 12", "float"),
    ];

    #[test]
    fn matches_oracle_for_all_pairs() {
        for &(s, p) in PAIRS {
            assert_eq!(ci_eq(s, p), oracle_eq(s, p), "ci_eq({s:?}, {p:?})");
            assert_eq!(
                ci_starts_with(s, p),
                oracle_starts_with(s, p),
                "ci_starts_with({s:?}, {p:?})"
            );
            assert_eq!(
                ci_ends_with(s, p),
                oracle_ends_with(s, p),
                "ci_ends_with({s:?}, {p:?})"
            );
            assert_eq!(
                ci_contains(s, p),
                oracle_contains(s, p),
                "ci_contains({s:?}, {p:?})"
            );
            assert_eq!(
                ci_alphanum_eq(s, p),
                oracle_alphanum_eq(s, p),
                "ci_alphanum_eq({s:?}, {p:?})"
            );
            assert_eq!(
                ci_alphanum_starts_with(s, p),
                oracle_alphanum_starts_with(s, p),
                "ci_alphanum_starts_with({s:?}, {p:?})"
            );
            assert_eq!(
                ci_alphanum_ends_with(s, p),
                oracle_alphanum_ends_with(s, p),
                "ci_alphanum_ends_with({s:?}, {p:?})"
            );
            assert_eq!(
                ci_alphanum_contains(s, p),
                oracle_alphanum_contains(s, p),
                "ci_alphanum_contains({s:?}, {p:?})"
            );
        }
    }

    #[test]
    fn ascii_pattern_against_unicode_subject_uses_fallback_path() {
        // A non-ASCII subject forces the Unicode fallback even for an ASCII pattern.
        assert!(!ci_starts_with("Café Münster", "cafe")); // é != e
        assert!(!ci_alphanum_starts_with("Café Münster", "cafemunster")); // é != e, ü != u
        assert!(ci_starts_with("Café Münster", "café"));
    }
}
