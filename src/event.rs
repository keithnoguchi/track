// SPDX-License-Identifier: GPL-2.0
use std::fmt;
use std::str;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
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

#[cfg(test)]
mod tests {
    use super::Event;
    #[test]
    fn from_str() {
        struct Test {
            name: &'static str,
            data: &'static str,
            want: Result<Event, ()>,
        }
        let tests = [
            Test {
                name: "twitter trend",
                data: "twitter",
                want: Ok(Event::Twitter),
            },
            Test {
                name: "facebook trend",
                data: "facebook",
                want: Ok(Event::Facebook),
            },
            Test {
                name: "google trend",
                data: "google",
                want: Ok(Event::Google),
            },
            Test {
                name: "travel trend",
                data: "travel",
                want: Ok(Event::Travel),
            },
            Test {
                name: "art trend",
                data: "art",
                want: Ok(Event::Art),
            },
            Test {
                name: "music trend",
                data: "music",
                want: Ok(Event::Music),
            },
            Test {
                name: "photography trend",
                data: "photography",
                want: Ok(Event::Photography),
            },
            Test {
                name: "love trend",
                data: "love",
                want: Ok(Event::Love),
            },
            Test {
                name: "fashion trend",
                data: "fashion",
                want: Ok(Event::Fashion),
            },
            Test {
                name: "food trend",
                data: "food",
                want: Ok(Event::Food),
            },
            Test {
                name: "other trend",
                data: "other",
                want: Ok(Event::Other),
            },
            Test {
                name: "another trend",
                data: "another",
                want: Ok(Event::Other),
            },
        ];
        for t in &tests {
            debug_assert_eq!(t.want, t.data.parse::<Event>(), "{}", t.name);
        }
    }
}
