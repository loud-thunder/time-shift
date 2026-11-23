use crate::engine::animation_eval::compute_spatial_phase;
use crate::engine::generator_eval::evaluate_generator;
use crate::engine::primitives::{commands_to_svg_d, GeneratedPrimitive, PathCommand};
use crate::model::animation::{
    ColorOpacityTrack, DriftTrack, PathMorphTrack, TransformTrack,
};
use crate::model::project::Project;
use std::collections::BTreeSet;

pub fn export_project_to_svg(project: &Project) -> String {
    let w = project.tile_width;
    let h = project.tile_height;

    let mut css_rules = Vec::new();
    let mut defs_elements = Vec::new();
    let mut body_elements = Vec::new();

    // Background color
    let bg = project.background_color;
    let bg_hex = format!("#{:02x}{:02x}{:02x}", bg[0], bg[1], bg[2]);
    let bg_opacity = bg[3] as f32 / 255.0;

    body_elements.push(format!(
        r#"  <rect width="{:.2}" height="{:.2}" fill="{}" fill-opacity="{:.2}"/>"#,
        w, h, bg_hex, bg_opacity
    ));

    for (layer_idx, layer) in project.layers.iter().enumerate() {
        if !layer.visible {
            continue;
        }

        let layer_css_id = format!("l{}", layer.id);
        let primitives = evaluate_generator(
            &layer.generator,
            w,
            h,
            layer.primary_color,
            layer.secondary_color,
        );

        // 1. Generate Keyframes CSS for Drift Track
        let drift = &layer.animations.drift;
        let has_drift = drift.enabled && (drift.tiles_dx != 0 || drift.tiles_dy != 0);

        if has_drift {
            let drift_kf_name = format!("{}-drift", layer_css_id);
            css_rules.push(generate_drift_keyframes(&drift_kf_name, drift, w, h));
            css_rules.push(format!(
                r#"  .{} {{
    animation: {} {:.2}s {} infinite {};
  }}"#,
                drift_kf_name,
                drift_kf_name,
                drift.duration_sec,
                drift.easing.css_value(),
                drift.direction.name(),
            ));
        }

        // 2. Generate Keyframes CSS for Transform Track
        let tf = &layer.animations.transform;
        let has_tf = tf.enabled;
        if has_tf {
            let tf_kf_name = format!("{}-tf", layer_css_id);
            css_rules.push(generate_transform_keyframes(&tf_kf_name, tf));
            css_rules.push(format!(
                r#"  .{} {{
    transform-box: fill-box;
    transform-origin: center;
    animation: {} {:.2}s {} infinite {};
  }}"#,
                tf_kf_name,
                tf_kf_name,
                tf.duration_sec,
                tf.easing.css_value(),
                tf.direction.name(),
            ));
        }

        // 3. Generate Keyframes CSS for Color/Opacity Track
        let co = &layer.animations.color_opacity;
        let has_co = co.enabled;
        if has_co {
            let co_kf_name = format!("{}-co", layer_css_id);
            css_rules.push(generate_color_opacity_keyframes(&co_kf_name, co));
            css_rules.push(format!(
                r#"  .{} {{
    animation: {} {:.2}s {} infinite {};
  }}"#,
                co_kf_name,
                co_kf_name,
                co.duration_sec,
                co.easing.css_value(),
                co.direction.name(),
            ));
        }

        // 4. Generate Keyframes CSS for Path Morph Track
        let pm = &layer.animations.path_morph;
        let has_pm = pm.enabled;

        // Collect spatial delay classes
        let mut delays: BTreeSet<i32> = BTreeSet::new(); // store delay * 100 for quantization

        // Build SVG elements for primitives inside this layer
        let mut layer_primitives_svg = Vec::new();

        for (prim_idx, prim) in primitives.iter().enumerate() {
            let elem_id = format!("{}-p{}", layer_css_id, prim_idx);

            // Compute spatial delays
            let tf_delay = if has_tf {
                compute_spatial_phase(prim.grid_uv, tf.phase_freq_x, tf.phase_freq_y)
                    * tf.duration_sec
            } else {
                0.0
            };

            let co_delay = if has_co {
                compute_spatial_phase(prim.grid_uv, co.phase_freq_x, co.phase_freq_y)
                    * co.duration_sec
            } else {
                0.0
            };

            let pm_delay = if has_pm {
                compute_spatial_phase(prim.grid_uv, pm.phase_freq_x, pm.phase_freq_y)
                    * pm.duration_sec
            } else {
                0.0
            };

            // Path morph keyframes per primitive if enabled
            let mut morph_class = String::new();
            if has_pm {
                let morph_kf_name = format!("{}-morph", elem_id);
                css_rules.push(generate_morph_keyframes(&morph_kf_name, prim, pm));
                css_rules.push(format!(
                    r#"  .{} {{
    animation: {} {:.2}s {} -{:.2}s infinite {};
  }}"#,
                    morph_kf_name,
                    morph_kf_name,
                    pm.duration_sec,
                    pm.easing.css_value(),
                    pm_delay,
                    pm.direction.name(),
                ));
                morph_class = format!(" {}", morph_kf_name);
            }

            // Combine animation classes
            let mut classes = Vec::new();
            if has_tf {
                classes.push(format!("{}-tf", layer_css_id));
            }
            if has_co {
                classes.push(format!("{}-co", layer_css_id));
            }

            let class_str = classes.join(" ");

            // Individual inline style or quantized delay
            let mut inline_styles = Vec::new();
            if has_tf && tf_delay > 0.001 {
                inline_styles.push(format!("animation-delay: -{:.2}s", tf_delay));
            }
            if has_co && co_delay > 0.001 {
                inline_styles.push(format!("animation-delay: -{:.2}s", co_delay));
            }

            let style_attr = if !inline_styles.is_empty() {
                format!(r#" style="{}""#, inline_styles.join("; "))
            } else {
                String::new()
            };

            let d_base = commands_to_svg_d(&prim.commands);

            // Styling colors
            let fill_str = if let Some(ov) = &prim.style_override {
                if let Some(fc) = ov.fill_color {
                    format!("#{:02x}{:02x}{:02x}", fc[0], fc[1], fc[2])
                } else if layer.filled {
                    format!(
                        "#{:02x}{:02x}{:02x}",
                        layer.primary_color[0], layer.primary_color[1], layer.primary_color[2]
                    )
                } else {
                    "none".to_string()
                }
            } else if layer.filled {
                format!(
                    "#{:02x}{:02x}{:02x}",
                    layer.primary_color[0], layer.primary_color[1], layer.primary_color[2]
                )
            } else {
                "none".to_string()
            };

            let stroke_str = if layer.stroked {
                format!(
                    "#{:02x}{:02x}{:02x}",
                    layer.secondary_color[0], layer.secondary_color[1], layer.secondary_color[2]
                )
            } else {
                "none".to_string()
            };

            let stroke_w = if layer.stroked {
                format!(r#" stroke-width="{:.2}""#, layer.stroke_width)
            } else {
                String::new()
            };

            // Main primitive path
            let full_class = format!("{}{}", class_str, morph_class).trim().to_string();
            let class_attr = if !full_class.is_empty() {
                format!(r#" class="{}""#, full_class)
            } else {
                String::new()
            };

            layer_primitives_svg.push(format!(
                r#"    <path d="{}" fill="{}" stroke="{}"{}{}{}/>"#,
                d_base, fill_str, stroke_str, stroke_w, class_attr, style_attr
            ));

            // Toroidal ghost copies for shapes straddling edges
            for offset in &prim.wrap_offsets {
                let ghost_cmds: Vec<PathCommand> = prim
                    .commands
                    .iter()
                    .map(|c| c.translate(offset[0], offset[1]))
                    .collect();
                let ghost_d = commands_to_svg_d(&ghost_cmds);
                layer_primitives_svg.push(format!(
                    r#"    <path d="{}" fill="{}" stroke="{}"{}{}{}/>"#,
                    ghost_d, fill_str, stroke_str, stroke_w, class_attr, style_attr
                ));
            }
        }

        // Layer grouping & Drift assembly
        let layer_content = layer_primitives_svg.join("\n");

        if has_drift {
            // Put the base layer in <defs> and tile a 3x3 grid inside the drifting group
            let def_id = format!("{}-tile", layer_css_id);
            defs_elements.push(format!(
                r#"    <g id="{}">
{}
    </g>"#,
                def_id, layer_content
            ));

            let mut use_grid = Vec::new();
            for oy in -1..=1 {
                for ox in -1..=1 {
                    use_grid.push(format!(
                        r##"      <use href="#{}" x="{:.2}" y="{:.2}"/>"##,
                        def_id,
                        ox as f32 * w,
                        oy as f32 * h
                    ));
                }
            }

            body_elements.push(format!(
                r#"  <g class="{}-drift" opacity="{:.2}">
{}
  </g>"#,
                layer_css_id,
                layer.opacity,
                use_grid.join("\n")
            ));
        } else {
            body_elements.push(format!(
                r#"  <g id="{}" opacity="{:.2}">
{}
  </g>"#,
                layer_css_id, layer.opacity, layer_content
            ));
        }
    }

    // Assemble final SVG
    let style_block = if !css_rules.is_empty() {
        format!(
            r#"  <style>
{}
  </style>"#,
            css_rules.join("\n\n")
        )
    } else {
        String::new()
    };

    let defs_block = if !defs_elements.is_empty() {
        format!(
            r#"  <defs>
{}
  </defs>"#,
            defs_elements.join("\n")
        )
    } else {
        String::new()
    };

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {:.2} {:.2}" width="{:.2}" height="{:.2}">
{}{}
{}
</svg>"#,
        w,
        h,
        w,
        h,
        style_block,
        if defs_block.is_empty() {
            String::new()
        } else {
            format!("\n{}", defs_block)
        },
        body_elements.join("\n")
    )
}

fn generate_drift_keyframes(
    name: &str,
    drift: &DriftTrack,
    tile_w: f32,
    tile_h: f32,
) -> String {
    let mut stops_css = Vec::new();
    for stop in &drift.stops {
        let x = stop.value.0 * drift.tiles_dx as f32 * tile_w;
        let y = stop.value.1 * drift.tiles_dy as f32 * tile_h;
        stops_css.push(format!(
            "    {:.1}% {{ transform: translate({:.2}px, {:.2}px); }}",
            stop.offset_pct, x, y
        ));
    }

    format!(
        r#"  @keyframes {} {{
{}
  }}"#,
        name,
        stops_css.join("\n")
    )
}

fn generate_transform_keyframes(name: &str, tf: &TransformTrack) -> String {
    let mut stops_css = Vec::new();
    for stop in &tf.stops {
        let s = &stop.value;
        stops_css.push(format!(
            "    {:.1}% {{ transform: scale({:.2}, {:.2}) rotate({:.1}deg) skewX({:.1}deg); }}",
            stop.offset_pct, s.scale_x, s.scale_y, s.rotate_deg, s.skew_x_deg
        ));
    }

    format!(
        r#"  @keyframes {} {{
{}
  }}"#,
        name,
        stops_css.join("\n")
    )
}

fn generate_color_opacity_keyframes(name: &str, co: &ColorOpacityTrack) -> String {
    let mut stops_css = Vec::new();
    for stop in &co.stops {
        let c = stop.value.color;
        let hex = format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2]);
        stops_css.push(format!(
            "    {:.1}% {{ fill: {}; stroke: {}; opacity: {:.2}; }}",
            stop.offset_pct, hex, hex, stop.value.opacity
        ));
    }

    format!(
        r#"  @keyframes {} {{
{}
  }}"#,
        name,
        stops_css.join("\n")
    )
}

fn generate_morph_keyframes(
    name: &str,
    prim: &GeneratedPrimitive,
    pm: &PathMorphTrack,
) -> String {
    let mut stops_css = Vec::new();
    for stop in &pm.stops {
        let cmds = prim.get_commands_at_morph(stop.value.morph_factor);
        let d = commands_to_svg_d(&cmds);
        stops_css.push(format!(
            r#"    {:.1}% {{ d: path("{}"); }}"#,
            stop.offset_pct, d
        ));
    }

    format!(
        r#"  @keyframes {} {{
{}
  }}"#,
        name,
        stops_css.join("\n")
    )
}
