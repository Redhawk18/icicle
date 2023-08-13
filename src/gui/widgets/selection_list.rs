use crate::gui::Message;

use iced::Length;
use iced_aw::SelectionList;

pub fn selection_list<'a>() -> SelectionList<'a, String, Message> {
    let myvec = vec![
        "Hours",
        "Minutes",
        "Seconds",
        "Milliseconds",
        "Mircoseconds",
        "Nanoseconds",
    ];
    let myvec2 = vec![
        String::from("val1"),
        String::from("val2"),
        String::from("val3"),
    ];
    SelectionList::new(myvec2, Message::SelectionList)
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

impl Time {
    const ALL: [Time; 6] = [
        Time::Hours,
        Time::Minutes,
        Time::Seconds,
        Time::Milliseconds,
        Time::Mircoseconds,
        Time::Nanoseconds,
    ];
}

pub fn options() -> Vec<&'static str> {
    vec![
        "Hours",
        "Minutes",
        "Seconds",
        "Milliseconds",
        "Mircoseconds",
        "Nanoseconds",
    ]
}
