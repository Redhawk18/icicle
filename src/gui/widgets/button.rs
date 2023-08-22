use crate::gui::Message;

use iced::widget::Button;

pub fn button<'a>() -> Button<'a, Message> {
    Button::new("Submit")
        .on_press(Message::Submit)
        .padding(10.0)
}
