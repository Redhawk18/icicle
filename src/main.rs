mod gui;
mod input;
mod types;
use gui::Icicle;

use iced::{window, Application, Settings};

fn main() -> iced::Result {
    Icicle::run(Settings {
        id: Some(String::from("Icicle")),
        window: window::Settings {
            size: (640, 480),
            ..Default::default()
        },
        ..Default::default()
    })
}
