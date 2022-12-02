use std::path::PathBuf;

use rocket::serde::json::serde_json;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncReadExt;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::api::SortingBy;
use crate::DATA_PATH;

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(crate = "rocket::serde")]
pub enum Style {
    Gaming,
    Mini,
    Standard,
    Slim,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(crate = "rocket::serde")]
pub enum ConnectionType {
    Wired,
    Wireless,
    #[serde(rename = "Bluetooth Wireless")]
    BluetoothWireless,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Keyboard {
    pub name: String,
    pub rating: u8,
    pub rating_count: u32,
    pub price_usd: Option<f32>,
    pub style: Style,
    pub switch_type: Option<String>,
    pub backlit: Option<String>,
    pub tenkeyless: bool,
    pub connection_type: ConnectionType,
    pub color: Option<String>,
}

pub async fn load() -> Result<Vec<Keyboard>, &'static str> {
    let path = PathBuf::from(format!("{}/{}", DATA_PATH, "keyboard.json"));
    let mut file = File::open(path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    let mut keyboards: Vec<Keyboard> = serde_json::from_str(contents.as_ref()).unwrap();
    keyboards.sort_by(|a, b| a.name.cmp(&b.name));
    keyboards.dedup_by(|a, b| a.name == b.name);

    Ok(keyboards)
}

pub async fn search(request: &String) -> Result<Vec<Keyboard>, &'static str> {
    load().await.map(|items| {
        items
            .into_iter()
            .filter_map(
                |item| match item.name.to_lowercase().contains(&request.to_lowercase()) {
                    true => Some(item),
                    false => None,
                },
            )
            .collect()
    })
}

pub fn sort(by: &Option<SortingBy>, mut keyboards: Vec<Keyboard>) -> Vec<Keyboard> {
    if by.is_none() {
        return keyboards;
    }

    keyboards.sort_by(|a, b| match by.as_ref().unwrap() {
        SortingBy::Price => b
            .price_usd
            .unwrap_or_default()
            .total_cmp(&a.price_usd.unwrap_or_default()),
        SortingBy::Rating => b
            .price_usd
            .unwrap_or_default()
            .total_cmp(&a.price_usd.unwrap_or_default()),
    });

    keyboards
}
