use app::App;
use eframe::egui::{self};
mod canvas;
mod app;
mod truck;

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

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.draw_simulation_canvas(ctx);
        self.draw_control_panel(ctx);
    }
}
