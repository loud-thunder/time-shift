mod engine;
mod export;
mod model;
mod presets;
mod ui;

use ui::app::TimeShiftApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Time-Shift: Tileable Animated SVG Background Generator")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([960.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Time-Shift",
        native_options,
        Box::new(|_cc| Ok(Box::new(TimeShiftApp::default()))),
    )
}

#[cfg(test)]
mod tests {
    use crate::engine::animation_eval::compute_progress;
    use crate::engine::generator_eval::evaluate_generator;
    use crate::export::svg_exporter::export_project_to_svg;
    use crate::model::animation::{AnimationDirection, EasingFunction};
    use crate::model::project::Project;
    use crate::presets::{get_curated_palettes, get_curated_presets};

    #[test]
    fn test_procedural_generator_primitives() {
        let p = Project::default();
        for layer in &p.layers {
            let prims = evaluate_generator(
                &layer.generator,
                p.tile_width,
                p.tile_height,
                layer.primary_color,
                layer.secondary_color,
            );
            assert!(!prims.is_empty(), "Generator produced empty primitives");
        }
    }

    #[test]
    fn test_curated_presets_produce_valid_svg() {
        for preset in get_curated_presets() {
            let project = (preset.project_fn)();
            let svg = export_project_to_svg(&project);
            assert!(svg.starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\""));
            assert!(svg.ends_with("</svg>"));
            assert!(svg.contains("@keyframes"));
        }
    }

    #[test]
    fn test_animation_loop_continuity() {
        let dur = 4.0;
        let p_start = compute_progress(0.0, dur, AnimationDirection::Normal, EasingFunction::Linear);
        let p_end = compute_progress(dur, dur, AnimationDirection::Normal, EasingFunction::Linear);
        assert!((p_start - 0.0).abs() < 1e-4);
        assert!((p_end - 0.0).abs() < 1e-4); // Fract(1.0) == 0.0 for seamless looping
    }

    #[test]
    fn test_white_background_palettes_and_default() {
        let p = Project::default();
        assert_eq!(p.background_color, [255, 255, 255, 255]);

        let palettes = get_curated_palettes();
        assert_eq!(palettes.len(), 8);
        for pal in palettes {
            // Backgrounds must be white or light (R, G, B >= 240)
            assert!(pal.background[0] >= 240, "Palette {} has non-light background R: {}", pal.name, pal.background[0]);
            assert!(pal.background[1] >= 240, "Palette {} has non-light background G: {}", pal.name, pal.background[1]);
            assert!(pal.background[2] >= 220, "Palette {} has non-light background B: {}", pal.name, pal.background[2]);
        }
    }
}
