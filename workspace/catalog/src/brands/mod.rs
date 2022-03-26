//! This module contains everything related to brands.
pub mod data;
pub mod use_cases;

use common::slug::Slug;
use std::{default, fmt, str};
use url::Url;

/// A model railways manufacturer.
#[derive(Debug, Default, Builder)]
pub struct Brand {
    name: String,
    slug: Slug,
    brand_type: BrandType,
    company_name: Option<String>,
    website: Option<Url>,
    active: bool,
}

impl Brand {
    /// Returns this brand name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns this brand slug
    pub fn slug(&self) -> &Slug {
        &self.slug
    }

    /// Returns this brand company name
    pub fn company_name(&self) -> Option<&String> {
        self.company_name.as_ref()
    }

    /// Returns this brand type
    pub fn brand_type(&self) -> BrandType {
        self.brand_type
    }

    pub fn website(&self) -> Option<&Url> {
        self.website.as_ref()
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl fmt::Display for Brand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// The different kinds for railway models brands
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BrandType {
    /// These manufactures produce models using the die casting method
    Industrial,

    /// These manufacturers produce models which are made of brass or similar alloys.
    ///
    /// They are usually more expensive than the industrial series due to the limited
    /// production quantities and the "hand made" nature of the production
    BrassModels,
}

impl default::Default for BrandType {
    fn default() -> Self {
        BrandType::Industrial
    }
}

impl str::FromStr for BrandType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("The brand type value cannot be blank");
        }

        match s {
            "industrial" => Ok(BrandType::Industrial),
            "brass_models" => Ok(BrandType::BrassModels),
            _ => Err("Invalid value for brand type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brand_tests {
        use super::*;

        #[test]
        fn it_should_create_new_brands() {
            let b = acme();
            assert_eq!("ACME", b.name());
            assert_eq!(&Slug::new("acme"), b.slug());
        }

        #[test]
        fn it_should_display_brand_as_string() {
            let b = acme();
            assert_eq!("ACME", b.to_string());
        }

        fn acme() -> Brand {
            BrandBuilder::default()
                .name("ACME".to_owned())
                .slug(Slug::new("ACME"))
                .active(true)
                .brand_type(BrandType::Industrial)
                .company_name(None)
                .website(None)
                .build()
                .unwrap()
        }
    }

    mod brand_types_tests {}
}
