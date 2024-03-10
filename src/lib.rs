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

pub use crate::enums::*;
pub use crate::alphanumeric::*;
pub use crate::segments::*;
pub use crate::simple_match::*;
pub use crate::enclose::*;
pub use crate::to_strings::*;
pub use crate::char_type::*;
pub use crate::bounds_builder::*;