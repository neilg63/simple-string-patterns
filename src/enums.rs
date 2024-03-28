/// Defines the matching bounds of simple string matches with case-insensitive/sensitive variants
/// and accepting the string pattern and positivity flag as arguments
#[derive(Debug, Clone)]
pub enum StringBounds<'a> {
  StartsWith(&'a str, bool, CaseMatchMode),
  EndsWith(&'a str, bool, CaseMatchMode),
  Contains(&'a str, bool, CaseMatchMode),
  Whole(&'a str, bool, CaseMatchMode),
  And(Vec<StringBounds<'a>>),
  Or(Vec<StringBounds<'a>>)
}

impl<'a> StringBounds<'a> {

  // Only used internally in utils
  // 0: starts with, 1 ends with, 2 (default) contains, 3 whole
  pub fn new(mode: BoundsPosition, txt: &'a str, is_positive: bool, case_mode: CaseMatchMode) -> StringBounds<'a> {
    match mode {
      BoundsPosition::Starts =>  Self::StartsWith(txt, is_positive, case_mode),
      BoundsPosition::Ends => Self::EndsWith(txt, is_positive, case_mode),
      BoundsPosition::Whole => Self::Whole(txt, is_positive, case_mode),
      _ => Self::Contains(txt, is_positive, case_mode),
    }
  }

  pub fn case_insensitive(&self) -> bool {
    match self {
      Self::StartsWith(_, _, cm) | Self::EndsWith(_, _, cm) | Self::Contains(_, _, cm) | Self::Whole(_, _, cm) => {
        match cm {
          CaseMatchMode::Sensitive => false,
          _ => true,
        }
      },
      _ => false, 
    }
  }

  pub fn case_mode(&self) -> CaseMatchMode {
    match self {
      Self::StartsWith(_, _, cm) | Self::EndsWith(_, _, cm) | Self::Contains(_, _, cm) | Self::Whole(_, _, cm) => {
        *cm
      },
      _ => CaseMatchMode::Sensitive, 
    }
  }

  pub fn pattern(&self) -> &'a str {
    match self {
      Self::StartsWith(txt, _, _) | Self::EndsWith(txt, _, _) |
      Self::Contains(txt, _, _) | Self::Whole(txt, _, _)
      => txt,
      _ => &""
    }.to_owned()
  }

  pub fn is_positive(&self) -> bool {
    match self {
      Self::StartsWith(_, is_pos, _) | Self::EndsWith(_, is_pos, _) |
      Self::Contains(_, is_pos, _) | Self::Whole(_, is_pos, _) => is_pos,
      _ => &false,
    }.to_owned()
  }

  pub fn starts_with(&self) -> bool {
    match self {
      Self::StartsWith(..) => true,
      _ => false
    }
  }

  pub fn ends_with(&self) -> bool {
    match self {
      Self::EndsWith(..) => true,
      _ => false
    }
  }

  pub fn matches_whole(&self) -> bool {
    match self {
      Self::Whole(..)=> true,
      _ => false
    }
  }

}


/// Simple enum to define position only, unlinke StringBounds methods with patterns and matching options
#[derive(Debug, Clone, Copy)]
pub enum BoundsPosition {
  Starts,
  Ends,
  Contains,
  Whole
}

/// Core matching mode corresponding to function name suffixes (_cs, _ci and _ci_alphanum)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseMatchMode {
  Sensitive,
  Insensitive,
  AlphanumInsensitive,
}

impl CaseMatchMode {
  /// Determines if case match mode requires the sample string and pattern to be lower-cased
  pub fn insensitive(case_insensitive: bool) -> Self {
    if case_insensitive { 
      Self::Insensitive
    } else {
      Self::Sensitive
    }
  }
}