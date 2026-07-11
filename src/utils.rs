use crate::{enums::StringBounds, BoundsPosition, CaseMatchMode};

/// Convert an array of strs to a vector of SimpleBounds with start/end/contains and case-sensitivity rules
/// as used in matched_conditional
/// Only used internally with interger mode
pub(crate) fn strs_to_string_bounds<'a>(
    strs: &'a [&str],
    case_mode: CaseMatchMode,
    mode: BoundsPosition,
) -> Vec<StringBounds<'a>> {
    strs.into_iter()
        .map(|txt| StringBounds::new(mode, *txt, true, case_mode))
        .collect()
}

/// Convert an array of strs to a vector of SimpleBounds with start/end/contains and case-sensitivity rules
/// as used in matched_conditional
/// Only used internally with interger mode
pub(crate) fn strs_to_negative_string_bounds<'a>(
    strs: &'a [&str],
    case_mode: CaseMatchMode,
    mode: BoundsPosition,
) -> Vec<StringBounds<'a>> {
    strs.into_iter()
        .map(|txt| StringBounds::new(mode, *txt, false, case_mode))
        .collect()
}

/// Convert an array of str/boolean tuples to a vector of SimpleBounds with start/end/contains
/// as used in matched_conditional
/// Only used internally with integer mode
pub(crate) fn pairs_to_string_bounds<'a>(
    pairs: &'a [(&str, bool)],
    mode: BoundsPosition,
) -> Vec<StringBounds<'a>> {
    pairs
        .into_iter()
        .map(|(txt, ci)| StringBounds::new(mode, *txt, true, CaseMatchMode::insensitive(*ci)))
        .collect()
}
