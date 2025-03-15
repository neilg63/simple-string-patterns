use crate::{simple_match::*, utils::extract_string_element_by_index};

/// Methods to split a longer strong on a separator and return a vector of strings,
/// a tuple of two strings or single optional string segment
/// Note some methods may return empty segments in the case of leading, trailing or repeated separators
/// See notes below
pub trait ToSegments {

  /// Extract a vector of non-empty strings from a string-like object with a given separator
  /// excluding leading, trailing or double separators
  fn to_segments(&self, separator: &str) -> Vec<String>;

  /// Extract a vector of strings from a string-like object with a given separator
  fn to_parts(&self, separator: &str) -> Vec<String>;

  /// Extract only the head before the first occurrence of a separator
  fn to_head(&self, separator: &str) -> String;

  /// Extract only the first segment before the first occurrence of a non-initial separator
  fn to_first(&self, separator: &str) -> String;

  /// Extract only the remainder after the first occurrence of a non-initial separator
  fn to_remainder_end(&self, separator: &str) -> String;

  /// Extract only the last segment after the last occurrence of a non-final separator
  fn to_last(&self, separator: &str) -> String;

  /// Extract only the beginning before the last segment following the last occurrence of a non-final separator
  fn to_remainder_start(&self, separator: &str) -> String;

  /// Extract only the last segment
  fn to_end(&self, separator: &str) -> String;

  /// Extract the start before the last occurrence of the separator
  /// or the whole string if the separator is absent
  fn to_start(&self, separator: &str) -> String;

  /// Extract a non-empty segment identified by its index from the components of a string with a given separator
  /// e.g. String::from("/User/maria/Documents") .to_segment(1) yields "maria"
  /// with the leading slash separator ignored
  /// A negative index parameter will start from the end ignoring trailing separators
  fn to_segment(&self, separator: &str, index: i32) -> Option<String> {
    let parts = self.to_segments(separator);
    extract_string_element_by_index(parts, index)
  }

  /// Extract a part identified by its index from the components of a string with a given separator
  /// e.g. String::from("10/11/2024") .to_parts(1) yields "11"
  /// A negative index parameter will start from the end 
  fn to_part(&self, separator: &str, index: i32) -> Option<String> {
    let parts = self.to_parts(separator);
    extract_string_element_by_index(parts, index)
  }

  /// Extract an inner segment via a set of separator + index tuples
  fn to_inner_segment(&self, groups: &[(&str, i32)]) -> Option<String>;

  /// extract the remainder after the head
  fn to_tail(&self, separator: &str) -> String;

  /// extract the first and last parts after the first occurrence of the separator
  fn to_head_tail(&self, separator: &str) -> (String, String);

  /// extract the first and last parts after the last occurrence of the separator
  fn to_start_end(&self, separator: &str) -> (String, String);

}

/// Implement string segment split and capture method for String
impl ToSegments for str {

  /// Splits a string on the exact separator, whether initial, final or repeated.
  /// May yield empty segments
  fn to_parts(&self, separator: &str) -> Vec<String> {
    let splitter = self.split(separator);
    splitter.into_iter().map(|s| s.to_string()).collect::<Vec<String>>()
  }

  /// Splits a string on a separator, but only returns an array of non-empty strings
  /// skipping leading, trailing or repeated separators that may otherwise yield empty strings
  fn to_segments(&self, separator: &str) -> Vec<String> {
    let splitter = self.split(separator);
    splitter.into_iter().map(|s| s.to_string()).filter(|s| s.len() > 0).collect::<Vec<String>>()
  }

  /// Extract only the head as a string. If the separator is absent return the whole string
  fn to_head(&self, separator: &str) -> String {
    if let Some((head, _tail)) = self.split_once(separator) {
      head.to_string()
    } else {
      self.to_owned()
    }
  }

  /// Extract only the last segment after the last occurrence of a non-final separator
  fn to_last(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.ends_with(separator) && self.len() > separator_len {
      let end_index = self.len() - separator_len;
      self[0..end_index].to_string().to_end(separator)
    } else {
      self.to_end(separator)
    }
  }

  /// extract the last segment whether empty or not
  fn to_end(&self, separator: &str) -> String {
    let (_start, end) = self.to_start_end(separator);
    end
  }

  /// extract the start before last occurrence of the separator
  /// or, if absent, return the whole string
  fn to_start(&self, separator: &str) -> String {
    let (start, _end) = self.to_start_end(separator);
    start
  }

  /// extract the remainder after the first split 
  /// or the whole string if the separator is absent
  fn to_tail(&self, separator: &str) -> String {
    let parts = self.to_parts(separator);
    let num_parts = parts.len();
    if num_parts > 0 {
      parts[1..num_parts].join(separator)
    } else {
      self.to_owned()
    }
  }

  /// Extract only the first segment before the first occurrence of a non-initial separator
  fn to_first(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.starts_with(separator) && self.len() > separator_len {
      self[separator_len..self.len()].to_string().to_head(separator)
    } else {
      self.to_head(separator)
    }
  }

  /// Extract only the remainder after the first occurrence of a non-initial separator
  fn to_remainder_end(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.starts_with(separator) && self.len() > separator_len {
      self[separator_len..].to_string().to_tail(separator)
    } else {
      self.to_tail(separator)
    }
  }
  
  /// Extract only the beginning before the last segment following the last occurrence of a non-final separator
  fn to_remainder_start(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.ends_with(separator) && self.len() > separator_len {
      let end_index = self.len() - separator_len;
      self[0..end_index].to_string().to_start(separator)
    } else {
      self.to_start(separator)
    }
  }

  /// extract an inner segment via a set of tuples with separators and indices.
  /// e.g. [("/", 1), ("-", 2)] applied to "pictures/holiday-france-1983/originals" 
  /// would match "1983" as an optional string
  fn to_inner_segment(&self, groups: &[(&str, i32)]) -> Option<String> {
    if groups.len() > 0 {
      let mut matched: Option<String> = None;
      let mut current_string = self.to_string();
      for group in groups {
        if current_string.len() > 0 {
          let (separator, index) = group;
          matched = current_string.to_segment(*separator, *index);
          current_string = matched.clone().unwrap_or("".to_string());
        }
      }
      matched
    } else {
      None
    }
  }

  /// Extract a tuple of the head and remainder
  /// like split_once but returninga tuple of strings
  /// If the separator is absent or at the start, the first part will be empty
  fn to_head_tail(&self, separator: &str) -> (String, String) {
    if let Some((head, tail)) = self.split_once(separator) {
      (head.to_string(), tail.to_string())
    } else {
      ("".to_owned(), self.to_owned())
    }
  }

  /// Extract a tuple of the start and the last part
  /// like split_once in reverse and returning a tuple of strings
  /// If the separator is absent or at the end, the second part will be empty
  fn to_start_end(&self, separator: &str) -> (String, String) {
    if let Some((start, end)) = self.rsplit_once(separator) {
      (start.to_string(), end.to_string())
    } else {
      (self.to_owned(), "".to_string())
    }
  }

}


/// Methods to split a &str/String on the first matched separator character
pub trait ToSegmentsFromChars {
  
  /// Split a string into parts separated by any of the referenced split characters
  fn split_on_any_char(&self, separators: &[char]) -> Vec<String>;

  /// Split a string into a head and tail separated by the first instance of the first matching separator
  /// If none of the separators are matched, the first element is
  ///  an empty string and the second the whole string
  fn to_head_tail_on_any_char(&self, separators: &[char]) -> (String, String);

  /// Split a string into s start and tail separated by the last instance of the first matching separator
  /// If none of the separators are matched, the first element is
  ///  an empty string and the second the whole string
  fn to_start_end_on_any_char(&self, separators: &[char]) -> (String, String);
}

impl ToSegmentsFromChars for str {

  /// Split a string on any of the referenced characters
  fn split_on_any_char(&self, separators: &[char]) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut has_match = false;
    let mut indices: Vec<usize> = Vec::new();
    for separator in separators {
      for matched_index in self.find_char_indices(*separator) {
        indices.push(matched_index);
      }
    }
    indices.sort_by(|a, b| a.cmp(b));
    let mut prev_start = 0;
    for index in indices {
      let segment = self[prev_start..index].to_string();
      parts.push(segment);
      has_match = true;
      prev_start = index + 1;
    }
    if has_match {
      parts.push(self[prev_start..].to_string());
      parts
    } else {
      vec![self.to_owned()]
    }
  }

  /// Split into head and tail components on the first occurrence of any of the referenced characters
  fn to_head_tail_on_any_char(&self, separators: &[char]) -> (String, String) {
    for ch in separators {
      if self.contains(*ch) {
        if let Some ((first, second)) = self.split_once(*ch) {
          return (first.to_string(), second.to_string());
        }
      }
    }
    ("".to_owned(), self.to_string())
  }

  /// Split into start and end components on the last occurrence of any of the referenced characters
  fn to_start_end_on_any_char(&self, separators: &[char]) -> (String, String) {
    for ch in separators {
      if self.contains(*ch) {
        if let Some ((first, second)) = self.rsplit_once(*ch) {
          return (first.to_string(), second.to_string());
        }
      }
    }
    (self.to_string(), "".to_owned())
  }

}
