use crate::types::Mode;

use inputbot::{KeybdKey, handle_input_events};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

///starts irreversible listening to the input bind for inputs, needs system level privileges.
pub fn init_input(
    duration: Duration,
    input: KeybdKey,
    mode: Mode,
    sequence: String,
    toggle: KeybdKey,
) -> JoinHandle<()> {
    match mode {
        Mode::Hold => spawn(move || hold(input, toggle)),
        Mode::Press => spawn(move || press(duration, input, toggle)),
        Mode::Sequence => spawn(move || key_sequence(duration, input, sequence, toggle)),
    }
}

fn hold(input: KeybdKey, toggle: KeybdKey) {
    toggle.bind(move || {
        while toggle.is_toggled() {
            input.press();

        }
        input.release();
    });

    handle_input_events();
}

fn press(duration: Duration, input: KeybdKey, toggle: KeybdKey) {
    sleep(duration);
    handle_input_events();
}

fn key_sequence(duration: Duration, input: KeybdKey, sequence: String, toggle: KeybdKey) {
    todo!()
}
