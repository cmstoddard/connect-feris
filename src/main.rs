use eframe::egui;
use egui::Vec2;
use std::collections::HashMap;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

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

struct Board {
    board_layout: std::collections::HashMap<(i32, i32), BoardSlot>,
    turn: i32,
    win_state: bool,
    win_slots: Vec<(i32, i32)>,
}

struct BoardSlot {
    x_coordinate: i32,
    y_coordinate: i32,
    slot_value: String,
}

impl Default for Board {
    fn default() -> Self {
        let mut b = Board::new();
        for y_cord in 0..9 {
            for x_cord in 0..9 {
                let bl = BoardSlot {
                    x_coordinate: x_cord,
                    y_coordinate: y_cord,
                    slot_value: String::from("  "),
                };
                //b.board_layout.push(bl);
                b.board_layout.insert((x_cord, y_cord), bl);
            }
        }
        b
    }
}
impl Board {
    fn new() -> Self {
        Board {
            //board_layout: Vec::new(),
            board_layout: HashMap::new(),
            turn: 0,
            win_state: false,
            win_slots: Vec::new(),
        }
    }
    fn paint_board(&mut self, ui: &mut egui::Ui) {
        for y_cord in 0..9 {
            ui.horizontal(|ui| {
                for x_cord in 0..9 {
                    let current_slot = self.board_layout.get(&(x_cord, y_cord)).unwrap();
                    //let b = egui::Button::new(format!("{}", x.slot_value))
                    let b = egui::Button::new(format!("{}", current_slot.slot_value))
                        .min_size(Vec2::new(50.0, 50.0));
                    if !self.win_state {
                        if ui
                            .add_enabled(true, b)
                            .on_hover_text(format!(
                                "x: {}, y: {}",
                                current_slot.x_coordinate, current_slot.y_coordinate
                            ))
                            .clicked()
                        {
                            self.change_value_slot(x_cord, y_cord);
                            self.check_if_won(x_cord, y_cord);
                        };
                    } else {
                        if self.win_slots.contains(&(x_cord, y_cord)) {
                            ui.add_enabled(false, b.fill(egui::Color32::YELLOW))
                                .on_hover_text(format!(
                                    "x: {}, y: {}",
                                    current_slot.x_coordinate, current_slot.y_coordinate
                                ));
                        } else {
                            ui.add_enabled(false, b).on_hover_text(format!(
                                "x: {}, y: {}",
                                current_slot.x_coordinate, current_slot.y_coordinate
                            ));
                        }
                    }
                }
            });
        }
    }
    fn change_value_slot(&mut self, x: i32, y: i32) {
        let turn = self.turn;
        let board_slot = self.board_layout.get_mut(&(x, y)).unwrap();
        //TODO: refactor slot_value
        if board_slot.slot_value == String::from("  ") {
            if turn == 0 {
                board_slot.slot_value = String::from(" O ");
                self.turn = 1;
            } else {
                board_slot.slot_value = String::from(" X ");
                self.turn = 0;
            }
        }
    }
    fn check_if_won(&mut self, x: i32, y: i32) {
        self.check_if_won_horizontally(x, y);
        self.check_if_won_vertically(x, y);
        self.check_if_won_diag_lr(x, y);
        self.check_if_won_diag_rl(x, y);
    }

    //TODO: you could make an enum with the values of x and y, and the vector that you want checked
    //to get rid of the 4 functions
    fn check_if_won_vertically(&mut self, x: i32, y: i32) {
        //check if won vertically
        if let Some(value) = self.board_layout.get(&(x, y)) {
            //looking for 4 in a sequence
            let mut iterations = 0;
            //this is the distance from the origin, to the current point
            let mut needle = 1;
            //0 is moving left, 1 is moving right to search
            let mut direction_of_needle = 0;
            //consecutive slots of the same value
            let mut count = 1;

            let mut winning_slots: Vec<(i32, i32)> = Vec::new();
            while iterations < 4 {
                if direction_of_needle == 0 {
                    if let Some(vert_up) = self.board_layout.get(&(x, y - needle)) {
                        if value.slot_value == vert_up.slot_value {
                            winning_slots.push((x, y - needle));
                            count += 1;
                            needle += 1;
                        } else {
                            direction_of_needle = 1;
                            needle = 1;
                        }
                    }
                } else {
                    if let Some(vert_up) = self.board_layout.get(&(x, y + needle)) {
                        if value.slot_value == vert_up.slot_value {
                            winning_slots.push((x, y + needle));
                            count += 1;
                            needle += 1;
                        } else {
                            break;
                        }
                    }
                }
                iterations += 1;
            }
            if count == 4 {
                self.win_state = true;
                winning_slots.push((x, y));
                self.win_slots = winning_slots;
            } else {
                winning_slots.clear();
            }
        }
    }

    fn check_if_won_horizontally(&mut self, x: i32, y: i32) {
        //check if won horizontally
        if let Some(value) = self.board_layout.get(&(x, y)) {
            //looking for 4 in a sequence
            let mut iterations = 0;
            //this is the distance from the origin, to the current point
            let mut needle = 1;
            //0 is moving left, 1 is moving right to search
            let mut direction_of_needle = 0;
            //consecutive slots of the same value
            let mut count = 1;
            while iterations < 4 {
                if direction_of_needle == 0 {
                    if let Some(vert_up) = self.board_layout.get(&(x - needle, y)) {
                        if value.slot_value == vert_up.slot_value {
                            count += 1;
                            needle += 1;
                        } else {
                            direction_of_needle = 1;
                            needle = 1;
                        }
                    }
                } else {
                    if let Some(vert_up) = self.board_layout.get(&(x + needle, y)) {
                        if value.slot_value == vert_up.slot_value {
                            count += 1;
                            needle += 1;
                        } else {
                            break;
                        }
                    }
                }
                iterations += 1;
            }
            if count == 4 {
                self.win_state = true;
            }
        }
    }

    fn check_if_won_diag_lr(&mut self, x: i32, y: i32) {
        if let Some(value) = self.board_layout.get(&(x, y)) {
            //looking for 4 in a sequence
            let mut iterations = 0;
            //this is the distance from the origin, to the current point
            let mut needle = 1;
            //0 is moving left, 1 is moving right to search
            let mut direction_of_needle = 0;
            //consecutive slots of the same value
            let mut count = 1;

            while iterations < 4 {
                if direction_of_needle == 0 {
                    if let Some(vert_up) = self.board_layout.get(&(x - needle, y + needle)) {
                        if value.slot_value == vert_up.slot_value {
                            count += 1;
                            needle += 1;
                        } else {
                            direction_of_needle = 1;
                            needle = 1;
                        }
                    }
                } else {
                    if let Some(vert_up) = self.board_layout.get(&(x + needle, y - needle)) {
                        if value.slot_value == vert_up.slot_value {
                            count += 1;
                            needle += 1;
                        } else {
                            break;
                        }
                    }
                }
                iterations += 1;
            }
            if count == 4 {
                self.win_state = true;
            }
        }
    }

    fn check_if_won_diag_rl(&mut self, x: i32, y: i32) {
        if let Some(value) = self.board_layout.get(&(x, y)) {
            //looking for 4 in a sequence
            let mut iterations = 0;
            //this is the distance from the origin, to the current point
            let mut needle = 1;
            //0 is moving left, 1 is moving right to search
            let mut direction_of_needle = 0;
            //consecutive slots of the same value
            let mut count = 1;

            while iterations < 4 {
                if direction_of_needle == 0 {
                    if let Some(vert_up) = self.board_layout.get(&(x + needle, y + needle)) {
                        if value.slot_value == vert_up.slot_value {
                            count += 1;
                            needle += 1;
                        } else {
                            direction_of_needle = 1;
                            needle = 1;
                        }
                    }
                } else {
                    if let Some(vert_up) = self.board_layout.get(&(x - needle, y - needle)) {
                        if value.slot_value == vert_up.slot_value {
                            count += 1;
                            needle += 1;
                        } else {
                            break;
                        }
                    }
                }
                iterations += 1;
            }
            if count == 4 {
                self.win_state = true;
            }
        }
    }
}

impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //egui::CentralPanel::default().show(ctx, |ui| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("connect feris :)");
            self.paint_board(ui);
        });
    }
}
