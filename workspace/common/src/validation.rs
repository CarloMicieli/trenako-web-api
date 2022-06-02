use std::collections;
use std::convert;
use validator::ValidationErrors;

#[derive(Debug, PartialEq, Eq)]
pub struct ValidationResult(collections::BTreeMap<String, Vec<String>>);

impl ValidationResult {
    pub fn to_map(self) -> collections::BTreeMap<String, Vec<String>> {
        self.0
    }
}

impl convert::From<ValidationErrors> for ValidationResult {
    fn from(validation_errors: ValidationErrors) -> Self {
        let mut fields = collections::BTreeMap::new();
        for (field, errors) in validation_errors.field_errors() {
            let value = errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>();
            fields.insert(field.to_owned(), value);
        }
        ValidationResult(fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod validation_errors {
        use super::*;
        use pretty_assertions::assert_eq;
        use validator::Validate;

        #[derive(Validate)]
        struct Foo {
            #[validate(length(max = 3), required)]
            field1: Option<String>,

            #[validate(range(min = 1, max = 3))]
            field2: i32,
        }

        // #[test]
        // fn it_should_convert_validation_errors() {
        //     let foo = Foo {
        //         field1: None,
        //         field2: -1,
        //     };
        //
        //     let mut errors = collections::BTreeMap::new();
        //     errors.insert(
        //         "field1".to_string(),
        //         vec!["Validation error: required [{\"value\": Null}]"
        //             .to_string()],
        //     );
        //     errors.insert("field2".to_string(), vec!["Validation error: range [{\"min\": Number(1.0), \"max\": Number(3.0), \"value\": Number(-1)}]".to_string()]);
        //     let expected = ValidationResult(errors);
        //
        //     let validated = foo.validate().map_err(|errors| errors.into());
        //     match validated {
        //         Ok(_) => assert_eq!(1, 2),
        //         Err(errors) => {
        //             assert_eq!(expected, errors);
        //         }
        //     }
        // }
    }
}
