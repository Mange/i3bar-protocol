use std::fmt;
use std::error::Error;

extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

//

mod mouse_button;
pub use mouse_button::MouseButton;

#[derive(Debug, Clone)]
pub enum ParseError {
    ReadError(String),
    JsonError(String),
    InvalidData(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error")
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self {
            &ParseError::ReadError(ref s) => s,
            &ParseError::JsonError(ref s) => s,
            &ParseError::InvalidData(ref s) => s,
        }
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(input: serde_json::Error) -> ParseError {
        use serde_json::error::Category;

        match input.classify() {
            Category::Io | Category::Eof => ParseError::ReadError(input.description().to_owned()),
            Category::Syntax => ParseError::JsonError(input.to_string()),
            Category::Data => ParseError::InvalidData(input.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ClickEvent {
    name: String,
    instance: String,
    button: MouseButton,
    x: u32,
    y: u32,
}

impl ClickEvent {
    pub fn from_str(str: &str) -> Result<ClickEvent, ParseError> {
        serde_json::from_str(str).map_err(|e| e.into())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn instance(&self) -> &str {
        &self.instance
    }

    pub fn button(&self) -> MouseButton {
        self.button
    }

    pub fn coordinates(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

impl fmt::Display for ClickEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = serde_json::to_string(self).unwrap_or(String::from("{}"));
        f.write_str(&string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serdes_mouse_event() {
        let event_string = r#"{"name":"ethernet","instance":"eth0","button":1,"x":1320,"y":1400}"#;
        let event = ClickEvent::from_str(event_string).expect("Failed to parse");

        assert_eq!(event.name(), "ethernet");
        assert_eq!(event.instance(), "eth0");
        assert_eq!(event.button(), MouseButton::Left);
        assert_eq!(event.coordinates(), (1320, 1400));

        assert_eq!(event.to_string(), String::from(event_string));
    }
}
