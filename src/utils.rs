
use crate::{enums::StringBounds, BoundsPosition, CaseMatchMode};

/// Miscellaneous utility functions that do not belong to structs
/// corrects a numeric string after it has been extracted by removing trailing dots or commas
pub(crate) fn add_sanitized_numeric_string(output: &mut Vec<String>, num_string: &str) {
  output.push(num_string.trim_end_matches(".").trim_end_matches(",").to_string());
}


/// Convert an array of strs to a vector of SimpleBounds with start/end/contains and case-sensity rules
/// as used in matched_conditional
/// Only used internally with interger mode
pub(crate) fn strs_to_string_bounds<'a>(strs: &'a [&str], case_mode: CaseMatchMode, mode: BoundsPosition) -> Vec<StringBounds<'a>> {
  strs.into_iter().map(|txt| StringBounds::new(mode, *txt, true, case_mode)).collect()
}

/// Convert an array of strs to a vector of SimpleBounds with start/end/contains and case-sensity rules
/// as used in matched_conditional
/// Only used internally with interger mode
pub(crate) fn strs_to_negative_string_bounds<'a>(strs: &'a [&str], case_mode: CaseMatchMode, mode: BoundsPosition) -> Vec<StringBounds<'a>> {
  strs.into_iter().map(|txt| StringBounds::new(mode, *txt, false, case_mode)).collect()
}

/// Convert an array of str/boolean tuples to a vector of SimpleBounds with start/end/contains
/// as used in matched_conditional
/// Only used internally with interger mode
pub(crate) fn pairs_to_string_bounds<'a>(pairs: &'a [(&str, bool)], mode: BoundsPosition) -> Vec<StringBounds<'a>> {
  pairs.into_iter().map(|(txt, ci)| StringBounds::new(mode, *txt, true, CaseMatchMode::insensitive(*ci))).collect()
}

/// Extract a string segment by its index where a negative value starts from the end
/// and an unmatched element returns None
pub(crate) fn extract_string_element_by_index(parts: Vec<String>, index: i32) -> Option<String> {
  let num_parts = parts.len();
  let target_index = if index >= 0 { index as usize } else { (num_parts as i32 + index) as usize };
  if target_index < num_parts {
    parts.get(target_index).map(|part| part.to_owned())
  } else {
    None
  }
}
