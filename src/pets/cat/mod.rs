use crate::animals::Animal;
use crate::species::default::Feline;
use crate::species::Species;
use std::fmt;

/// Cat
pub struct Cat {
    name: Option<String>,
    species: Feline,
}

impl Cat {
    pub fn new(name: Option<String>) -> Self {
        Cat {
            name,
            species: Feline {},
        }
    }
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "Name: {}, Species: {}", name, self.species),
            None => write!(f, "Name: Unnamed, Species: {}", self.species),
        }
    }
}

impl Default for Cat {
    fn default() -> Self {
        Cat {
            name: None,
            species: Feline {},
        }
    }
}

impl Animal for Cat {
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
        let test_cat = Cat::new(Some("Tiger".to_string()));
        assert_eq!(test_cat.name, Some("Tiger".to_string()));
        assert_eq!(test_cat.species, Feline {});
    }

    #[test]
    fn canine_species_string() {
        let test_species = Feline {};
        assert_eq!(test_species.to_string(), "Feline");
    }
}
