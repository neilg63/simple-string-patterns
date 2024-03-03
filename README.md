[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/simple-string-patterns)
[![crates.io](https://img.shields.io/crates/v/simple-string-patterns.svg)](https://crates.io/crates/simple-string-patterns)
[![docs.rs](https://docs.rs/simple-string-patterns/badge.svg)](https://docs.rs/simple-string-patterns)

# Simple String Patterns

This library makes it easier to match, split and extract strings in Rust. It builds on the Rust standard library. A parallel [string-patterns](https://crates.io/crates/string-patterns) crate provides extensions to work with *regular expressions*. Together, these crates aim to make working with strings as easy in Rust as it is Javascript or Python with cleaner syntax.

Simpler string matching methods such as starts_with, contains or ends_with will always perform better, especially when processing large data sets. To this end, the crate provides methods such as *starts_with_ci* and *starts_with_ci_alphanum* for basic string validation without regular expressions as well as extension methods to split strings into vectors of strings or a *head* and *tail* components.

### Method overview
- Many methods without *_ci* or *_cs* suffixes require a boolean *case_insensitive* parameter
- Methods ending in *_cs* are case-sensitive
- Methods ending in *_ci* are case-insensitive
- Methods containing *_split* return either a vector or tuple pair.
- Methods containing *_part(s)* always include leading or trailing separators and may return empty elements in vectors
- Methods containing *_segment(s)* ignore leading, trailing, repeated consecutive separators and thus exclude empty elements
- In tuples returned from *segment(s)* and *part(s)* methods, *head* means the segment before the first split and tail the remainder, while *start* means the whole string before the last split and *end* only the last part of the last matched separator.
- Enclose or wrap methods ending in *_escaped* have an optional escape character parameter
- Enclose or wrap methods ending in *_safe* insert a backslash before the any non-final occurrences of the closing characters unless already present

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

##### Filter a vector of strings by their first alphanumeric characters
```rust
// Methods ending in _alphanum are good for filtering strings that may have other
// to strings() converts as an array of &str references to a vector of strings
let sample_strs = [
  "/blue-sky.jpg",
  "----bluesky.png",
  "-B-l-u-e--sky",
  "Blueberry",
  " Blues sky thinking"
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

##### Extract the *head and tail* or *start and end* from a longer string 
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
// should yield two file names: ["photo_Nepal_Jan-2005.jpg", "pic_nepal_Dec-2004.png"]
// This will now return Vec<&str> or Vec<String> depending on the source string type.
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
  **MatchOccurrences**: Returns the indices of all ocurrences of an exact string
- **CharGroupMatch**:	Has methods to validate strings with character classes, has_digits, has_alphanumeric, has_alphabetic
- **IsNumeric**	Provides a method to check if the string may be parsed to an integer or float
- **StripCharacters**:	Set of methods to strip unwanted characters by type or extract vectors of numeric strings, integers or floats without regular expressions
- **SimpleMatch**:	Regex-free *match* methods for common validation rules, e.g. starts_with_ci_alphanum checks if the first letters or numerals in a sample string in case-insensitive mode without regular expressions.
- **SimpleMatchesMany**:	Regex-free multiple *match* methods accepting an array of StringBounds items, tuples or patterns and returning a vector of boolean results. matched_conditional
- **SimpleMatchAll**:	Regex-free multiple *match* methods accepting an array of StringBounds items, tuples or patterns and returning a boolean if all are matched
- **SimpleFilterAll**: Applies simple Regex-free multiple *match* methods to an array or vector of strings and returns a filtered vector of string slices
- **SimpleEnclose**: Wraps strings in pairs of matching characters with variants for different escape character rules
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
- **CharType**: Defines categories, sets or ranges of characters as well as single characters.
  - Any: will match any characters
  - DecDigit => Match 0-9 only (is_ascii_digit)
  - Digit(radix) => Match digit with the specified radix (e.g. 16 for hexadecimal)
  - Numeric => Match number-like characters in the decimal base. Unlike the is_numeric() extension method this excludes . and -. Use to_numbers_conditional() to extract valid decimal number as strings;
  - AlphaNum => Match any alphanumeric characters (is_alphanumeric)
  - Lower => Match lower case letters (is_lowercase),
  - Upper => Match upper case letters (is_uppercase)
  - Alpha => Match any letters in most supported alphabets (is_alphabetic)
  - Spaces => Match spaces c.is_whitespace(),
  - Punctuation => c.is_ascii_punctuation(),
  - Char(char) => match a single chars
  - Chars(&[char]) => Match an array of chars
  - Range(Range<char>) => Match an Range e.g. 'a'..'d' will include a, b and c, but not d. This follows the Unicode sequence.
  - Between(c1, c2) => Match characters betweeen the specified characters e.g. Between('a', 'd') will include d.

### Dev Notes

This crate serves as a building block for other crates as well as to supplement a future version of *string-patterns*. Some updates reflect minor editorial changes.

Versions of the *string-patterns* crate before 0.3.0 contained many of these extensions. Since version 0.3.0 all traits, enums and methods defined in this *simple-string-patterns* have been removed. These crates supplement each other, but may be installed independently.
