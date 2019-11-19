use serde_json::value::Value;

#[derive(Debug, Clone)]
pub struct Button {
    pub status: bool,
    pub optional: Value,
}

impl Button {
    pub fn new() -> Self {
        Self {
            status: false,
            optional: Value::Null,
        }
    }
}

pub const LOGIN: &str = "login";
pub const UP: &str = "up";
pub const DOWN: &str = "down";
pub const LEFT: &str = "left";
pub const RIGHT: &str = "right";