use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    WheelUp,
    WheelDown,
    Forward,
    Back,
    Unknown,
}

impl MouseButton {
    fn number_code(&self) -> u64 {
        match self {
            &MouseButton::Left => 1,
            &MouseButton::Middle => 2,
            &MouseButton::Right => 3,
            &MouseButton::WheelUp => 4,
            &MouseButton::WheelDown => 5,
            &MouseButton::Forward => 9,
            &MouseButton::Back => 8,
            &MouseButton::Unknown => 0,
        }
    }
}

impl<'de> Deserialize<'de> for MouseButton {
    fn deserialize<D>(deserializer: D) -> Result<MouseButton, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_u64(MouseButtonVisitor)
    }
}

struct MouseButtonVisitor;

impl<'de> de::Visitor<'de> for MouseButtonVisitor {
    type Value = MouseButton;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "integer")
    }

    fn visit_u64<E>(self, value: u64) -> Result<MouseButton, E>
        where E: de::Error
    {
        Ok(match value {
               1 => MouseButton::Left,
               2 => MouseButton::Middle,
               3 => MouseButton::Right,
               4 => MouseButton::WheelUp,
               5 => MouseButton::WheelDown,
               9 => MouseButton::Forward,
               8 => MouseButton::Back,
               _ => MouseButton::Unknown,
           })
    }
}

impl Serialize for MouseButton {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u64(self.number_code())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Markup {
    None,
    Pango,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

const VALID_MARKUPS: &'static [&'static str] = &["pango", "none"];

impl Markup {
    fn from_symbol(symbol: &str) -> Option<Markup> {
        match symbol {
            "none" => Some(Markup::None),
            "pango" => Some(Markup::Pango),
            _ => None,
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            &Markup::None => "none",
            &Markup::Pango => "pango",
        }
    }
}

impl Default for Markup {
    fn default() -> Markup {
        Markup::None
    }
}

const VALID_ALIGNMENTS: &'static [&'static str] = &["left", "center", "right"];

impl Alignment {
    fn from_symbol(symbol: &str) -> Option<Alignment> {
        match symbol {
            "center" => Some(Alignment::Center),
            "left" => Some(Alignment::Left),
            "right" => Some(Alignment::Right),
            _ => None,
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            &Alignment::Center => "center",
            &Alignment::Left => "left",
            &Alignment::Right => "right",
        }
    }
}

impl Default for Alignment {
    fn default() -> Alignment {
        Alignment::Left
    }
}

struct MarkupVisitor;
struct AlignmentVisitor;

impl<'de> Deserialize<'de> for Markup {
    fn deserialize<D>(deserializer: D) -> Result<Markup, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(MarkupVisitor)
    }
}

impl<'de> de::Visitor<'de> for MarkupVisitor {
    type Value = Markup;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "none or pango")
    }

    fn visit_str<E>(self, value: &str) -> Result<Markup, E>
        where E: de::Error
    {
        Markup::from_symbol(value).ok_or_else(|| E::unknown_variant(value, VALID_MARKUPS))
    }
}

impl Serialize for Markup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.symbol())
    }
}

impl<'de> Deserialize<'de> for Alignment {
    fn deserialize<D>(deserializer: D) -> Result<Alignment, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(AlignmentVisitor)
    }
}

impl<'de> de::Visitor<'de> for AlignmentVisitor {
    type Value = Alignment;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "none or pango")
    }

    fn visit_str<E>(self, value: &str) -> Result<Alignment, E>
        where E: de::Error
    {
        Alignment::from_symbol(value).ok_or_else(|| E::unknown_variant(value, VALID_ALIGNMENTS))
    }
}

impl Serialize for Alignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.symbol())
    }
}
