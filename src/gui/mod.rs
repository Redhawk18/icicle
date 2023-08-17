mod widgets;
use widgets::{
    button::button,
    number_input::number_input,
    selection_list::{selection_key, selection_time, Key, Time},
};

use iced::widget::{column, row, text};
use iced::{Element, Sandbox};
use inputbot::KeybdKey;
use std::time::Duration;

pub struct Icicle {
    duration: Duration,
    interval: u64,
    unit: Time,
}

#[derive(Debug, Clone)]
pub enum Message {
    //button
    Submit,

    //number input
    Interval(u64),

    //selection list
    Key(usize, Key),
    Unit(usize, Time),
}

impl Sandbox for Icicle {
    type Message = Message;

    fn new() -> Self {
        Self {
            duration: Duration::default(),
            interval: 0,
            unit: Time::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Icile")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Submit => match self.unit {
                Time::Minutes => self.duration = Duration::from_secs(self.interval * 60),
                Time::Seconds => self.duration = Duration::from_secs(self.interval),
                Time::Milliseconds => self.duration = Duration::from_millis(self.interval),
                Time::Mircoseconds => self.duration = Duration::from_micros(self.interval),
                Time::Nanoseconds => self.duration = Duration::from_nanos(self.interval),
            },

            Message::Interval(interval) => self.interval = interval,

            Message::Key(_, key) => println!("{key}  "),
            Message::Unit(_, unit) => self.unit = unit,
        }
    }

    fn view(&self) -> Element<Message> {
        column!(
            row!(
                text("Press Interval"),
                number_input(self.interval),
                selection_time(),
            ),
            row!(selection_key()),
            row!(button())
        )
        .into()
    }
}
