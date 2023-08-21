use crate::gui::widgets::{
    number_input::number_input,
    selection_list::{selection_key, selection_time},
};
use crate::gui::Message;

use iced::widget::{column, row, text, text_input, Column};
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
            selection_key("Bind key"),
            selection_key("Input key"),
        )
        .into(),
        Input::Press => column!(
            row!(
                selection_key("Bind key"),
                selection_key("Input key"),
            ),
            text("Time"),
            number_input(interval),
            selection_time("Unit"),
        )
        .into(),
        Input::Sequence => column!(
            row!(
                selection_key("Bind key"),
                selection_key("Input key"),
            ),
            text("Time"),
            number_input(interval),
            selection_time("Unit"),
            column!(
                text("sequence string"),
                text_input("www would press w 3 times", "")
            )
        )
        .into(),
    }
}
