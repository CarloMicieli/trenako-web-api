use crate::brands::BrandType;
use async_trait::async_trait;
use common::address::Address;
use common::slug::Slug;
use std::fmt;
use std::fmt::Formatter;

type DatabaseResult<T> = std::result::Result<T, DbError>;

#[derive(Debug)]
pub struct NewBrand {
    pub brand_id: Slug,
    pub name: String,
    pub description: Option<String>,
    pub company_name: Option<String>,
    pub group_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub kind: Option<BrandType>,
    pub active: Option<bool>,
    pub address: Option<Address>,
}

#[async_trait]
pub trait BrandsRepository {
    async fn exists_by_name(&self, name: &str) -> DatabaseResult<bool>;
    async fn create(&self, new_brand: NewBrand) -> DatabaseResult<Slug>;
}

#[derive(Debug)]
pub enum DbError {
    GenericError(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
