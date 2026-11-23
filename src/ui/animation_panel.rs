use super::layer_panel::color_picker_button;
use crate::export::svg_exporter::export_project_to_svg;
use crate::model::animation::{
    AnimationDirection, ColorOpacityState, EasingFunction, KeyframeStop,
    TransformState,
};
use crate::model::project::Project;
use egui::{DragValue, Slider, Ui};

pub struct AnimationPanelState {
    pub show_svg_preview: bool,
    pub status_message: Option<(String, f32)>, // message and timestamp
}

impl Default for AnimationPanelState {
    fn default() -> Self {
        Self {
            show_svg_preview: false,
            status_message: None,
        }
    }
}

pub fn render_animation_panel(
    ui: &mut Ui,
    project: &mut Project,
    panel_state: &mut AnimationPanelState,
) {
    ui.heading("CSS @keyframes & Animation Tracks");
    ui.add_space(4.0);

    let selected_layer_idx = project.selected_layer_idx;
    let selected_layer_id = project.layers.get(selected_layer_idx).map(|l| l.id);
    let Some(layer_id) = selected_layer_id else {
        ui.label("Select a layer to configure animation tracks.");
        return;
    };

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.push_id(("layer_anim", layer_id), |ui| {
            // Track 1: Element Transform Pulse / Spin
            if let Some(layer) = project.layers.get_mut(selected_layer_idx) {
                ui.push_id("transform_track", |ui| {
                    render_transform_track_ui(ui, &mut layer.animations.transform);
                });
            }
            ui.separator();

            // Track 2: Color & Opacity Cycling
            if let Some(layer) = project.layers.get_mut(selected_layer_idx) {
                ui.push_id("color_opacity_track", |ui| {
                    render_color_opacity_track_ui(ui, &mut layer.animations.color_opacity);
                });
            }
            ui.separator();

            // Track 3: Path Morphing (d attribute keyframes)
            if let Some(layer) = project.layers.get_mut(selected_layer_idx) {
                ui.push_id("path_morph_track", |ui| {
                    render_path_morph_track_ui(ui, &mut layer.animations.path_morph);
                });
            }
            ui.separator();
        });

        // Live SVG code preview & Export options
        ui.heading("Export & Code Preview");
        ui.horizontal(|ui| {
            if ui.button("📋 Copy SVG Markup").clicked() {
                let svg_code = export_project_to_svg(project);
                ui.ctx().copy_text(svg_code);
                panel_state.status_message = Some(("SVG copied to clipboard!".to_string(), 3.0));
            }

            if ui.button("💾 Export .svg File").clicked() {
                let svg_code = export_project_to_svg(project);
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("SVG Image", &["svg"])
                    .set_file_name("time-shift-pattern.svg")
                    .save_file()
                {
                    if let Err(e) = std::fs::write(&path, svg_code) {
                        panel_state.status_message =
                            Some((format!("Export failed: {}", e), 4.0));
                    } else {
                        panel_state.status_message =
                            Some((format!("Saved to {}", path.display()), 4.0));
                    }
                }
            }

            ui.checkbox(&mut panel_state.show_svg_preview, "Show SVG Code");
        });

        if let Some((msg, _)) = &panel_state.status_message {
            ui.colored_label(egui::Color32::from_rgb(0, 240, 255), msg);
        }

        if panel_state.show_svg_preview {
            let svg_code = export_project_to_svg(project);
            let mut text = svg_code.as_str();
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut text)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY),
                    );
                });
        }
    });
}

fn render_transform_track_ui(ui: &mut Ui, tf: &mut crate::model::animation::TransformTrack) {
    ui.horizontal(|ui| {
        ui.checkbox(&mut tf.enabled, "");
        ui.strong("🔄 Element Transform Pulse / Spin");
    });

    if !tf.enabled {
        return;
    }

    ui.horizontal(|ui| {
        ui.label("Duration (s):");
        ui.add(DragValue::new(&mut tf.duration_sec).range(0.2..=30.0).speed(0.1));

        easing_dropdown(ui, &mut tf.easing);
        direction_dropdown(ui, &mut tf.direction);
    });

    ui.horizontal(|ui| {
        ui.label("Spatial Phase Freq X/Y:");
        ui.add(DragValue::new(&mut tf.phase_freq_x).range(0..=8));
        ui.add(DragValue::new(&mut tf.phase_freq_y).range(0..=8));
    });

    egui::CollapsingHeader::new("Keyframe Stops")
        .id_salt("transform_stops")
        .show(ui, |ui| {
            let mut remove_idx = None;
            let stops_count = tf.stops.len();
            for (i, stop) in tf.stops.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{:.0}%", stop.offset_pct));
                    ui.label("Scale:");
                    ui.add(DragValue::new(&mut stop.value.scale_x).range(0.1..=4.0).speed(0.05));
                    ui.label("Rot:");
                    ui.add(DragValue::new(&mut stop.value.rotate_deg).range(-360.0..=360.0).speed(5.0));

                    if stops_count > 2 && ui.small_button("✖").clicked() {
                        remove_idx = Some(i);
                    }
                });
            }
            if let Some(idx) = remove_idx {
                tf.stops.remove(idx);
            }

            ui.horizontal(|ui| {
                if ui.button("➕ Add Stop").clicked() {
                    let last_pct = tf.stops.last().map(|s| s.offset_pct).unwrap_or(50.0);
                    tf.stops.push(KeyframeStop {
                        offset_pct: (last_pct * 0.5).min(90.0),
                        value: TransformState::default(),
                    });
                    tf.stops.sort_by(|a, b| a.offset_pct.partial_cmp(&b.offset_pct).unwrap());
                }

                if ui.button("🔁 Loop Match (0% = 100%)").clicked() {
                    if let Some(first) = tf.stops.first().cloned() {
                        if let Some(last) = tf.stops.last_mut() {
                            last.value = first.value;
                        }
                    }
                }
            });
        });
}

fn render_color_opacity_track_ui(
    ui: &mut Ui,
    co: &mut crate::model::animation::ColorOpacityTrack,
) {
    ui.horizontal(|ui| {
        ui.checkbox(&mut co.enabled, "");
        ui.strong("🎨 Color & Opacity Cycling");
    });

    if !co.enabled {
        return;
    }

    ui.horizontal(|ui| {
        ui.label("Duration (s):");
        ui.add(DragValue::new(&mut co.duration_sec).range(0.2..=30.0).speed(0.1));

        easing_dropdown(ui, &mut co.easing);
        direction_dropdown(ui, &mut co.direction);
    });

    ui.horizontal(|ui| {
        ui.label("Spatial Phase Freq X/Y:");
        ui.add(DragValue::new(&mut co.phase_freq_x).range(0..=8));
        ui.add(DragValue::new(&mut co.phase_freq_y).range(0..=8));
    });

    egui::CollapsingHeader::new("Keyframe Stops")
        .id_salt("color_opacity_stops")
        .show(ui, |ui| {
        let mut remove_idx = None;
        let stops_count = co.stops.len();
        for (i, stop) in co.stops.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("{:.0}%", stop.offset_pct));
                color_picker_button(ui, &mut stop.value.color);
                ui.label("Opacity:");
                ui.add(Slider::new(&mut stop.value.opacity, 0.0..=1.0).step_by(0.05));

                if stops_count > 2 && ui.small_button("✖").clicked() {
                    remove_idx = Some(i);
                }
            });
        }
        if let Some(idx) = remove_idx {
            co.stops.remove(idx);
        }

        ui.horizontal(|ui| {
            if ui.button("➕ Add Stop").clicked() {
                co.stops.push(KeyframeStop {
                    offset_pct: 50.0,
                    value: ColorOpacityState::default(),
                });
                co.stops.sort_by(|a, b| a.offset_pct.partial_cmp(&b.offset_pct).unwrap());
            }

            if ui.button("🔁 Loop Match (0% = 100%)").clicked() {
                if let Some(first) = co.stops.first().cloned() {
                    if let Some(last) = co.stops.last_mut() {
                        last.value = first.value;
                    }
                }
            }
        });
    });
}

fn render_path_morph_track_ui(ui: &mut Ui, pm: &mut crate::model::animation::PathMorphTrack) {
    ui.horizontal(|ui| {
        ui.checkbox(&mut pm.enabled, "");
        ui.strong("✨ Path Morphing (d attribute keyframes)");
    });

    if !pm.enabled {
        return;
    }

    ui.horizontal(|ui| {
        ui.label("Duration (s):");
        ui.add(DragValue::new(&mut pm.duration_sec).range(0.2..=30.0).speed(0.1));

        easing_dropdown(ui, &mut pm.easing);
        direction_dropdown(ui, &mut pm.direction);
    });

    ui.horizontal(|ui| {
        ui.label("Spatial Phase Freq X/Y:");
        ui.add(DragValue::new(&mut pm.phase_freq_x).range(0..=8));
        ui.add(DragValue::new(&mut pm.phase_freq_y).range(0..=8));
    });

    egui::CollapsingHeader::new("Keyframe Stops")
        .id_salt("path_morph_stops")
        .show(ui, |ui| {
            for stop in pm.stops.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(format!("{:.0}%", stop.offset_pct));
                    ui.add(Slider::new(&mut stop.value.morph_factor, -1.0..=1.0).text("Factor"));
                });
            }

            if ui.button("🔁 Loop Match (0% = 100%)").clicked() {
                if let Some(first) = pm.stops.first().cloned() {
                    if let Some(last) = pm.stops.last_mut() {
                        last.value = first.value;
                    }
                }
            }
        });
}

fn easing_dropdown(ui: &mut Ui, easing: &mut EasingFunction) {
    egui::ComboBox::from_id_salt(ui.id().with("easing"))
        .selected_text(easing.name())
        .show_ui(ui, |ui| {
            for e in EasingFunction::all() {
                if ui.selectable_label(*easing == *e, e.name()).clicked() {
                    *easing = *e;
                }
            }
        });
}

fn direction_dropdown(ui: &mut Ui, dir: &mut AnimationDirection) {
    egui::ComboBox::from_id_salt(ui.id().with("direction"))
        .selected_text(dir.name())
        .show_ui(ui, |ui| {
            for d in AnimationDirection::all() {
                if ui.selectable_label(*dir == *d, d.name()).clicked() {
                    *dir = *d;
                }
            }
        });
}
