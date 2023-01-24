use eframe::egui;
use egui::Vec2;
use std::collections::HashMap;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(Board::default())),
    )
}

struct Board {
    //board_layout: Vec<BoardSlot>,
    board_layout: std::collections::HashMap<(i32, i32), BoardSlot>,
    turn: i32,
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
        }
    }
    fn paint_meme(&mut self, ui: &mut egui::Ui) {
        for x_cord in 0..9 {
            ui.horizontal(|ui| {
                for y_cord in 0..9 {
                    let x = &self.board_layout.get(&(x_cord, y_cord)).unwrap();
                    let b = egui::Button::new(format!("{}", x.slot_value))
                        .min_size(Vec2::new(30.0, 30.0));
                    ui.add(b);
                }
            });
        }
    }
}

impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("hi mom");
            self.paint_meme(ui);
        });
    }
}

// impl eframe::App for Board {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Connect Feris");
//             let mut curr_count = 0;
//             for x_cord in 0..9 {
//                 ui.horizontal(|ui| {
//                     for y_cord in 0..9 {
//                         let mut curr = self.board_layout.get_mut(&(x_cord, y_cord)).unwrap();
//                         let b = egui::Button::new(&curr.slot_value).min_size(Vec2::new(50.0, 50.0));
//                         if ui
//                             //.button(&curr.slot_value)
//                             .add(b)
//                             .on_hover_text(format!(
//                                 "x: {}, y: {}",
//                                 curr.x_coordinate, curr.y_coordinate
//                             ))
//                             .clicked()
//                         {
//                             if curr.slot_value == String::from("  ") {
//                                 if self.turn == 0 {
//                                     curr.slot_value = String::from("X");
//                                     self.turn = 1;
//                                 } else {
//                                     curr.slot_value = String::from("O");
//                                     self.turn = 0;
//                                 }
//                             }
//                             println!(
//                                 "{},{},{}",
//                                 curr.x_coordinate, curr.y_coordinate, curr.slot_value
//                             );
//                         };
//                         curr_count += 1;
//                     }
//                 });
//             }
//         });
//     }
// }
