use crate::species::Species;
///
/// Trait required for all animals in the petshop
pub trait Animal: std::fmt::Display {
    fn name(&self) -> Option<String>;
    fn set_name(&mut self, new_name: Option<String>);
    fn species(&self) -> Box<dyn Species>;
}
