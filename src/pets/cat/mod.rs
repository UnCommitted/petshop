use crate::animals::{Animal, Species};
use std::fmt;

/// Create a new Feline Species
#[derive(Clone, Debug, PartialEq)]
struct Feline;

impl Species for Feline {}

impl fmt::Display for Feline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Feline")
    }
}

/// Cat
pub struct Cat<'a> {
    name: Option<&'a str>,
    species: Feline,
}

impl<'a> Cat<'a> {
    #[allow(dead_code)]
    pub fn new(name: Option<&'a str>) -> Self {
        Cat {
            name: name,
            species: Feline {},
        }
    }
}

impl<'a> fmt::Display for Cat<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name {
            Some(name) => write!(f, "Name: {}, Species: {}", name, self.species),
            None => write!(f, "Name: Unnamed, Species: {}", self.species),
        }
    }
}

impl<'a> Default for Cat<'a> {
    fn default() -> Self {
        Cat {
            name: None,
            species: Feline {},
        }
    }
}

impl<'a> Animal for Cat<'a> {
    fn name(&self) -> Option<&str> {
        self.name
    }

    fn species(&self) -> Box<dyn Species> {
        Box::new(self.species.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_correct() {
        let test_cat = Cat::default();
        assert_eq!(test_cat.name, None);
        assert_eq!(test_cat.species, Feline {});
    }

    #[test]
    fn new_no_name() {
        let test_cat = Cat::new(None);
        assert_eq!(test_cat.name, None);
        assert_eq!(test_cat.species, Feline {});
    }

    #[test]
    fn new_with_name() {
        let test_cat = Cat::new(Some("Tiger"));
        assert_eq!(test_cat.name, Some("Tiger"));
        assert_eq!(test_cat.species, Feline {});
    }

    #[test]
    fn canine_species_string() {
        let test_species = Feline {};
        assert_eq!(test_species.to_string(), "Feline");
    }
}
