use iced::widget::column;
use iced::{Element, Sandbox};

pub struct Icicle {}

#[derive(Debug, Clone, Copy)]
pub enum Message {

}

impl Sandbox for Icicle {
    type Message = Message;

    fn new() -> Self {
        Self {  }
    }

    fn title(&self) -> String {
        String::from("Icile")
    }

    fn update(&mut self, message: Message) {

    }

    fn view(&self) -> Element<Message> {
        column!().into()
    }
}