use crate::keyboards;
use rocket::get;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, JsonSchema};

#[derive(Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Keyboards,
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
#[get("/items/<category>")]
pub async fn items(category: Category) -> Result<Json<Vec<keyboards::Keyboard>>, &'static str> {
    match category {
        Category::Keyboards => keyboards::load().await.map(Json),
    }
}
