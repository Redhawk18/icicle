use crate::gui::widgets::{
    number_input::number_input,
    selection_list::{selection_keys, selection_time},
};
use crate::gui::Message;

use iced::widget::{column, text, text_input};
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
        Input::Press => column!(selection_keys(), selection_time(interval)).into(),
        Input::Sequence => column!(
            selection_keys(),
            selection_time(interval),
            column!(text("sequence string"), text_input("! equals shift+1", ""))
        )
        .into(),
    }
}
