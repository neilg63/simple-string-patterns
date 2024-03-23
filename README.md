[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/simple-string-patterns)
[![crates.io](https://img.shields.io/crates/v/simple-string-patterns.svg)](https://crates.io/crates/simple-string-patterns)
[![docs.rs](https://docs.rs/simple-string-patterns/badge.svg)](https://docs.rs/simple-string-patterns)

# Simple String Patterns

This library makes it easier to match, split and extract strings in Rust. It builds on the Rust standard library. A parallel [string-patterns](https://crates.io/crates/string-patterns) crate provides extensions to work with *regular expressions*. Together, these crates aim to make working with strings as easy in Rust as it is Javascript or Python with cleaner syntax.

Simpler string matching methods such as *starts_with, contains or ends_with* will always perform better, especially when processing large data sets. Methods such as *starts_with_ci* and *starts_with_ci_alphanum* build on these core methods to facilitate string manipulation without *regular expressions*.

Version 0.3.0 sees a radical revision of the enums used to define string matching rules in the *matched_by_rules()*, *matched_conditional()*, *filter_all_rules()* and *filter_any_rules()* methods.

## Simple Patterns versus Regular Expressions
The main advantages of *simple-string-patterns* lie in its readability and miniminal additional overhead in lightweight applications that would not otherwise need regex support. Under the hood, regular expression engines compile regex syntax and convert them into more efficient string matching subroutines. Preliminary benchmarks show that rule sets with basic matching methods such as *contains_ci* perform better than their regex counterparts, but if you need to add multiple nested rules, a *regex* may be faster. The sibling regex-powered *string-patterns* crate makes this very easy. This crate is best suited to small utilities that need to process large quantities of strings with a range of highly predictable formats, e.g. in cryptography, logging.

### Method overview
- Many methods without *_ci* or *_cs* suffixes require a boolean *case_insensitive* parameter
- Methods ending in *_cs* are case-sensitive
- Methods ending in *_ci* are case-insensitive
- Methods ending in *_ci_alphanum* are case-insensitive and remove all non-alphanumeric letters from the sample string before camparison
- Methods ending in *_rules* accept a *BoundsBuilder* object created via bounds_builder()
- Methods ending in *_conditional* accept an array of *StringBounds* rules
- Methods with *filter_all* filter arrays or vectors that match all of the rules (and logic)
- Methods with *filter_any* filter arrays or vectors that match any of the rules (or logic)
- Methods with *_split* return either a vector or tuple pair.
- Methods with *_part(s)* always include leading or trailing separators and may return empty elements in vectors
- Methods with *_segment(s)* ignore leading, trailing, repeated consecutive separators and thus exclude empty elements
- In tuples returned from segment(s) and part(s) methods, head means the segment before the first split and tail the remainder, while start means the whole string before the last split and end only the last part of the last matched separator.
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
As of verson 0.3.0 you may add nested rule sets with *and* / *or* logic. The former case is true only if all conditions are met, while the latter is true if any of the conditions are met. The *BoundsBuilder* struct now has a set of methods starting with *and* or *or*. You may call *and(rules: BoundsBuilder)* or *or(rules: BoundsBuilder)* directly with a nested rule set if you have a mix of rule types. However, if all rules have the same bounds, other methods accepting a simple array of patterns are available, e.g.
- or_starts_with_ci(patterns: &[&str])
- or_starts_with_ci_alphanum(patterns: &[&str])
- or_contains_ci(patterns: &[&str])
- or_ends_with_ci(patterns: &[&str])

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
  .or_ends_with(&[".jpg", ".jpeg"]);

let matched_files = filenames.filter_all_rules(&rules);
/// Should yield an array with "my_CaT_2020.jpg" and "daughters_Dog_2023.png"
```
The above example reproduces the following example *regular expression* /(cat|dog).*?\.jpe?g$/. The _alphanum-suffixed variants let match only on numbers and letters within a string, i.e. ignorning any spaces or punctuation. 

##### Filter by any pattern rules without regular expressions
```rust
/

// The same array may also be expressed via the new bounds_builder() function with chainable rules:
// Call .as_vec() at the end
let mixed_conditions = bounds_builder()
  .containing_ci("nepal")
  .containing_ci("india")
  .as_vec();

let file_names = [
  "edited-img-Nepal-Feb-2003.psd",
  "image-Thailand-Mar-2003.jpg",
  "photo_Nepal_Jan-2005.jpg",
  "image-India-Mar-2003.jpg",
  "pic_nepal_Dec-2004.png"
];
  
let nepal_and_india_source_files: Vec<&str> = file_names.filter_any_conditional(&mixed_conditions);
// should yield two file names: ["edited-img-Nepal-Feb-2003.psd", "photo_Nepal_Jan-2005.jpg", "image-India-Mar-2003.jpg", "pic_nepal_Dec-2004.png"]

/// You can combine the above with an filter_all_conditional
let extension_rules = bounds_builder().ending_with_ci(".jpg");
let nepal_and_india_source_files_jpgs: Vec<&str> = file_names.filter_any_conditional(&mixed_conditions).filter_any_conditional(&extension_rules);
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
  **MatchOccurrences**: Has methods to return the indices of all ocurrences of an exact string (find_matched_indices) or single character (find_char_indices);
- **CharGroupMatch**:	Has methods to validate strings with character classes, has_digits, has_alphanumeric, has_alphabetic
- **IsNumeric**	Provides a method to check if the string may be parsed to an integer or float
- **StripCharacters**:	Set of methods to strip unwanted characters by type or extract vectors of numeric strings, integers or floats without regular expressions
- **SimpleMatch**:	Regex-free *match* methods for common validation rules, e.g. starts_with_ci_alphanum checks if the first letters or numerals in a sample string in case-insensitive mode without regular expressions.
- **SimpleMatchesMany**:	Regex-free multiple *match* methods accepting an array of StringBounds items, tuples or patterns and returning a vector of boolean results. matched_conditional
- **SimpleMatchAll**:	Regex-free multiple *match* methods accepting an array of StringBounds items, tuples or patterns and returning a boolean if all are matched
- **SimpleFilterAll**: Applies simple Regex-free multiple *match* methods to an array or vector of strings and returns a filtered vector of string slices
- **SimpleEnclose**: Wraps strings in pairs of matching characters with variants for different escape character rules
- **ToStrings**:	Converts arrays or vectors of strs to a vector of owned strings
- **ToSegments**: Splits strings into parts, segments or head and tail pairs on a separator
- **ToSegmentFromChars**: Splits strings into parts on any of any array of characters

### Enums
- **CaseMatchMode**: Defines case-sensitivity mode

  Options:
  - **Sensitive**: Case sensitive
  - **Insensitive**: Case-insensitive, casts both the needle and haystack all strings to lower case for comparison
  - **AlphanumInsensitive**: Removes all non-alphanumeric characters from the sample string and cast both the needle and haystack to lower case for comparison
- **StringBounds**: Defines simple match rules with the pattern and a positivty flag, e.g. StringBounds::Contains("report", true, CaseMatchMode::Insensitive) or StringBounds::EndsWith(".docx", CaseMatchMode::Insensitive). The *bounds_builder* method helps build these rule sets.
  
  Options:
  - StartsWith(&str, bool, CaseMatchMode) *starts with* + boolean positivity and CaseMatchMode flags
  - EndsWith(&str, bool, CaseMatchMode) case-insensitive *ends with* + is_positive flag
  - Contains(&str, bool, CaseMatchMode) case-insensitive *contains* + is_positive flag
  - Whole(&str, bool, CaseMatchMode) case-insensitive whole string match + is_positive flag
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
  - Char(char) => match a single character
  - Chars(&[char]) => Match an array of characters
  - Range(Range<char>) => Match an Range e.g. 'a'..'d' will include a, b and c, but not d. This follows the Unicode sequence.
  - Between(c1, c2) => Match characters betweeen the specified characters e.g. Between('a', 'd') will include d.

### Structs

#### BoundsBuilder
This struct helps you build string pattern rules for use with the *matched_by_rules()*, *filter_all_rules()* and *filter_any_rules()* methods.
The *bounds_builder()* function returns a base instance on which you may chain any number of rules and sub-rules.
- *starting_with_ci(pattern: &str)* => Start with a pattern in case-insensitive mode
- *starting_with_ci_alphanum(pattern: &str)* => Start with a pattern in case-insensitive mode
- *starting_with_cs(pattern: &str)* => Start with a pattern in case-sensitive mode after stripping all non-alphanumeric characters
- *not_starting_with_ci(pattern: &str)* => Does not start with a pattern in case-insensitive mode
- *not_starting_with_cs(pattern: &str)* => Does not start with a pattern in case-sensitive mode
- *containing_ci(pattern: &str)* => Contains a pattern in case-insensitive mode
- *containing_cs(pattern: &str)* => Contains a pattern in case-sensitive mode
- *not_containing_ci(pattern: &str)* => Does not contain a pattern in case-insensitive mode
- *not_containing_ci_alphanum(pattern: &str)* => Does not contain a pattern in case-insensitive mode after stripping all non-alphanumeric characters
- *not_containing_cs(pattern: &str)* => Does not contain a pattern in case-sensitive mode
- *is_ci(pattern: &str)* => Matches a whole pattern in case-insensitive mode
- *is_cs(pattern: &str)* => Matches a whole  pattern in case-sensitive mode
- *is_not_ci(pattern: &str)* => Does not match a whole pattern in case-insensitive mode
- *is_not_cs(pattern: &str)* => Does not match a whole  pattern in case-sensitive mode
- *and(rules: BoundsBuilder) => Adds a subset of rules with *and* logic.
- *or(rules: BoundsBuilder) => Adds a subset of rules with *or* logic.

### Dev Notes

This crate serves as a building block for other crates as well as to supplement a future version of *string-patterns*. Some updates reflect minor editorial changes.

##### *Version 0.3.0* expands the range of rules available BoundsBuilder
As this version introduces a radical revision to the StringBouunds enum with new enums for BoundsPosition and CaseMatchMode that affect an expanded range of rules available via *bounds_builder()* for use with matched_by_rules(), filter_all_rules() and filter_any_rules().

Full documentation for the 0.2.* series is available in the [Github repo](https://github.com/neilg63/simple-string-patterns) in the *v0-2* branch.

##### *Version 0.2.5* introduces SimpleMatchAny and Whole matches in StringBounds.

This supplements SimpleMatchAll to apply *or* logic with rules sets (StringBound, tuples or simple strs). The StringBounds enum now has whole string match options (with case-insensitive and case-sensitive variants) to accommodate a mix of partial and whole string matches. It also adds a range of single-argument methods for bounds_builder().

Versions of the *string-patterns* crate before 0.3.0 contained many of these extensions. Since version 0.3.0 all traits, enums and methods defined in this *simple-string-patterns* have been removed. These crates supplement each other, but may be installed independently.

##### Version 0.2.2 introduces three new features:
- *bounds_builder()* makes it easier to define string matching rules methods requiring an array of *StringBounds* rules such as filter_all_conditional(). See example above.
- *ToSegmentFromChars* provides new methods to split on any of an array of characters, e.g. when processing common patterns that may use a predictable set of separators. This mimics characters classes in regular expressions and is more efficient when you only need to allow for a limited set of split characters.
- *MatchOccurrences* has a variant *find_char_indices* method that accepts a *char* rather than a *&str*. This avoids any need to cast a character to a string.

