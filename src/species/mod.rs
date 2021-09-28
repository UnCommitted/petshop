pub mod default;

/// Used only for allowing dynamic known species
pub trait Species: std::fmt::Display {
    fn name(&self) -> &str;
}
