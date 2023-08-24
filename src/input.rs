use crate::types::Mode;

use inputbot::{handle_input_events, stop_handling_input_events, KeybdKey, KeySequence};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

///starts irreversible listening to the input bind for inputs, needs system level privileges.
pub fn start_input(
    duration: Duration,
    input: KeybdKey,
    mode: Mode,
    sequence: String,
    toggle: KeybdKey,
) -> JoinHandle<()> {
    match mode {
        Mode::Hold => spawn(move || hold(input, toggle)),
        Mode::Press => spawn(move || press(duration, input, toggle)),
        Mode::Sequence => spawn(move || key_sequence(duration, sequence, toggle)),
    }
}

pub fn end_input() {
    stop_handling_input_events()
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
    toggle.bind(move || {
        while toggle.is_toggled() {
            input.press();
            input.release();
            sleep(duration);
        }
    });

    handle_input_events();
}

fn key_sequence(duration: Duration, sequence: String, toggle: KeybdKey) {
    toggle.bind(move || {
        while toggle.is_toggled() {
            KeySequence(sequence.as_str()).send();
            sleep(duration);
        }
    });

    handle_input_events();
}
