use std::path::PathBuf;

use rocket::serde::json::serde_json;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncReadExt;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

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
    name: String,
    rating: u8,
    rating_count: u32,
    price_usd: Option<f32>,
    style: Style,
    switch_type: Option<String>,
    backlit: Option<String>,
    tenkeyless: bool,
    connection_type: ConnectionType,
    color: Option<String>,
}

pub async fn load() -> Result<Vec<Keyboard>, &'static str> {
    let path = PathBuf::from(format!("{}/{}", DATA_PATH, "keyboard.json"));
    let mut file = File::open(path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    Ok(serde_json::from_str(contents.as_ref()).unwrap())
}
