use crate::web::urn::Urn;
use actix_web::{http, HttpResponse};
use chrono::{DateTime, Utc};
use common::validation::ValidationResult;
use derive_builder::Builder;
use serde::Serialize;
use std::collections::BTreeMap;

/// From RFC-7807
/// "problem detail" is a way to carry machine-readable details of errors in a HTTP response to avoid
/// the need to define new error response formats for HTTP APIs.
#[derive(Debug, Serialize, Builder)]
pub struct ProblemDetail {
    problem_type: Urn,
    timestamp: DateTime<Utc>,
    title: String,
    detail: String,
    fields: std::collections::BTreeMap<String, Vec<String>>,
    status_code: u16,
    instance: Urn,
}

impl ProblemDetail {
    pub fn new(problem_id: uuid::Uuid) -> ProblemDetail {
        ProblemDetailBuilder::default()
            .instance(Urn::instance(&problem_id))
            .problem_type(Urn::error())
            .title(String::from("Internal Server Error"))
            .build()
            .unwrap()
    }

    pub fn from_error(error: &str) -> ProblemDetail {
        let now = chrono::Utc::now();
        let id = uuid::Uuid::new_v4();
        ProblemDetail {
            problem_type: Urn::error(),
            timestamp: now,
            title: String::from("Internal Server Error"),
            detail: error.to_owned(),
            fields: BTreeMap::new(),
            status_code: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            instance: Urn::instance(&id),
        }
    }

    /// Creates a new Uniform Resource Name (URN) for an invalid request
    pub fn from_invalid_request(
        entity: &str,
        validation_result: ValidationResult,
    ) -> ProblemDetail {
        let now = chrono::Utc::now();
        let id = uuid::Uuid::new_v4();

        ProblemDetail {
            problem_type: Urn::invalid_request(entity),
            timestamp: now,
            title: String::from("Invalid request"),
            detail: String::from(
                "Fields validation failed for this request. Check them before you try again.",
            ),
            fields: validation_result.to_map(),
            status_code: http::StatusCode::BAD_REQUEST.as_u16(),
            instance: Urn::from_uuid(entity, &id),
        }
    }

    pub fn to_response(&self) -> HttpResponse {
        const CONTENT_TYPE: &str = "application/problem+json";

        let mut builder = match self.status_code {
            400 => HttpResponse::BadRequest(),
            422 => HttpResponse::UnprocessableEntity(),
            _ => HttpResponse::InternalServerError(),
        };

        builder.content_type(CONTENT_TYPE).json(self)
    }
}
