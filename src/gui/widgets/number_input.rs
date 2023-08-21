use crate::gui::Message;

use iced::widget::{column, text, Column};
use iced::Length;
use iced_aw::NumberInput;
use std::time::Duration;

pub fn number_input(interval: u64) -> Column<'static, Message> {
    column!(
        text("Interval:"),
        NumberInput::new(interval, Duration::MAX.as_secs(), Message::Interval)
            .width(Length::Shrink)
    )
}
