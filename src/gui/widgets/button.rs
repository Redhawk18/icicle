use crate::gui::Message;

use iced::widget::Button;

pub fn start() -> Button<'static, Message> {
    Button::new("Submit").on_press(Message::Start)
}

pub fn stop() -> Button<'static, Message> {
    Button::new("Stop").on_press(Message::Stop)
}
