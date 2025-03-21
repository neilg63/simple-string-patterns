[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/simple-string-patterns)
[![crates.io](https://img.shields.io/crates/v/simple-string-patterns.svg)](https://crates.io/crates/simple-string-patterns)
[![docs.rs](https://docs.rs/simple-string-patterns/badge.svg)](https://docs.rs/simple-string-patterns)

# Simple String Patterns

This library makes it easier to match, split and extract strings in Rust. It builds on the Rust standard library. A parallel [string-patterns](https://crates.io/crates/string-patterns) crate provides extensions to work with _regular expressions_. Together, these crates aim to make working with strings as easy in Rust as it is Javascript or Python with cleaner syntax.
Simpler string matching methods such as `starts_with`, `contains` or `ends_with` will always perform better, especially when processing large data sets. Methods such as `equals_ci`, `equals_ci_alphanum`, `starts_with_ci` and `starts_with_ci_alphanum` build on these core methods to facilitate string manipulation without _regular expressions_.
Version 0.3.0 sees a radical revision of the enums used to define string matching rules in the _matched_by_rules()_, _matched_conditional()_, _filter_all_rules()_ and _filter_any_rules()_ methods.

## Simple Patterns versus Regular Expressions

The main advantages of _simple-string-patterns_ lie in readability and miniminal overhead in lightweight applications that would not otherwise need regex support. Under the hood, regular expression engines compile regex syntax and convert them into more efficient string matching subroutines. Preliminary benchmarks show that rule sets with basic matching methods such as _contains_ci_ perform better than their regex counterparts, but if you need to add multiple nested rules, a _regex_ may be faster. The sibling regex-powered _string-patterns_ crate makes this very easy. This crate is best suited to small utilities that need to process large quantities of strings with a range of highly predictable formats, e.g. in cryptography, logging.

### Method overview

| Component<br /><sup>position</sup> | Meaning                                                                                                         |
| ---------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| - <sub>⇥</sup>                     | Many methods without _\_ci_ or _\_cs_ suffixes have an extra a boolean _case_insensitive_ parameter             |
| \_ci <sub>⇥</sub>                  | case-insensitive (cast to lower case for comparison)                                                            |
| \_cs <sub>⇥</sub>                  | case-sensitive                                                                                                  |
| \_ci_alphanum <sub>⇥</sub>         | case-insensitive match on only alphanumeric letters in the sample string                                        |
| \_rules <sub>⇥</sub>               | Accepts a set of rules defined via bounds_builder(), see below for examples                                     |
| \_conditional <sub>⇥</sup>         | Accepts an array of StringBounds rules, mainly for internal use                                                 |
| strip*by* <sub>⇤</sub>             | Return a string without the specified character type(s)                                                         |
| filter*by* <sub>⇤</sub>            | Return a string with only specified character type(s)                                                           |
| filter_all <sub>⇤</sub>            | Filter arrays or vectors that match all of the rules (and logic)                                                |
| filter_any <sub>⇤</sub>            | filter arrays or vectors that match any of the rules (or logic)                                                 |
| to_parts <sub>⇤</sup>              | To a vector of string parts split by a separator                                                                |
| to_segments <sub>⇤</sub>           | To a vector of non-empty string parts split by a separator                                                      |
| \_part(s) <sub>↔︎⇥</sub>           | Including leading or trailing separators and may return empty elements in vectors                               |
| \_segment(s)\* <sub>↔︎⇥</sub>      | Excluding leading, trailing, repeated consecutive separators and thus exclude empty elements                    |
| \_head, \_tail <sub>↔︎⇥</sub>      | With split methods, head means the segment before the first split and tail the remainder                        |
| \_start, \_end <sub>↔︎⇥</sub>      | _start_ means the whole string before the last split and _end_ only the last part of the last matched separator |
| \_escaped <sub>⇥</sub>             | Add an optional escape character parameter with _enclose_ or _wrap_ methods                                     |
| \_safe <sub>⇥</sub>                | Insert a backslash before the any non-final occurrences of the closing characters unless already present        |

##### Simple case-insensitive match

```rust
let str_1 = "Dog food";
if str_1.starts_with_ci("dog") {
  println!("{} is dog-related", str_1);
}
```

##### Simple case-insensitive match on the alphanumeric characters only in a longer text

```rust
// Theses methods are handy for validating text values from external data sources with
// inconsistent naming conventions, e.g. first-name, first_name, firstName or "first name"
let str_1 = "Do you spell hip-hop with a hyphen?";
if str_1.contains_ci_alphanum("hiphop") {
  println!("{} is hip-hop-related", str_1);
}
// Compare a single term with or without hyphens, spaces or other punctuation
let sample_str = "Start-up";
if str_1.equals_ci_alphanum("startup") {
  println!("It's a start-up company");
}
```

##### Filter a vector of strings by their first alphanumeric characters

```rust
// Methods ending in _alphanum are good for filtering strings that may have other
// to_strings() converts an array of &str references to a vector of strings
let sample_strs = [
  "/blue-sky.jpg",
  "----bluesky.png",
  "-B-l-u-e--sky",
  "Blueberry",
  " Blue sky thinking"
].to_strings();
let strings_starting_with_blue = sample_strs
  .into_iter()
  .filter(|s| s.starts_with_ci_alphanum("bluesky"))
  .collect::<Vec<String>>();
// should return all except "Blueberry"
```

##### Extract the third non-empty segment of a long path name

```rust
let path_string = "/var/www/mysite.com/web/uploads";
if let Some(domain) = path_string.to_segment("/", 2) {
  println!("The domain folder name is: {}", domain); // "mysite.com" is an owned string
}
```

##### Extract the _head and tail_ or _start and end_ from a longer string

```rust
let test_string = "long-list-of-technical-words"
let (head, tail) = test_string.to_head_tail("-");
println!("Head: {}, tail: {}", head, tail); // Head: long, tail: list-of-technical-words
let (start, end) = test_string.to_start_end("-");
println!("Start: {}, end: {}", start, end); // Start: long-list-of-technical, end: words
```

##### Capture an inner segment via multiple patterns

```rust
let source_str = "long/path/with-a-long-title/details";
  let target_str = "long";
  if let Some(inner_segment) = source_str.to_inner_segment(&[("/", 2), ("-", 2)]) {
    println!("The inner segment between 'a' and 'title' is: '{}'", inner_segment); // should read 'long'
  }
```

##### Extract the first decimal value as an f64 from a longer string

```rust
const GBP_TO_EURO: f64 = 0.835;
let sample_str = "Price £12.50 each";
if let Some(price_gbp) = sample_str.to_first_number::<f64>() {
    let price_eur = price_gbp / GBP_TO_EURO;
    println!("The price in euros is {:.2}", price_eur);
}
```

##### Extract numeric sequences from phrases and convert them to a vector of floats

```rust
// extract European-style numbers with commas as decimal separators and points as thousand separators
let sample_str = "2.500 grammi di farina costa 9,90€ al supermercato.";
let numbers: Vec<f32> = sample_str.to_numbers_euro();
// If two valid numbers are matched assume the first is the weight
if numbers.len() > 1 {
  let weight_grams = numbers[0];
  let price_euros = numbers[1];
  let price_per_kg = price_euros / (weight_grams / 1000f32);
  // the price in kg should be 3.96
  println!("Flour costs €{:.2} per kilo", price_per_kg);
}
```

##### Split a string list of numbers into floats

```rust
// extract 64-bit floats from a comma-separated list
// numbers within each segment are evaluated separately
let sample_str = "34.2929,-93.701";
let numbers = sample_str.split_to_numbers::<f64>(",");
// should yield vec![34.2929,-93.701]; (Vec<f64>)
```

##### Match by all or any pattern rules without regular expressions

```rust
// Call .as_vec() at the end
let mixed_conditions = bounds_builder()
  .containing_ci("nepal")
  .ending_with_ci(".jpg");
let sample_name_1 = "picture-Nepal-1978.jpg";
let sample_name_1 = "edited_picture-Nepal-1978.psd";
// contains `nepal` and ends with .jpg
sample_name_1.match_all_rules(&mixed_conditions); // true
// contains `nepal` but does not end with .jpg
sample_name_2.match_all_rules(&mixed_conditions); // false
// contains `nepal` and/or .jpg
sample_name_1.match_any_rules(&mixed_conditions); // true
// contains `nepal` and/or .jpg
sample_name_2.match_any_rules(&mixed_conditions); // true
```

##### Filter by all pattern rules without regular expressions

```rust
// The same array may also be expressed via the new bounds_builder() function with chainable rules:
// You may call .as_vec() to convert to a vector of StringBounds rules as used by methods ending in _conditional
let mixed_conditions = bounds_builder()
  .containing_ci("nepal")
  .not_ending_with_ci(".psd");
let file_names = [
  "edited-img-Nepal-Feb-2003.psd",
  "image-Thailand-Mar-2003.jpg",
  "photo_Nepal_Jan-2005.jpg",
  "image-India-Mar-2003.jpg",
  "pic_nepal_Dec-2004.png"
];
/// The filter_all_rules() method accepts a *BoundsBuilder* object.
let nepal_source_files: Vec<&str> = file_names.filter_all_rules(&mixed_conditions);
// should yield two file names: ["photo_Nepal_Jan-2005.jpg", "pic_nepal_Dec-2004.png"]
// This will now return Vec<&str> or Vec<String> depending on the source string type.
```

##### Nested Rule Sets

As of verson 0.3.0 you may add nested rule sets with _and_ / _or_ logic. The former case is true only if all conditions are met, while the latter is true if any of the conditions are met. The _BoundsBuilder_ struct now has a set of methods starting with _and_ or _or_. You may call _and(rules: BoundsBuilder)_ or _or(rules: BoundsBuilder)_ directly with a nested rule set if you have a mix of rule types. However, if all rules have the same bounds, other methods accepting a simple array of patterns are available, e.g.

- or_starting_with_ci(patterns: &[&str])
- or_starting_with_ci_alphanum(patterns: &[&str])
- or_containing_ci(patterns: &[&str])
- or_ending_with_ci(patterns: &[&str])
- and_not_ending_with_ci(patterns: &[&str])

```rust
let filenames = [
  "my_rabbit_2019.webp",
  "my_CaT_2020.jpg",
  "neighbours_Dog_2021.gif",
  "daughters_Dog_2023.jpeg",
  "big cat.psd"
];
/// Match files containing the letter sequences "cat" or "dog" and ending in ".jpg" or ".jpeg";
let rules = bounds_builder()
  .or_contains_ci(&["cat", "dog"])
  .or_ends_with_ci(&[".jpg", ".jpeg"]);
let matched_files = filenames.filter_all_rules(&rules);
/// Should yield an array with "my_CaT_2020.jpg" and "daughters_Dog_2023.png"
```

The above example reproduces the following example _regular expression_ /(cat|dog).\*?\.jpe?g$/. The \_alphanum-suffixed variants let match only on numbers and letters within a string, i.e. ignorning any spaces or punctuation.

##### Filter by any pattern rules without regular expressions

```rust
// The same array may also be expressed via the new bounds_builder() function with chainable rules:
// Call .as_vec() at the end
let mixed_or_conditions = bounds_builder()
  .containing_ci("nepal")
  .containing_ci("india");
let file_names = &[
  "edited-img-Nepal-Feb-2003.psd",
  "image-Thailand-Mar-2003.jpg",
  "photo_Nepal_Jan-2005.jpg",
  "image-India-Mar-2003.jpg",
  "pic_nepal_Dec-2004.png"
];
let nepal_and_india_source_files: Vec<&str> = file_names.filter_any_rules(&mixed_or_conditions);
// should yield two file names: ["edited-img-Nepal-Feb-2003.psd", "photo_Nepal_Jan-2005.jpg", "image-India-Mar-2003.jpg", "pic_nepal_Dec-2004.png"]
// To combine and/or logic, you can filter all rules with a nested "or" clause.
let mixed_conditions_jpeg_only = bounds_builder()
  .ending_with_ci(".jpg")
  .or(mixed_or_conditions);
let nepal_and_india_source_files_jpgs: Vec<&str> = file_names.filter_all_rules(&mixed_conditions_jpeg_only);
// should yield two file names: ["photo_Nepal_Jan-2005.jpg", "image-India-Mar-2003.jpg"]
```

#### Enclose strings in common bounding characters

```rust
let sample_phrase = r#"LLM means "large language model""#;
let phrase_in_round_brackets = sample_phrase.parenthesize();
// yields (LLM means "large language model")
// but will not escape any parentheses in the source string.
let phrase_in_left_right_quotes = sample_phrase.enclose('“', '”');
// yields “LLM means "large language model"”
// in custom left and right quotation marks, but will not escape double quotes.
let phrase_in_double_quotes = sample_phrase.double_quotes_safe();
// yields “LLM means \"large language model\"" with backslash-escaped double quotes
```

#### Filter strings by character categories

```rust
let sample_str = "Products: $9.99 per unit, £19.50 each, €15 only. Zürich café cañon";
let vowels_only = sample_str.filter_by_type(CharType::Chars(&['a','e','i','o', 'u', 'é', 'ü', 'y']));
println!("{}", vowels_only);
// should print "oueuieaoyüiaéao"
let lower_case_letters_a_to_m_only = sample_str.filter_by_type(CharType::Range('a'..'n'));
println!("{}", lower_case_letters_a_to_m_only);
// should print  "dceieachlichcafca"
/// You can filter strings by multiple character categories
let sample_with_lower_case_chars_and_spaces = sample_str.filter_by_types(&[CharType::Lower, CharType::Spaces]);
println!("{}", sample_with_lower_case_chars_and_spaces);
// Should print "roducts  per unit  each  only ürich café cañon"
```

#### Strip spaces only

```rust
let sample_str = "19 May 2021 ";
let sample_without_spaces = sample_str.strip_spaces();
println!("{}", sample_without_spaces);
  // should print "19May2021";
```

#### Remove character categories from strings

```rust
let sample_without_punctuation = sample_str.strip_by_type(CharType::Punctuation);
println!("{}", sample_without_punctuation);
// should print "Products 999 per unit £1950 each €15 only Zürich café cañon";
let sample_without_spaces_and_punct = sample_str.strip_by_types(&[CharType::Spaces, CharType::Punctuation]);
println!("{}", sample_without_spaces_and_punct);
// should print "Products999perunit£1950each€15onlyZürichcafécañon";
```

#### Split a string on any of set of characters

```rust
let sample_str = "jazz-and-blues_music/section";
let parts = sample_str.split_on_any_char(&['-','_', '/']);
// should yield "jazz", "and", "blues", "music", "section" as a vector of strings
```

### Traits

| Name               | No. of methods | Method description                                                                                                                                                                                                   |
| ------------------ | -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| MatchOccurrences   | 2              | Return the indices of all ocurrences of an exact string (find_matched_indices) or single character (find_char_indices)                                                                                               |
| CharGroupMatch     | 6              | Validate strings with character classes, has_digits, has_alphanumeric, has_alphabetic                                                                                                                                |
| IsNumeric          | 1              | Check if the string may be parsed to an integer or float                                                                                                                                                             |
| StripCharacters    | 17             | Strip unwanted characters by type or extract vectors of numeric strings, integers or floats without regular expressions                                                                                              |
| SimpleMatch        | 6              | Match strings without regular expression with common validation rules, e.g. starts_with_ci_alphanum checks if the first letters or numerals in a sample string in case-insensitive mode without regular expressions. |
| SimpleMatchesMany  | 6              | Regex-free multiple _match_ methods accepting an array of StringBounds items, tuples or patterns and returning a vector of boolean results                                                                           |
| SimpleMatchAll     | 4              | Regex-free multiple _match_ methods accepting an array of StringBounds items, tuples or patterns and returning a boolean if all are matched                                                                          |
| SimpleMatchAany    | 4              | Regex-free multiple _match_ methods accepting an array of StringBounds items, tuples or patterns and returning a vector of boolean results                                                                           |
| SimpleFilterAll    | 2              | Applies simple regex-free multiple _match_ methods to an array or vector of strings and returns a filtered vector of string slices                                                                                   |
| ToSegments         | 14             | Split strings into parts, segments or head and tail pairs on a separator                                                                                                                                             |
| ToSegmentFromChars | 3              | Split strings into parts on any of any array of characters                                                                                                                                                           |
| SimpleEnclose      | 10             | Wrap strings in pairs of matching characters with variants for different escape character rules                                                                                                                      |
| ToStrings          | 1              | Converts arrays or vectors of strs to a vector of owned strings                                                                                                                                                      |

### Enums

#### CaseMatchMode

Defines case-sensitivity and alphanumeric-only modes.
| Name | suffix equivalent | Meaning |
| ------------------- | ----------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Sensitive | \_cs | Case sensitive |
| Insensitive | \_ci | Case-insensitive, casts both the needle and haystack all strings to lower case for comparison |
| AlphanumInsensitive | \_ci_alphanum | Removes all non-alphanumeric characters from the sample string and cast both the needle and haystack to lower case for comparison |

#### StringBounds

Defines simple match rules with the pattern and a positivty flag, e.g. StringBounds::Contains("report", true, CaseMatchMode::Insensitive) or StringBounds::EndsWith(".docx", CaseMatchMode::Insensitive). The _bounds_builder_ method helps build these rule sets.
All options have _pattern: &str_, _is_positive: bool_ and _case match mode_ flags and acceot the same three arguments `(&str, bool, CaseMatchMode)` for the match pattern, positivity and case match mode.
| Name | Meaning |
| ---------- | ------------------ |
| StartsWith | starts with |
| EndsWith | ends with |
| Contains | contains |
| Whole | whole string match |

#### CharType

Defines categories, sets or ranges of characters as well as single characters.
| Name | Arguments | Meaning |
| ----------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Any | - | will match any characters |
| DecDigit | - | Match 0-9 only (is_ascii_digit) |
| Digit | (u8) | Match digit with the specified radix (e.g. 16 for hexadecimal) |
| Numeric | - | Match number-like $ in the decimal base. Unlike the is_numeric() extension method this excludes . and -. Use to_numbers_conditional() to extract valid decimal number as strings |
| AlphaNum | - | Match any alphanumeric characters (is_alphanumeric) |
| Lower | - | Match lower case letters (is_lowercase) |
| Upper | - | Match upper case letters (is_uppercase) |
| Alpha | - | Match any letters in most supported alphabets (is_alphabetic) |
| Spaces | - | Match spaces c.is_whitespace() |
| Punctuation | - | c.is_ascii_punctuation() |
| Char | (char) | match a single character |
| Chars | (&[char]) | Match an array of characters |
| Range | (Range<char>) | Match an Range e.g. 'a'..'d' will include a, b and c, but not d. This follows the Unicode sequence. |
| Between | (char, char) | Match characters betweeen the specified characters e.g. Between('a', 'd') will include d. |

### Structs

#### BoundsBuilder

This struct helps you build string pattern rules for use with the _matched_by_rules()_, _filter_all_rules()_ and _filter_any_rules()_ methods.
The _bounds_builder()_ function returns a base instance on which you may chain any number of rules and sub-rules.
| Rule type<br /><sup>(with suffix)</sup> | meaning | arguments | variants |
| --------------------------------------- | -------------------------------------------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| starting*with*&nbsp;(✓) | Starts with | pattern: &str | \_ci, \_cs, \_ci_alphanum |
| containing\_&nbsp;(✓) | Contains | pattern: &str | \_ci, \_cs, \_ci_alphanum |
| ending*with*&nbsp;(✓) | Ends with | pattern: &str | \_ci, \_cs, \_ci_alphanum |
| is\_&nbsp;(✓) | Matches a whole pattern | pattern: &str | \_ci, \_cs, \_ci_alphanum |
| not*starting_with*&nbsp;(✓) | Does not start withx | pattern: &str | \_ci, \_cs, \_ci_alphanum |
| not*containing*&nbsp;(✓) | Does not contain | pattern: &str | \_ci, \_cs, \_ci_alphanum |
| not*ending_with*&nbsp;(✓) | Does not end with | pattern: &str | \_ci, \_cs, \_ci_alphanum |
| is*not*&nbsp;(✓) | Does not match a whole pattern | pattern: &str | \_ci, \_cs, \_ci_alphanum |
| starts_with (⤬) | Starts with | pattern: &str<br />is_positive: bool<br />case_insensitive: bool | - |
| contains (⤬) | Contains | pattern: &str<br />is_positive: bool<br />case_insensitive: bool | - |
| ends_with (⤬) | Ends with | pattern: &str<br />is_positive: bool<br />case_insensitive: bool | - |
| whole (⤬) | Matches a whole pattern | pattern: &str , is_positive: bool, case_insensitive: bool | - |
| or (⤬) | Matches any of the specified rules | rules: &BoundsBuilder | - |
| or\_&nbsp;(✓) | Matches any of the patterns with the implicit rule | patterns: &[&str] | all in the starting*with*, containing*, ending_with* and is\_ series |
| and (⤬) | Matches all the specified rules | rules: &BoundsBuilder | - |
| and\_&nbsp;(✓) | Matches all of the patterns with the implicit rule | patterns: &[&str] | all in the starting*with*, containing*, ending_with* and is* series as well as their \_not* equivalents |

### Dev Notes

This crate serves as a building block for other crates as well as to supplement a future version of _string-patterns_. Some updates reflect minor editorial changes.
_Version 0.3.13_ introduces the `.strip_spaces()` method as shorthand for `.strip_by_type(CharType::Spaces)`.
_Version 0.3.11_ introduces a `.split_to_numbers::<T>(pattern: &str)` method to split a string list of numbers into a vector of the specified number type. This is handy when parsing common input formats such as latitudes and longitudes represented as `"42.282,-89.3938"`. This might fail via `.to_numbers()` when commas or points used as separators may be confused with decimal or thousand separators without other characters in between.

##### _Version 0.3.8_ New \*and*not*+ rules methods

This version introduced a set of _and*not*_-prefixed rule methods to filter strings do not match the specified array of patterns, e.g. if we have a list image file names that start with animal names and we want to match those beginning with case-insensitive "cat" or "dog", but excluding those ending in "".psd" or ".pdf".

```rust
  /// file names starting with cat or dog, but not ending in .pdf or .psd
  let file_names = [
    "CAT-pic-912.png", // OK
    "dog-pic-234.psd",
    "dOg-photo-876.png", // OK
    "rabbit-pic-194.jpg",
    "cat-pic-787.pdf",
    "cats-image-873.webp", // OK
    "cat-pic-090.jpg", // OK
  ];
  let rules = bounds_builder()
    .or_starting_with_ci(&["cat", "dog"])
    .and_not_ending_with_ci(&[".psd", ".pdf"]);
  let matched_files = file_names.filter_all_rules(&rules);
  /// This should yield ["CAT-pic-912.png", "dOg-photo-876.png", "cats-image-873.webp", "cat-pic-090.jpg"]
```

##### *Version 0.3.17* Added new `to_start(separator: &str)` and `to_remainder_start(separator: &str)` method
This version introduced a missing `to_start(&str)` method to return the start of a string before the last occurrence of the separator, i.e. to opposite `to_tail()`. `to_remainder_start()` supplements this method returning start of a string before the last non-final occurrence of a separator, e.g. with the string `one/two/three/`, `to_remainder_start("/")` yields `one/two`, while `to_start("/")` yields `one/two/three`, removing only the trailing slash.

##### _Version 0.3.16_ expands the range of rules available

Introduced `equals_ci(pattern: &str)` and `equals_ci_alphanum(pattern: &str)` to match whole strings in case-sensitive with or with without non-alphanumeric characters stripped.

##### _Version 0.3.15_ expands the range of rules available

Corrected bug in is_numeric() when used with empty strings.

##### _Version 0.3.0_ expands the range of rules available

This version introduced a radical revision to the StringBounds enum with supplementary _BoundsPosition_ and _CaseMatchMode_ enums, to handle the full range of rules available via _bounds_builder()_. These rule sets may be used with with the matched_by_rules(), filter_all_rules() and filter_any_rules().
Full documentation for the 0.2.* series is available in the [Github repo](https://github.com/neilg63/simple-string-patterns) in the *v0-2\* branch.

##### _Version 0.2.5_ introduces SimpleMatchAny and Whole matches in StringBounds.

This supplements SimpleMatchAll to apply _or_ logic with rules sets (StringBound, tuples or simple strs). The StringBounds enum now has whole string match options (with case-insensitive and case-sensitive variants) to accommodate a mix of partial and whole string matches. It also adds a range of single-argument methods for bounds*builder().
Versions of the \_string-patterns* crate before 0.3.0 contained many of these extensions. Since version 0.3.0 all traits, enums and methods defined in this _simple-string-patterns_ have been removed. These crates supplement each other, but may be installed independently.

##### Version 0.2.2 introduces three new features:

- _bounds_builder()_ makes it easier to define string matching rules methods requiring an array of _StringBounds_ rules such as filter_all_conditional(). See example above.
- _ToSegmentFromChars_ provides new methods to split on any of an array of characters, e.g. when processing common patterns that may use a predictable set of separators. This mimics characters classes in regular expressions and is more efficient when you only need to allow for a limited set of split characters.
- _MatchOccurrences_ has a variant _find_char_indices_ method that accepts a _char_ rather than a _&str_. This avoids any need to cast a character to a string.
