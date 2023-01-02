use blockfall_engine::{Engine, Operation, StandardBlockFactory, Vec2};
use eframe::egui;
use egui::{Color32, Stroke, Ui};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 320.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Block! Block! Block!",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

struct App {
    engine: Engine<StandardBlockFactory>,
}

impl Default for App {
    fn default() -> Self {
        let factory = StandardBlockFactory::new();
        let engine = Engine::new(factory);
        Self { engine }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input().key_pressed(egui::Key::A) {
            self.engine.update(Some(Operation::Left));
        } else if ctx.input().key_pressed(egui::Key::D) {
            self.engine.update(Some(Operation::Right));
        } else if ctx.input().key_pressed(egui::Key::L) {
            self.engine.update(Some(Operation::RotateClockwise));
        } else if ctx.input().key_pressed(egui::Key::K) {
            self.engine.update(Some(Operation::RotateCounterClockwise));
        } else {
            self.engine.update(None);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.ctx().request_repaint();

            // 背景
            ui.painter().rect(
                egui::Rect {
                    min: egui::pos2(90.0, 0.0),
                    max: egui::pos2(200.0, 260.0),
                },
                0.0, /*round */
                Color32::DARK_GRAY,
                Stroke::default(),
            );

            // 操作中のミノ
            push_draw_command(
                ui,
                &self.engine.get_current_block_translations(),
                Color32::RED,
            );

            // 積まれたミノ
            push_draw_command(ui, self.engine.get_block_translations(), Color32::BLUE);
        });
    }
}

fn push_draw_command(ui: &mut Ui, blocks: &[Vec2], color: Color32) {
    for block in blocks {
        let min_x = ((block.x + 10) * 10) as f32;
        let min_y = (250 - block.y * 10) as f32;
        let width = 8.0;
        let half_width = width / 2.0;

        ui.painter().rect(
            egui::Rect {
                min: egui::Pos2 {
                    x: min_x - half_width,
                    y: min_y - half_width,
                },
                max: egui::Pos2 {
                    x: min_x + half_width,
                    y: min_y + half_width,
                },
            },
            0.1, /*rounfing*/
            color,
            Stroke::default(),
        );
    }
}
