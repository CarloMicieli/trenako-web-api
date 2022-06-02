use crate::web::pagination::PaginateQuery;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

pub async fn post_new_catalog_item(
    new_catalog_item: web::Json<NewCatalogItemRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn get_catalog_item(
    slug: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn delete_catalog_item(
    slug: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn edit_catalog_item(
    slug: web::Path<String>,
    modified_catalog_item: web::Json<EditCatalogItemRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Debug, Deserialize)]
pub struct NewCatalogItemRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EditCatalogItemRequest {
    pub name: String,
}
