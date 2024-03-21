use crate::enums::StringBounds;

/// Build a set of string matching rules
#[derive(Debug, Clone)]
pub struct BoundsBuilder<'a> {
  string_bounds: Vec<StringBounds<'a>>,
}

impl<'a> BoundsBuilder<'a> {
  pub fn new() -> Self {
    BoundsBuilder {
      string_bounds: Vec::new()
    }
  }

  /// Return a vector of StringBounds enum rules for use with filter_all_conditional()
  pub fn as_vec(&self) -> Vec<StringBounds<'a>> {
    self.string_bounds.clone()
  }

  /// Add a "start_with" rule with positive and case-insensitive flags
  fn starts_with(&mut self, pattern: &'a str, is_positive: bool, case_insensitive: bool) -> Self {
    let sb = if case_insensitive {
      StringBounds::StartsWithCi(pattern, is_positive)
    } else {
      StringBounds::StartsWithCs(pattern, is_positive)
    };
    self.string_bounds.push(sb);
    self.to_owned()
  }

  /// Add a "starts_with" rule with a positive flags in case-insensitive mode
  pub fn starts_with_ci(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.starts_with(pattern, is_positive, true)
  }

  /// Add a "starts_with" rule with a positive flag in case-sensitive mode
  pub fn starts_with_cs(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.starts_with(pattern, is_positive, false)
  }

  /// Add a positive "starts_with" rule with a case-insensitive flag
  pub fn starting_with(&mut self, pattern: &'a str, case_insensitive: bool) -> Self {
    self.starts_with(pattern, true, case_insensitive)
  }

  /// Add a positive "starts_with" rule in case-insensitive mode
  pub fn starting_with_ci(&mut self, pattern: &'a str) -> Self {
    self.starting_with(pattern, true)
  }

  /// Add a positive "starts_with" rule in case-sensitive mode
  pub fn starting_with_cs(&mut self, pattern: &'a str) -> Self {
    self.starting_with(pattern, false)
  }

  /// Add a negative "starts_with" rule with a case-insensitive flag
  pub fn not_starting_with(&mut self, pattern: &'a str, case_insensitive: bool) -> Self {
    self.starts_with(pattern, false, case_insensitive)
  }

  /// Add a negative "starts_with" rule in case-insensitive mode
  pub fn not_starting_with_ci(&mut self, pattern: &'a str) -> Self {
    self.not_starting_with(pattern, true)
  }

  /// Add a negative "starts_with" rule in case-sensitive mode
  pub fn not_starting_with_cs(&mut self, pattern: &'a str) -> Self {
    self.not_starting_with(pattern, false)
  }

  /// Add a "contains" rule with a positive flag in case-insensitive mode
  pub fn contains(&mut self, pattern: &'a str, is_positive: bool, case_insensitive: bool) -> Self {
    let sb = if case_insensitive {
      StringBounds::ContainsCi(pattern, is_positive)
    } else {
      StringBounds::ContainsCs(pattern, is_positive)
    };
    self.string_bounds.push(sb);
    self.to_owned()
  }

  /// Add a "contains" rule with a positive flags in case-insensitive mode
  pub fn contains_ci(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.contains(pattern, is_positive, true)
  }

  /// Add a "contains" rule with a positive flags in case-sensitive mode
  pub fn contains_cs(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.contains(pattern, is_positive, false)
  }

  /// Add a positive "contains" rule with a case-insensitive flag
  pub fn containing(&mut self, pattern: &'a str, case_insensitive: bool) -> Self {
    self.contains(pattern, true, case_insensitive)
  }

  /// Add a positive "contains" rule in case-insensitive mode
  pub fn containing_ci(&mut self, pattern: &'a str) -> Self {
    self.containing(pattern, true)
  }

  /// Add a positive "contains" rule as true in case-sensitive mode
  pub fn containing_cs(&mut self, pattern: &'a str) -> Self {
    self.containing(pattern, false)
  }

  /// Add a negative "contains" rule with a case-insensitive flag
  pub fn not_containing(&mut self, pattern: &'a str, case_insensitive: bool) -> Self {
    self.contains(pattern, false, case_insensitive)
  }

  /// Add a negative "contains" rule in case-insensitive mode
  pub fn not_containing_ci(&mut self, pattern: &'a str) -> Self {
    self.not_containing(pattern, true)
  }

  /// Add a negative "contains" rule in case-sensitive mode
  pub fn not_containing_cs(&mut self, pattern: &'a str) -> Self {
    self.not_containing(pattern, false)
  }

  /// Add an "ends_with" rule with a positive and case-insensitive flags
  fn ends_with(&mut self, pattern: &'a str, is_positive: bool, case_insensitive: bool) -> Self {
    let sb = if case_insensitive {
      StringBounds::EndsWithCi(pattern, is_positive)
    } else {
      StringBounds::EndsWithCs(pattern, is_positive)
    };
    self.string_bounds.push(sb);
    self.to_owned()
  }

  /// Add an "ends_with" rule with a positive flag in case-insensitive mode
  pub fn ends_with_ci(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.ends_with(pattern, is_positive, true)
  }

  /// Add an "ends_with" rule with a positive flag in case-sensitive mode
  pub fn ends_with_cs(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.ends_with(pattern, is_positive, false)
  }

  /// Add a positive "ends_with" rule with a case-insensitive flag
  pub fn ending_with(&mut self, pattern: &'a str, case_insensitive: bool) -> Self {
    self.ends_with(pattern, true, case_insensitive)
  }

  /// Add a positive "ends_with" rule in case-insensitive mode
  pub fn ending_with_ci(&mut self, pattern: &'a str) -> Self {
    self.ending_with(pattern, true)
  }

  /// Add a positive "ends_with" rule in case-sensitive mode
  pub fn ending_with_cs(&mut self, pattern: &'a str) -> Self {
    self.ending_with(pattern, false)
  }

  /// Add a negative "ends_with" rule  with a case-insensitive flag
  pub fn not_ending_with(&mut self, pattern: &'a str, case_insensitive: bool) -> Self {
    self.ends_with(pattern, false, case_insensitive)
  }

  /// Add a negative "ends_with" rule in case-insensitive mode
  pub fn not_ending_with_ci(&mut self, pattern: &'a str) -> Self {
    self.not_ending_with(pattern, true)
  }

  /// Add a negative "ends_with" in case-sensitive mode
  pub fn not_ending_with_cs(&mut self, pattern: &'a str) -> Self {
    self.not_ending_with(pattern, false)
  }

  /// Add an "whole_match" rule with a positive and case-insensitive flags
  pub fn matches_whole(&mut self, pattern: &'a str, is_positive: bool, case_insensitive: bool) -> Self {
    let sb = if case_insensitive {
      StringBounds::WholeCi(pattern, is_positive)
    } else {
      StringBounds::WholeCs(pattern, is_positive)
    };
    self.string_bounds.push(sb);
    self.to_owned()
  }

  /// Add an "whole_match" rule with a positive flag in case-sensitive mode
  pub fn matches_whole_ci(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.matches_whole(pattern, is_positive, true)
  }

  /// Add an "whole_match" rule with a positive flag in case-insensitive mode
  pub fn matches_whole_cs(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.matches_whole(pattern, is_positive, false)
  }

  pub fn is(&mut self, pattern: &'a str, case_insensitive: bool) -> Self {
    self.matches_whole(pattern, true, case_insensitive)
  }

  pub fn is_ci(&mut self, pattern: &'a str) -> Self {
    self.matches_whole(pattern, true, true)
  }

  pub fn is_not(&mut self, pattern: &'a str, case_insensitive: bool) -> Self {
    self.matches_whole(pattern, false, case_insensitive)
  }

  pub fn is_not_ci(&mut self, pattern: &'a str) -> Self {
    self.matches_whole(pattern, false, true)
  }

  pub fn is_not_cs(&mut self, pattern: &'a str) -> Self {
    self.matches_whole(pattern, false, false)
  }


}

/// Convenience method to build rule-sets
/// This starts a new BoundBuilder object with chained rule sets
pub fn bounds_builder<'a>() -> BoundsBuilder<'a> {
  BoundsBuilder::new()
}