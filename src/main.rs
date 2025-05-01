use eframe::egui::{self, Color32, Pos2, Rect, Stroke};
use env_logger::fmt::style::Color;
use uuid::Uuid;
mod canvas;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Fleet Sync",
        options,
        Box::new(|cc| Ok(Box::<App>::default())),
    )
}

pub struct Truck {
    pub id: Uuid,
}

pub struct SimulationState {
    pub trucks: Vec<Truck>,
}

pub struct UiState {
    pub pan: egui::Vec2,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self { trucks: Vec::new() }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            pan: egui::Vec2::ZERO,
        }
    }
}

pub struct App {
    pub sim: SimulationState,
    pub ui: UiState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            sim: SimulationState::default(),
            ui: UiState::default(),
        }
    }
}

impl App {
    pub fn draw_simulation_canvas(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();
            painter.rect_filled(rect, 0.0, Color32::from_gray(30));

            let road_width = rect.width() * 0.8;
            let road_height = rect.height() * 0.1;

            
            
            let road_rect = Rect::from_min_size(
                Pos2::new(rect.left() + 100.0, rect.top() + 200.0),
                egui::vec2(600.0, 60.0),
            );

            painter.rect_filled(road_rect, 0.0, Color32::DARK_GRAY);

            // draw centered white line

            let lane_y = road_rect.center().y;
            let lane_x_start = road_rect.left() + 20.0;
            let lane_x_end = road_rect.right() - 20.0;

            let mut x = lane_x_start;
            while x < lane_x_end {
                painter.line_segment(
                    [
                        Pos2::new(x, lane_y - 1.0),
                        Pos2::new(x + 10.0, lane_y - 1.0),
                    ],
                    Stroke::new(2.0, Color32::WHITE),
                );
                x+=20.0;
            }
        });
    }

    pub fn draw_control_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("control_panel")
            .default_height(80.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if ui.button("⏯ Play/Pause").clicked() {}
                    if ui.button("⏭ Step").clicked() {}
                    if ui.button("⟲ Reset").clicked() {}
                });
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.draw_simulation_canvas(ctx);
        self.draw_control_panel(ctx);
    }
}
