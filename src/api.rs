use crate::keyboards;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::{get, FromForm, FromFormField};
use rocket_okapi::{openapi, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "lowercase")]
#[serde(tag = "itemType")]
pub enum Item {
    Keyboard(keyboards::Keyboard),
}

#[derive(Debug, PartialEq, JsonSchema, FromFormField, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Keyboards,
}

#[derive(Debug, PartialEq, JsonSchema, Deserialize, Serialize, FromFormField)]
pub enum SortingDirection {
    Ascending,
    Descending,
}

#[derive(Debug, PartialEq, JsonSchema, Deserialize, Serialize, FromFormField)]
pub enum SortingBy {
    Rating,
    Price,
}

#[derive(Debug, PartialEq, JsonSchema, FromForm, Deserialize, Serialize)]
pub struct QueryParams {
    pub pg_limit: usize,
    pub pg_offset: usize,
    pub sort_direction: Option<SortingDirection>,
    pub sort_by: Option<SortingBy>,
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
#[get("/items/<category>?<params..>")]
pub async fn items(
    category: Category,
    params: QueryParams,
) -> Result<Json<Vec<Item>>, &'static str> {
    match category {
        Category::Keyboards => keyboards::load()
            .await
            .map(|items| keyboards::sort(&params.sort_by, items))
            .map(
                |items| match params.pg_limit + params.pg_offset > items.len() {
                    true => items[params.pg_offset..params.pg_offset + params.pg_limit].to_vec(),
                    false => items[params.pg_offset..].to_vec(),
                },
            )
            .map(|mut items| {
                match params.sort_direction {
                    Some(sd) => match sd {
                        SortingDirection::Ascending => items.reverse(),
                        SortingDirection::Descending => {}
                    },
                    None => {}
                }
                items
            })
            .map(|items| items.into_iter().map(Item::Keyboard).collect())
            .map(Json),
    }
}

#[openapi]
#[get("/search?<category>&<request>")]
pub async fn search(
    request: String,
    category: Option<Category>,
) -> Result<Json<Vec<Item>>, &'static str> {
    match category {
        Some(cat) => match cat {
            Category::Keyboards => keyboards::search(&request)
                .await
                .map(|mut items| {
                    items.truncate(20);
                    items
                })
                .map(|items| items.into_iter().map(Item::Keyboard).collect())
                .map(Json),
        },
        None => Ok(Json(Vec::<Item>::new())),
    }
}
