[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/simple-string-patterns)
[![crates.io](https://img.shields.io/crates/v/string-patterns.svg)](https://crates.io/crates/simple-string-patterns)
[![docs.rs](https://docs.rs/string-patterns/badge.svg)](https://docs.rs/simple-string-patterns)

# Simple String Patterns

This library makes it easier to match, split and extract strings in Rust. It builds on the Rust standard library with other dependencies. A parallel [string-patterns]()https://crates.io/crates/string-patterns crate provides extensions to work with *regular expressions*. Together, they aims to make working with strings as easy in Rust as it is Javascript or Python with cleaner syntax. Simpler string matching methods such as starts_with, contains or ends_with will always perform better, especially when processing large data sets. To this end, the crate provides methods such as *starts_with_ci* and *starts_with_ci_alphanum* for basic string validation without regular expressions as well as extension methods to split strings into vectors of strings or a *head* and *tail* components.

### Method overview
- Many methods without *_ci* or *_cs* suffixes require a boolean *case_insensitive* parameter
- Methods ending in *_cs* are case-sensitive
- Methods ending in *_ci* are case-insensitive
- Methods containing *_split* return either a vector or tuple pair.
- Methods containing *_part(s)* always include leading or trailing separators and may return empty elements in vectors
- Methods containing *segment(s)* ignore leading, trailing, repeated consecutive separators and thus exclude empty elements
- In tuples returned from *segment(s)* and *part(s)* methods, *head* means the segment before the first split and tail the remainder, while *start* means the whole string before the last split and *end* only the last part of the last matched separator.

Version 0.2.0 introduced additional methods to capture and count matched strings with offsets and version 0.2.5 added methods to match, replace, capture and count words without intrusive word boundary anchors.


##### Simple case-insensitive match
```rust
let str_1 = "Dog food";
if str_1.starts_with_ci("dog") {
  println!("{} is dog-related", str_1);
}
```

##### Simple case-insensitive match on the alphanumeric characters only in a longer text
```rust
// This method is handy for validating text values from external data sources with
// inconsistent naming conventions, e.g. first-name, first_name, firstName or "first name"
let str_1 = "Do you spell hip-hop with a hyphen?";
if str_1.contains_ci_alphanum("hiphop") {
  println!("{} is hip-hop-related", str_1);
}
```

##### Extract the first match from a string
```rust
let str_1 = "The park has many lions, spotted hyenas, leopards, rhinoceroses, hippopotamuses, giraffes, cheetahs and baboons";
if let Some(matched_item) = str_1.pattern_first_match(r#"\bspotted\s+\w+\b"#, true) {
  println!("`{}` occurs between positions {} and {}", matched_item.as_str(), matched_item.start(), matched_item.end());
}
```

##### Match within an array of strings
```rust
let sample_strs = [
  "pictures_Italy-1997",
  "photos-portugal-2001",
  "imagini-italia_2002",
  "images-france-2003",
];
let test_pattern = r#"[^a-z]ital(y|ia)"#; // matches 'italy' or 'italia'
// The regular expression will only be compiled once
if sample_strs.pattern_match_ci(test_pattern) {
  println!("Some of these folders are related to Italy");
}

// Filter the above array
let filtered_strs = sample_strs.pattern_matches_filtered_ci(test_pattern);
// should yield ["pictures_Italy-1997","imagini-italia_2002"]
```

##### Extract the third non-empty segment of a long path name
```rust
let path_string = "/var/www/mysite.com/web/uploads";
if let Some(domain) = path_string.to_segment("/", 2) {
  println!("The domain folder name is: {}", domain); // "mysite.com" is an owned string
}
```

##### Extract the *head and tail* or *start and end* from a longer string 
```rust
let test_string = "long-list-of-technical-words"
let (head, tail) = test_string.to_head_tail("-");
println!("Head: {}, tail: {}", head, tail); // Head: long, tail: list-of-technical-words

let (start, end) = test_string.to_start_end("-");
println!("Start: {}, end: {}", start, end); // Start: long-list-of-technical, end: words
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

##### Extract three float values from a longer string
```rust

let input_str = "-78.29826, 34.15 160.9";
// the pattern expects valid decimal numbers separated by commas and/or one or more spaces
let split_pattern = r#"(\s*,\s*|\s+)"#;

let numbers: Vec<f64> = input_str.pattern_split_cs(split_pattern)
    .into_iter().map(|s| s.to_first_number::<f64>())
    .filter(|nr| nr.is_some())
    .map(|s| s.unwrap()).collect();
// yields a vector of three f64 numbers [-78.29826, 34.15, 160.9];
```

##### Match multiple patterns without regular expressions
```rust
// Match only file names that contain the character sequence "nepal" and do not end in .psd 
// This is very useful for prefiltering large sets of simple strings 
// representing things like file names.
// Ci, Cs suffixes mean case-insensitive and case-sensitive respectively
  let mixed_conditions = [
    StringBounds::ContainsCi("nepal", true),
    StringBounds::EndsWithCi(".psd", false),
  ];

  let file_names = [
    "edited-img-Nepal-Feb-2003.psd",
    "image-Thailand-Mar-2003.jpg",
    "photo_Nepal_Jan-2005.jpg",
    "image-India-Mar-2003.jpg",
    "pic_nepal_Dec-2004.png"
  ];
  
  let nepal_source_files: Vec<&str> = file_names.filter_all_conditional(&mixed_conditions);
  /// should yield two file names: ["photo_Nepal_Jan-2005.jpg", "pic_nepal_Dec-2004.png"]
```


##### Test the proximity of two words.
NB: This will be moved to another crate in future versions. The functionality can be reproduced from **String.pattern_captures()**.
```rust
let source_str = "Lions are unique among cats in that they live in a group or pride.";
// Do the words 'lion(s)' and 'cat(s)' occur within 20 characters of each other?
if source_str.match_words_by_proximity("lions?", "cats?", -20, 20, true) {
  println!("This sentence mentions lions in the context of cats");
}
```

### Traits

- **CharGroupMatch**:	Has methods to validate strings with character classes, has_digits, has_alphanumeric, has_alphabetic
- **IsNumeric**	Provides a method to check if the string may be parsed to an integer or float
- **StripCharacters**:	Set of methods to strip unwanted characters by type or extract vectors of numeric strings, integers or floats without regular expressions
- **SimpleMatch**:	Regex-free *match* methods for common validation rules, e.g. starts_with_ci_alphanum checks if the first letters or numerals in a sample string in case-insensitive mode without regular expressions.
- **SimpleMatchesMany**:	Regex-free multiple *match* methods accepting an array of StringBounds items, tuples or patterns and returning a vector of boolean results. matched_conditional
- **SimpleMatchAll**:	Regex-free multiple *match* methods accepting an array of StringBounds items, tuples or patterns and returning a boolean if all are matched
- **SimpleFilterAll**: Applies simple Regex-free multiple *match* methods to an array or vector of strings and returns a filtered vector of string slices
- **ToStrings**:	Converts arrays or vectors of strs to a vector of owned strings

### Enums
- **StringBounds**: Defines simple match rules with the pattern and a positivty flag, e.g. StringBounds::ContainsCi("report", true) or StringBounds::EndsWithCi(".docx", false). The *Ci* and *Cs* variants affect case-sensitivity.
  Options:
  - StartsWithCi(&str, bool) case-insensitive *starts with* + boolean positivity flag
  - EndsWithCi(&str, bool) case-insensitive *ends with* + is_positive flag
  - ContainsCi(&str, bool) case-insensitive *contains* + is_positive flag
  - StartsWithCs(&str, bool) case-sensitive *starts with* + is_positive flag
  - EndsWithCs(&str, bool) case-sensitive *ends with* + is_positive flag
  - ContainsCs(&str, bool) case-sensitive *contains* + is_positive flag

### Dev Notes
This crate is still in its alpha stage and serves as a building block for other crates as well as to supplement a future version of *string-patterns*.
