mod utils;
pub mod enums;
pub mod alphanumeric;
pub mod segments;
pub mod simple_match;
pub mod enclose;
pub mod to_strings;
pub mod char_type;
pub mod bounds_builder;

/// This library provides a set of traits and extension methods for &str and/or String
/// to facilitate common string manipulations routines that may otherwise require multiple steps
/// Some methods have variants with a case_insensitive flag and without (_ci and _cs).
/// Always consider the simplest strategy for extracting text, e.g. via to_head_tail(), to_segments().

/// Core enums defining string matching rules and relative positions
pub use crate::enums::*;
/// Methods to strip or filter character types within strings and to extract integers or floats
pub use crate::alphanumeric::*;
/// Split strings into tuples or vectors of strings
pub use crate::segments::*;
/// Simple string match methods
pub use crate::simple_match::*;
/// Wrap or enclose strings in matching or complementary characters
pub use crate::enclose::*;
/// cast to vector of owned strings
pub use crate::to_strings::*;
pub use crate::char_type::*;
/// rules builder
pub use crate::bounds_builder::*;