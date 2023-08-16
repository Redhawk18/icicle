mod widgets;
use iced::widget::{button, column, row, text};
use iced::{Element, Sandbox};
use std::time::Duration;
use widgets::number_input::number_input;
use widgets::selection_list::{selection_list, Time};

pub struct Icicle {
    duration: Duration,
    interval: u64,
    unit: Time,
}

#[derive(Debug, Clone)]
pub enum Message {
    //number input
    NumberInput(u64),

    //selection list
    SelectionList(usize, Time),

    Submit,
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

            Message::NumberInput(interval) => self.interval = interval,

            Message::SelectionList(_, unit) => self.unit = unit,
        }
    }

    fn view(&self) -> Element<Message> {
        println!("{:#?}", self.duration);
        column!(
            row!(
                text("Press Interval"),
                number_input(self.interval),
                selection_list(),
            ),
            row!(button("Submit").on_press(Message::Submit))
        )
        .into()
    }
}
