use crate::gui::Message;
use crate::gui::{number_input, selection_key};

use iced::widget::{column, row, text, text_input, Column, Radio, Row};
use iced::Element;

use super::selection_list::selection_time;

pub fn radio<'a>(input: Input, interval: u64) -> Column<'a, Message> {
    column!(head(), body(input, interval))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Input {
    Hold,
    Press,
    Sequence,
}

fn head() -> Row<'static, Message> {
    let selected_choice = Some(Input::Hold); //todo fix

    let hold = Radio::new("Hold", Input::Hold, selected_choice, Message::Input).spacing(3);
    let press = Radio::new("Press", Input::Press, selected_choice, Message::Input).spacing(3);
    let sequence =
        Radio::new("Sequence", Input::Sequence, selected_choice, Message::Input).spacing(3);

    row![hold, press, sequence].spacing(20)
}

fn body(input: Input, interval: u64) -> Element<'static, Message> {
    match input {
        Input::Hold => column!(text("Input Key"), selection_key()).into(),
        Input::Press => row!(column!(
            text("Time"),
            number_input(interval)),
            selection_time(),
        )
        .into(),
        Input::Sequence => row!(column!(
            text("Time"),
            number_input(interval)),
            selection_time(),
            column!(
                text("sequence string"),
                text_input("www would press w 3 times", "")
            )
        )
        .into(),
    }
}
