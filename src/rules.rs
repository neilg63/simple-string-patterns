use crate::utils::{pairs_to_string_bounds, strs_to_string_bounds};
use crate::{enums::StringBounds, BoundsBuilder};
use crate::{BoundsPosition, CaseMatchMode, SimpleMatch};
use alphanumeric::StripCharacters;

/// Test for any of multiple pattern rules and return boolean
pub trait SimpleMatchAny
where
    Self: SimpleMatchesMany,
{
    /// test for multiple conditions. All other trait methods are derived from this
    fn match_any_conditional(&self, pattern_sets: &[StringBounds]) -> bool;

    /// test for multiple conditions with simple tuple pairs of pattern + case-insenitive flag
    fn contains_any_conditional(&self, pattern_sets: &[(&str, bool)]) -> bool {
        let pattern_sets: Vec<StringBounds> =
            pairs_to_string_bounds(pattern_sets, BoundsPosition::Contains);
        self.match_any_conditional(&pattern_sets)
    }

    /// Test for presecnce of simple patterns in case-insensitive mode
    fn contains_any_conditional_ci(&self, patterns: &[&str]) -> bool {
        let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(
            patterns,
            CaseMatchMode::Insensitive,
            BoundsPosition::Contains,
        );
        self.match_any_conditional(&pattern_sets)
    }

    /// Test for presecnce of simple patterns in case-sensitive mode
    fn contains_any_conditional_cs(&self, patterns: &[&str]) -> bool {
        let pattern_sets: Vec<StringBounds> =
            strs_to_string_bounds(patterns, CaseMatchMode::Sensitive, BoundsPosition::Contains);
        self.match_any_conditional(&pattern_sets)
    }
}

/// Test multiple patterns and return vector of booleans with the results for each item
pub trait SimpleMatchesMany
where
    Self: SimpleMatch,
{
    /// test for multiple conditions. All other trait methods are derived from this
    fn matched_conditional(&self, pattern_sets: &[StringBounds]) -> Vec<bool>;

    /// test for multiple conditions with simple tuple pairs of pattern + case-insenitive flag
    fn contains_conditional(&self, pattern_sets: &[(&str, bool)]) -> Vec<bool> {
        let pattern_sets: Vec<StringBounds> =
            pairs_to_string_bounds(pattern_sets, BoundsPosition::Contains);
        self.matched_conditional(&pattern_sets)
    }

    /// Test for presecnce of simple patterns in case-insensitive mode
    fn contains_conditional_ci(&self, patterns: &[&str]) -> Vec<bool> {
        let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(
            patterns,
            CaseMatchMode::Insensitive,
            BoundsPosition::Contains,
        );
        self.matched_conditional(&pattern_sets)
    }

    /// Test for presecnce of simple patterns in case-sensitive mode
    fn contains_conditional_cs(&self, patterns: &[&str]) -> Vec<bool> {
        let pattern_sets: Vec<StringBounds> =
            strs_to_string_bounds(patterns, CaseMatchMode::Sensitive, BoundsPosition::Contains);
        self.matched_conditional(&pattern_sets)
    }
}


/// Common function to match scalar StringBounds rules
pub(crate) fn match_bounds_rule(txt: &str, item: &StringBounds) -> bool {
    let cm = item.case_mode();
    let ci = item.case_insensitive();
    // cast the sample string to lowercase for case-insenitive matches
    let base = if ci {
        match cm {
            CaseMatchMode::AlphanumInsensitive => txt.to_lowercase().strip_non_alphanum(),
            _ => txt.to_lowercase(),
        }
    } else {
        txt.to_owned()
    };
    // cast the simple pattern to lowercase for case-insenitive matches
    let pattern = if ci {
        item.pattern().to_lowercase()
    } else {
        item.pattern().to_owned()
    };
    // check if outcome of starts_with, ends_with or contains test matches the positivity value
    let is_matched = if item.starts_with() {
        base.starts_with(&pattern)
    } else if item.ends_with() {
        base.ends_with(&pattern)
    } else if item.matches_whole() {
        base == pattern
    } else {
        base.contains(&pattern)
    } == item.is_positive();
    is_matched
}

/// Common function to match StringBounds rule sets handling both and/or sub rules and scalar rules
pub(crate) fn match_bounds_rule_set(txt: &str, item: &StringBounds) -> bool {
    match item {
        StringBounds::And(inner_rules) => txt
            .matched_conditional(&inner_rules)
            .into_iter()
            .all(|result| result),
        StringBounds::Or(inner_rules) => txt
            .matched_conditional(&inner_rules)
            .into_iter()
            .any(|result| result),
        _ => match_bounds_rule(txt, item),
    }
}

impl<T: AsRef<str>> SimpleMatchesMany for T {
    fn matched_conditional(&self, pattern_sets: &[StringBounds]) -> Vec<bool> {
        let s = self.as_ref();
        let mut matched_items: Vec<bool> = Vec::with_capacity(pattern_sets.len());
        for item in pattern_sets {
            matched_items.push(match_bounds_rule_set(s, item));
        }
        matched_items
    }
}

impl<T: AsRef<str>> SimpleMatchAny for T {
    fn match_any_conditional(&self, pattern_sets: &[StringBounds]) -> bool {
        let s = self.as_ref();
        for item in pattern_sets {
            if match_bounds_rule_set(s, item) {
                return true;
            }
        }
        false
    }
}

/// Test multiple patterns and return boolean
pub trait SimpleMatchAll
where
    Self: SimpleMatchesMany,
{
    /// test for multiple conditions. All other trait methods are derived from this
    fn match_all_conditional(&self, pattern_sets: &[StringBounds]) -> bool;

    /// test for multiple conditions with simple tuple pairs of pattern + case-insenitive flag
    fn contains_all_conditional(&self, pattern_sets: &[(&str, bool)]) -> bool {
        let pattern_sets: Vec<StringBounds> =
            pairs_to_string_bounds(pattern_sets, BoundsPosition::Contains);
        self.match_all_conditional(&pattern_sets)
    }

    /// Test for presecnce of simple patterns in case-insensitive mode
    fn contains_all_conditional_ci(&self, patterns: &[&str]) -> bool {
        let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(
            patterns,
            CaseMatchMode::Insensitive,
            BoundsPosition::Contains,
        );
        self.match_all_conditional(&pattern_sets)
    }

    /// Test for presecnce of simple patterns in case-sensitive mode
    fn contains_all_conditional_cs(&self, patterns: &[&str]) -> bool {
        let pattern_sets: Vec<StringBounds> =
            strs_to_string_bounds(patterns, CaseMatchMode::Sensitive, BoundsPosition::Contains);
        self.match_all_conditional(&pattern_sets)
    }
}

impl<T: AsRef<str>> SimpleMatchAll for T {
    fn match_all_conditional(&self, pattern_sets: &[StringBounds]) -> bool {
        let s = self.as_ref();
        if pattern_sets.len() > 0 {
            for item in pattern_sets {
                if !match_bounds_rule_set(s, item) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

/// Test multiple patterns and return a filtered vector of string slices by all pattern rules
pub trait SimpleFilterAll<'a, T> {
    /// test for multiple conditions. All other trait methods are derived from this
    fn filter_all_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<T>;

    fn filter_all_rules(&'a self, rules: &BoundsBuilder) -> Vec<T> {
        self.filter_all_conditional(&rules.as_vec())
    }
}

/// Filter strings by one or more StringBounds rules
impl<'a> SimpleFilterAll<'a, &'a str> for [&str] {
    // filter string slices by multiple conditions
    fn filter_all_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<&'a str> {
        self.into_iter()
            .map(|s| s.to_owned())
            .filter(|s| s.match_all_conditional(pattern_sets))
            .collect::<Vec<&'a str>>()
    }
}

/// Variant implementation for owned strings
impl<'a> SimpleFilterAll<'a, String> for [String] {
    // filter strings by multiple conditions
    fn filter_all_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<String> {
        self.into_iter()
            .filter(|s| s.match_all_conditional(pattern_sets))
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
    }
}

/// Test multiple patterns and return a filtered vector of string slices by any of the pattern rules
pub trait SimpleFilterAny<'a, T> {
    /// test for multiple conditions. All other trait methods are derived from this
    fn filter_any_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<T>;

    fn filter_any_rules(&'a self, rules: &BoundsBuilder) -> Vec<T> {
        self.filter_any_conditional(&rules.as_vec())
    }
}

/// Filter strings by one or more StringBounds rules
impl<'a> SimpleFilterAny<'a, &'a str> for [&str] {
    // filter string slices by multiple conditions
    fn filter_any_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<&'a str> {
        self.into_iter()
            .map(|s| s.to_owned())
            .filter(|s| s.match_any_conditional(pattern_sets))
            .collect::<Vec<&'a str>>()
    }
}

/// Variant implementation for owned strings
impl<'a> SimpleFilterAny<'a, String> for [String] {
    // filter strings by multiple conditions
    fn filter_any_conditional(&'a self, pattern_sets: &[StringBounds]) -> Vec<String> {
        self.into_iter()
            .filter(|s| s.match_any_conditional(pattern_sets))
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_simple_filter_all() {
        let source_strs = [
            "Cat image",
            "dog picture",
            "elephant image",
            "CAT_Video",
            "cat Picture",
        ];
        let target_strs = ["Cat image", "cat Picture"];

        let conditions = bounds_builder()
            .starting_with_ci("cat")
            .not_containing_ci("video")
            .as_vec();
        assert_eq!(source_strs.filter_all_conditional(&conditions), target_strs);
    }

    #[test]
    fn test_nested_rules_with_filter_all() {
        let source_strs = [
            "_Cat image.jpg",
            "-dog picture.png",
            "_DOG pc.jpg",
            "elephant image.psd",
            "CAT_Video.mp4",
            "lion Picture.jpg",
        ];
        let target_strs = ["_Cat image.jpg", "_DOG pc.jpg"];
        let patterns = vec!["cat", "dog"];
        let conditions = bounds_builder()
            .or_starting_with_ci_alphanum(&patterns)
            .ending_with_ci(".jpg");
        assert_eq!(source_strs.filter_all_rules(&conditions), target_strs);
    }

    #[test]
    fn test_nested_rules_with_filter_any() {
        let source_strs = [
            "_Cat image.jpg",
            "-dog picture.png",
            "_DOG pc.jpg",
            "elephant image.psd",
            "CAT_Video.mp4",
            "lion Picture.jpg",
        ];
        let target_strs = ["_Cat image.jpg", "elephant image.psd"];
        let conditions = bounds_builder()
            .and(
                bounds_builder()
                    .starting_with_ci_alphanum("cat")
                    .ending_with_ci(".jpg"),
            )
            .ending_with_ci(".psd");
        assert_eq!(source_strs.filter_any_rules(&conditions), target_strs);
    }

    #[test]
    fn test_matched_conditional() {
        let conditions = [
            StringBounds::StartsWith("jan", true, CaseMatchMode::Insensitive),
            StringBounds::EndsWith("images", true, CaseMatchMode::Insensitive),
            StringBounds::Contains("2023", true, CaseMatchMode::Insensitive),
        ];

        let folder_1 = "Jan_2023_IMAGES";
        let folder_2 = "january_2024_Images";

        assert_eq!(
            folder_1.matched_conditional(&conditions),
            vec![true, true, true]
        );
        assert!(folder_1.match_all_conditional(&conditions));
        assert_eq!(
            folder_2.matched_conditional(&conditions),
            vec![true, true, false]
        );

        let test_strs = ["image", "cat", "garden"];
        let folder_3 = "cat-IMAGES_Garden";
        let folder_4 = "images-of-cats-and-dogs-in-the-park";

        assert!(folder_3.contains_all_conditional_ci(&test_strs));
        assert_eq!(folder_4.contains_all_conditional_ci(&test_strs), false);

        let file_names = [
            "edited-img-Nepal-Feb-2003.psd",
            "image-Thailand-Mar-2003.jpg",
            "photo_Nepal_Jan-2005.jpg",
            "image-India-Mar-2003.jpg",
            "pic_nepal_Dec-2004.png",
        ];

        let mixed_conditions = bounds_builder()
            .containing_ci("nepal")
            .not_ending_with_ci(".psd")
            .as_vec();

        let file_name_a = file_names[0];
        let file_name_b = file_names[2];

        assert!(file_name_a.match_all_conditional(&mixed_conditions) == false);
        assert!(file_name_b.match_all_conditional(&mixed_conditions));

        let nepal_jpg_files: Vec<&str> = file_names.filter_all_conditional(&mixed_conditions);
        assert_eq!(nepal_jpg_files.len(), 2);
        assert_eq!(nepal_jpg_files[0], file_name_b);

        let file_names_vector = file_names
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let nepal_jpg_files_vector: Vec<String> =
            file_names_vector.filter_all_conditional(&mixed_conditions);
        assert_eq!(nepal_jpg_files_vector.len(), 2);
    }

    #[test]
    fn test_matched_any_conditional() {
        let false_conditions = [
            StringBounds::Whole("no", true, CaseMatchMode::Insensitive),
            StringBounds::Whole("false", true, CaseMatchMode::Insensitive),
            StringBounds::Contains("not", true, CaseMatchMode::Insensitive),
            StringBounds::Contains("negative", true, CaseMatchMode::Insensitive),
        ];

        let boolean_strs_1 = ["NO", "FALSE", "not at all", "negative result", "Noon"];
        let falsy_strs = ["NO", "FALSE", "not at all", "negative result"];

        assert_eq!(
            boolean_strs_1.filter_any_conditional(&false_conditions),
            falsy_strs
        );

        let true_conditions = bounds_builder()
            .is_ci("yes")
            .is_ci("y")
            .starting_with_ci("ok")
            .containing_ci("positive")
            .as_vec();
        let boolean_strs_2 = ["Yes", "y", "Yep", "okay", "positive result", "good"];
        let truthy_strs = ["Yes", "y", "okay", "positive result"];

        assert_eq!(
            boolean_strs_2.filter_any_conditional(&true_conditions),
            truthy_strs
        );
    }

    #[test]
    fn test_bounds_builder() {
        let rules = bounds_builder()
            .starting_with_ci("cat")
            .not_ending_with_ci(".jpg");

        let sample_strs = [
            "cat-picture.jpg",
            "Dog-picture.png",
            "CAT-image.png",
            "rabbit-photo.png",
            "cAt-pic.webp",
        ];

        let filtered_lines = sample_strs.filter_all_rules(&rules);
        let expected_lines = vec!["CAT-image.png", "cAt-pic.webp"];
        assert_eq!(filtered_lines, expected_lines);

        let rules_2 = bounds_builder()
            .starting_with_ci("cat")
            .or_ending_with_ci(&[".jpg", ".png"]);

        let filtered_lines_2 = sample_strs.filter_all_rules(&rules_2);
        let expected_lines_2 = vec!["cat-picture.jpg", "CAT-image.png"];
        assert_eq!(filtered_lines_2, expected_lines_2);
    }

    #[test]
    fn test_and_starting_with_ci_positive() {
        // and_starting_with_ci pushes a positive StartsWith AND-group, so it should behave
        // like starting_with_ci for a single pattern, not like its not_-prefixed counterpart.
        let source_strs = ["cat-picture.jpg", "Dog-picture.png", "CAT-image.png"];
        let rules = bounds_builder().and_starting_with_ci(&["cat"]);
        let filtered = source_strs.filter_all_rules(&rules);
        assert_eq!(filtered, vec!["cat-picture.jpg", "CAT-image.png"]);
    }

    #[test]
    fn test_and_or_inner_bounds_builder() {
        let sample_strs = [
            "cat-picture.jpg",
            "Dog-picture.png",
            "DOG_portrait-2018.webp",
            "lion-image-2009.webp",
            "CAT-image.png",
            "rabbit-photo.png",
            "cAt-pic.webp",
        ];

        let rules_3 = bounds_builder()
            .and_not_starting_with_ci(&["cat", "lion"])
            .or_ending_with_ci(&[".png", ".webp"]);

        let filtered_lines_3 = sample_strs.filter_all_rules(&rules_3);
        let expected_lines_3 = vec![
            "Dog-picture.png",
            "DOG_portrait-2018.webp",
            "rabbit-photo.png",
        ];
        assert_eq!(filtered_lines_3, expected_lines_3);
    }
}
