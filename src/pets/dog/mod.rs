use crate::animals::{Animal, Species};
use std::fmt;

/// Create a new Canine Species
#[derive(Clone, Debug, PartialEq)]
struct Canine;

impl Species for Canine {}

impl fmt::Display for Canine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Canine")
    }
}

/// Dog
pub struct Dog<'a> {
    name: Option<&'a str>,
    species: Canine,
}

impl<'a> Dog<'a> {
    #[allow(dead_code)]
    pub fn new(name: Option<&'a str>) -> Self {
        Dog {
            name: name,
            species: Canine {},
        }
    }
}

impl<'a> fmt::Display for Dog<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name {
            Some(name) => write!(f, "Name: {}, Species: {}", name, self.species),
            None => write!(f, "Name: Unnamed, Species: {}", self.species),
        }
    }
}

impl<'a> Default for Dog<'a> {
    fn default() -> Self {
        Dog {
            name: None,
            species: Canine {},
        }
    }
}

impl<'a> Animal for Dog<'a> {
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
        let test_dog = Dog::default();
        assert_eq!(test_dog.name, None);
        assert_eq!(test_dog.species, Canine {});
    }

    #[test]
    fn new_no_name() {
        let test_dog = Dog::new(None);
        assert_eq!(test_dog.name, None);
        assert_eq!(test_dog.species, Canine {});
    }

    #[test]
    fn new_with_name() {
        let test_dog = Dog::new(Some("Rover"));
        assert_eq!(test_dog.name, Some("Rover"));
        assert_eq!(test_dog.species, Canine {});
    }

    #[test]
    fn canine_species_string() {
        let test_species = Canine {};
        assert_eq!(test_species.to_string(), "Canine");
    }
}
