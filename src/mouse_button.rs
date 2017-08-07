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
