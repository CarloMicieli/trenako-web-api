use std::fmt;

/// A Uniform Resource Name (URN) is a Uniform Resource Identifier (URI) that uses the urn scheme.
/// URNs are globally unique persistent identifiers assigned within defined namespaces so they will
/// be available for a long period of time, even after the resource which they identify ceases to
/// exist or becomes unavailable. URNs cannot be used to directly locate an item and need not be
// resolvable, as they are simply templates that another parser may use to find an item.
#[derive(Debug, Clone)]
pub struct Urn {
    namespace_identifier: String,
    namespace_specific: String,
    q_component: Option<String>,
    r_component: Option<String>,
}

static TRENAKO: &str = "trenako";

impl Urn {
    /// Creates a new Uniform Resource Name (URN) for errors
    pub fn error() -> Urn {
        Urn {
            namespace_identifier: String::from(TRENAKO),
            namespace_specific: String::from("internal-server-error"),
            q_component: None,
            r_component: None,
        }
    }

    /// Creates a new Uniform Resource Name (URN) for invalid requests
    pub fn invalid_request(entity: &str) -> Urn {
        Urn {
            namespace_identifier: String::from(TRENAKO),
            namespace_specific: String::from("bad-request"),
            q_component: Some(entity.to_owned()),
            r_component: None,
        }
    }

    /// Creates a new Uniform Resource Name (URN) for unprocessable entities
    pub fn unprocessable_entity(entity: &str) -> Urn {
        Urn {
            namespace_identifier: String::from(TRENAKO),
            namespace_specific: String::from("unprocessable-entity"),
            q_component: Some(entity.to_owned()),
            r_component: None,
        }
    }

    pub fn instance(uuid: &uuid::Uuid) -> Urn {
        Urn {
            namespace_identifier: String::from(TRENAKO),
            namespace_specific: uuid.to_string(),
            q_component: None,
            r_component: None,
        }
    }

    /// Creates a new Uniform Resource Name (URN)
    pub fn from_uuid(entity: &str, uuid: &uuid::Uuid) -> Urn {
        Urn {
            namespace_identifier: String::from(TRENAKO),
            namespace_specific: entity.to_owned(),
            q_component: Some(uuid.to_string()),
            r_component: None,
        }
    }
}

impl fmt::Display for Urn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let q_comp = self
            .q_component
            .as_deref()
            .map(|s| format!(":{}", s))
            .unwrap_or_else(|| String::from(""));
        let r_comp = self
            .r_component
            .as_deref()
            .map(|s| format!(":{}", s))
            .unwrap_or_else(|| String::from(""));

        write!(
            f,
            "urn:{}:{}{}{}",
            self.namespace_identifier, self.namespace_specific, q_comp, r_comp
        )
    }
}

impl serde::ser::Serialize for Urn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_produce_urn_values_for_errors() {
        let urn = Urn::error();
        assert_eq!("urn:trenako:internal-server-error", urn.to_string());
    }

    #[test]
    fn should_produce_urn_values_for_invalid_requests() {
        let urn = Urn::invalid_request("entity.name");
        assert_eq!("urn:trenako:bad-request:entity.name", urn.to_string());
    }

    #[test]
    fn should_produce_urn_values_for_unprocessable_entitys() {
        let urn = Urn::unprocessable_entity("entity.name");
        assert_eq!(
            "urn:trenako:unprocessable-entity:entity.name",
            urn.to_string()
        );
    }

    #[test]
    fn should_produce_urn_values_from_uuid() {
        let id = uuid::Uuid::new_v4();
        let urn = Urn::from_uuid("entity.name", &id);
        assert_eq!(format!("urn:trenako:entity.name:{}", id), urn.to_string());
    }
}
