use crate::gui::Message;

use iced::widget::{column, text, TextInput, Column};

pub fn text_input(sequence: &str) -> Column<'static, Message> {
    column!(text("Sequence:"), TextInput::new("! equals shift+1", sequence).on_input(Message::Sequence))
}
