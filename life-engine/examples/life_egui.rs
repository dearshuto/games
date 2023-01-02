use egui::{Color32, Stroke};
use life_engine::{Engine, Request, StandardRule, Status, Template};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 640.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Life Game",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

struct App {
    engine: Engine<StandardRule>,
    frame: u8,
}

impl Default for App {
    fn default() -> Self {
        let mut toad: Vec<Request<Status>> = Template::toad(Status::Alive)
            .iter()
            .map(|r| Request {
                x: r.x + 20,
                y: r.y + 10,
                status: r.status,
            })
            .collect();
        let mut beacon: Vec<Request<Status>> = Template::beacon(Status::Alive)
            .iter()
            .map(|r| Request {
                x: r.x + 10,
                y: r.y + 10,
                status: r.status,
            })
            .collect();
        let mut octagon: Vec<Request<Status>> = Template::octagon(Status::Alive)
            .iter()
            .map(|r| Request {
                x: r.x + 35,
                y: r.y + 8,
                status: r.status,
            })
            .collect();

        let mut requests = Vec::new();
        requests.append(&mut toad);
        requests.append(&mut beacon);
        requests.append(&mut octagon);

        let factory = StandardRule::new();
        let engine = Engine::new_init(factory, &requests);
        Self { engine, frame: 0 }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame += 1;
        if 20 < self.frame {
            self.engine.update(&[]);
            self.frame = 0;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.ctx().request_repaint();

            for y in 0..256 {
                for x in 0..256 {
                    let status = self.engine.get_status(x, y);
                    let color = match status {
                        life_engine::Status::Alive => Color32::LIGHT_GRAY,
                        life_engine::Status::Dead => Color32::DARK_GRAY,
                    };

                    let x = x as f32 * 10.0;
                    let y = y as f32 * 10.0;
                    ui.painter().rect(
                        egui::Rect {
                            min: egui::pos2(x, y),
                            max: egui::pos2(x + 7.0, y + 7.0),
                        },
                        0.0, /*round */
                        color,
                        Stroke::default(),
                    );
                }
            }
        });
    }
}
