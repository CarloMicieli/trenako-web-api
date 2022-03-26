use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use slug::slugify;
use std::default;
use std::fmt;
use std::fmt::Formatter;

/// A SEO friendly string
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Slug(String);

impl Slug {
    /// Create a new Slug from the string slice in input.
    pub fn new(value: &str) -> Self {
        Slug(slugify(value))
    }

    /// Create a new Slug composing the two string slices as input.
    pub fn of(first: &str, second: &str) -> Self {
        let value = format!("{}-{}", slugify(first), slugify(second));
        Slug(value)
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for Slug {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

struct SlugString;

impl<'de> Visitor<'de> for SlugString {
    type Value = Slug;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a slugified string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let s2 = slugify(s);
        if s == s2 {
            Ok(Slug::new(s))
        } else {
            Err(serde::de::Error::custom(&format!("Invalid slug: {}", s)))
        }
    }
}

impl<'de> Deserialize<'de> for Slug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SlugString)
    }
}

impl default::Default for Slug {
    fn default() -> Self {
        Slug::new("")
    }
}

pub trait AsSlug {
    /// Convert this value to a slug, a SEO friendly text
    fn as_slug(&self) -> Slug;

    /// Combine two values to produce a new Slug
    fn combine_with<U: AsSlug>(&self, other: &U) -> Slug {
        let slug1 = self.as_slug();
        let slug2 = other.as_slug();
        Slug(format!("{}-{}", slug1, slug2))
    }
}

impl AsSlug for String {
    fn as_slug(&self) -> Slug {
        Slug::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::slug::Slug;

    mod slug_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use serde_json;

        #[test]
        fn it_should_create_slugs_from_strings() {
            let result = Slug::new("my first string");
            assert_eq!(result.to_string(), "my-first-string");
        }

        #[test]
        fn it_should_create_slugs_from_two_string_slices() {
            let result = Slug::of("first item", "second item");
            assert_eq!(result.to_string(), "first-item-second-item");
        }

        #[test]
        fn it_should_serialize_to_json() {
            let slug = Slug::new("my slug");
            let serialized = serde_json::to_string(&slug)
                .expect("error during serialization");

            assert_eq!("\"my-slug\"", serialized);
        }

        #[test]
        fn it_should_deserialize_from_json() {
            let deserialized: Slug = serde_json::from_str("\"my-slug\"")
                .expect("error during deserialization");
            assert_eq!(Slug::new("my-slug"), deserialized);
        }
    }
}
