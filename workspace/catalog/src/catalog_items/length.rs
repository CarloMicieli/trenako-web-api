use std::fmt::Formatter;
use std::{fmt, ops};

/// The lenght over buffer for the model.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct LengthOverBuffer(u32);

impl LengthOverBuffer {
    /// Creates a new value, the provided value must be positive.
    pub fn new(value: u32) -> Self {
        if value == 0 {
            panic!("Length over buffer cannot be 0 or negative");
        }
        LengthOverBuffer(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for LengthOverBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Add for LengthOverBuffer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign for LengthOverBuffer {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod length_over_buffer_tests {
        use super::*;

        #[test]
        fn it_should_create_a_new_value() {
            let len = LengthOverBuffer::new(100);
            assert_eq!(len.value(), 100);
        }

        #[test]
        fn it_should_add_two_length_values() {
            let len1 = LengthOverBuffer::new(21);
            let len2 = LengthOverBuffer::new(21);

            let result = len1 + len2;

            assert_eq!(result.value(), 42);
        }

        #[test]
        fn it_should_print_length_as_strings() {
            let len = LengthOverBuffer::new(100);
            assert_eq!(len.to_string(), "100".to_string())
        }
    }
}
