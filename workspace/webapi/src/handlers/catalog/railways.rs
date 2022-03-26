use crate::web::pagination::PaginateQuery;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

pub async fn get_all_railways(
    paginate_params: web::Query<PaginateQuery>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn post_new_railway(
    new_railway: web::Json<NewRailwayRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn get_railway(
    slug: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn delete_railway(
    slug: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn edit_railway(
    slug: web::Path<String>,
    modified: web::Json<EditRailwayRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Debug, Deserialize)]
pub struct NewRailwayRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EditRailwayRequest {
    pub name: String,
}
