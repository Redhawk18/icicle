use crate::gui::Message;

use iced::Length;
use iced_aw::SelectionList;

pub fn selection_list<'a>() -> SelectionList<'a, Time, Message> {
    SelectionList::new(&[Time::Hours, Time::Minutes], Message::SelectionList)
        .height(Length::Fixed(50.0))
        .width(Length::Fixed(90.0))
}

#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
pub enum Time {
    Hours,
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
                Time::Hours => "Hours",
                Time::Minutes => "Minutes",
                Time::Seconds => "Seconds",
                Time::Milliseconds => "Milliaseconds",
                Time::Mircoseconds => "Mircoseconds",
                Time::Nanoseconds => "Nanoseconds",
            }
        )
    }
}
