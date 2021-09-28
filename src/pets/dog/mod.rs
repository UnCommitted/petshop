use crate::animals::Animal;
use crate::species::default::Canine;
use crate::species::Species;
use std::fmt;

/// Dog
pub struct Dog {
    name: Option<String>,
    species: Canine,
}

impl Dog {
    pub fn new(name: Option<String>) -> Self {
        Dog {
            name,
            species: Canine {},
        }
    }
}

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "Name: {}, Species: {}", name, self.species),
            None => write!(f, "Name: Unnamed, Species: {}", self.species),
        }
    }
}

impl Default for Dog {
    fn default() -> Self {
        Dog {
            name: None,
            species: Canine {},
        }
    }
}

impl Animal for Dog {
    fn name(&self) -> Option<String> {
        self.name.clone()
    }

    fn species(&self) -> Box<dyn Species> {
        Box::new(self.species.clone())
    }

    fn set_name(&mut self, new_name: Option<String>) {
        self.name = new_name;
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
        let test_dog = Dog::new(Some("Rover".to_string()));
        assert_eq!(test_dog.name, Some("Rover".to_string()));
        assert_eq!(test_dog.species, Canine {});
    }

    #[test]
    fn canine_species_string() {
        let test_species = Canine {};
        assert_eq!(test_species.to_string(), "Canine");
    }
}
