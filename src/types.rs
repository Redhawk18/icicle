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

impl std::fmt::Display for Time {
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
