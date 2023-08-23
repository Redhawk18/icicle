mod widgets;
use crate::input::init_input;
use crate::types::*;
use widgets::{button::button, tabs::tabs};

use iced::widget::column;
use iced::{font, Application, Command, Element};
use inputbot::KeybdKey;
use std::time::Duration;

pub struct Icicle {
    duration: Duration,
    input: KeybdKey,
    interval: u64,
    mode: Mode,
    sequence: String,
    toggle: KeybdKey,
    unit: Time,
}

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(Result<(), font::Error>),

    //button
    Submit,

    //number input
    Interval(u64),

    //selection list
    KeyInput(usize, KeybdKey),
    KeyToggle(usize, KeybdKey),
    Unit(usize, Time),

    //tabs
    Tabs(Mode),

    //text input
    Sequence(String),
}

impl Application for Icicle {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                duration: Duration::default(),
                mode: Mode::default(),
                input: KeybdKey::WKey,
                interval: 0,
                sequence: String::default(),
                toggle: KeybdKey::CapsLockKey,
                unit: Time::default(),
            },
            font::load(iced_aw::graphics::icons::ICON_FONT_BYTES).map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Icile")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::FontLoaded(_) => {}

            Message::Submit => {
                match self.unit {
                    Time::Minutes => self.duration = Duration::from_secs(self.interval * 60),
                    Time::Seconds => self.duration = Duration::from_secs(self.interval),
                    Time::Milliseconds => self.duration = Duration::from_millis(self.interval),
                    Time::Mircoseconds => self.duration = Duration::from_micros(self.interval),
                    Time::Nanoseconds => self.duration = Duration::from_nanos(self.interval),
                }
                init_input(
                    self.duration,
                    self.input,
                    self.mode,
                    self.sequence.clone(),
                    self.toggle,
                );
            }

            Message::Interval(interval) => self.interval = interval,

            Message::KeyInput(_, key) => self.input = key,
            Message::KeyToggle(_, key) => self.toggle = key,
            Message::Unit(_, unit) => self.unit = unit,

            Message::Tabs(input) => self.mode = input,

            Message::Sequence(sequence) => self.sequence = sequence,
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column!(
            tabs(self.mode, self.interval, self.sequence.as_str()),
            button(),
        )
        .spacing(30.0)
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}
