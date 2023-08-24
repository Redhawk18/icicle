mod widgets;
use crate::input::{end_input, start_input};
use crate::types::{Mode, Time};
use widgets::{
    button::{start, stop},
    tabs::tabs,
};

use iced::{
    font,
    widget::{column, container, row},
    Alignment, Application, Command, Element, Length,
};
use inputbot::KeybdKey;
use std::time::Duration;

pub struct Icicle {
    active: bool,
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
    Start,
    Stop,

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
                active: false,
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
        if self.active {
            String::from(match self.mode {
                Mode::Hold => format!("Toggle {} to hold {}", self.toggle, self.input),
                Mode::Press => format!("Toggle {} to press {} every {:?}", self.toggle, self.input, self.duration),
                Mode::Sequence => format!("Toggle {} to send a sequence of {} every {:?}", self.toggle, self.sequence, self.duration),
            })
        } else {
            String::from("Icile")
        }
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::FontLoaded(_) => {}

            Message::Start => {
                match self.unit {
                    Time::Minutes => self.duration = Duration::from_secs(self.interval * 60),
                    Time::Seconds => self.duration = Duration::from_secs(self.interval),
                    Time::Milliseconds => self.duration = Duration::from_millis(self.interval),
                    Time::Mircoseconds => self.duration = Duration::from_micros(self.interval),
                    Time::Nanoseconds => self.duration = Duration::from_nanos(self.interval),
                }
                start_input(
                    self.duration,
                    self.input,
                    self.mode,
                    self.sequence.clone(),
                    self.toggle,
                );
                self.active = true;
            }
            Message::Stop => {
                end_input();
                self.active = false;
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
        let r1 = row!(tabs(self.mode, self.interval, self.sequence.as_str())).spacing(30.0);

        let c1 = column!().width(Length::Fill);

        let mut c2 = column!().width(100).align_items(Alignment::End);

        if self.active {
            if cfg!(target_os = "linux") {
                c2 = c2.push(stop())
            }
        } else {
            c2 = c2.push(start())
        }

        let r2 = row![c1, c2]
            .height(Length::Fill)
            .align_items(Alignment::End);

        let content = column!(r1, r2);
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }
}
