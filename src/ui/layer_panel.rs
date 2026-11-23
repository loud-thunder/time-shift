use crate::model::generators::{
    ConcentricRipplesParams, FlowFieldDashesParams, GeneratorKind, IsometricHexGridParams,
    MotifShape, PolkaHalftoneParams, TessellationMode, TruchetLayout, TruchetTilesParams,
    TruchetVariant, WaveOrientation, WavesSineStripesParams,
};
use crate::model::project::{PatternLayer, Project};
use egui::{Color32, DragValue, Slider, Ui};

pub fn render_layer_panel(ui: &mut Ui, project: &mut Project) {
    ui.heading("Layers");
    ui.add_space(6.0);

    // Layer list header actions
    ui.horizontal(|ui| {
        if ui.button("➕ Add Layer").clicked() {
            let id = project.next_layer_id;
            project.next_layer_id += 1;
            let new_layer = PatternLayer::new_polka(id, &format!("Layer {}", id));
            project.layers.push(new_layer);
            project.selected_layer_idx = project.layers.len() - 1;
        }

        if !project.layers.is_empty() {
            if ui.button("🗑 Delete").clicked() {
                if project.layers.len() > 1 {
                    project.layers.remove(project.selected_layer_idx);
                    if project.selected_layer_idx >= project.layers.len() {
                        project.selected_layer_idx = project.layers.len() - 1;
                    }
                }
            }
        }
    });

    ui.separator();

    // Layer stack list
    let mut selected_idx = project.selected_layer_idx;
    egui::ScrollArea::vertical()
        .max_height(160.0)
        .show(ui, |ui| {
            for (idx, layer) in project.layers.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let mut vis = layer.visible;
                    if ui.checkbox(&mut vis, "").changed() {
                        layer.visible = vis;
                    }

                    let label = format!("{}. {}", idx + 1, layer.name);
                    if ui.selectable_label(selected_idx == idx, label).clicked() {
                        selected_idx = idx;
                    }
                });
            }
        });
    project.selected_layer_idx = selected_idx;

    ui.separator();

    // Selected layer inspector
    if let Some(layer) = project.selected_layer_mut() {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Layer Properties");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut layer.name);
            });

            ui.horizontal(|ui| {
                ui.label("Opacity:");
                ui.add(Slider::new(&mut layer.opacity, 0.0..=1.0).step_by(0.05));
            });

            // Styling: Fill / Stroke
            ui.collapsing("Fill & Stroke Styling", |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut layer.filled, "Filled");
                    if layer.filled {
                        color_picker_button(ui, &mut layer.primary_color);
                    }
                });

                ui.horizontal(|ui| {
                    ui.checkbox(&mut layer.stroked, "Stroked");
                    if layer.stroked {
                        color_picker_button(ui, &mut layer.secondary_color);
                    }
                });

                if layer.stroked {
                    ui.horizontal(|ui| {
                        ui.label("Stroke Width:");
                        ui.add(Slider::new(&mut layer.stroke_width, 0.5..=20.0));
                    });
                }
            });

            ui.add_space(6.0);
            ui.heading("Generator Recipe");

            // Generator Kind Switcher
            let current_kind_name = layer.generator.name();
            egui::ComboBox::from_label("Recipe")
                .selected_text(current_kind_name)
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(
                            matches!(layer.generator, GeneratorKind::PolkaHalftone(_)),
                            "Polka Dots / Halftone Grid",
                        )
                        .clicked()
                    {
                        layer.generator =
                            GeneratorKind::PolkaHalftone(PolkaHalftoneParams::default());
                    }
                    if ui
                        .selectable_label(
                            matches!(layer.generator, GeneratorKind::WavesSineStripes(_)),
                            "Waves / Sine Stripes",
                        )
                        .clicked()
                    {
                        layer.generator =
                            GeneratorKind::WavesSineStripes(WavesSineStripesParams::default());
                    }
                    if ui
                        .selectable_label(
                            matches!(layer.generator, GeneratorKind::TruchetTiles(_)),
                            "Truchet Tiles",
                        )
                        .clicked()
                    {
                        layer.generator =
                            GeneratorKind::TruchetTiles(TruchetTilesParams::default());
                    }
                    if ui
                        .selectable_label(
                            matches!(layer.generator, GeneratorKind::IsometricHexGrid(_)),
                            "Isometric / Hex Grid",
                        )
                        .clicked()
                    {
                        layer.generator =
                            GeneratorKind::IsometricHexGrid(IsometricHexGridParams::default());
                    }
                    if ui
                        .selectable_label(
                            matches!(layer.generator, GeneratorKind::ConcentricRipples(_)),
                            "Concentric Ripples",
                        )
                        .clicked()
                    {
                        layer.generator =
                            GeneratorKind::ConcentricRipples(ConcentricRipplesParams::default());
                    }
                    if ui
                        .selectable_label(
                            matches!(layer.generator, GeneratorKind::FlowFieldDashes(_)),
                            "Flow Field Dashes",
                        )
                        .clicked()
                    {
                        layer.generator =
                            GeneratorKind::FlowFieldDashes(FlowFieldDashesParams::default());
                    }
                });

            ui.add_space(6.0);

            // Specific generator parameters
            match layer.generator {
                GeneratorKind::PolkaHalftone(ref mut params) => {
                    ui.horizontal(|ui| {
                        ui.label("Cols / Rows:");
                        ui.add(DragValue::new(&mut params.cols).range(1..=32));
                        ui.add(DragValue::new(&mut params.rows).range(1..=32));
                    });

                    egui::ComboBox::from_label("Shape")
                        .selected_text(params.shape.name())
                        .show_ui(ui, |ui| {
                            for s in MotifShape::all() {
                                if ui
                                    .selectable_label(params.shape == *s, s.name())
                                    .clicked()
                                {
                                    params.shape = *s;
                                }
                            }
                        });

                    ui.add(
                        Slider::new(&mut params.base_radius_ratio, 0.05..=0.5)
                            .text("Radius Ratio"),
                    );
                    ui.checkbox(&mut params.stagger_odd_rows, "Stagger Odd Rows");
                    ui.add(
                        Slider::new(&mut params.size_modulation_amp, 0.0..=0.8)
                            .text("Modulation Amp"),
                    );
                    if params.size_modulation_amp > 0.0 {
                        ui.horizontal(|ui| {
                            ui.label("Modulation Freq X/Y:");
                            ui.add(DragValue::new(&mut params.size_freq_x).range(1..=8));
                            ui.add(DragValue::new(&mut params.size_freq_y).range(1..=8));
                        });
                    }
                    ui.add(
                        Slider::new(&mut params.base_rotation_deg, 0.0..=360.0)
                            .text("Rotation Deg"),
                    );
                }

                GeneratorKind::WavesSineStripes(ref mut params) => {
                    ui.horizontal(|ui| {
                        ui.label("Band Count:");
                        ui.add(DragValue::new(&mut params.band_count).range(1..=32));
                    });

                    egui::ComboBox::from_label("Orientation")
                        .selected_text(params.orientation.name())
                        .show_ui(ui, |ui| {
                            for o in WaveOrientation::all() {
                                if ui
                                    .selectable_label(params.orientation == *o, o.name())
                                    .clicked()
                                {
                                    params.orientation = *o;
                                }
                            }
                        });

                    ui.add(Slider::new(&mut params.amplitude, 0.0..=60.0).text("Amplitude"));
                    ui.horizontal(|ui| {
                        ui.label("Periods per Tile:");
                        ui.add(DragValue::new(&mut params.frequency).range(1..=12));
                    });
                    ui.add(
                        Slider::new(&mut params.harmonic_ratio, 0.0..=1.0)
                            .text("Harmonic Ratio"),
                    );
                }

                GeneratorKind::TruchetTiles(ref mut params) => {
                    ui.horizontal(|ui| {
                        ui.label("Grid Size:");
                        ui.add(DragValue::new(&mut params.cols).range(1..=24));
                        ui.add(DragValue::new(&mut params.rows).range(1..=24));
                    });

                    egui::ComboBox::from_label("Variant")
                        .selected_text(params.variant.name())
                        .show_ui(ui, |ui| {
                            for v in TruchetVariant::all() {
                                if ui
                                    .selectable_label(params.variant == *v, v.name())
                                    .clicked()
                                {
                                    params.variant = *v;
                                }
                            }
                        });

                    egui::ComboBox::from_label("Layout")
                        .selected_text(params.layout.name())
                        .show_ui(ui, |ui| {
                            for l in TruchetLayout::all() {
                                if ui
                                    .selectable_label(params.layout == *l, l.name())
                                    .clicked()
                                {
                                    params.layout = *l;
                                }
                            }
                        });

                    if matches!(params.variant, TruchetVariant::QuarterCircles) {
                        ui.checkbox(&mut params.double_arc, "Double Arc");
                        ui.add(
                            Slider::new(&mut params.corner_radius_ratio, 0.2..=0.8)
                                .text("Corner Radius"),
                        );
                    }

                    if matches!(params.layout, TruchetLayout::HashSeed) {
                        ui.horizontal(|ui| {
                            ui.label("Seed:");
                            ui.add(DragValue::new(&mut params.seed));
                        });
                    }
                }

                GeneratorKind::IsometricHexGrid(ref mut params) => {
                    ui.horizontal(|ui| {
                        ui.label("Tessellation Size:");
                        ui.add(DragValue::new(&mut params.cols).range(1..=24));
                        ui.add(DragValue::new(&mut params.rows).range(1..=24));
                    });

                    egui::ComboBox::from_label("Mode")
                        .selected_text(params.mode.name())
                        .show_ui(ui, |ui| {
                            for m in TessellationMode::all() {
                                if ui
                                    .selectable_label(params.mode == *m, m.name())
                                    .clicked()
                                {
                                    params.mode = *m;
                                }
                            }
                        });

                    ui.add(
                        Slider::new(&mut params.inset_ratio, 0.0..=0.3).text("Inset Ratio"),
                    );
                    if matches!(params.mode, TessellationMode::IsometricCubes) {
                        ui.checkbox(&mut params.face_shading, "3D Face Shading");
                    }
                }

                GeneratorKind::ConcentricRipples(ref mut params) => {
                    ui.horizontal(|ui| {
                        ui.label("Centers X/Y:");
                        ui.add(DragValue::new(&mut params.center_count_x).range(1..=8));
                        ui.add(DragValue::new(&mut params.center_count_y).range(1..=8));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Rings per Center:");
                        ui.add(DragValue::new(&mut params.rings_per_center).range(1..=16));
                    });

                    ui.add(
                        Slider::new(&mut params.max_radius_ratio, 0.2..=1.5)
                            .text("Max Radius Ratio"),
                    );
                    ui.add(
                        Slider::new(&mut params.ring_spacing_exp, 0.5..=2.5)
                            .text("Spacing Exponent"),
                    );
                    ui.add(
                        Slider::new(&mut params.jitter_amount, 0.0..=0.5).text("Center Jitter"),
                    );
                }

                GeneratorKind::FlowFieldDashes(ref mut params) => {
                    ui.horizontal(|ui| {
                        ui.label("Grid Cols/Rows:");
                        ui.add(DragValue::new(&mut params.cols).range(2..=24));
                        ui.add(DragValue::new(&mut params.rows).range(2..=24));
                    });

                    ui.add(
                        Slider::new(&mut params.dash_length_ratio, 0.2..=1.5)
                            .text("Dash Length"),
                    );
                    ui.add(
                        Slider::new(&mut params.angle_amplitude_deg, 0.0..=180.0)
                            .text("Angle Amp (deg)"),
                    );
                    ui.horizontal(|ui| {
                        ui.label("Field Freq X/Y:");
                        ui.add(DragValue::new(&mut params.freq_x).range(1..=8));
                        ui.add(DragValue::new(&mut params.freq_y).range(1..=8));
                    });
                    ui.checkbox(&mut params.arrow_heads, "Arrow Heads");
                }
            }
        });
    }
}

pub fn color_picker_button(ui: &mut Ui, color: &mut [u8; 4]) {
    let mut egui_c = Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
    if egui::color_picker::color_edit_button_srgba(
        ui,
        &mut egui_c,
        egui::color_picker::Alpha::OnlyBlend,
    )
    .changed()
    {
        *color = [egui_c.r(), egui_c.g(), egui_c.b(), egui_c.a()];
    }
}
