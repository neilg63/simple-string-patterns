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

  pub fn as_vec(&self) -> Vec<StringBounds<'a>> {
    self.string_bounds.clone()
  }

  fn starts_with(&mut self, pattern: &'a str, is_positive: bool, case_insensitive: bool) -> Self {
    let sb = if case_insensitive {
      StringBounds::StartsWithCi(pattern, is_positive)
    } else {
      StringBounds::StartsWithCs(pattern, is_positive)
    };
    self.string_bounds.push(sb);
    self.to_owned()
  }

  pub fn starts_with_ci(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.starts_with(pattern, is_positive, true)
  }

  pub fn starts_with_cs(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.starts_with(pattern, is_positive, false)
  }

  pub fn contains(&mut self, pattern: &'a str, is_positive: bool, case_insensitive: bool) -> Self {
    let sb = if case_insensitive {
      StringBounds::ContainsCi(pattern, is_positive)
    } else {
      StringBounds::ContainsCs(pattern, is_positive)
    };
    self.string_bounds.push(sb);
    self.to_owned()
  }

  pub fn contains_ci(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.contains(pattern, is_positive, true)
  }

  pub fn contains_cs(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.contains(pattern, is_positive, false)
  }

  fn ends_with(&mut self, pattern: &'a str, is_positive: bool, case_insensitive: bool) -> Self {
    let sb = if case_insensitive {
      StringBounds::EndsWithCi(pattern, is_positive)
    } else {
      StringBounds::EndsWithCs(pattern, is_positive)
    };
    self.string_bounds.push(sb);
    self.to_owned()
  }

  pub fn ends_with_ci(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.ends_with(pattern, is_positive, true)
  }

  pub fn ends_with_cs(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.ends_with(pattern, is_positive, false)
  }

}

/// Convenience metthod to build rule-sets
pub fn bounds_builder<'a>() -> BoundsBuilder<'a> {
  BoundsBuilder::new()
}