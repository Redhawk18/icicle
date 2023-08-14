mod widgets;
use iced::widget::{row, text};
use iced::{Element, Sandbox};
use std::time::Duration;
use widgets::number_input::number_input;
use widgets::selection_list::{selection_list, Time};

pub struct Icicle {
    interval: Duration,
    unit: Time,
}

#[derive(Debug, Clone)]
pub enum Message {
    //number input
    NumberInput(u64),

    //selection list
    SelectionList(usize, Time),
}

impl Sandbox for Icicle {
    type Message = Message;

    fn new() -> Self {
        Self {
            interval: Duration::default(),
            unit: Time::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Icile")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NumberInput(interval) => {
                self.interval = Duration::new(interval, 0);
            }
            Message::SelectionList(i, text) => println!("{i} {text}"),
        }
    }

    fn view(&self) -> Element<Message> {
        row!(
            text("Press Interval"),
            number_input(self.interval),
            selection_list(),
        )
        .into()
    }
}
