use crate::truck::Truck;
use eframe::egui::{self, Color32, Pos2, Rect, Stroke};

pub struct SimulationState {
    pub trucks: Vec<Truck>,
}

pub struct UiState {
    pub pan_offset: egui::Vec2,
    pub is_panning: bool,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self { trucks: Vec::new() }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            pan_offset: egui::Vec2::ZERO,
            is_panning: false,
        }
    }
}

pub struct App {
    pub ui: UiState,
    pub sim: SimulationState,
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
    
            // Fill background
            painter.rect_filled(rect, 0.0, Color32::from_gray(30));
    
            // 1) Road thickness
            let road_thickness = 85.0;
    
            // 2) Common center point
            let center = rect.center();
    
            // 3) Horizontal road centered
            let h_road = Rect::from_center_size(
                center,
                egui::vec2(rect.width(), road_thickness),
            );
            painter.rect_filled(h_road, 0.0, Color32::DARK_GRAY);
    
            // 4) Vertical road centered
            let v_road = Rect::from_center_size(
                center,
                egui::vec2(road_thickness, rect.height()),
            );
            painter.rect_filled(v_road, 0.0, Color32::DARK_GRAY);
    
            // Helper to draw dashed line along X or Y
            let draw_dashes = |start: Pos2, end: Pos2, along_x: bool| {
                let dash = 10.0;
                let gap = 10.0;
                if along_x {
                    let mut x = start.x;
                    while x < end.x {
                        painter.line_segment(
                            [Pos2::new(x, start.y), Pos2::new(x + dash, start.y)],
                            Stroke::new(2.0, Color32::WHITE),
                        );
                        x += dash + gap;
                    }
                } else {
                    let mut y = start.y;
                    while y < end.y {
                        painter.line_segment(
                            [Pos2::new(start.x, y), Pos2::new(start.x, y + dash)],
                            Stroke::new(2.0, Color32::WHITE),
                        );
                        y += dash + gap;
                    }
                }
            };
    
            // 5) Dashed center‐line for horizontal road
            let hy = center.y - 1.0;
            draw_dashes(
                Pos2::new(h_road.left() + 20.0, hy),
                Pos2::new(h_road.right() - 20.0, hy),
                true,
            );
    
            // 6) Dashed center‐line for vertical road
            let vx = center.x - 1.0;
            draw_dashes(
                Pos2::new(vx, v_road.top() + 20.0),
                Pos2::new(vx, v_road.bottom() - 20.0),
                false,
            );
    
            // 7) “Erase” any dashes in the overlap by painting the intersection last
            if let intersection = h_road.intersect(v_road) {
                painter.rect_filled(intersection, 0.0, Color32::DARK_GRAY);
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
