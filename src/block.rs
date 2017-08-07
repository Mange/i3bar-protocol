extern crate serde_json;

use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use super::{ParseError, Markup, Alignment};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    name: Option<String>,
    instance: Option<String>,

    full_text: String,

    #[serde(default)]
    urgent: bool,

    #[serde(default)]
    separator: bool,

    #[serde(default)]
    markup: Markup,

    #[serde(default, rename = "align")]
    alignment: Alignment,

    short_text: Option<String>,
    color: Option<String>,
    background: Option<String>,
    border: Option<String>,
    min_width: Option<MinWidth>,
    separator_block_width: Option<u32>,
}

impl Block {
    pub fn from_str(str: &str) -> Result<Block, ParseError> {
        serde_json::from_str(str).map_err(Into::into)
    }

    pub fn full_text(&self) -> &str {
        &self.full_text
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_ref)
    }

    pub fn instance(&self) -> Option<&str> {
        self.instance.as_ref().map(String::as_ref)
    }

    pub fn short_text(&self) -> Option<&str> {
        self.short_text.as_ref().map(String::as_ref)
    }

    pub fn min_width(&self) -> Option<&MinWidth> {
        self.min_width.as_ref()
    }

    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    pub fn markup(&self) -> Markup {
        self.markup
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_ref().map(String::as_ref)
    }

    pub fn border(&self) -> Option<&str> {
        self.border.as_ref().map(String::as_ref)
    }

    pub fn background(&self) -> Option<&str> {
        self.background.as_ref().map(String::as_ref)
    }

    pub fn is_urgent(&self) -> bool {
        self.urgent
    }

    pub fn has_separator(&self) -> bool {
        self.separator
    }

    pub fn separator_block_width(&self) -> Option<u32> {
        self.separator_block_width
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MinWidth {
    Pixels(u32),
    Example(String),
}

impl From<String> for MinWidth {
    fn from(string: String) -> MinWidth {
        MinWidth::Example(string)
    }
}

impl<'a> From<&'a str> for MinWidth {
    fn from(str: &'a str) -> MinWidth {
        MinWidth::Example(str.to_owned())
    }
}

impl From<u32> for MinWidth {
    fn from(pixels: u32) -> MinWidth {
        MinWidth::Pixels(pixels)
    }
}

struct MinWidthVisitor;

impl<'de> de::Visitor<'de> for MinWidthVisitor {
    type Value = MinWidth;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a positive integer or a string")
    }

    fn visit_u32<E>(self, value: u32) -> Result<MinWidth, E>
        where E: de::Error
    {
        Ok(MinWidth::from(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<MinWidth, E>
        where E: de::Error
    {
        if value > ::std::u32::MAX as u64 {
            Ok(MinWidth::from(::std::u32::MAX))
        } else {
            Ok(MinWidth::from(value as u32))
        }
    }

    fn visit_str<E>(self, value: &str) -> Result<MinWidth, E>
        where E: de::Error
    {
        Ok(MinWidth::from(value))
    }

    fn visit_string<E>(self, value: String) -> Result<MinWidth, E>
        where E: de::Error
    {
        Ok(MinWidth::from(value))
    }
}

impl<'de> Deserialize<'de> for MinWidth {
    fn deserialize<D>(deserializer: D) -> Result<MinWidth, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_any(MinWidthVisitor)
    }
}

impl Serialize for MinWidth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self {
            &MinWidth::Pixels(num) => serializer.serialize_u32(num),
            &MinWidth::Example(ref str) => serializer.serialize_str(&str),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_deserializes_minwidth_numbers() {
        let parsed: MinWidth = serde_json::from_str("56").expect("Could not parse small number");
        assert_eq!(parsed, MinWidth::Pixels(56));

        let parsed: MinWidth = serde_json::from_str("0").expect("Could not parse zero");
        assert_eq!(parsed, MinWidth::Pixels(0));

        let parsed: Result<MinWidth, _> = serde_json::from_str("-56");
        assert!(parsed.is_err());

        let parsed: Result<MinWidth, _> = serde_json::from_str("5.6");
        assert!(parsed.is_err());

        let parsed: MinWidth =
            serde_json::from_str("1234567890000").expect("Could not parse very large number");
        assert_eq!(parsed, MinWidth::Pixels(::std::u32::MAX));
    }

    #[test]
    fn it_deserializes_minwidth_strings() {
        let parsed: MinWidth =
            serde_json::from_str("\"hello world\"").expect("Could not parse text");
        assert_eq!(parsed, MinWidth::Example(String::from("hello world")));

        let parsed: MinWidth =
            serde_json::from_str("\"14\"").expect("Could not parse number-like string");
        assert_eq!(parsed, MinWidth::Example(String::from("14")));
    }

    #[test]
    fn it_serializes_minwidth() {
        let pixels = MinWidth::Pixels(500);
        let example = MinWidth::Example(String::from("Example text"));

        assert_eq!(serde_json::to_string(&pixels).unwrap(), "500");
        assert_eq!(serde_json::to_string(&example).unwrap(), "\"Example text\"");
    }

    #[test]
    fn it_parses_minimal_block() {
        let json = r#"{"full_text":"E: 10.0.0.1 (1000 Mbit/s)"}"#;
        let block = Block::from_str(json).expect("Could not parse");

        assert_eq!(block.full_text(), "E: 10.0.0.1 (1000 Mbit/s)");
        assert_eq!(block.name(), None);
        assert_eq!(block.instance(), None);
        assert_eq!(block.short_text(), None);
        assert_eq!(block.alignment(), Alignment::Left);
        assert_eq!(block.markup(), Markup::None);
        assert_eq!(block.min_width(), None);
    }

    #[test]
    fn it_parses_full_block() {
        let json = r##"
            {
             "full_text": "E: 10.0.0.1 (1000 Mbit/s)",
             "short_text": "10.0.0.1",
             "color": "#00ff00",
             "background": "#1c1c1c",
             "border": "#ee0000",
             "min_width": 300,
             "align": "right",
             "urgent": false,
             "name": "ethernet",
             "instance": "eth0",
             "separator": true,
             "separator_block_width": 9
            }
        "##;
        let block = Block::from_str(json).expect("Could not parse");

        assert_eq!(block.full_text(), "E: 10.0.0.1 (1000 Mbit/s)");
        assert_eq!(block.short_text(), Some("10.0.0.1"));
        assert_eq!(block.color(), Some("#00ff00"));
        assert_eq!(block.background(), Some("#1c1c1c"));
        assert_eq!(block.border(), Some("#ee0000"));
        assert_eq!(block.min_width(), Some(&MinWidth::Pixels(300)));
        assert_eq!(block.alignment(), Alignment::Right);
        assert_eq!(block.is_urgent(), false);
        assert_eq!(block.name(), Some("ethernet"));
        assert_eq!(block.instance(), Some("eth0"));
        assert_eq!(block.has_separator(), true);
        assert_eq!(block.separator_block_width(), Some(9));
    }
}
