use crate::gui::{widgets::number_input::number_input, Message};
use crate::types::Time;

use iced::{
    widget::{column, row, text, Column, Row},
    Alignment, Length,
};
use iced_aw::SelectionList;
use inputbot::KeybdKey;

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
static KEYS: &[KeybdKey] = &[
    KeybdKey::BackspaceKey,
    KeybdKey::TabKey,
    KeybdKey::EnterKey,
    KeybdKey::EscapeKey,
    KeybdKey::SpaceKey,
    KeybdKey::PageUpKey,
    KeybdKey::PageDownKey,
    KeybdKey::EndKey,
    KeybdKey::HomeKey,
    KeybdKey::LeftKey,
    KeybdKey::UpKey,
    KeybdKey::RightKey,
    KeybdKey::DownKey,
    KeybdKey::InsertKey,
    KeybdKey::DeleteKey,
    KeybdKey::Numrow0Key,
    KeybdKey::Numrow1Key,
    KeybdKey::Numrow2Key,
    KeybdKey::Numrow3Key,
    KeybdKey::Numrow4Key,
    KeybdKey::Numrow5Key,
    KeybdKey::Numrow6Key,
    KeybdKey::Numrow7Key,
    KeybdKey::Numrow8Key,
    KeybdKey::Numrow9Key,
    KeybdKey::AKey,
    KeybdKey::BKey,
    KeybdKey::CKey,
    KeybdKey::DKey,
    KeybdKey::EKey,
    KeybdKey::FKey,
    KeybdKey::GKey,
    KeybdKey::HKey,
    KeybdKey::IKey,
    KeybdKey::JKey,
    KeybdKey::KKey,
    KeybdKey::LKey,
    KeybdKey::MKey,
    KeybdKey::NKey,
    KeybdKey::OKey,
    KeybdKey::PKey,
    KeybdKey::QKey,
    KeybdKey::RKey,
    KeybdKey::SKey,
    KeybdKey::TKey,
    KeybdKey::UKey,
    KeybdKey::VKey,
    KeybdKey::WKey,
    KeybdKey::XKey,
    KeybdKey::YKey,
    KeybdKey::ZKey,
    KeybdKey::LSuper,
    KeybdKey::RSuper,
    KeybdKey::Numpad0Key,
    KeybdKey::Numpad1Key,
    KeybdKey::Numpad2Key,
    KeybdKey::Numpad3Key,
    KeybdKey::Numpad4Key,
    KeybdKey::Numpad5Key,
    KeybdKey::Numpad6Key,
    KeybdKey::Numpad7Key,
    KeybdKey::Numpad8Key,
    KeybdKey::Numpad9Key,
    KeybdKey::F1Key,
    KeybdKey::F2Key,
    KeybdKey::F3Key,
    KeybdKey::F4Key,
    KeybdKey::F5Key,
    KeybdKey::F6Key,
    KeybdKey::F7Key,
    KeybdKey::F8Key,
    KeybdKey::F9Key,
    KeybdKey::F10Key,
    KeybdKey::F11Key,
    KeybdKey::F12Key,
    KeybdKey::F13Key,
    KeybdKey::F14Key,
    KeybdKey::F15Key,
    KeybdKey::F16Key,
    KeybdKey::F17Key,
    KeybdKey::F18Key,
    KeybdKey::F19Key,
    KeybdKey::F20Key,
    KeybdKey::F21Key,
    KeybdKey::F22Key,
    KeybdKey::F23Key,
    KeybdKey::F24Key,
    KeybdKey::NumLockKey,
    KeybdKey::ScrollLockKey,
    KeybdKey::CapsLockKey,
    KeybdKey::LShiftKey,
    KeybdKey::RShiftKey,
    KeybdKey::LControlKey,
    KeybdKey::RControlKey,
    KeybdKey::LAltKey,
    KeybdKey::RAltKey,
    KeybdKey::BrowserBackKey,
    KeybdKey::BrowserForwardKey,
    KeybdKey::BrowserRefreshKey,
    KeybdKey::VolumeMuteKey,
    KeybdKey::VolumeDownKey,
    KeybdKey::VolumeUpKey,
    KeybdKey::MediaNextTrackKey,
    KeybdKey::MediaPrevTrackKey,
    KeybdKey::MediaStopKey,
    KeybdKey::MediaPlayPauseKey,
    KeybdKey::BackquoteKey,
    KeybdKey::SlashKey,
    KeybdKey::BackslashKey,
    KeybdKey::CommaKey,
    KeybdKey::PeriodKey,
    KeybdKey::MinusKey,
    KeybdKey::QuoteKey,
    KeybdKey::SemicolonKey,
    KeybdKey::LBracketKey,
    KeybdKey::RBracketKey,
    KeybdKey::EqualKey,
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
