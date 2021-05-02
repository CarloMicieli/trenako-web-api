//! This module contains everything related to manufactures.
use std::fmt;

/// A model railways manufacturer.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Manufacturer(String);

impl Manufacturer {
    /// Creates a new Manufacturer with the given name.
    pub fn new(name: &str) -> Self {
        Manufacturer(name.to_owned())
    }

    /// Returns this Manufacturer name
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Manufacturer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod manufacturer_tests {
        use super::*;

        #[test]
        fn it_should_create_new_manufacturer() {
            let b = Manufacturer::new("ACME");
            assert_eq!("ACME", b.name());
        }

        #[test]
        fn it_should_display_manufacturer_as_string() {
            let b = Manufacturer::new("ACME");
            assert_eq!("ACME", b.to_string());
        }
    }
}
