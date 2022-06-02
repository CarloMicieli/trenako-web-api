use std::fmt;

/// It represent a catalog item number.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ItemNumber(String);

impl ItemNumber {
    /// Creates a new ItemNumber from the string slice, it needs to panic when the
    /// provided string slice is empty.
    pub fn new(value: &str) -> Result<Self, &'static str> {
        if value.is_empty() {
            Err("Item number cannot blank")
        } else {
            Ok(ItemNumber(value.to_owned()))
        }
    }

    /// Returns the item number value, this cannot be blank.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ItemNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod item_number_tests {
        use super::*;

        #[test]
        fn it_should_create_new_item_numbers() {
            let n = ItemNumber::new("123456");
            assert_eq!(n.unwrap().value(), "123456");
        }

        #[test]
        fn it_should_fail_to_convert_empty_string_slices_as_item_numbers() {
            let item_number = ItemNumber::new("");
            assert!(item_number.is_err());
        }
    }
}
