mod gui;
use gui::Icicle;

use iced::{Application, Settings};

fn main() -> iced::Result {
    Icicle::run(Settings::default())
}