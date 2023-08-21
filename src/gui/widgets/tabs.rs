use crate::gui::widgets::{
    selection_list::{selection_keys, selection_time},
    text_input::text_input_sequence,
};
use crate::gui::Message;

use iced::widget::{column, row};
use iced::Element;
use iced_aw::{TabLabel, Tabs};

pub fn tabs(input: Input, interval: u64) -> Tabs<'static, Message, Input> {
    Tabs::new(Message::Tabs)
        .push(
            Input::Hold,
            TabLabel::Text(String::from("Hold")),
            body(Input::Hold, interval),
        )
        .push(
            Input::Press,
            TabLabel::Text(String::from("Press")),
            body(Input::Press, interval),
        )
        .push(
            Input::Sequence,
            TabLabel::Text(String::from("Sequence")),
            body(Input::Sequence, interval),
        )
        .set_active_tab(&input)
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum Input {
    #[default]
    Hold,
    Press,
    Sequence,
}

fn body(input: Input, interval: u64) -> Element<'static, Message> {
    match input {
        Input::Hold => selection_keys().into(),
        Input::Press => row!(selection_keys(), selection_time(interval))
            .spacing(70.0)
            .into(),
        Input::Sequence => column!(
            row!(selection_keys(), selection_time(interval)).spacing(70.0),
            text_input_sequence()
        )
        .spacing(20.0)
        .into(),
    }
}
