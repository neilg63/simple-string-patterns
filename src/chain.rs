use crate::SimpleMatch;

/// A lazily-combined boolean match expression over a single string.
/// Lets you compose several simple pattern checks in one fluent, left-to-right
/// readable chain instead of nesting boolean expressions, e.g.
/// ```
/// use simple_string_patterns::*;
///
/// let subject = "Floating-point error at line 667";
/// let is_float_error = subject.match_contains_ci("error")
///   .and_starts_with_ci_alphanum("float")
///   .result();
/// assert!(is_float_error);
/// ```
/// Each `and_*`/`or_*` method re-tests the same subject string, so there is no need to repeat it.
#[derive(Debug, Clone, Copy)]
pub struct StringMatch<'a> {
    subject: &'a str,
    matched: bool,
}

impl<'a> StringMatch<'a> {
    fn new(subject: &'a str, matched: bool) -> Self {
        StringMatch { subject, matched }
    }

    /// Returns the accumulated boolean result of the chain so far
    pub fn result(&self) -> bool {
        self.matched
    }

    /// The subject string this match chain was built from
    pub fn subject(&self) -> &'a str {
        self.subject
    }

    // ---- and_* (all conditions so far and this one must match) ----

    pub fn and_contains_ci(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject.contains_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_contains_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject.contains_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_contains_cs(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject.contains(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_starts_with_ci(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject.starts_with_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_starts_with_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject.starts_with_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_starts_with_cs(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject.starts_with(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_ends_with_ci(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject.ends_with_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_ends_with_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject.ends_with_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_ends_with_cs(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject.ends_with(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_equals_ci(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject.equals_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_equals_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject.equals_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_equals_cs(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject == pattern;
        Self::new(self.subject, matched)
    }

    // ---- and_not_* (all conditions so far must match and this one must not) ----

    pub fn and_not_contains_ci(self, pattern: &str) -> Self {
        let matched = self.matched && !self.subject.contains_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_not_contains_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched && !self.subject.contains_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_not_contains_cs(self, pattern: &str) -> Self {
        let matched = self.matched && !self.subject.contains(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_not_starts_with_ci(self, pattern: &str) -> Self {
        let matched = self.matched && !self.subject.starts_with_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_not_starts_with_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched && !self.subject.starts_with_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_not_starts_with_cs(self, pattern: &str) -> Self {
        let matched = self.matched && !self.subject.starts_with(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_not_ends_with_ci(self, pattern: &str) -> Self {
        let matched = self.matched && !self.subject.ends_with_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_not_ends_with_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched && !self.subject.ends_with_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_not_ends_with_cs(self, pattern: &str) -> Self {
        let matched = self.matched && !self.subject.ends_with(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_not_equals_ci(self, pattern: &str) -> Self {
        let matched = self.matched && !self.subject.equals_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_not_equals_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched && !self.subject.equals_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn and_not_equals_cs(self, pattern: &str) -> Self {
        let matched = self.matched && self.subject != pattern;
        Self::new(self.subject, matched)
    }

    // ---- or_* (either the conditions so far or this one may match) ----

    pub fn or_contains_ci(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject.contains_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn or_contains_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject.contains_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn or_contains_cs(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject.contains(pattern);
        Self::new(self.subject, matched)
    }

    pub fn or_starts_with_ci(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject.starts_with_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn or_starts_with_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject.starts_with_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn or_starts_with_cs(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject.starts_with(pattern);
        Self::new(self.subject, matched)
    }

    pub fn or_ends_with_ci(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject.ends_with_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn or_ends_with_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject.ends_with_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn or_ends_with_cs(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject.ends_with(pattern);
        Self::new(self.subject, matched)
    }

    pub fn or_equals_ci(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject.equals_ci(pattern);
        Self::new(self.subject, matched)
    }

    pub fn or_equals_ci_alphanum(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject.equals_ci_alphanum(pattern);
        Self::new(self.subject, matched)
    }

    pub fn or_equals_cs(self, pattern: &str) -> Self {
        let matched = self.matched || self.subject == pattern;
        Self::new(self.subject, matched)
    }
}

impl<'a> From<StringMatch<'a>> for bool {
    fn from(m: StringMatch<'a>) -> bool {
        m.matched
    }
}

impl<'a> PartialEq<bool> for StringMatch<'a> {
    fn eq(&self, other: &bool) -> bool {
        self.matched == *other
    }
}

/// Entry points that start a fluent [`StringMatch`] chain from a single simple pattern check
pub trait ToStringMatch {
    /// Start a chain testing if the string contains a pattern in case-insensitive mode
    fn match_contains_ci<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
    /// Start a chain testing if the string contains a case-insensitive alphanumeric sequence
    fn match_contains_ci_alphanum<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
    /// Start a chain testing if the string contains a pattern in case-sensitive mode
    fn match_contains_cs<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
    /// Start a chain testing if the string starts with a pattern in case-insensitive mode
    fn match_starts_with_ci<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
    /// Start a chain testing if the string starts with a case-insensitive alphanumeric sequence
    fn match_starts_with_ci_alphanum<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
    /// Start a chain testing if the string starts with a pattern in case-sensitive mode
    fn match_starts_with_cs<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
    /// Start a chain testing if the string ends with a pattern in case-insensitive mode
    fn match_ends_with_ci<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
    /// Start a chain testing if the string ends with a case-insensitive alphanumeric sequence
    fn match_ends_with_ci_alphanum<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
    /// Start a chain testing if the string ends with a pattern in case-sensitive mode
    fn match_ends_with_cs<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
    /// Start a chain testing if the whole string matches a pattern in case-insensitive mode
    fn match_equals_ci<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
    /// Start a chain testing if the whole string matches a case-insensitive alphanumeric sequence
    fn match_equals_ci_alphanum<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
    /// Start a chain testing if the whole string matches a pattern in case-sensitive mode
    fn match_equals_cs<'a>(&'a self, pattern: &str) -> StringMatch<'a>;
}

impl<T: AsRef<str>> ToStringMatch for T {
    fn match_contains_ci<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s.contains_ci(pattern))
    }

    fn match_contains_ci_alphanum<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s.contains_ci_alphanum(pattern))
    }

    fn match_contains_cs<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s.contains(pattern))
    }

    fn match_starts_with_ci<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s.starts_with_ci(pattern))
    }

    fn match_starts_with_ci_alphanum<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s.starts_with_ci_alphanum(pattern))
    }

    fn match_starts_with_cs<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s.starts_with(pattern))
    }

    fn match_ends_with_ci<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s.ends_with_ci(pattern))
    }

    fn match_ends_with_ci_alphanum<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s.ends_with_ci_alphanum(pattern))
    }

    fn match_ends_with_cs<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s.ends_with(pattern))
    }

    fn match_equals_ci<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s.equals_ci(pattern))
    }

    fn match_equals_ci_alphanum<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s.equals_ci_alphanum(pattern))
    }

    fn match_equals_cs<'a>(&'a self, pattern: &str) -> StringMatch<'a> {
        let s = self.as_ref();
        StringMatch::new(s, s == pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_chain() {
        let sample_1 = "Floating-point error at line 667";
        let sample_2 = "[floating-point] error in line 999";
        let sample_3 = "Rounding error at line 12";

        assert!(sample_1
            .match_contains_ci("error")
            .and_starts_with_ci_alphanum("float")
            .result());
        assert!(sample_2
            .match_contains_ci("error")
            .and_starts_with_ci_alphanum("float")
            .result());
        assert!(!sample_3
            .match_contains_ci("error")
            .and_starts_with_ci_alphanum("float")
            .result());
    }

    #[test]
    fn test_and_not_chain() {
        let sample_1 = "cat-picture.jpg";
        let sample_2 = "cat-picture.psd";

        assert!(sample_1
            .match_starts_with_ci("cat")
            .and_not_ends_with_ci(".psd")
            .result());
        assert!(!sample_2
            .match_starts_with_ci("cat")
            .and_not_ends_with_ci(".psd")
            .result());
    }

    #[test]
    fn test_or_chain() {
        let sample_1 = "cat-picture.jpg";
        let sample_2 = "dog-picture.jpg";
        let sample_3 = "rabbit-picture.jpg";

        let is_cat_or_dog = |s: &str| {
            s.match_starts_with_ci("cat")
                .or_starts_with_ci("dog")
                .result()
        };
        assert!(is_cat_or_dog(sample_1));
        assert!(is_cat_or_dog(sample_2));
        assert!(!is_cat_or_dog(sample_3));
    }

    #[test]
    fn test_result_conversions() {
        let sample = "Start-up";
        let chain = sample.match_equals_ci_alphanum("startup");
        assert!(chain.result());
        assert_eq!(chain, true);
        let as_bool: bool = chain.into();
        assert!(as_bool);
    }
}
