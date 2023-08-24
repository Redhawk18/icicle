use crate::gui::widgets::{
    selection_list::{selection_keys, selection_time},
    text_input::text_input,
};
use crate::gui::Message;
use crate::types::Mode;

use iced::{
    widget::{column, row},
    Element,
};
use iced_aw::{TabLabel, Tabs};

pub fn tabs(input: Mode, interval: u64, sequence: &str) -> Tabs<'static, Message, Mode> {
    Tabs::new(Message::Tabs)
        .push(
            Mode::Hold,
            TabLabel::Text(String::from("Hold")),
            body(Mode::Hold, interval, sequence),
        )
        .push(
            Mode::Press,
            TabLabel::Text(String::from("Press")),
            body(Mode::Press, interval, sequence),
        )
        .push(
            Mode::Sequence,
            TabLabel::Text(String::from("Sequence")),
            body(Mode::Sequence, interval, sequence),
        )
        .set_active_tab(&input)
}

fn body(input: Mode, interval: u64, sequence: &str) -> Element<'static, Message> {
    match input {
        Mode::Hold => selection_keys().into(),
        Mode::Press => row!(selection_keys(), selection_time(interval))
            .spacing(70.0)
            .into(),
        Mode::Sequence => column!(
            row!(selection_keys(), selection_time(interval)).spacing(70.0),
            text_input(sequence)
        )
        .spacing(20.0)
        .into(),
    }
}
