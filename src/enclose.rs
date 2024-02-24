use std::str::Chars;


/// Traits with extension emthods to wrap strings in bounding characters
pub trait SimpleEnclose {
  
  /// Enclose in a start and an end character with an optional prefix
  /// before the main content but after the first character
  /// This is a common syntactical pattern in many markup and programming languages
   /// the closing character will be the same opening character
  /// The optional escape character is inserted before occurrences of the end character
  /// unless the preceding character is the escape character itself to avoid double escaping of pre-escaped strings
  fn enclose_in_chars(& self, start: char, end: char, prefix: Option<&str>, escape_char: Option<char>) -> String;

  /// Enclose in a start and an end character with an optional prefix after the first character
  fn enclose_escaped(& self, start: char, end: char, escape_char: Option<char>) -> String {
    self.enclose_in_chars(start, end, None, escape_char)
  }

  /// Enclose in a start and an end character with an optional prefix after the first character
  fn enclose(& self, start: char, end: char) -> String {
    self.enclose_in_chars(start, end, None, None)
  }

  /// Enclose in a start and an end character with an optional prefix after the first character
  /// escaped where necessary with a backslash \
  fn enclose_safe(& self, start: char, end: char) -> String {
    self.enclose_in_chars(start, end, None, Some('\\'))
  }

  /// Wrap a string in a pair of characters, with the closing character matching the first character
  /// if it a parenthesis (round bracket), angle bracket, (square)  bracket or curly brace. Otherwise
  /// the closing character will be the same opening character
  /// The optional escape character is inserted before occurrences of the end character
  /// unless the preceding character is the escape character itself to avoid double escaping of pre-escaped strings
  fn wrap_escaped(& self, opening: char, escape_char: Option<char>) -> String {
    let end = match opening {
      '(' => ')',
      '<' => '>',
      '{' => '}',
      '[' => ']',
      _ => opening
    };
    self.enclose_in_chars(opening, end, None, escape_char)
  }

  fn wrap(& self, opening: char) -> String {
    let end = match opening {
      '(' => ')',
      '<' => '>',
      '{' => '}',
      '[' => ']',
      _ => opening
    };
    self.enclose_in_chars(opening, end, None, None)
  }

  /// Wrap in matching characters escaped by a backslash \
  fn wrap_safe(& self, opening: char) -> String {
    let end = match opening {
      '(' => ')',
      '<' => '>',
      '{' => '}',
      '[' => ']',
      _ => opening
    };
    self.enclose_in_chars(opening, end, None, Some('\\'))
  }

  /// wrap in parentheses (sound brackets) with an optional prefix before the main content
  fn in_parentheses(& self, prefix: Option<&str>) -> String {
    self.enclose_in_chars('(', ')', prefix, None)
  }

  /// wrap in parentheses (sound brackets)
  fn parenthesize(& self) -> String {
    self.wrap('(')
  }

  /// wrap in parentheses (sound brackets)
  fn parenthesize_safe(& self) -> String {
    self.wrap_safe('(')
  }

  /// wrap in parentheses (sound brackets)
  fn double_quotes(& self) -> String {
    self.wrap('"')
  }

  fn single_quotes(& self) -> String {
    self.wrap('\'')
  }

  fn double_quotes_safe(& self) -> String {
    self.wrap_escaped('"', Some('\\'))
  }

  fn single_quotes_safe(& self) -> String {
    self.wrap_escaped('\'', Some('\\'))
  }

}


// Implement the base method for &str/String
impl SimpleEnclose for str {
  fn enclose_in_chars(&self, start: char, end: char, prefix: Option<&str>, escape_char: Option<char>) -> String {
    let mut out = match escape_char {
      Some(esc_char) => {
        if self.contains(end) {
          escape_in_str(self.chars(), end, esc_char)
        } else {
          self.to_owned()
        }
      },
      _ => self.to_owned()
    };
    out.insert(0, start);
    if let Some(pre) = prefix {
      out.insert_str(1, pre);
    }
    out.push(end);
    out
  }
}

pub fn escape_in_str(chars: Chars, end: char, esc_char: char) -> String {
  let mut new_string = String::new();
  let mut prev_char = ' ';
  for ch in chars {
    if ch == end {
      // do not escape escaped characters
      if prev_char != esc_char && new_string.len() > 0 {
        new_string.push(esc_char);
      }
    }
    new_string.push(ch);
    prev_char = ch;
  }
  new_string
}