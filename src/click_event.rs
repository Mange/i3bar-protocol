extern crate serde_json;

use std::fmt;
use super::{MouseButton, ParseError};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ClickEvent {
    name: String,
    instance: Option<String>,
    button: MouseButton,
    x: Option<u32>,
    y: Option<u32>,
}

impl ClickEvent {
    pub fn from_str(str: &str) -> Result<ClickEvent, ParseError> {
        serde_json::from_str(str).map_err(|e| e.into())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn instance(&self) -> Option<&str> {
        self.instance.as_ref().map(String::as_ref)
    }

    pub fn button(&self) -> MouseButton {
        self.button
    }

    pub fn coordinates(&self) -> Option<(u32, u32)> {
        if let (Some(x), Some(y)) = (self.x, self.y) {
            Some((x, y))
        } else {
            None
        }
    }

    pub fn x(&self) -> Option<u32> {
        self.x
    }

    pub fn y(&self) -> Option<u32> {
        self.y
    }
}

pub struct ClickEventBuilder {
    name: String,
    button: MouseButton,

    instance: Option<String>,
    x: Option<u32>,
    y: Option<u32>,
}

impl ClickEventBuilder {
    pub fn new<S>(name: S, button: MouseButton) -> ClickEventBuilder
        where S: Into<String>
    {
        ClickEventBuilder {
            name: name.into(),
            button: button,
            instance: None,
            x: None,
            y: None,
        }
    }

    pub fn instance<V, S>(mut self, value: V) -> Self
        where V: Into<Option<S>>,
              S: Into<String>
    {
        self.instance = value.into().map(Into::into);
        self
    }

    pub fn coordinates(mut self, x: u32, y: u32) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn build(self) -> ClickEvent {
        ClickEvent {
            name: self.name,
            instance: self.instance,
            button: self.button,
            x: self.x,
            y: self.y,
        }
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
    fn it_allows_building_of_click_events() {
        let small_event = ClickEventBuilder::new("name", MouseButton::Right).build();
        assert_eq!(small_event.name(), "name");
        assert_eq!(small_event.instance(), None);
        assert_eq!(small_event.button(), MouseButton::Right);
        assert_eq!(small_event.coordinates(), None);

        let big_event = ClickEventBuilder::new("name", MouseButton::Right)
            .instance("instance")
            .coordinates(100, 200)
            .build();
        assert_eq!(big_event.name(), "name");
        assert_eq!(big_event.instance(), Some("instance"));
        assert_eq!(big_event.button(), MouseButton::Right);
        assert_eq!(big_event.coordinates(), Some((100, 200)));
    }

    #[test]
    fn it_serdes_mouse_event() {
        let event_string = r#"{"name":"ethernet","instance":"eth0","button":1,"x":1320,"y":1400}"#;
        let event = ClickEvent::from_str(event_string).expect("Failed to parse");

        assert_eq!(event.name(), "ethernet");
        assert_eq!(event.instance(), Some("eth0"));
        assert_eq!(event.button(), MouseButton::Left);
        assert_eq!(event.coordinates(), Some((1320, 1400)));

        assert_eq!(event.to_string(), String::from(event_string));
    }
}
