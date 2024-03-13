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

  /// Add an "ends_with" rule with a positive flags in case-insensitive mode
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

}

/// Convenience method to build rule-sets
/// This starts a new BoundBuilder object with chained rule sets
pub fn bounds_builder<'a>() -> BoundsBuilder<'a> {
  BoundsBuilder::new()
}