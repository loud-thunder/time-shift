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

    #[test]
    fn test_element_state_opacity_default_is_one() {
        use crate::engine::animation_eval::evaluate_element_state;
        let mut layer = crate::model::project::PatternLayer::new_polka(1, "Test");
        layer.opacity = 0.55;
        // color_opacity is disabled by default
        assert!(!layer.animations.color_opacity.enabled);

        let elem_state = evaluate_element_state(
            &layer.animations,
            0.0,
            (0.5, 0.5),
            layer.primary_color,
        );

        // Element-level opacity multiplier must be 1.0, NOT layer.opacity (0.55)
        assert_eq!(elem_state.color_opacity.opacity, 1.0);

        // Canvas final alpha must be (color.a * elem_state.opacity * layer.opacity)
        let layer_alpha = layer.opacity.clamp(0.0, 1.0);
        let canvas_alpha = (elem_state.color_opacity.color[3] as f32
            * elem_state.color_opacity.opacity
            * layer_alpha) as u8;

        // With color[3] = 255 and layer.opacity = 0.55:
        // Expected alpha is 140 (approx 55%), NOT 77 (which was 0.55 * 0.55 = 30.25%)
        assert_eq!(canvas_alpha, 140);
    }

    #[test]
    fn test_srgb_color32_premultiplication_parity() {
        use crate::ui::canvas::srgb_color32;

        // Primary color `#0e52c4` (r=14, g=82, b=196) at opacity 0.10 (a=25)
        let c_25 = srgb_color32(14, 82, 196, 25);
        // Correct sRGB premultiplied values:
        // pr = (14 * 25 + 127) / 255 = 1
        // pg = (82 * 25 + 127) / 255 = 8
        // pb = (196 * 25 + 127) / 255 = 19
        assert_eq!(c_25.r(), 1);
        assert_eq!(c_25.g(), 8);
        assert_eq!(c_25.b(), 19);
        assert_eq!(c_25.a(), 25);

        // Verify theoretical composite over white (255) using GL_ONE, GL_ONE_MINUS_SRC_ALPHA:
        // dst = premul + 255 * (1 - 25/255) = premul + 230
        let r_comp = c_25.r() as u16 + 230;
        let g_comp = c_25.g() as u16 + 230;
        let b_comp = c_25.b() as u16 + 230;
        assert_eq!(r_comp, 231);
        assert_eq!(g_comp, 238);
        assert_eq!(b_comp, 249);
        // Blue channel is strictly greater than Green channel (preserves blue hue, no cyan distortion)
        assert!(b_comp > g_comp);
        assert_eq!(b_comp - g_comp, 11);

        // Primary color `#0e52c4` at opacity 0.55 (a=140)
        let c_140 = srgb_color32(14, 82, 196, 140);
        // pr = (14 * 140 + 127) / 255 = 8
        // pg = (82 * 140 + 127) / 255 = 45
        // pb = (196 * 140 + 127) / 255 = 108
        assert_eq!(c_140.r(), 8);
        assert_eq!(c_140.g(), 45);
        assert_eq!(c_140.b(), 108);
        assert_eq!(c_140.a(), 140);

        // Over white (dst = 255, 1 - 140/255 = 115):
        let r_comp55 = c_140.r() as u16 + 115;
        let g_comp55 = c_140.g() as u16 + 115;
        let b_comp55 = c_140.b() as u16 + 115;
        assert_eq!(r_comp55, 123);
        assert_eq!(g_comp55, 160);
        assert_eq!(b_comp55, 223);
        assert!(b_comp55 > g_comp55);
    }

    #[test]
    fn test_svg_export_opacity_and_alpha_parity() {
        let mut p = Project::default();
        let l1 = p.layers.get_mut(0).unwrap();
        l1.opacity = 0.55;
        l1.primary_color = [14, 82, 196, 200]; // custom alpha < 255
        p.layers.truncate(1); // Keep only layer 1

        let svg = export_project_to_svg(&p);
        // Verify layer group has opacity 0.55
        assert!(svg.contains(r#"opacity="0.55""#));
        // Verify fill has hex color
        assert!(svg.contains(r##"fill="#0e52c4""##));
        // Verify fill-opacity attribute is present for alpha < 255
        assert!(svg.contains(r#"fill-opacity="0.78""#));
    }
}
