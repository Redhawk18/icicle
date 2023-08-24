use crate::types::Mode;

#[cfg(target_os = "linux")]
use inputbot::stop_handling_input_events;
use inputbot::{handle_input_events, KeySequence, KeybdKey};
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
    #[cfg(target_os = "linux")]
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
