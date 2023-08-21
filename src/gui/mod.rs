mod widgets;
use widgets::{
    button::button,
    selection_list::{Key, Time},
    tabs::{tabs, Input},
};

use iced::widget::column;
use iced::{Element, Sandbox};

use std::time::Duration;

pub struct Icicle {
    duration: Duration,
    input: Input,
    interval: u64,
    sequence: String,
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

    //tabs
    Tabs(Input),

    //text input
    Sequence(String),
}

impl Sandbox for Icicle {
    type Message = Message;

    fn new() -> Self {
        Self {
            duration: Duration::default(),
            input: Input::Hold, //todo impl default
            interval: 0,
            sequence: String::default(),
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

            Message::Key(_, key) => println!("{key}   "),
            Message::Unit(_, unit) => self.unit = unit,

            Message::Tabs(input) => match input {
                Input::Hold => self.input = input,
                Input::Press => self.input = input,
                Input::Sequence => self.input = input,
            },

            Message::Sequence(sequence) => self.sequence = sequence,
        }
    }

    fn view(&self) -> Element<Message> {
        column!(tabs(self.input, self.interval, self.sequence.as_str()), button(),).into()
    }
}
