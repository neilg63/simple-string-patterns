use crate::{enums::StringBounds, utils::{pairs_to_string_bounds, strs_to_string_bounds}, BoundsBuilder, BoundsPosition, CaseMatchMode, CharType, StripCharacters};

/// Regex-free matcher methods for common use cases
/// There are no plain and _cs-suffixed variants because the standard
/// starts_with(pat: &str), contains(pat: &str) and ends_with(pat: &str) methods meet those needs
pub trait SimpleMatch {

  /// Matches the whole string in case-insensitive mode
  fn equals_ci(&self, pattern: &str) -> bool;

  /// Matches the the plain Latin letters [a-z] and numerals [0=9] in the string in case-insensitive mode
  fn equals_ci_alphanum(&self, pattern: &str) -> bool;

  /// Starts with a case-insensitive alphanumeric sequence
  fn starts_with_ci(&self, pattern: &str) -> bool;
  
  /// Starts with a case-insensitive alphanumeric sequence
  fn starts_with_ci_alphanum(&self, pattern: &str) -> bool;
  
  /// Ends with a case-insensitive alphanumeric sequence
  fn ends_with_ci(&self, pattern: &str) -> bool;
  
  /// Ends with a case-insensitive alphanumeric sequence
  fn ends_with_ci_alphanum(&self, pattern: &str) -> bool;

  /// Contains a case-insensitive alphanumeric sequence
  fn contains_ci(&self, pattern: &str) -> bool;
  
  /// Contains a case-insensitive alphanumeric sequence
  fn contains_ci_alphanum(&self, pattern: &str) -> bool;
}

/// Implementation for &str/String 
impl SimpleMatch for str {

   /// Starts with a case-insensitive sequence
  fn equals_ci(&self, pattern: &str) -> bool {
    self.to_lowercase() == pattern.to_lowercase()
  }
  
  /// Starts with a case-insensitive alphanumeric sequence
  fn equals_ci_alphanum(&self, pattern: &str) -> bool {
    self.to_lowercase().strip_non_alphanum() ==  pattern.to_lowercase().strip_non_alphanum()
  }

  /// Starts with a case-insensitive sequence
  fn starts_with_ci(&self, pattern: &str) -> bool {
    self.to_lowercase().starts_with(&pattern.to_lowercase())
  }
  
  /// Starts with a case-insensitive alphanumeric sequence
  fn starts_with_ci_alphanum(&self, pattern: &str) -> bool {
    self.to_lowercase().strip_non_alphanum().starts_with(&pattern.to_lowercase())
  }
  
  /// Ends with a case-insensitive sequence
  fn ends_with_ci(&self, pattern: &str) -> bool {
    self.to_lowercase().ends_with(&pattern.to_lowercase())
  }
  
  /// Ends with a case-insensitive alphanumeric sequence
  fn ends_with_ci_alphanum(&self, pattern: &str) -> bool {
    self.to_lowercase().strip_non_alphanum().ends_with(&pattern.to_lowercase())
  }

  /// Contains a case-insensitive sequence
  fn contains_ci(&self, pattern: &str) -> bool {
    self.to_lowercase().contains(&pattern.to_lowercase())
  }
  
  /// Contains a case-insensitive alphanumeric sequence
  fn contains_ci_alphanum(&self, pattern: &str) -> bool {
    self.to_lowercase().strip_non_alphanum().contains(&pattern.to_lowercase())
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


impl MatchOccurrences for str {
    /// Return the indices only of all matches of a given regular expression
  fn find_matched_indices(&self, pat: &str) -> Vec<usize> {
    self.match_indices(pat).into_iter().map(|pair| pair.0).collect::<Vec<usize>>()
  }

  /// As above, but with a character to avoid coercion
  fn find_char_indices(&self, pat: char) -> Vec<usize> {
    self.match_indices(pat).into_iter().map(|pair| pair.0).collect::<Vec<usize>>()
  }
}


/// Test multiple patterns and return vector of booleans with the results for each item
pub trait SimpleMatchesMany where Self:SimpleMatch {

  /// test for multiple conditions. All other trait methods are derived from this
  fn matched_conditional(&self, pattern_sets: &[StringBounds]) -> Vec<bool>;

  /// test for multiple conditions with simple tuple pairs of pattern + case-insenitive flag
  fn contains_conditional(&self, pattern_sets: &[(&str, bool)]) -> Vec<bool> {
    let pattern_sets: Vec<StringBounds> = pairs_to_string_bounds(pattern_sets, BoundsPosition::Contains);
    self.matched_conditional(&pattern_sets)
   }

  /// Test for presecnce of simple patterns in case-insensitive mode
  fn contains_conditional_ci(&self, patterns: &[&str]) -> Vec<bool> {
    let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(patterns, CaseMatchMode::Insensitive, BoundsPosition::Contains);
    self.matched_conditional(&pattern_sets)
  }

  /// Test for presecnce of simple patterns in case-sensitive mode
  fn contains_conditional_cs(&self, patterns: &[&str]) -> Vec<bool> {
    let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(patterns, CaseMatchMode::Sensitive, BoundsPosition::Contains);
    self.matched_conditional(&pattern_sets)
  }
  
}

/*
* Common function to match scalar StringBounds rules
*/
pub(crate) fn match_bounds_rule(txt: &str, item: &StringBounds) -> bool {
  let cm = item.case_mode();
  let ci = item.case_insensitive();
  // cast the sample string to lowercase for case-insenitive matches
  let base = if ci {
    match cm {
      CaseMatchMode::AlphanumInsensitive => txt.to_lowercase().strip_non_alphanum(),
      _ => txt.to_lowercase()
    }
  } else {
    txt.to_owned()
  };
  // cast the simple pattern to lowercase for case-insenitive matches
  let pattern = if ci {
    item.pattern().to_lowercase()
  } else {
    item.pattern().to_owned()
  };
  // check if outcome of starts_with, ends_with or contains test matches the positivity value
  let is_matched = if item.starts_with() {
    base.starts_with(&pattern)
  } else if item.ends_with() {
    base.ends_with(&pattern)
  } else if item.matches_whole() {
    base == pattern
  } else {
    base.contains(&pattern)
  } == item.is_positive();
  is_matched
}

/*
* Common function to match StringBounds rule sets handling  both and/or sub  rules and scalar rules
*/
pub(crate) fn match_bounds_rule_set(txt: &str, item: &StringBounds) -> bool {
  match item {
    StringBounds::And(inner_rules) => txt.matched_conditional(&inner_rules).into_iter().all(|result| result),
    StringBounds::Or(inner_rules) => txt.matched_conditional(&inner_rules).into_iter().any(|result| result),
    _ => match_bounds_rule(txt, item)
  }
}

impl SimpleMatchesMany for str {

  // test for multiple conditions. All other trait methods are derived from this
  fn matched_conditional(&self, pattern_sets: &[StringBounds]) -> Vec<bool> {
    let mut matched_items: Vec<bool> = Vec::with_capacity(pattern_sets.len());
    for item in pattern_sets {
       matched_items.push(match_bounds_rule_set(self, item));
     }
     matched_items
   }
}

/// Test multiple patterns and return boolean
pub trait SimpleMatchAll where Self:SimpleMatchesMany {

  /// test for multiple conditions. All other trait methods are derived from this
  fn match_all_conditional(&self, pattern_sets: &[StringBounds]) -> bool;

  /// test for multiple conditions with simple tuple pairs of pattern + case-insenitive flag
  fn contains_all_conditional(&self, pattern_sets: &[(&str, bool)]) -> bool {
    let pattern_sets: Vec<StringBounds> = pairs_to_string_bounds(pattern_sets, BoundsPosition::Contains);
    self.match_all_conditional(&pattern_sets)
  }

  /// Test for presecnce of simple patterns in case-insensitive mode
  fn contains_all_conditional_ci(&self, patterns: &[&str]) -> bool {
    let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(patterns, CaseMatchMode::Insensitive, BoundsPosition::Contains);
    self.match_all_conditional(&pattern_sets)
  }

  /// Test for presecnce of simple patterns in case-sensitive mode
  fn contains_all_conditional_cs(&self, patterns: &[&str]) -> bool {
    let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(patterns, CaseMatchMode::Sensitive, BoundsPosition::Contains);
    self.match_all_conditional(&pattern_sets)
  }
  
}

impl SimpleMatchAll for str {

  // test for multiple conditions. All other 'many' trait methods are derived from this
  fn match_all_conditional(&self, pattern_sets: &[StringBounds]) -> bool {
    // self.matched_conditional(pattern_sets).into_iter().all(|matched| matched)
    if pattern_sets.len() > 0 {
      for item in pattern_sets {
        // do not evaluate more rules one is not matched
        if !match_bounds_rule_set(self, item) {
          return false;
        }
      }
      // return true if one or rules are matched
      true
    } else {
      // return false if no rules are provided
      false
    }
  }

}

/// Test for any of multiple pattern rules and return boolean
pub trait SimpleMatchAny where Self:SimpleMatchesMany {

  /// test for multiple conditions. All other trait methods are derived from this
  fn match_any_conditional(&self, pattern_sets: &[StringBounds]) -> bool;

  /// test for multiple conditions with simple tuple pairs of pattern + case-insenitive flag
  fn contains_any_conditional(&self, pattern_sets: &[(&str, bool)]) -> bool {
    let pattern_sets: Vec<StringBounds> = pairs_to_string_bounds(pattern_sets, BoundsPosition::Contains);
    self.match_any_conditional(&pattern_sets)
  }

  /// Test for presecnce of simple patterns in case-insensitive mode
  fn contains_any_conditional_ci(&self, patterns: &[&str]) -> bool {
    let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(patterns, CaseMatchMode::Insensitive, BoundsPosition::Contains);
    self.match_any_conditional(&pattern_sets)
  }

  /// Test for presecnce of simple patterns in case-sensitive mode
  fn contains_any_conditional_cs(&self, patterns: &[&str]) -> bool {
    let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(patterns, CaseMatchMode::Sensitive, BoundsPosition::Contains);
    self.match_any_conditional(&pattern_sets)
  }
  
}

impl SimpleMatchAny for str {

  // Test for multiple conditions. All other 'any' trait methods are derived from this
  fn match_any_conditional(&self, pattern_sets: &[StringBounds]) -> bool {
    for item in pattern_sets {
      // if one rule is matched, return true as other rules need not be evaluated
      if match_bounds_rule_set(self, item) {
        return true;
      }
    }
    // return false if no rules are matched or provided
    false
  }

}

/// Test if character set (CharType) is in the string
pub trait SimplContainsType where Self:SimpleMatch {

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

/// Implement character-set matching on &str/String
impl SimplContainsType for str {

  // test for multiple conditions. All other 'many' trait methods are derived from this
  fn contains_type(&self, char_type: CharType) -> bool {
    self.chars().any(|ch| char_type.is_in_range(&ch))
  }

  fn contains_types(&self, char_types: &[CharType]) -> bool {
    self.chars().any(|ch| char_types.into_iter().any(|ct| ct.is_in_range(&ch)))
  }

   /// starts with one or more characters in the specified set
   fn starts_with_type(&self, char_type: CharType) -> bool {
    if let Some(first) = self.chars().nth(0) {
      char_type.is_in_range(&first)
    } else {
      false
    }
   }

   /// starts with one or more characters in the specified set
   fn starts_with_types(&self, char_types: &[CharType]) -> bool {
    if let Some(first) = self.chars().nth(0) {
      char_types.into_iter().any(|ct| ct.is_in_range(&first))
    } else {
      false
    }
   }
 
   /// ends with one or more characters in the specified sets
   fn ends_with_type(&self, char_type: CharType) -> bool {
    if let Some(first) = self.chars().last() {
      char_type.is_in_range(&first)
    } else {
      false
    }
   }
 
   /// ends with one or more characters in the specified sets
   fn ends_with_types(&self, char_types: &[CharType]) -> bool {
    if let Some(first) = self.chars().last() {
      char_types.into_iter().any(|ct| ct.is_in_range(&first))
    } else {
      false
    }
   }
   

}


/// Test multiple patterns and return a filtered vector of string slices by all pattern rules
pub trait SimpleFilterAll<'a, T> {

  /// test for multiple conditions. All other trait methods are derived from this
  fn filter_all_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<T>;

  fn filter_all_rules(&'a self, rules: &BoundsBuilder) -> Vec<T> {
    self.filter_all_conditional(&rules.as_vec())
  }
  
}

/// Filter strings by one or more StringBounds rules
impl<'a> SimpleFilterAll<'a, &'a str> for [&str] {

  // filter string slices by multiple conditions
  fn filter_all_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<&'a str> {
    self.into_iter().map(|s| s.to_owned()).filter(|s| s.match_all_conditional(pattern_sets)).collect::<Vec<&'a str>>()
  }

}

/// Variant implementation for owned strings
impl<'a> SimpleFilterAll<'a, String> for [String] {
  // filter strings by multiple conditions
  fn filter_all_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<String> {
    self.into_iter().filter(|s| s.match_all_conditional(pattern_sets)).map(|s| s.to_owned()).collect::<Vec<String>>()
  }

}

/// Test multiple patterns and return a filtered vector of string slices by any of the pattern rules
pub trait SimpleFilterAny<'a, T> {

  /// test for multiple conditions. All other trait methods are derived from this
  fn filter_any_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<T>;

  fn filter_any_rules(&'a self, rules: &BoundsBuilder) -> Vec<T> {
    self.filter_any_conditional(&rules.as_vec())
  }
  
}

/// Filter strings by one or more StringBounds rules
impl<'a> SimpleFilterAny<'a, &'a str> for [&str] {

  // filter string slices by multiple conditions
  fn filter_any_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<&'a str> {
    self.into_iter().map(|s| s.to_owned()).filter(|s| s.match_any_conditional(pattern_sets)).collect::<Vec<&'a str>>()
  }

}

/// Variant implementation for owned strings
impl<'a> SimpleFilterAny<'a, String> for [String] {
  // filter strings by multiple conditions
  fn filter_any_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<String> {
    self.into_iter().filter(|s| s.match_any_conditional(pattern_sets)).map(|s| s.to_owned()).collect::<Vec<String>>()
  }

}
