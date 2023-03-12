use eframe::egui;
use egui::Vec2;
use std::collections::HashMap;

pub enum Direction {
    Vertical,
    Horizontal,
    DiagonalLR,
    DiagonalRL,
}
#[derive(Clone)]
pub struct Board {
    board_layout: HashMap<(i32, i32), BoardSlot>,
    turn: i32,
    win_state: bool,
    win_slots: Vec<(i32, i32)>,
}
#[derive(Clone)]
pub struct BoardSlot {
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
                b.board_layout.insert((x_cord, y_cord), bl);
            }
        }
        b
    }
}

impl Board {
    fn new() -> Self {
        Board {
            board_layout: HashMap::new(),
            turn: 0,
            win_state: false,
            win_slots: Vec::new(),
        }
    }
    fn reset(&mut self) {
        *self = Self::default();
    }
    fn paint_board(&mut self, ui: &mut egui::Ui) {
        for y_cord in 0..9 {
            ui.horizontal(|ui| {
                for x_cord in 0..9 {
                    let current_slot = self.board_layout.get(&(x_cord, y_cord)).unwrap();
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
                            log::debug!("Selected: x: {}, y: {}", x_cord, y_cord);
                            self.change_value_slot(x_cord, y_cord, self.turn);
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
    fn change_value_slot(&mut self, x: i32, y: i32, current_turn: i32) {
        let turn = current_turn;
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
        //self.check_if_won_horizontally(x, y);
        //self.check_if_won_vertically(x, y);
        //self.check_if_won_diag_lr(x, y);
        //self.check_if_won_diag_rl(x, y);
        self.check_win_condition(x, y, Direction::Vertical);
        self.check_win_condition(x, y, Direction::Horizontal);
        self.check_win_condition(x, y, Direction::DiagonalLR);
        self.check_win_condition(x, y, Direction::DiagonalRL);
    }

    fn check_win_condition(&mut self, x: i32, y: i32, direction: Direction) {
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

            match direction {
                Direction::Vertical => {
                    let mut slots_left: Vec<(&str, i32, i32)> = Vec::new();
                    let mut slots_right: Vec<(&str, i32, i32)> = Vec::new();
                    // we are doing this twice, as we want to break out of the loop if the board slot at x-i/x+i isn't the same value
                    // as the board slot value of value
                    // TODO: this could all be made with one loop checking all vectors,
                    // don't be lazy and fix this

                    for i in 1..4 {
                        if let Some(slot) = self.board_layout.get(&(x, y + i)) {
                            if slot.slot_value != value.slot_value {
                            } else {
                                slots_left.push((slot.slot_value.as_str(), x, y + i));
                            }
                        }
                    }
                    for i in 1..4 {
                        if let Some(slot) = self.board_layout.get(&(x, y - i)) {
                            if slot.slot_value != value.slot_value {
                                break;
                            } else {
                                slots_right.push((slot.slot_value.as_str(), x, y-i));
                            }
                        }
                    }
                    //this just appends the left and right vecs, and now slots_right no longer exists after this
                    slots_left.push((value.slot_value.as_str(),x,y));
                    slots_left.append(&mut slots_right);
                    if let Some(value) =
                        Board::check_potential_win(slots_left, value.slot_value.clone())
                    {
                        self.win_slots = value;
                        self.win_state = true;
                    }
                }
                Direction::Horizontal => {
                    while iterations < 4 {
                        if direction_of_needle == 0 {
                            if let Some(vert_up) = self.board_layout.get(&(x - needle, y)) {
                                if value.slot_value == vert_up.slot_value {
                                    winning_slots.push((x - needle, y));
                                    count += 1;
                                    needle += 1;
                                } else {
                                    direction_of_needle = 1;
                                    needle = 1;
                                }
                            } else {
                                direction_of_needle = 1;
                                needle = 1;
                            }
                        } else {
                            // TODO: change vert_up
                            if let Some(vert_up) = self.board_layout.get(&(x + needle, y)) {
                                if value.slot_value == vert_up.slot_value {
                                    winning_slots.push((x + needle, y));
                                    count += 1;
                                    needle += 1;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        iterations += 1;
                    }
                }
                Direction::DiagonalLR => {
                    while iterations < 4 {
                        if direction_of_needle == 0 {
                            if let Some(vert_up) = self.board_layout.get(&(x + needle, y - needle))
                            {
                                if value.slot_value == vert_up.slot_value {
                                    winning_slots.push((x + needle, y - needle));
                                    count += 1;
                                    needle += 1;
                                } else {
                                    direction_of_needle = 1;
                                    needle = 1;
                                }
                            } else {
                                direction_of_needle = 1;
                                needle = 1;
                            }
                        } else {
                            if let Some(vert_up) = self.board_layout.get(&(x - needle, y + needle))
                            {
                                if value.slot_value == vert_up.slot_value {
                                    winning_slots.push((x - needle, y + needle));
                                    count += 1;
                                    needle += 1;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        iterations += 1;
                    }
                }
                Direction::DiagonalRL => {
                    while iterations < 4 {
                        if direction_of_needle == 0 {
                            if let Some(vert_up) = self.board_layout.get(&(x + needle, y + needle))
                            {
                                if value.slot_value == vert_up.slot_value {
                                    winning_slots.push((x + needle, y + needle));
                                    count += 1;
                                    needle += 1;
                                } else {
                                    direction_of_needle = 1;
                                    needle = 1;
                                }
                            } else {
                                direction_of_needle = 1;
                                needle = 1;
                            }
                        } else {
                            if let Some(vert_up) = self.board_layout.get(&(x - needle, y - needle))
                            {
                                if value.slot_value == vert_up.slot_value {
                                    winning_slots.push((x - needle, y - needle));
                                    count += 1;
                                    needle += 1;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        iterations += 1;
                    }
                }
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
    fn check_potential_win(
        slots_values: Vec<(&str, i32, i32)>,
        slot_value: String,
    ) -> Option<Vec<(i32, i32)>> {
        let mut stack: Vec<(i32, i32)> = Vec::new();
        for slot in slots_values.iter() {
            //should this be 0?
            if slot.0 == slot_value {
                stack.push((slot.1, slot.2));
                if stack.len() == 4 {
                    return Some(stack.clone());
                }
            } else {
                stack.clear();
            }
        }
        return None;
    }
}

impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let reset_button = egui::Button::new("RESET GAME");
            ui.horizontal_top(|ui| {
                ui.label("CONNECT FERRIS");
                if ui.add(reset_button).clicked() {
                    self.reset();
                }
                if self.turn == 0 {
                    ui.label("O's turn");
                } else {
                    ui.label("X's turn");
                }
            });

            self.paint_board(ui);
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;
    #[test]
    fn test_horizontal() {
        let mut b = Board::default();
        b.change_value_slot(0, 0, 0);
        b.change_value_slot(1, 0, 0);
        b.change_value_slot(2, 0, 0);
        b.change_value_slot(3, 0, 0);

        b.check_if_won(1, 0);
        assert_eq!(b.win_state, true)
    }
    #[test]
    fn test_vertical() {
        let mut b = Board::default();
        b.change_value_slot(0, 1, 0);
        b.change_value_slot(0, 2, 0);
        b.change_value_slot(0, 3, 0);
        b.change_value_slot(0, 4, 0);

        b.check_if_won(0, 1);
        assert_eq!(b.win_state, true)
    }
    #[test]
    fn test_diag_lr() {
        let mut b = Board::default();
        b.change_value_slot(0, 8, 0);
        b.change_value_slot(1, 7, 0);
        b.change_value_slot(2, 6, 0);
        b.change_value_slot(3, 5, 0);

        b.check_if_won(0, 1);
        assert_eq!(b.win_state, true)
    }
    #[test]
    fn test_diag_rl() {
        let mut b = Board::default();
        b.change_value_slot(8, 8, 0);
        b.change_value_slot(7, 7, 0);
        b.change_value_slot(6, 6, 0);
        b.change_value_slot(5, 5, 0);

        b.check_if_won(0, 1);
        assert_eq!(b.win_state, true)
    }
}
