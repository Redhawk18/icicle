use crate::gui::Message;

use iced::Length;
use iced_aw::NumberInput;
use std::time::Duration;

pub fn number_input<'a>(interval: u64) -> NumberInput<'a, u64, Message> {
    NumberInput::new(interval, Duration::MAX.as_secs(), Message::Interval).width(Length::Shrink)
}
