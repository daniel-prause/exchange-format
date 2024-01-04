use std::{collections::HashMap, ffi::CString};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ExchangeFormat {
    pub items: Vec<Item>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Item {
    Text(Text),
    Image(Image),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ConfigParam {
    Integer(u32),
    String(String),
    Float(f32),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExchangeableConfig {
    // format: "param_key" -> (Label, ConfigParam)
    pub params: HashMap<String, ConfigParam>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Text {
    pub value: String,
    pub x: i32,
    pub y: i32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub color: Vec<u8>, // RGB
    pub symbol: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Image {
    pub value: Vec<u8>,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Default for ExchangeFormat {
    fn default() -> ExchangeFormat {
        ExchangeFormat { items: vec![] }
    }
}

impl Default for ExchangeableConfig {
    fn default() -> ExchangeableConfig {
        ExchangeableConfig {
            params: HashMap::new(),
        }
    }
}

impl Default for Text {
    fn default() -> Text {
        Text {
            color: vec![255, 255, 255],
            value: "".into(),
            scale_x: 16f32,
            scale_y: 16f32,
            symbol: false,
            x: 0,
            y: 0,
        }
    }
}

pub trait Exchangeable {
    type Output;
    fn serialize(&self) -> String;
}

impl Exchangeable for ExchangeFormat {
    type Output = Self;
    fn serialize(&self) -> String {
        serde_json::json!(self).to_string()
    }
}

impl Exchangeable for ExchangeableConfig {
    type Output = Self;
    fn serialize(&self) -> String {
        serde_json::json!(self).to_string()
    }
}

pub fn deserialize_config_from_string(data: String) -> ExchangeableConfig {
    serde_json::from_str(data.as_str()).unwrap_or_default()
}

pub fn deserialize_config_from_cstring(data: CString) -> ExchangeableConfig {
    serde_json::from_str(&data.to_str().unwrap_or_default()).unwrap_or_default()
}
