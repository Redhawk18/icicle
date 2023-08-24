use inputbot::{KeybdKey, MouseButton};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Input {
    Keyboard(KeybdKey),
    Mouse(MouseButton),
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Input::Keyboard(k) => write!(f, "{}", k),
            Input::Mouse(m) => write!(f, "{}", m),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum Mode {
    #[default]
    Hold,
    Press,
    Sequence,
}

#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
pub enum Time {
    Minutes,
    #[default]
    Seconds,
    Milliseconds,
    Mircoseconds,
    Nanoseconds,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Time::Minutes => "Minutes",
                Time::Seconds => "Seconds",
                Time::Milliseconds => "Milliaseconds",
                Time::Mircoseconds => "Mircoseconds",
                Time::Nanoseconds => "Nanoseconds",
            }
        )
    }
}
