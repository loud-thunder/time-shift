use super::animation_panel::{render_animation_panel, AnimationPanelState};
use super::canvas::{render_canvas, CanvasState};
use super::layer_panel::{color_picker_button, render_layer_panel};
use crate::export::svg_exporter::export_project_to_svg;
use crate::model::project::Project;
use crate::presets::{get_curated_palettes, get_curated_presets};
use egui::{DragValue, Slider};
use std::time::Instant;

pub struct TimeShiftApp {
    pub project: Project,
    pub canvas_state: CanvasState,
    pub anim_panel_state: AnimationPanelState,
    pub playing: bool,
    pub current_time: f32,
    pub playback_speed: f32,
    pub last_frame_time: Instant,
}

impl Default for TimeShiftApp {
    fn default() -> Self {
        Self {
            project: Project::default(),
            canvas_state: CanvasState::default(),
            anim_panel_state: AnimationPanelState::default(),
            playing: true,
            current_time: 0.0,
            playback_speed: 1.0,
            last_frame_time: Instant::now(),
        }
    }
}

impl eframe::App for TimeShiftApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Advance animation time
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        if self.playing {
            self.current_time += dt * self.playback_speed;
            ctx.request_repaint();
        }

        // Top Menu / Toolbar
        egui::TopBottomPanel::top("top_toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⏱ Time-Shift");
                ui.separator();

                // Presets Dropdown
                egui::ComboBox::from_label("Starter Preset")
                    .selected_text("Load Preset...")
                    .show_ui(ui, |ui| {
                        for p in get_curated_presets() {
                            if ui.button(p.name).on_hover_text(p.description).clicked() {
                                self.project = (p.project_fn)();
                                self.current_time = 0.0;
                            }
                        }
                    });

                // Color Palettes Dropdown
                egui::ComboBox::from_label("Palette")
                    .selected_text("Apply Palette...")
                    .show_ui(ui, |ui| {
                        for pal in get_curated_palettes() {
                            if ui.button(pal.name).clicked() {
                                self.project.background_color = pal.background;
                                if let Some(l1) = self.project.layers.get_mut(0) {
                                    l1.primary_color = pal.primary;
                                    l1.secondary_color = pal.secondary;
                                }
                                if let Some(l2) = self.project.layers.get_mut(1) {
                                    l2.primary_color = pal.accent;
                                    l2.secondary_color = pal.primary;
                                }
                            }
                        }
                    });

                ui.separator();

                // Background Color
                ui.label("Background:");
                color_picker_button(ui, &mut self.project.background_color);

                ui.separator();

                // Tile Dimensions
                ui.label("Tile Size:");
                ui.add(DragValue::new(&mut self.project.tile_width).range(32.0..=1024.0).suffix("px"));
                ui.label("×");
                ui.add(DragValue::new(&mut self.project.tile_height).range(32.0..=1024.0).suffix("px"));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("📋 Copy SVG").clicked() {
                        let svg = export_project_to_svg(&self.project);
                        ui.ctx().copy_text(svg);
                        self.anim_panel_state.status_message =
                            Some(("SVG copied to clipboard!".to_string(), 3.0));
                    }
                });
            });
        });

        // Bottom Transport & Viewport Controls Bar
        egui::TopBottomPanel::bottom("bottom_transport").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Play / Pause
                let play_btn_label = if self.playing { "⏸ Pause" } else { "▶ Play" };
                if ui.button(play_btn_label).clicked() {
                    self.playing = !self.playing;
                }

                // Time Scrubber
                ui.label(format!("{:.2}s", self.current_time));
                let mut display_time = self.current_time % 30.0;
                if ui.add(Slider::new(&mut display_time, 0.0..=30.0).text("Scrub")).changed() {
                    self.current_time = display_time;
                }

                // Playback Speed
                ui.label("Speed:");
                ui.add(DragValue::new(&mut self.playback_speed).range(0.1..=4.0).speed(0.1));

                ui.separator();

                // Repeat Grid
                ui.label("Repeat:");
                ui.add(DragValue::new(&mut self.canvas_state.repeat_x).range(1..=6));
                ui.label("×");
                ui.add(DragValue::new(&mut self.canvas_state.repeat_y).range(1..=6));

                ui.checkbox(&mut self.canvas_state.show_tile_grid, "Tile Grid Overlay");

                if ui.button("Reset View").clicked() {
                    self.canvas_state.pan = egui::Vec2::ZERO;
                    self.canvas_state.zoom = 1.0;
                }
            });
        });

        // Left Panel: Layer Stack & Generator Parameters
        egui::SidePanel::left("left_layers_panel")
            .default_width(320.0)
            .width_range(260.0..=480.0)
            .show(ctx, |ui| {
                render_layer_panel(ui, &mut self.project);
            });

        // Right Panel: CSS @keyframes Animation Inspector & SVG Exporter
        egui::SidePanel::right("right_animation_panel")
            .default_width(360.0)
            .width_range(300.0..=520.0)
            .show(ctx, |ui| {
                render_animation_panel(ui, &mut self.project, &mut self.anim_panel_state);
            });

        // Central Panel: Interactive Tiled Canvas Preview
        egui::CentralPanel::default().show(ctx, |ui| {
            render_canvas(ui, &self.project, self.current_time, &mut self.canvas_state);
        });
    }
}
