/// Definitions for base known species
use crate::species::Species;
use std::fmt;

/// Create a new Canine Species
#[derive(Clone, Debug, PartialEq)]
pub struct Canine;

impl Species for Canine {
    fn name(&self) -> &str {
        "Canine"
    }
}

impl fmt::Display for Canine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Create a new Feline Species
#[derive(Clone, Debug, PartialEq)]
pub struct Feline;

impl Species for Feline {
    fn name(&self) -> &str {
        "Feline"
    }
}

impl fmt::Display for Feline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod test_feline {
    use super::Feline;
    use crate::species::Species;

    #[test]
    fn feline_species_name() {
        assert_eq!("Feline", Feline {}.name());
    }
}

#[cfg(test)]
mod test_canine {
    use super::Canine;
    use crate::species::Species;

    #[test]
    fn canine_species_name() {
        assert_eq!("Canine", Canine {}.name());
    }
}
