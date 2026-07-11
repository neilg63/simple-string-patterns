[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/simple-string-patterns)
[![crates.io](https://img.shields.io/crates/v/simple-string-patterns.svg)](https://crates.io/crates/simple-string-patterns)
[![docs.rs](https://docs.rs/simple-string-patterns/badge.svg)](https://docs.rs/simple-string-patterns)

# Simple String Patterns

This library makes it easier to match, split and extract strings in Rust. It builds on the Rust standard library. A parallel [string-patterns](https://crates.io/crates/string-patterns) crate provides extensions to work with _regular expressions_. Together, these crates aim to make working with strings as easy in Rust as it is Javascript or Python with cleaner syntax.
Simpler string matching methods such as `starts_with`, `contains` or `ends_with` will always perform better, especially when processing large data sets. Methods such as `equals_ci`, `equals_ci_alphanum`, `starts_with_ci` and `starts_with_ci_alphanum` build on these core methods to facilitate string manipulation without _regular expressions_.

In version 0.4.0, only the core methods are included by default. String bounds rule sets and the fluent match chain are both opt-in features, and other supplementary functions have been transferred to other library crates.

A compelling case for this crate is small microservices and CLI utilities that parse predictable, pre-formatted text output from other tools or services — log lines, status messages, command output — where a regex is overkill and manually normalizing case and punctuation by hand is repetitive boilerplate. It's tempting to reach for `contains()` to spot a severity level in a log line, but that gives false positives whenever the word shows up incidentally in the message body rather than as the actual tag:

```rust
let warning_msg = "[warning] high CPU usage. This may cause errors.";
let error_msg = "[error] cannot divide by 0. You have been warned";

warning_msg.contains_ci("error"); // true — WRONG, this is a warning, not an error
error_msg.contains_ci("warn");    // true — WRONG, this is an error, not a warning
```

Anchoring the check with `starts_with_ci_alphanum` instead of scanning the whole message avoids that, since it only looks at the tag, not the prose that follows:

```rust
warning_msg.starts_with_ci_alphanum("warning"); // true
warning_msg.starts_with_ci_alphanum("error");   // false
error_msg.starts_with_ci_alphanum("error");     // true
error_msg.starts_with_ci_alphanum("warning");   // false
```

It's also more robust than an exact literal prefix check, since it strips punctuation from both sides before comparing — so it survives the tag's delimiters changing (e.g. a log format moving from `[warning]` to `**warning**:`) without needing to be updated:

```rust
let reformatted = "**warning**: High CPU usage";
reformatted.starts_with_ci("[warning]");        // false — breaks: the delimiters changed
reformatted.starts_with_ci_alphanum("warning"); // true — still works
```

which in turn is more direct than doing the equivalent normalization by hand:

```rust
if input_string
    .to_lowercase()
    .chars()
    .filter(|c| c.is_alphanumeric())
    .collect::<String>()
    .starts_with("warning") {
  // ...
}
```

and, as of the 0.4.0 `ci_engine` rewrite, no longer pays for building that intermediate `String` on every call either — see [Simple Patterns versus Regular Expressions](#simple-patterns-versus-regular-expressions) below. Above all, it simply avoids the need to add regex as a dependency at all for a lightweight app that only has to check log files or call out to other CLI tools — no extra compile time or binary size for a job this small.

## Features

The crate core is always just the `SimpleMatch`, `MatchOccurrences` and `SimplContainsType` traits (in `matches.rs`) — no other functionality compiles without opting in via Cargo features. Neither `rules` nor `chain` is enabled by default.

| Feature | Default | Adds |
| ------- | ------- | ---- |
| _(none)_ | always on | `SimpleMatch`, `MatchOccurrences`, `SimplContainsType` — plain `_ci`/`_cs`/`_ci_alphanum` matching methods |
| `rules` | ✗ (opt-in) | `bounds_builder()`, `BoundsBuilder`, `StringBounds` and the `_conditional`/`_rules` methods for composable, reusable rule sets applied to many strings |
| `chain` | ✗ (opt-in) | `StringMatch` and the `match_*`/`and_*`/`or_*` fluent chain for combining several checks on a single string, see below |

```toml
simple-string-patterns = { version = "0.4", features = ["rules", "chain"] }
```

## Dependencies

- [**alphanumeric**](https://crates.io/crates/alphanumeric) — Provides `IsNumeric`, `StripCharacters`, and `CharGroupMatch` traits for international number format parsing, character filtering, and numeric string extraction

## Supplementary crates

The string-splitting and enclosing functionality previously bundled in this crate now lives in standalone crates. They are not dependencies of _simple-string-patterns_, but pair well with it if you need that functionality too:

- [**to_segments**](https://crates.io/crates/to_segments) — The `ToSegments` trait for ergonomic string splitting with readable methods for common manipulation tasks.
- [**enclose-strings**](https://crates.io/crates/enclose-strings) — Wrap or enclose strings in matching or complementary characters with optional escaping.

## Simple Patterns versus Regular Expressions

_simple-string-patterns_ aims to improve readability and minimise overhead in lightweight applications that would not otherwise need regex support. Under the hood, regular expression engines compile regex syntax and convert them into more efficient string matching subroutines.

Benchmarked against a compiled `regex` over 200,000 synthetic log lines (`"line".starts_with_ci_alphanum("float") && "line".contains_ci("error")` vs `/^[^a-z0-9]*float.*error/i`): the two are now close, ~29ns/line for this crate's methods versus ~15ns/line for the compiled regex — a modest, not dramatic, gap. `starts_with_ci`/`ends_with_ci`/`equals_ci` can bail out after inspecting only a handful of characters and are the strongest case for this crate; `contains_ci` alone has to scan the whole string and is the part of that gap that remains, since `regex`'s substring search (two-way algorithm plus a `memchr`-based literal prefilter) is hard to beat without reimplementing it. If you need to add multiple nested rules or complex patterns, a _regex_ is likely to stay faster and more flexible — the sibling regex-powered _string-patterns_ crate makes that easy, and as of its own 0.4.0 shares a single compiled-once regex cache across all its matching methods, so "compile once, apply to many rows" is no longer something you have to build yourself in either crate. This crate is best suited to small utilities that need to process large quantities of strings with a range of highly predictable formats, e.g. in cryptography, logging.

### Method overview

| Component<br /><sup>position</sup> | Meaning                                                                                                         |
| ---------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| - <sub>⇥</sup>                     | Many methods without _\_ci_ or _\_cs_ suffixes have an extra a boolean _case_insensitive_ parameter             |
| \_ci <sub>⇥</sub>                  | case-insensitive (cast to lower case for comparison)                                                            |
| \_cs <sub>⇥</sub>                  | case-sensitive                                                                                                  |
| \_ci_alphanum <sub>⇥</sub>         | case-insensitive match on only alphanumeric letters in the sample string                                        |
| strip*by* <sub>⇤</sub>             | Return a string without the specified character type(s)                                                         |
| filter*by* <sub>⇤</sub>            | Return a string with only specified character type(s)                                                           |
| filter_all <sub>⇤</sub>            | Filter arrays or vectors that match all of the rules (and logic)                                                |
| filter_any <sub>⇤</sub>            | filter arrays or vectors that match any of the rules (or logic)                                                 |
| \_rules <sub>⇥</sub>               | (**rules feature**) Accepts a set of rules defined via bounds_builder(), see below for examples                                     |
| \_conditional <sub>⇥</sup>         | (**rules feature**) Accepts an array of StringBounds rules, mainly for internal use                                                 |

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
// punctuation or casing inconsistencies around the part you care about
let sample_strs = [
  "/blue-sky.jpg",
  "----bluesky.png",
  "-B-l-u-e--sky",
  "Blueberry",
  " Blue sky thinking"
];
let strings_starting_with_blue = sample_strs
  .into_iter()
  .filter(|s| s.starts_with_ci_alphanum("bluesky"))
  .collect::<Vec<&str>>();
// should return all except "Blueberry"
```

##### Chain several checks on one string (`chain` feature)

```rust
// Requires the optional "chain" feature. Unlike contains_ci() and starts_with_ci_alphanum(),
// which return a plain bool and so cannot be chained further, match_contains_ci() returns a
// StringMatch that carries the accumulated result and the subject string through the chain.
let sample_1 = "Floating-point error at line 667";
let sample_2 = "[floating-point] error in line 999";
let sample_3 = "Rounding error at line 12";

let is_float_error = |s: &str| s.match_contains_ci("error")
  .and_starts_with_ci_alphanum("float")
  .result();

assert!(is_float_error(sample_1));
assert!(is_float_error(sample_2));
assert!(!is_float_error(sample_3));

// and_not_* and or_* combinators are also available, e.g. to exclude a suffix:
let is_cat_but_not_psd = "cat-picture.jpg".match_starts_with_ci("cat")
  .and_not_ends_with_ci(".psd")
  .result(); // true
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
// This crate uses the alphanumeric dependency for advanced number extraction
// See https://crates.io/crates/alphanumeric for international number format support
let sample_str = "2.500 grammi di farina costa 9,90€ al supermercato.";
let numbers: Vec<f32> = sample_str.to_numbers();
// If two valid numbers are matched assume the first is the weight
if numbers.len() > 1 {
  let weight_grams = numbers[0];
  let price_euros = numbers[1];
  let price_per_kg = price_euros / (weight_grams / 1000f32);
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

##### Match by all or any pattern rules without regular expressions (`rules` feature)

```rust
// Call .as_vec() to convert the BoundsBuilder to a slice of StringBounds rules
let mixed_conditions = bounds_builder()
  .containing_ci("nepal")
  .ending_with_ci(".jpg");
let sample_name_1 = "picture-Nepal-1978.jpg";
let sample_name_2 = "edited_picture-Nepal-1978.psd";
// contains `nepal` and ends with .jpg
sample_name_1.match_all_conditional(&mixed_conditions.as_vec()); // true
// contains `nepal` but does not end with .jpg
sample_name_2.match_all_conditional(&mixed_conditions.as_vec()); // false
// contains `nepal` and/or .jpg
sample_name_1.match_any_conditional(&mixed_conditions.as_vec()); // true
// contains `nepal` and/or .jpg
sample_name_2.match_any_conditional(&mixed_conditions.as_vec()); // true
```

##### Filter by all pattern rules without regular expressions (`rules` feature)

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

##### Nested Rule Sets (`rules` feature)

As of version 0.3.0 you may add nested rule sets with _and_ / _or_ logic. The former case is true only if all conditions are met, while the latter is true if any of the conditions are met. The _BoundsBuilder_ struct now has a set of methods starting with _and_ or _or_. You may call _and(rules: BoundsBuilder)_ or _or(rules: BoundsBuilder)_ directly with a nested rule set if you have a mix of rule types. However, if all rules have the same bounds, other methods accepting a simple array of patterns are available, e.g.

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
  .or_containing_ci(&["cat", "dog"])
  .or_ending_with_ci(&[".jpg", ".jpeg"]);
let matched_files = filenames.filter_all_rules(&rules);
/// Should yield an array with "my_CaT_2020.jpg" and "daughters_Dog_2023.jpeg"
```

The above example reproduces the following example _regular expression_ /(cat|dog).\*?\.jpe?g$/. The \_alphanum-suffixed variants let match only on numbers and letters within a string, i.e. ignorning any spaces or punctuation.

##### Filter by any pattern rules without regular expressions (`rules` feature)

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

### Traits

| Name               | No. of methods | Method description                                                                                                                                                                                                   |
| ------------------ | -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| MatchOccurrences   | 2              | Return the indices of all ocurrences of an exact string (find_matched_indices) or single character (find_char_indices)                                                                                               |
| CharGroupMatch     | 6              | Validate strings with character classes, has_digits, has_alphanumeric, has_alphabetic                                                                                                                                |
| IsNumeric          | 1              | Check if the string may be parsed to an integer or float (from `alphanumeric` dependency)                                                                                                                            |
| StripCharacters    | 17             | Strip unwanted characters by type or extract vectors of numeric strings, integers or floats without regular expressions (from `alphanumeric` dependency)                                                             |
| SimpleMatch        | 6              | Match strings without regular expression with common validation rules, e.g. starts_with_ci_alphanum checks if the first letters or numerals in a sample string in case-insensitive mode without regular expressions. |
| SimpleMatchesMany  | 6              | (**rules feature**) Regex-free multiple _match_ methods accepting an array of StringBounds items, tuples or patterns and returning a vector of boolean results                                                       |
| SimpleMatchAll     | 4              | (**rules feature**) Regex-free multiple _match_ methods accepting an array of StringBounds items, tuples or patterns and returning a boolean if all are matched                                                      |
| SimpleMatchAany    | 4              | (**rules feature**) Regex-free multiple _match_ methods accepting an array of StringBounds items, tuples or patterns and returning a vector of boolean results                                                       |
| SimpleFilterAll    | 2              | (**rules feature**) Applies simple regex-free multiple _match_ methods to an array or vector of strings and returns a filtered vector of string slices                                                               |
| ToStringMatch      | 12             | (**chain feature**) Entry points (match_contains_ci, match_starts_with_ci_alphanum, etc.) that start a chainable `StringMatch` expression on a string                                                                |
| StringMatch        | 36             | (**chain feature**) `and_*`/`and_not_*`/`or_*` methods to combine further checks on the same string, plus `.result()` to get the final bool                                                                          |

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

##### _Version 0.4.0_ streamlines the crate, fixes a performance regression, and corrects two bugs

**Crate structure.** The `ToSegments`, `ToSegmentFromChars`, `SimpleEnclose` and `ToStrings` traits described in earlier versions of these notes have been removed from this crate and now live in the standalone [to_segments](https://crates.io/crates/to_segments) and [enclose-strings](https://crates.io/crates/enclose-strings) crates (see [Supplementary crates](#supplementary-crates) above). The string bounds rule set (`bounds_builder()`, `StringBounds`, and the `_rules`/`_conditional` methods) is now gated behind the `rules` feature, and a new `chain` feature adds `StringMatch`, a fluent `match_*`/`and_*`/`or_*` chain for combining several checks on one string (see [Features](#features) and the chaining example above) — **neither feature is enabled by default.** By default the crate now exposes only `SimpleMatch`, `MatchOccurrences` and `SimplContainsType`.

> **Upgrading from 0.3.x:** `BoundsBuilder`/`bounds_builder()`/`StringBounds` were always compiled in on 0.3.x. If you use them, add `features = ["rules"]` explicitly — without it, code that called `bounds_builder()` will fail to compile after upgrading to 0.4.0, not just silently lose behaviour.

**Performance.** The `_ci`/`_ci_alphanum` methods on `SimpleMatch` (`contains_ci`, `starts_with_ci_alphanum`, etc.) are now backed by an internal `ci_engine` module instead of the previous `self.to_lowercase().strip_non_alphanum()...` approach, which allocated a full transformed copy of the *subject* string on every call. The new implementation dispatches to a byte-level ASCII fast path (no allocation, no UTF-8 decoding) when both the subject and pattern are ASCII, falling back to a Unicode-correct character stream otherwise (café/naïve-style input is still handled correctly — see the tests in `ci_engine.rs`). Method names and `bool` return types are unchanged; only the internals got faster. See [Simple Patterns versus Regular Expressions](#simple-patterns-versus-regular-expressions) above for current numbers against a compiled regex, and `benches/matching.rs` (run with `cargo bench`) to reproduce them.

**Bug fixes.** `starts_with_ci_alphanum`, `ends_with_ci_alphanum` and `contains_ci_alphanum` now strip non-alphanumeric characters from *both* the subject and the pattern, matching `equals_ci_alphanum`'s existing behaviour — previously only the subject was stripped, so a pattern containing punctuation or spaces behaved inconsistently across these methods. Separately, `BoundsBuilder::and_starting_with_ci` (rules feature) was silently installing the *negated* rule instead of the positive one it's named for, due to a copy-paste mistake; it now behaves as documented, and a regression test (`test_and_starting_with_ci_positive`) covers it.

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

##### _Version 0.3.17_ Added new `to_start(separator: &str)` and `to_remainder_start(separator: &str)` method

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
