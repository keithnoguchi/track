// SPDX-License-Identifier: GPL-2.0
use std::fmt;
use std::str;

#[derive(Clone, Copy)]
pub enum Event {
    Twitter,
    Facebook,
    Google,
    Travel,
    Art,
    Music,
    Photography,
    Love,
    Fashion,
    Food,
    Other,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::Twitter => write!(f, "T"),
            Event::Facebook => write!(f, "F"),
            Event::Google => write!(f, "G"),
            Event::Travel => write!(f, "Tr"),
            Event::Art => write!(f, "Ar"),
            Event::Music => write!(f, "Mu"),
            Event::Photography => write!(f, "Ph"),
            Event::Love => write!(f, "Lo"),
            Event::Fashion => write!(f, "Fa"),
            Event::Food => write!(f, "Fo"),
            _ => write!(f, "."),
        }
    }
}

impl str::FromStr for Event {
    type Err = ();
    fn from_str(s: &str) -> Result<Event, ()> {
        match s {
            "twitter" => Ok(Event::Twitter),
            "facebook" => Ok(Event::Facebook),
            "google" => Ok(Event::Google),
            "travel" => Ok(Event::Travel),
            "art" => Ok(Event::Art),
            "music" => Ok(Event::Music),
            "photography" => Ok(Event::Photography),
            "love" => Ok(Event::Love),
            "fashion" => Ok(Event::Fashion),
            "food" => Ok(Event::Food),
            _ => Ok(Event::Other),
        }
    }
}
