use super::selection_list::selection_time;
use crate::gui::Message;
use crate::gui::{number_input, selection_input};

use iced::widget::{column, row, text, text_input, Column, Radio, Row};
use iced::Element;
use iced_aw::{TabLabel, Tabs};

pub fn tabs(input: Input) -> Column<'static, Message> {
    column!(head(input))
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum Input {
    #[default]
    Hold,
    Press,
    Sequence,
}

fn head(input: Input) -> Tabs<'static, Message, Input> {
    Tabs::new(Message::Input)
        .push(
            Input::Hold,
            TabLabel::Text(String::from("Hold")),
            body(Input::Hold, 1),
        )
        .push(
            Input::Press,
            TabLabel::Text(String::from("Press")),
            body(Input::Press, 1),
        )
        .push(
            Input::Sequence,
            TabLabel::Text(String::from("Sequence")),
            body(Input::Sequence, 1),
        )
        .set_active_tab(&input)
}

fn body(input: Input, interval: u64) -> Element<'static, Message> {
    match input {
        Input::Hold => row!(
            column!(text("Bind key"), selection_input()),
            column!(text("Input key"), selection_input()),
        )
        .into(),
        Input::Press => column!(
            row!(
                column!(text("Bind key"), selection_input()),
                column!(text("Input key"), selection_input()),
            ),
            text("Time"),
            number_input(interval),
            selection_time(),
        )
        .into(),
        Input::Sequence => column!(
            row!(
                column!(text("Bind key"), selection_input()),
                column!(text("Input key"), selection_input()),
            ),
            text("Time"),
            number_input(interval),
            selection_time(),
            column!(
                text("sequence string"),
                text_input("www would press w 3 times", "")
            )
        )
        .into(),
    }
}
