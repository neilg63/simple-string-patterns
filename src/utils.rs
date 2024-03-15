
use crate::enums::StringBounds;

/// Miscellaneous utility functions that do not belong to structs
/// corrects a numeric string after it has been extracted by removing trailing dots or commas
pub(crate) fn add_sanitized_numeric_string(output: &mut Vec<String>, num_string: &str) {
  output.push(num_string.trim_end_matches(".").trim_end_matches(",").to_string());
}


/// Convert an array of strs to a vector of SimpleBounds with start/end/contains and case-sensity rules
/// as used in matched_conditional
/// Only used internally with interger mode
/// 0 = Start, 1 = End, 2+ = Contains
pub(crate) fn strs_to_string_bounds<'a>(strs: &'a [&str], case_sensitive: bool, mode: u8) -> Vec<StringBounds<'a>> {
  strs.into_iter().map(|txt| StringBounds::new(mode, *txt, true, case_sensitive)).collect()
}

/// Convert an array of str/boolean tuples to a vector of SimpleBounds with start/end/contains
/// as used in matched_conditional
/// Only used internally with interger mode
/// 0 = Start, 1 = End, 2+ = Contains
pub(crate) fn pairs_to_string_bounds<'a>(pairs: &'a [(&str, bool)], mode: u8) -> Vec<StringBounds<'a>> {
  pairs.into_iter().map(|(txt, ci)| StringBounds::new(mode, *txt, true, *ci)).collect()
}
