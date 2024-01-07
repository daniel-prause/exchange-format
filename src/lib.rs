use indexmap::{
    map::serde_seq::{self},
    IndexMap,
};
use lazy_static::*;
use serde::{Deserialize, Serialize};
use std::ffi::{c_char, CString};
use std::sync::Mutex;

lazy_static! {
    static ref CURRENT_CONFIG: Mutex<ExchangeableConfig> =
        Mutex::new(ExchangeableConfig::default());
}

pub fn get_current_config() -> ExchangeableConfig {
    CURRENT_CONFIG.lock().unwrap().clone()
}

#[no_mangle]
pub extern "C" fn set_current_config(serialized_config: *mut i8) {
    unsafe {
        let config = ExchangeableConfig::from(CString::from_raw(serialized_config));
        *CURRENT_CONFIG.lock().unwrap() = config;
    }
}

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
    Password(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExchangeableConfig {
    // format: "param_key" -> (Label, ConfigParam)
    #[serde(with = "serde_seq")]
    pub params: IndexMap<String, ConfigParam>,
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
            params: IndexMap::new(),
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
    fn to_raw(&self) -> *mut c_char {
        CString::new(Exchangeable::serialize(self))
            .unwrap()
            .into_raw()
    }
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

impl ExchangeFormat {
    pub fn new(items: Vec<Item>) -> ExchangeFormat {
        ExchangeFormat { items }
    }
}

impl ExchangeableConfig {
    pub fn add(&mut self, key: String, config_param: ConfigParam) {
        self.params.insert(key, config_param);
    }

    pub fn get(&self, key: String) -> Option<ConfigParam> {
        match self.params.get(&key) {
            Some(config_param) => Some(config_param.clone()),
            None => None,
        }
    }
}

impl From<CString> for ExchangeableConfig {
    fn from(value: CString) -> Self {
        serde_json::from_str(&value.to_str().unwrap_or_default()).unwrap_or_default()
    }
}

impl From<String> for ExchangeableConfig {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap_or_default()
    }
}

impl Into<u32> for ConfigParam {
    fn into(self) -> u32 {
        match self {
            ConfigParam::Integer(value) => value,
            _ => panic!("Cannot convert {:?} into u32", self),
        }
    }
}

impl Into<String> for ConfigParam {
    fn into(self) -> String {
        match self {
            ConfigParam::String(value) => value,
            ConfigParam::Password(value) => value,
            _ => panic!("Cannot convert {:?} into String", self),
        }
    }
}

impl Into<f32> for ConfigParam {
    fn into(self) -> f32 {
        match self {
            ConfigParam::Float(value) => value,
            _ => panic!("Cannot convert {:?} into f32", self),
        }
    }
}
