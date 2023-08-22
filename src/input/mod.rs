use crate::types::{Key, Mode};

use inputbot::handle_input_events;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

///starts irreversible listening to the input bind for inputs, needs system level privileges.
pub fn init_input(
    duration: Duration,
    input: Key,
    mode: Mode,
    sequence: String,
    toggle: Key,
) -> JoinHandle<()> {
    match mode {
        Mode::Hold => spawn(move || hold(input, toggle)),
        Mode::Press => spawn(move || press(duration, input, toggle)),
        Mode::Sequence => spawn(move || key_sequence(duration, input, sequence, toggle)),
    }
}

fn hold(input: Key, toggle: Key) {
    

    handle_input_events();
}

fn press(duration: Duration, input: Key, toggle: Key) {
    sleep(duration);
    handle_input_events();
}

fn key_sequence(duration: Duration, input: Key, sequence: String, toggle: Key) {
    todo!()
}
