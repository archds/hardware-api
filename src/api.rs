use crate::keyboards;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::{get, FromForm};
use rocket_okapi::{openapi, JsonSchema};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "lowercase")]
#[serde(tag = "itemType")]
pub enum Item {
    Keyboard(keyboards::Keyboard),
}

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
) -> Result<Json<Vec<Item>>, &'static str> {
    match category {
        Category::Keyboards => keyboards::load()
            .await
            .map(|items| items[pagination.offset..pagination.limit].to_vec())
            .map(|items| items.into_iter().map(Item::Keyboard).collect())
            .map(Json),
    }
}
