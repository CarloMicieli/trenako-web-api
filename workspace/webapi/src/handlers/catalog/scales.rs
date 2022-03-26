use crate::web::pagination::PaginateQuery;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

pub async fn get_all_scales(
    paginate_params: web::Query<PaginateQuery>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn post_new_scale(
    new_scale: web::Json<NewScaleRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn delete_scale(
    slug: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn get_scale(
    slug: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn edit_scale(
    slug: web::Path<String>,
    modified: web::Json<EditScaleRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Debug, Deserialize)]
pub struct NewScaleRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EditScaleRequest {
    pub name: String,
}
