#[cfg(feature = "rules")]
pub mod bounds_builder;
#[cfg(feature = "chain")]
pub mod chain;
#[cfg(feature = "rules")]
pub mod enums;
pub mod matches;
#[cfg(feature = "rules")]
pub mod rules;
#[cfg(feature = "rules")]
mod utils;

/// Build a set of string matching rules
#[cfg(feature = "rules")]
pub use crate::bounds_builder::*;
/// Fluent, chainable and_*/or_* match expressions over a single string
#[cfg(feature = "chain")]
pub use crate::chain::*;
/// Core enums defining string matching rules and relative positions
#[cfg(feature = "rules")]
pub use crate::enums::*;
/// Simple string match methods
pub use crate::matches::*;
/// Methods with string bounds rules
#[cfg(feature = "rules")]
pub use crate::rules::*;
/// Methods to strip or filter character types within strings and to extract integers or floats
pub use alphanumeric::{CharGroupMatch, CharType, IsNumeric, StripCharacters};
