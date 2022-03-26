use crate::brands::data::{BrandsRepository, DbError, NewBrand};
use crate::brands::{Brand, BrandType};
use common::address::Address;
use common::slug::Slug;
use common::validation::ValidationResult;
use serde::{Deserialize, Serialize};
use std::convert;
use std::result;
use validator::Validate;

#[derive(Debug, PartialEq, Eq)]
pub enum CreateBrandError {
    GenericError(String),
    AlreadyExists(String),
    InvalidInput(ValidationResult),
    DatabaseError(String),
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBrandInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub company_name: Option<String>,
    pub group_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub kind: Option<String>,
    pub active: Option<bool>,
    pub address: Option<Address>,
}

#[derive(Debug, Serialize)]
pub struct CreateBrandOutput(Slug);

impl CreateBrandOutput {
    pub fn brand_id(&self) -> &Slug {
        &self.0
    }
}

impl convert::TryFrom<CreateBrandInput> for NewBrand {
    type Error = &'static str;

    fn try_from(input: CreateBrandInput) -> Result<Self, Self::Error> {
        let name = input.name.expect("brand name is required");
        let brand_id = Slug::new(&name);
        let brand_type = input
            .kind
            .map(|s| s.parse::<BrandType>().expect("invalid brand type"));

        Ok(Self {
            brand_id,
            name,
            description: input.description,
            company_name: input.company_name,
            group_name: input.group_name,
            email: input.email,
            phone_number: input.phone_number,
            kind: brand_type,
            active: input.active,
            address: input.address,
        })
    }
}

impl convert::From<DbError> for CreateBrandError {
    fn from(db_error: DbError) -> Self {
        CreateBrandError::DatabaseError(db_error.to_string())
    }
}

/// Creates a new brand
async fn create_new_brand<R>(
    input: CreateBrandInput,
    repository: R,
) -> result::Result<CreateBrandOutput, CreateBrandError>
where
    R: BrandsRepository,
{
    let validation = input.validate().map_err(|errors| errors.into());

    match validation {
        Ok(_) => {
            let new_brand: NewBrand =
                input.try_into().map_err(|err_msg: &str| {
                    CreateBrandError::GenericError(err_msg.to_owned())
                })?;

            let exists = repository.exists_by_name(&new_brand.name).await?;
            if exists {
                Err(CreateBrandError::AlreadyExists(new_brand.name))
            } else {
                let new_brand_id = repository.create(new_brand).await?;
                Ok(CreateBrandOutput(new_brand_id))
            }
        }
        Err(validation_result) => {
            Err(CreateBrandError::InvalidInput(validation_result))
        }
    }
}
