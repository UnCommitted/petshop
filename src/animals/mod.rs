use crate::species::Species;

/// Trait required for all animals in the petshop
pub trait Animal {
    fn name(&self) -> Option<&str>;
    fn species(&self) -> Box<dyn Species>;
}
