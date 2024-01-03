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
