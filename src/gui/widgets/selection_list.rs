use crate::gui::widgets::number_input::number_input;
use crate::gui::Message;
use crate::types::{Key, Time};

use iced::widget::{column, row, text, Column, Row};
use iced::{Alignment, Length};
use iced_aw::SelectionList;

pub fn selection_keys() -> Row<'static, Message> {
    row!(selection_key_toggle(), selection_key_input()).spacing(15.0)
}

fn selection_key_input() -> Column<'static, Message> {
    column!(
        text("Input Key:"),
        SelectionList::new(KEYS, Message::KeyInput)
            .height(Length::Fixed(120.0))
            .width(Length::Fixed(100.0))
    )
}

fn selection_key_toggle() -> Column<'static, Message> {
    column!(
        text("Toggle Key:"),
        SelectionList::new(KEYS, Message::KeyToggle)
            .height(Length::Fixed(120.0))
            .width(Length::Fixed(100.0))
    )
}
//TODO input and toggle can not be the same set of keys because mousebuttons can not be toggled like keyboard keys can
static KEYS: &[Key] = &[
    Key::Backspace,
    Key::Tab,
    Key::Enter,
    Key::Escape,
    Key::Space,
    Key::PageUp,
    Key::PageDown,
    Key::End,
    Key::Home,
    Key::Left,
    Key::Up,
    Key::Right,
    Key::Down,
    Key::Insert,
    Key::Delete,
    Key::Numrow0,
    Key::Numrow1,
    Key::Numrow2,
    Key::Numrow3,
    Key::Numrow4,
    Key::Numrow5,
    Key::Numrow6,
    Key::Numrow7,
    Key::Numrow8,
    Key::Numrow9,
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
    Key::LSu,
    Key::RSu,
    Key::Numpad0,
    Key::Numpad1,
    Key::Numpad2,
    Key::Numpad3,
    Key::Numpad4,
    Key::Numpad5,
    Key::Numpad6,
    Key::Numpad7,
    Key::Numpad8,
    Key::Numpad9,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::F11,
    Key::F12,
    Key::F13,
    Key::F14,
    Key::F15,
    Key::F16,
    Key::F17,
    Key::F18,
    Key::F19,
    Key::F20,
    Key::F21,
    Key::F22,
    Key::F23,
    Key::F24,
    Key::NumLock,
    Key::ScrollLock,
    Key::CapsLock,
    Key::LShift,
    Key::RShift,
    Key::LControl,
    Key::RControl,
    Key::LAlt,
    Key::RAlt,
    Key::BrowserBack,
    Key::BrowserForward,
    Key::BrowserRefresh,
    Key::VolumeMute,
    Key::VolumeDown,
    Key::VolumeUp,
    Key::MediaNextTrack,
    Key::MediaPrevTrack,
    Key::MediaStop,
    Key::MediaPlayPause,
    Key::Backquote,
    Key::Slash,
    Key::Backslash,
    Key::Comma,
    Key::Period,
    Key::Minus,
    Key::Quote,
    Key::Semicolon,
    Key::LBracket,
    Key::RBracket,
    Key::Equal,
];

pub fn selection_time(interval: u64) -> Row<'static, Message> {
    row!(
        number_input(interval),
        column!(
            text("Unit:"),
            SelectionList::new(
                &[
                    Time::Minutes,
                    Time::Seconds,
                    Time::Milliseconds,
                    Time::Mircoseconds,
                    Time::Nanoseconds,
                ],
                Message::Unit,
            )
            .height(Length::Fixed(120.0))
            .width(Length::Fixed(100.0))
        )
    )
    .align_items(Alignment::Center)
    .spacing(10.0)
}
