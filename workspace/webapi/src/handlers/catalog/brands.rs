use crate::web::pagination::PaginateQuery;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use common::address::Address;

pub async fn get_all_brands(
    paginate_params: web::Query<PaginateQuery>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn post_new_brand(
    new_brand: web::Json<NewBrandRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn get_brand(
    slug: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn delete_brand(
    slug: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn edit_brand(
    slug: web::Path<String>,
    modified_brand: web::Json<EditBrandRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Debug, Deserialize)]
pub struct NewBrandRequest {
    pub name: String,
    pub company_name: Option<String>,
    pub address: Option<Address>,
    pub email: Option<String>,
    pub website_url: Option<String>,
    pub kind: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EditBrandRequest {
    pub name: String,
    pub company_name: Option<String>,
    pub address: Option<Address>,
    pub email: Option<String>,
    pub website_url: Option<String>,
    pub kind: Option<String>,
}
