pub mod board;
use crate::board::Board;
use eframe::egui;

pub fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(530.0, 510.0)),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(Board::default())),
    )
}
