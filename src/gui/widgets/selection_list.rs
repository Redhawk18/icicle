use crate::gui::Message;

use iced::widget::{column, text, Column, Row, row};
use iced::Length;
use iced_aw::SelectionList;

pub fn selection_keys() -> Row<'static, Message> {
    row!(selection_key_toggle(), selection_key_input()).padding(20)
}

fn selection_key_input() -> Column<'static, Message> {
    column!(
        text("Input Key:"),
        SelectionList::new(KEYS, Message::Key,)
            .height(Length::Fixed(120.0))
            .width(Length::Fixed(100.0))
    )
}

fn selection_key_toggle() -> Column<'static, Message> {
    column!(
        text("Toggle Key:"),
        SelectionList::new(KEYS, Message::Key,)
            .height(Length::Fixed(120.0))
            .width(Length::Fixed(100.0))
    )
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Key {
    Backspace,
    Tab,
    Enter,
    Escape,
    Space,
    PageUp,
    PageDown,
    End,
    Home,
    Left,
    Up,
    Right,
    Down,
    Insert,
    Delete,
    Numrow0,
    Numrow1,
    Numrow2,
    Numrow3,
    Numrow4,
    Numrow5,
    Numrow6,
    Numrow7,
    Numrow8,
    Numrow9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LSu,
    RSu,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    NumLock,
    ScrollLock,
    CapsLock,
    LShift,
    RShift,
    LControl,
    RControl,
    LAlt,
    RAlt,
    BrowserBack,
    BrowserForward,
    BrowserRefresh,
    VolumeMute,
    VolumeDown,
    VolumeUp,
    MediaNextTrack,
    MediaPrevTrack,
    MediaStop,
    MediaPlayPause,
    Backquote,
    Slash,
    Backslash,
    Comma,
    Period,
    Minus,
    Quote,
    Semicolon,
    LBracket,
    RBracket,
    Equal,
}

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

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Key::Backspace => "Backspace",
                Key::Tab => "Tab",
                Key::Enter => "Enter",
                Key::Escape => "Escape",
                Key::Space => "Space",
                Key::PageUp => "Page Up",
                Key::PageDown => "Page Down",
                Key::End => "End",
                Key::Home => "Home",
                Key::Left => "Left",
                Key::Up => "Up",
                Key::Right => "Right",
                Key::Down => "Down",
                Key::Insert => "Insert",
                Key::Delete => "Delete",
                Key::Numrow0 => "0",
                Key::Numrow1 => "1",
                Key::Numrow2 => "2",
                Key::Numrow3 => "3",
                Key::Numrow4 => "4",
                Key::Numrow5 => "5",
                Key::Numrow6 => "6",
                Key::Numrow7 => "7",
                Key::Numrow8 => "8",
                Key::Numrow9 => "9",
                Key::A => "A",
                Key::B => "B",
                Key::C => "C",
                Key::D => "D",
                Key::E => "E",
                Key::F => "F",
                Key::G => "G",
                Key::H => "H",
                Key::I => "I",
                Key::J => "J",
                Key::K => "K",
                Key::L => "L",
                Key::M => "M",
                Key::N => "N",
                Key::O => "O",
                Key::P => "P",
                Key::Q => "q",
                Key::R => "R",
                Key::S => "S",
                Key::T => "T",
                Key::U => "U",
                Key::V => "V",
                Key::W => "W",
                Key::X => "X",
                Key::Y => "Y",
                Key::Z => "Z",
                Key::LSu =>
                    if cfg!(target_family = "windows") {
                        "Left Windows"
                    } else {
                        "Left Super"
                    },
                Key::RSu =>
                    if cfg!(target_family = "windows") {
                        "Right Windows"
                    } else {
                        "Right Super"
                    },
                Key::Numpad0 => "Numpad 0",
                Key::Numpad1 => "Numpad 1",
                Key::Numpad2 => "Numpad 2",
                Key::Numpad3 => "Numpad 3",
                Key::Numpad4 => "Numpad 4",
                Key::Numpad5 => "Numpad 5",
                Key::Numpad6 => "Numpad 6",
                Key::Numpad7 => "Numpad 7",
                Key::Numpad8 => "Numpad 8",
                Key::Numpad9 => "Numpad 9",
                Key::F1 => "F1",
                Key::F2 => "F2",
                Key::F3 => "F3",
                Key::F4 => "F4",
                Key::F5 => "F5",
                Key::F6 => "F6",
                Key::F7 => "F7",
                Key::F8 => "F8",
                Key::F9 => "F9",
                Key::F10 => "F10",
                Key::F11 => "F11",
                Key::F12 => "F12",
                Key::F13 => "F13",
                Key::F14 => "F14",
                Key::F15 => "F15",
                Key::F16 => "F16",
                Key::F17 => "F17",
                Key::F18 => "F18",
                Key::F19 => "F19",
                Key::F20 => "F20",
                Key::F21 => "F21",
                Key::F22 => "F22",
                Key::F23 => "F23",
                Key::F24 => "F24",
                Key::NumLock => "NumLock",
                Key::ScrollLock => "ScrollLock",
                Key::CapsLock => "CapsLock",
                Key::LShift => "Left Shift",
                Key::RShift => "Right Shift",
                Key::LControl => "Left Control",
                Key::RControl => "Right Control",
                Key::LAlt => "Left Alt",
                Key::RAlt => "Right Alt",
                Key::BrowserBack => "Back",
                Key::BrowserForward => "Forward",
                Key::BrowserRefresh => "Refresh",
                Key::VolumeMute => "Volume Mute",
                Key::VolumeDown => "Volume Down",
                Key::VolumeUp => "Volume Up",
                Key::MediaNextTrack => "Media Next",
                Key::MediaPrevTrack => "Media Previous",
                Key::MediaStop => "Media Stop",
                Key::MediaPlayPause => "Media Pause",
                Key::Backquote => "Backquote",
                Key::Slash => "Slash",
                Key::Backslash => "Backslash",
                Key::Comma => "Comma",
                Key::Period => "Period",
                Key::Minus => "Minus",
                Key::Quote => "Quote",
                Key::Semicolon => "Semicolon",
                Key::LBracket => "Left Bracket",
                Key::RBracket => "Right Bracket",
                Key::Equal => "Equal",
            }
        )
    }
}

pub fn selection_time(header: &str) -> Column<'static, Message> {
    column!(
        text(header),
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
}

#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
pub enum Time {
    Minutes,
    #[default]
    Seconds,
    Milliseconds,
    Mircoseconds,
    Nanoseconds,
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Time::Minutes => "Minutes",
                Time::Seconds => "Seconds",
                Time::Milliseconds => "Milliaseconds",
                Time::Mircoseconds => "Mircoseconds",
                Time::Nanoseconds => "Nanoseconds",
            }
        )
    }
}
