use crate::keyboards;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::{get, FromForm};
use rocket_okapi::{openapi, JsonSchema};

#[derive(Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Keyboards,
}

#[derive(Debug, PartialEq, JsonSchema, FromForm)]
pub struct Pagination {
    pub limit: usize,
    pub offset: usize,
}

impl FromParam<'_> for Category {
    type Error = &'static str;

    fn from_param(param: &str) -> Result<Self, Self::Error> {
        match param {
            "keyboards" => Ok(Category::Keyboards),
            _ => Err("Category does not exist!"),
        }
    }
}

#[openapi]
#[get("/items/<category>?<pagination..>")]
pub async fn items(
    category: Category,
    pagination: Pagination,
) -> Result<Json<Vec<keyboards::Keyboard>>, &'static str> {
    match category {
        Category::Keyboards => keyboards::load()
            .await
            .map(|items| items[pagination.offset..pagination.limit].to_vec())
            .map(Json),
    }
}
