use std::ops::Range;

/// Defines character group types with special custom types (Char, Chars, Range, Between)
#[derive(Debug, Clone)]
pub enum CharType<'a> {
  Any,
  DecDigit, // is_ascii_digit
  Digit(u32), // define the number base, e.g. 16 for hexdecimal
  Numeric, // as defined by the std library, i.e. a number-like character, but not decimal points or minus
  AlphaNum,
  Upper,
  Lower,
  Alpha,
  Spaces,
  Punctuation,
  Char(char),
  Chars(&'a [char]),
  Range(Range<char>),
  Between(char, char),
}

impl<'a> CharType<'a> {
  pub fn is_in_range(&self, c_ref: &char) -> bool {
    let c = c_ref.to_owned();
    match self {
      Self::Any => true,
      Self::DecDigit => c.is_ascii_digit(),
      Self::Digit(radix) => c.is_digit(*radix),
      Self::Numeric => c.is_numeric(),
      Self::AlphaNum => c.is_alphanumeric(),
      Self::Lower => c.is_lowercase(),
      Self::Upper => c.is_uppercase(),
      Self::Alpha => c.is_alphabetic(),
      Self::Spaces => c.is_whitespace(),
      Self::Punctuation => c.is_ascii_punctuation(),
      Self::Char(ch) => c == *ch,
      Self::Chars(chars) => chars.contains(&c),
      Self::Range(cr) => cr.contains(&c),
      Self::Between(c1, c2) => c >= *c1 && c <= *c2,
    }
  }
}