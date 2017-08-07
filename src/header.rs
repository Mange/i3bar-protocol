extern crate libc;
extern crate serde_json;

use std::fmt;
use std::str::FromStr;
use super::ParseError;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Header {
    version: u8,

    #[serde(default)]
    stop_signal: i32,

    #[serde(default, rename = "cont_signal")]
    continue_signal: i32,

    #[serde(default)]
    click_events: bool,
}

impl Default for Header {
    fn default() -> Header {
        Header {
            version: 1,
            stop_signal: libc::SIGSTOP,
            continue_signal: libc::SIGCONT,
            click_events: false,
        }
    }
}

impl FromStr for Header {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Header, ParseError> {
        let mut header: Header = serde_json::from_str(str)?;

        if header.stop_signal == 0 {
            header.stop_signal = libc::SIGSTOP;
        }

        if header.continue_signal == 0 {
            header.continue_signal = libc::SIGCONT;
        }

        Ok(header)
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = serde_json::to_string(self).unwrap_or(String::from(r#"{"version":1}"#));
        f.write_str(&string)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct HeaderBuilder {
    stop_signal: i32,
    continue_signal: i32,
    click_events: bool,
}

impl HeaderBuilder {
    pub fn new() -> HeaderBuilder {
        HeaderBuilder {
            stop_signal: libc::SIGSTOP,
            continue_signal: libc::SIGCONT,
            click_events: false,
        }
    }

    pub fn click_events(&mut self, enabled: bool) -> &mut Self {
        self.click_events = enabled;
        self
    }

    pub fn continue_signal(&mut self, signal: i32) -> &mut Self {
        self.continue_signal = signal;
        self
    }

    pub fn stop_signal(&mut self, signal: i32) -> &mut Self {
        self.stop_signal = signal;
        self
    }

    pub fn build(&self) -> Header {
        Header {
            version: 1,
            stop_signal: self.stop_signal,
            continue_signal: self.continue_signal,
            click_events: self.click_events,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_header() {
        let header = HeaderBuilder::new()
            .click_events(false)
            .continue_signal(5)
            .stop_signal(libc::SIGKILL)
            .build();

        assert_eq!(header.version, 1);
        assert_eq!(header.click_events, false);
        assert_eq!(header.continue_signal, 5);
        assert_eq!(header.stop_signal, libc::SIGKILL);
    }

    #[test]
    fn it_serdes_headers() {
        let json = r#"{"version":1,"stop_signal":10,"cont_signal":12,"click_events":true}"#;
        let header: Header = json.parse().expect("Could not parse Header");

        assert_eq!(header.version, 1);
        assert_eq!(header.click_events, true);
        assert_eq!(header.continue_signal, 12);
        assert_eq!(header.stop_signal, 10);

        assert_eq!(header.to_string(), json);
    }

    #[test]
    fn it_gets_sane_defaults_on_missing_fields_in_json() {
        let json = r#"{"version":1}"#;
        let header: Header = json.parse().expect("Could not parse Header");

        assert_eq!(header.version, 1);
        assert_eq!(header.click_events, false);
        assert_eq!(header.continue_signal, libc::SIGCONT);
        assert_eq!(header.stop_signal, libc::SIGSTOP);
    }
}
