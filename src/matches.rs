use alphanumeric::{CharType, StripCharacters};

/// Regex-free matcher methods for common use cases
/// There are no plain and _cs-suffixed variants because the standard
/// starts_with(pat: &str), contains(pat: &str) and ends_with(pat: &str) methods meet those needs
pub trait SimpleMatch {
    /// Matches the whole string in case-insensitive mode
    fn equals_ci(&self, pattern: &str) -> bool;

    /// Matches the the plain Latin letters [a-z] and numerals [0=9] in the string in case-insensitive mode
    fn equals_ci_alphanum(&self, pattern: &str) -> bool;

    /// Starts with a pattern in case-insensitive mode
    fn starts_with_ci(&self, pattern: &str) -> bool;

    /// Starts with a case-insensitive alphanumeric sequence, ignoring any other characters in the pattern or sample string
    fn starts_with_ci_alphanum(&self, pattern: &str) -> bool;

    /// Ends with a pattern in case-insensitive mode
    fn ends_with_ci(&self, pattern: &str) -> bool;

    /// Ends with a case-insensitive alphanumeric sequence, ignoring any other characters in the pattern or sample string
    fn ends_with_ci_alphanum(&self, pattern: &str) -> bool;

    /// Contains a pattern in case-insensitive mode
    fn contains_ci(&self, pattern: &str) -> bool;

    /// Contains a case-insensitive alphanumeric sequence, ignoring any other characters in the pattern or sample string
    fn contains_ci_alphanum(&self, pattern: &str) -> bool;
}

impl<T: AsRef<str>> SimpleMatch for T {
    fn equals_ci(&self, pattern: &str) -> bool {
        self.as_ref().to_lowercase() == pattern.to_lowercase()
    }

    fn equals_ci_alphanum(&self, pattern: &str) -> bool {
        self.as_ref().to_lowercase().strip_non_alphanum()
            == pattern.to_lowercase().strip_non_alphanum()
    }

    fn starts_with_ci(&self, pattern: &str) -> bool {
        self.as_ref()
            .to_lowercase()
            .starts_with(&pattern.to_lowercase())
    }

    fn starts_with_ci_alphanum(&self, pattern: &str) -> bool {
        self.as_ref()
            .to_lowercase()
            .strip_non_alphanum()
            .starts_with(&pattern.to_lowercase())
    }

    fn ends_with_ci(&self, pattern: &str) -> bool {
        self.as_ref()
            .to_lowercase()
            .ends_with(&pattern.to_lowercase())
    }

    fn ends_with_ci_alphanum(&self, pattern: &str) -> bool {
        self.as_ref()
            .to_lowercase()
            .strip_non_alphanum()
            .ends_with(&pattern.to_lowercase())
    }

    fn contains_ci(&self, pattern: &str) -> bool {
        self.as_ref()
            .to_lowercase()
            .contains(&pattern.to_lowercase())
    }

    fn contains_ci_alphanum(&self, pattern: &str) -> bool {
        self.as_ref()
            .to_lowercase()
            .strip_non_alphanum()
            .contains(&pattern.to_lowercase())
    }
}

/// Return the indices of all ocurrences of a string
pub trait MatchOccurrences {
    /// Return the indices only of all matches of a given string pattern (not a regular expression)
    /// Builds on match_indices in the Rust standard library
    fn find_matched_indices(&self, pat: &str) -> Vec<usize>;

    /// Match occurrences of a single character
    fn find_char_indices(&self, pat: char) -> Vec<usize>;
}

impl<T: AsRef<str>> MatchOccurrences for T {
    fn find_matched_indices(&self, pat: &str) -> Vec<usize> {
        self.as_ref()
            .match_indices(pat)
            .into_iter()
            .map(|pair| pair.0)
            .collect::<Vec<usize>>()
    }

    fn find_char_indices(&self, pat: char) -> Vec<usize> {
        self.as_ref()
            .match_indices(pat)
            .into_iter()
            .map(|pair| pair.0)
            .collect::<Vec<usize>>()
    }
}

/// Test if character set (CharType) is in the string
pub trait SimplContainsType
where
    Self: SimpleMatch,
{
    /// contains characters in the specified set
    fn contains_type(&self, char_type: CharType) -> bool;

    /// contains characters in the specified sets
    fn contains_types(&self, char_types: &[CharType]) -> bool;

    /// starts with one or more characters in the specified set
    fn starts_with_type(&self, char_type: CharType) -> bool;

    /// starts with one or more characters in the specified set
    fn starts_with_types(&self, char_types: &[CharType]) -> bool;

    /// ends with one or more characters in the specified sets
    fn ends_with_type(&self, char_type: CharType) -> bool;

    /// ends with one or more characters in the specified sets
    fn ends_with_types(&self, char_types: &[CharType]) -> bool;
}

impl<T: AsRef<str>> SimplContainsType for T {
    fn contains_type(&self, char_type: CharType) -> bool {
        self.as_ref().chars().any(|ch| char_type.is_in_range(&ch))
    }

    fn contains_types(&self, char_types: &[CharType]) -> bool {
        self.as_ref()
            .chars()
            .any(|ch| char_types.into_iter().any(|ct| ct.is_in_range(&ch)))
    }

    fn starts_with_type(&self, char_type: CharType) -> bool {
        if let Some(first) = self.as_ref().chars().nth(0) {
            char_type.is_in_range(&first)
        } else {
            false
        }
    }

    fn starts_with_types(&self, char_types: &[CharType]) -> bool {
        if let Some(first) = self.as_ref().chars().nth(0) {
            char_types.into_iter().any(|ct| ct.is_in_range(&first))
        } else {
            false
        }
    }

    fn ends_with_type(&self, char_type: CharType) -> bool {
        if let Some(first) = self.as_ref().chars().last() {
            char_type.is_in_range(&first)
        } else {
            false
        }
    }

    fn ends_with_types(&self, char_types: &[CharType]) -> bool {
        if let Some(first) = self.as_ref().chars().last() {
            char_types.into_iter().any(|ct| ct.is_in_range(&first))
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equals_ci() {
        assert!("Hello World".equals_ci("hello world"));
        assert!("RUST".equals_ci("rust"));
        assert!(!"Hello".equals_ci("World"));
        assert!("".equals_ci(""));
    }

    #[test]
    fn test_equals_ci_alphanum() {
        assert!("Start-Up".equals_ci_alphanum("startup"));
        assert!("hello---world".equals_ci_alphanum("HELLOWORLD"));
        assert!(!"abc".equals_ci_alphanum("xyz"));
    }

    #[test]
    fn test_starts_with_ci() {
        assert!("Hello World".starts_with_ci("hello"));
        assert!("Hello World".starts_with_ci("HELLO"));
        assert!(!"Hello World".starts_with_ci("world"));
        assert!("Hello".starts_with_ci(""));
    }

    #[test]
    fn test_starts_with_ci_alphanum() {
        assert!("_Cat-image".starts_with_ci_alphanum("cat"));
        assert!("---Hello!!!".starts_with_ci_alphanum("hello"));
        assert!(!"Dog-photo".starts_with_ci_alphanum("cat"));
    }

    #[test]
    fn test_ends_with_ci() {
        assert!("Hello World".ends_with_ci("WORLD"));
        assert!("photo.JPG".ends_with_ci(".jpg"));
        assert!(!"photo.png".ends_with_ci(".jpg"));
        assert!("Hello".ends_with_ci(""));
    }

    #[test]
    fn test_ends_with_ci_alphanum() {
        assert!("photo-file.JPG!!!".ends_with_ci_alphanum("jpg"));
        assert!("data---CSV".ends_with_ci_alphanum("csv"));
        assert!(!"photo.png".ends_with_ci_alphanum("jpg"));
    }

    #[test]
    fn test_contains_ci() {
        assert!("Hello World".contains_ci("LO WO"));
        assert!("ABCDEF".contains_ci("cde"));
        assert!(!"Hello".contains_ci("xyz"));
        assert!("Hello".contains_ci(""));
    }

    #[test]
    fn test_contains_ci_alphanum() {
        assert!("He--llo Wo!!rld".contains_ci_alphanum("lloworld"));
        assert!("a-b-c-d".contains_ci_alphanum("ABCD"));
        assert!(!"Hello".contains_ci_alphanum("xyz"));
    }

    #[test]
    fn test_find_matched_indices() {
        assert_eq!("abcabc".find_matched_indices("bc"), vec![1, 4]);
        assert_eq!("hello".find_matched_indices("ll"), vec![2]);
        assert_eq!("hello".find_matched_indices("xyz"), vec![]);
        assert_eq!("aaa".find_matched_indices("a"), vec![0, 1, 2]);
    }

    #[test]
    fn test_find_char_indices() {
        assert_eq!("a.b.c".find_char_indices('.'), vec![1, 3]);
        assert_eq!("hello".find_char_indices('l'), vec![2, 3]);
        assert_eq!("hello".find_char_indices('z'), vec![]);
    }

    #[test]
    fn test_contains_type() {
        assert!("abc123".contains_type(CharType::DecDigit));
        assert!(!"abcdef".contains_type(CharType::DecDigit));
        assert!("Hello World".contains_type(CharType::Spaces));
        assert!(!"HelloWorld".contains_type(CharType::Spaces));
        assert!("hello!".contains_type(CharType::Punctuation));
    }

    #[test]
    fn test_contains_types() {
        assert!("abc".contains_types(&[CharType::DecDigit, CharType::Alpha]));
        assert!(!"123".contains_types(&[CharType::Alpha, CharType::Spaces]));
        assert!("hello world".contains_types(&[CharType::Spaces, CharType::Upper]));
    }

    #[test]
    fn test_starts_with_type() {
        assert!("123abc".starts_with_type(CharType::DecDigit));
        assert!(!"abc123".starts_with_type(CharType::DecDigit));
        assert!("Hello".starts_with_type(CharType::Upper));
        assert!(!"hello".starts_with_type(CharType::Upper));
        assert!(!("".starts_with_type(CharType::Alpha)));
    }

    #[test]
    fn test_starts_with_types() {
        assert!("Hello".starts_with_types(&[CharType::Upper, CharType::DecDigit]));
        assert!("9lives".starts_with_types(&[CharType::Upper, CharType::DecDigit]));
        assert!(!"hello".starts_with_types(&[CharType::Upper, CharType::DecDigit]));
        assert!(!("".starts_with_types(&[CharType::Alpha])));
    }

    #[test]
    fn test_ends_with_type() {
        assert!("abc9".ends_with_type(CharType::DecDigit));
        assert!(!"abc".ends_with_type(CharType::DecDigit));
        assert!("hellO".ends_with_type(CharType::Upper));
        assert!(!("".ends_with_type(CharType::Alpha)));
    }

    #[test]
    fn test_ends_with_types() {
        assert!("hello!".ends_with_types(&[CharType::Punctuation, CharType::DecDigit]));
        assert!("test9".ends_with_types(&[CharType::Punctuation, CharType::DecDigit]));
        assert!(!"hello".ends_with_types(&[CharType::Punctuation, CharType::DecDigit]));
        assert!(!("".ends_with_types(&[CharType::Alpha])));
    }
}
