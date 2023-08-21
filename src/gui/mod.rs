mod widgets;
use widgets::{
    button::button,
    selection_list::{Key, Time},
    tabs::{tabs, Mode},
};

use iced::widget::column;
use iced::{Element, Sandbox};

use std::time::Duration;

pub struct Icicle {
    duration: Duration,
    input: Key,
    interval: u64,
    mode: Mode,
    sequence: String,
    toggle: Key,
    unit: Time,
}

#[derive(Debug, Clone)]
pub enum Message {
    //button
    Submit,

    //number input
    Interval(u64),

    //selection list
    KeyInput(usize, Key),
    KeyToggle(usize, Key),
    Unit(usize, Time),

    //tabs
    Tabs(Mode),

    //text input
    Sequence(String),
}

impl Sandbox for Icicle {
    type Message = Message;

    fn new() -> Self {
        Self {
            duration: Duration::default(),
            mode: Mode::default(),
            input: Key::W,
            interval: 0,
            sequence: String::default(),
            toggle: Key::CapsLock,
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

            Message::KeyInput(_, key) => self.input = key,
            Message::KeyToggle(_, key) => self.toggle = key,
            Message::Unit(_, unit) => self.unit = unit,

            Message::Tabs(input) => self.mode = input,

            Message::Sequence(sequence) => self.sequence = sequence,
        }
    }

    fn view(&self) -> Element<Message> {
        column!(
            tabs(self.mode, self.interval, self.sequence.as_str()),
            button(),
        )
        .spacing(30.0)
        .into()
    }
}
