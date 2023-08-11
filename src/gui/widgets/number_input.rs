use crate::gui::Message;

use iced_aw::NumberInput;
use std::time::Duration;

pub fn number_input<'a>(interval: Duration) -> NumberInput<'a, u64, Message> {
    NumberInput::new(interval.as_secs(), Duration::MAX.as_secs(), Message::NumberInput)
}
