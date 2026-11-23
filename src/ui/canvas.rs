use crate::engine::animation_eval::{
    evaluate_element_state, transform_commands,
};
use crate::engine::generator_eval::evaluate_generator;
use crate::engine::primitives::PathCommand;
use crate::model::project::Project;
use egui::{
    pos2, vec2, Color32, Painter, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2,
};

pub struct CanvasState {
    pub repeat_x: u32,
    pub repeat_y: u32,
    pub show_tile_grid: bool,
    pub zoom: f32,
    pub pan: Vec2,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            repeat_x: 2,
            repeat_y: 2,
            show_tile_grid: true,
            zoom: 1.0,
            pan: Vec2::ZERO,
        }
    }
}

pub fn render_canvas(
    ui: &mut Ui,
    project: &Project,
    current_time: f32,
    state: &mut CanvasState,
) -> Response {
    let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::drag());

    // Pan with drag
    if response.dragged() {
        state.pan += response.drag_delta();
    }

    // Zoom with scroll wheel
    let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
    if scroll_delta != 0.0 && response.hovered() {
        let zoom_factor = (scroll_delta * 0.002).exp();
        state.zoom = (state.zoom * zoom_factor).clamp(0.2, 5.0);
    }

    let rect = response.rect;
    let tile_w = project.tile_width * state.zoom;
    let tile_h = project.tile_height * state.zoom;
    let total_w = tile_w * state.repeat_x as f32;
    let total_h = tile_h * state.repeat_y as f32;

    let center = rect.center() + state.pan;
    let top_left = center - vec2(total_w * 0.5, total_h * 0.5);
    let canvas_bounds = Rect::from_min_size(top_left, vec2(total_w, total_h));

    // Paint Canvas Background
    let bg = project.background_color;
    let bg_color = Color32::from_rgba_unmultiplied(bg[0], bg[1], bg[2], bg[3]);
    painter.rect_filled(canvas_bounds, 4.0, bg_color);

    // Clip rendering to canvas bounds
    let clipped_painter = painter.with_clip_rect(canvas_bounds.intersect(rect));

    // Render layers across repeated tiles
    for layer in &project.layers {
        if !layer.visible {
            continue;
        }

        let prims = evaluate_generator(
            &layer.generator,
            project.tile_width,
            project.tile_height,
            layer.primary_color,
            layer.secondary_color,
        );

        let layer_alpha = layer.opacity.clamp(0.0, 1.0);

        for ty in 0..state.repeat_y {
            for tx in 0..state.repeat_x {
                let tile_origin = top_left + vec2(tx as f32 * tile_w, ty as f32 * tile_h);

                for prim in &prims {
                    let elem_state = evaluate_element_state(
                        &layer.animations,
                        current_time,
                        prim.grid_uv,
                        layer.primary_color,
                    );

                    let morphed_cmds = prim.get_commands_at_morph(elem_state.morph_factor);
                    let final_cmds = transform_commands(
                        &morphed_cmds,
                        prim.pivot,
                        &elem_state.transform,
                    );

                    let draw_primitive = |cmds: &[PathCommand], painter: &Painter| {
                        let fill_c = if let Some(ov) = &prim.style_override {
                            if let Some(fc) = ov.fill_color {
                                Color32::from_rgba_unmultiplied(
                                    fc[0],
                                    fc[1],
                                    fc[2],
                                    (fc[3] as f32 * layer_alpha) as u8,
                                )
                            } else if layer.filled {
                                Color32::from_rgba_unmultiplied(
                                    elem_state.color_opacity.color[0],
                                    elem_state.color_opacity.color[1],
                                    elem_state.color_opacity.color[2],
                                    (elem_state.color_opacity.color[3] as f32
                                        * elem_state.color_opacity.opacity
                                        * layer_alpha) as u8,
                                )
                            } else {
                                Color32::TRANSPARENT
                            }
                        } else if layer.filled {
                            Color32::from_rgba_unmultiplied(
                                elem_state.color_opacity.color[0],
                                elem_state.color_opacity.color[1],
                                elem_state.color_opacity.color[2],
                                (elem_state.color_opacity.color[3] as f32
                                    * elem_state.color_opacity.opacity
                                    * layer_alpha) as u8,
                            )
                        } else {
                            Color32::TRANSPARENT
                        };

                        let stroke_c = if layer.stroked {
                            Color32::from_rgba_unmultiplied(
                                layer.secondary_color[0],
                                layer.secondary_color[1],
                                layer.secondary_color[2],
                                (layer.secondary_color[3] as f32
                                    * elem_state.color_opacity.opacity
                                    * layer_alpha) as u8,
                            )
                        } else {
                            Color32::TRANSPARENT
                        };

                        let stroke = Stroke::new(layer.stroke_width * state.zoom, stroke_c);

                        draw_path_commands(
                            cmds,
                            tile_origin,
                            state.zoom,
                            fill_c,
                            stroke,
                            prim.is_closed,
                            painter,
                        );
                    };

                    draw_primitive(&final_cmds, &clipped_painter);

                    // Also draw toroidal ghost copies if any
                    for offset in &prim.wrap_offsets {
                        let ghost_cmds: Vec<PathCommand> = final_cmds
                            .iter()
                            .map(|c| c.translate(offset[0], offset[1]))
                            .collect();
                        draw_primitive(&ghost_cmds, &clipped_painter);
                    }
                }
            }
        }
    }

    // Draw tile boundary grid overlay
    if state.show_tile_grid {
        let is_light_bg = (0.299 * bg[0] as f32 + 0.587 * bg[1] as f32 + 0.114 * bg[2] as f32) > 140.0;
        let (grid_color, internal_color) = if is_light_bg {
            (
                Color32::from_rgba_unmultiplied(30, 35, 45, 90),
                Color32::from_rgba_unmultiplied(14, 82, 196, 90),
            )
        } else {
            (
                Color32::from_rgba_unmultiplied(255, 255, 255, 70),
                Color32::from_rgba_unmultiplied(0, 240, 255, 120),
            )
        };
        let grid_stroke = Stroke::new(1.5_f32, grid_color);
        let dashed_stroke = Stroke::new(1.0_f32, internal_color);

        // Outer border
        clipped_painter.rect_stroke(canvas_bounds, 0.0, grid_stroke, egui::StrokeKind::Outside);

        // Internal tile grid lines
        for tx in 1..state.repeat_x {
            let x = top_left.x + tx as f32 * tile_w;
            clipped_painter.line_segment([pos2(x, top_left.y), pos2(x, top_left.y + total_h)], dashed_stroke);
        }
        for ty in 1..state.repeat_y {
            let y = top_left.y + ty as f32 * tile_h;
            clipped_painter.line_segment([pos2(top_left.x, y), pos2(top_left.x + total_w, y)], dashed_stroke);
        }
    }

    response
}

fn draw_path_commands(
    cmds: &[PathCommand],
    origin: Pos2,
    zoom: f32,
    fill: Color32,
    stroke: Stroke,
    is_closed: bool,
    painter: &Painter,
) {
    let mut current_pt = Pos2::ZERO;
    let mut poly_pts: Vec<Pos2> = Vec::new();

    let map_pt = |p: [f32; 2]| -> Pos2 {
        origin + vec2(p[0] * zoom, p[1] * zoom)
    };

    for cmd in cmds {
        match cmd {
            PathCommand::MoveTo(p) => {
                if poly_pts.len() >= 2 {
                    if is_closed && fill != Color32::TRANSPARENT {
                        painter.add(egui::Shape::convex_polygon(poly_pts.clone(), fill, stroke));
                    } else if stroke.color != Color32::TRANSPARENT {
                        painter.add(egui::Shape::Path(egui::epaint::PathShape::line(
                            poly_pts.clone(),
                            stroke,
                        )));
                    }
                    poly_pts.clear();
                }
                current_pt = map_pt(*p);
                poly_pts.push(current_pt);
            }
            PathCommand::LineTo(p) => {
                current_pt = map_pt(*p);
                poly_pts.push(current_pt);
            }
            PathCommand::CubicTo(p1, p2, p3) => {
                let cp1 = map_pt(*p1);
                let cp2 = map_pt(*p2);
                let cp3 = map_pt(*p3);

                // Flatten cubic to 8 line segments for rendering
                for i in 1..=8 {
                    let t = i as f32 / 8.0;
                    let it = 1.0 - t;
                    let x = it * it * it * current_pt.x
                        + 3.0 * it * it * t * cp1.x
                        + 3.0 * it * t * t * cp2.x
                        + t * t * t * cp3.x;
                    let y = it * it * it * current_pt.y
                        + 3.0 * it * it * t * cp1.y
                        + 3.0 * it * t * t * cp2.y
                        + t * t * t * cp3.y;
                    poly_pts.push(pos2(x, y));
                }
                current_pt = cp3;
            }
            PathCommand::Close => {
                if poly_pts.len() >= 3 {
                    if fill != Color32::TRANSPARENT {
                        painter.add(egui::Shape::convex_polygon(poly_pts.clone(), fill, stroke));
                    } else if stroke.color != Color32::TRANSPARENT {
                        painter.add(egui::Shape::Path(egui::epaint::PathShape::closed_line(
                            poly_pts.clone(),
                            stroke,
                        )));
                    }
                }
                poly_pts.clear();
            }
        }
    }

    if poly_pts.len() >= 2 {
        if is_closed && fill != Color32::TRANSPARENT {
            painter.add(egui::Shape::convex_polygon(poly_pts, fill, stroke));
        } else if stroke.color != Color32::TRANSPARENT {
            painter.add(egui::Shape::Path(egui::epaint::PathShape::line(
                poly_pts, stroke,
            )));
        }
    }
}
