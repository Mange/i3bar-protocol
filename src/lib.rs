use std::fmt;
use std::error::Error;

extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

//

mod mouse_button;
pub use mouse_button::MouseButton;

mod click_event;
pub use click_event::{ClickEvent, ClickEventBuilder};

mod header;
pub use header::{Header, HeaderBuilder};

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
