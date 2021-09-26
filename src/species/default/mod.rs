/// Definitions for base known species
use crate::species::Species;
use std::fmt;

/// Create a new Canine Species
#[derive(Clone, Debug, PartialEq)]
pub struct Canine;

impl Species for Canine {}

impl fmt::Display for Canine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Canine")
    }
}

/// Create a new Feline Species
#[derive(Clone, Debug, PartialEq)]
pub struct Feline;

impl Species for Feline {}

impl fmt::Display for Feline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Feline")
    }
}
