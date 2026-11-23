use super::primitives::{GeneratedPrimitive, PathCommand, PrimitiveStyleOverride};
use crate::model::generators::{
    ConcentricRipplesParams, FlowFieldDashesParams, GeneratorKind, IsometricHexGridParams,
    MotifShape, PolkaHalftoneParams, TessellationMode, TruchetLayout, TruchetTilesParams,
    TruchetVariant, WaveOrientation, WavesSineStripesParams,
};
use std::f32::consts::PI;

pub fn evaluate_generator(
    kind: &GeneratorKind,
    tile_w: f32,
    tile_h: f32,
    primary_color: [u8; 4],
    secondary_color: [u8; 4],
) -> Vec<GeneratedPrimitive> {
    match kind {
        GeneratorKind::PolkaHalftone(params) => eval_polka(params, tile_w, tile_h),
        GeneratorKind::WavesSineStripes(params) => eval_waves(params, tile_w, tile_h),
        GeneratorKind::TruchetTiles(params) => eval_truchet(params, tile_w, tile_h),
        GeneratorKind::IsometricHexGrid(params) => {
            eval_isometric(params, tile_w, tile_h, primary_color, secondary_color)
        }
        GeneratorKind::ConcentricRipples(params) => eval_ripples(params, tile_w, tile_h),
        GeneratorKind::FlowFieldDashes(params) => eval_flow_field(params, tile_w, tile_h),
    }
}

// ----------------------------------------------------------------------------
// Polka Dots / Halftone Grid
// ----------------------------------------------------------------------------
fn eval_polka(params: &PolkaHalftoneParams, w: f32, h: f32) -> Vec<GeneratedPrimitive> {
    let mut primitives = Vec::new();
    let cols = params.cols.max(1);
    let rows = params.rows.max(1);
    let cell_w = w / cols as f32;
    let cell_h = h / rows as f32;
    let base_radius = cell_w.min(cell_h) * params.base_radius_ratio.clamp(0.05, 0.5);

    for r in 0..rows {
        let stagger = if params.stagger_odd_rows && (r % 2 == 1) {
            0.5 * cell_w
        } else {
            0.0
        };

        for c in 0..cols {
            let cx = (c as f32 + 0.5) * cell_w + stagger;
            let cy = (r as f32 + 0.5) * cell_h;
            let u = cx / w;
            let v = cy / h;

            // Size modulation
            let wave = ((u * params.size_freq_x as f32 * 2.0 * PI).sin()
                + (v * params.size_freq_y as f32 * 2.0 * PI).cos())
                * 0.5;
            let radius = (base_radius * (1.0 + wave * params.size_modulation_amp)).max(1.0);

            let (base_cmds, morph_neg, morph_pos) =
                make_motif_commands(params.shape, cx, cy, radius, params.base_rotation_deg);

            // Compute toroidal wrap offsets if shape extends beyond [0, w] or [0, h]
            let wrap_offsets = compute_wrap_offsets(cx, cy, radius * 1.5, w, h);

            primitives.push(GeneratedPrimitive {
                commands: base_cmds,
                morph_neg: Some(morph_neg),
                morph_pos: Some(morph_pos),
                pivot: [cx, cy],
                grid_uv: (u % 1.0, v % 1.0),
                is_closed: true,
                style_override: None,
                wrap_offsets,
            });
        }
    }

    primitives
}

fn make_motif_commands(
    shape: MotifShape,
    cx: f32,
    cy: f32,
    r: f32,
    rot_deg: f32,
) -> (Vec<PathCommand>, Vec<PathCommand>, Vec<PathCommand>) {
    let rot = rot_deg.to_radians();

    match shape {
        MotifShape::Circle => {
            // Cubic bezier 4-segment circle:
            // Morph -1: small circle or square-ish, Morph +1: star-ish pinch
            let base = circle_cubic(cx, cy, r);
            let neg = circle_cubic(cx, cy, r * 0.5);
            let pos = circle_cubic(cx, cy, r * 1.4);
            (base, neg, pos)
        }
        MotifShape::Square => {
            let base = regular_polygon_as_cubics(cx, cy, r, 4, rot + PI * 0.25);
            let neg = regular_polygon_as_cubics(cx, cy, r * 0.5, 4, rot + PI * 0.25);
            let pos = regular_polygon_as_cubics(cx, cy, r * 1.3, 4, rot + PI * 0.25);
            (base, neg, pos)
        }
        MotifShape::Diamond => {
            let base = regular_polygon_as_cubics(cx, cy, r, 4, rot);
            let neg = regular_polygon_as_cubics(cx, cy, r * 0.5, 4, rot);
            let pos = regular_polygon_as_cubics(cx, cy, r * 1.3, 4, rot);
            (base, neg, pos)
        }
        MotifShape::Star => {
            let base = star_as_cubics(cx, cy, r, r * 0.45, 5, rot);
            let neg = star_as_cubics(cx, cy, r, r * 0.2, 5, rot); // sharper star
            let pos = star_as_cubics(cx, cy, r, r * 0.8, 5, rot); // fuller star
            (base, neg, pos)
        }
        MotifShape::Cross => {
            let base = cross_as_cubics(cx, cy, r, 0.35, rot);
            let neg = cross_as_cubics(cx, cy, r, 0.15, rot); // thin cross
            let pos = cross_as_cubics(cx, cy, r, 0.65, rot); // thick cross
            (base, neg, pos)
        }
    }
}

// 4-cubic circle
fn circle_cubic(cx: f32, cy: f32, r: f32) -> Vec<PathCommand> {
    let k = 0.55228475 * r;
    vec![
        PathCommand::MoveTo([cx, cy - r]),
        PathCommand::CubicTo([cx + k, cy - r], [cx + r, cy - k], [cx + r, cy]),
        PathCommand::CubicTo([cx + r, cy + k], [cx + k, cy + r], [cx, cy + r]),
        PathCommand::CubicTo([cx - k, cy + r], [cx - r, cy + k], [cx - r, cy]),
        PathCommand::CubicTo([cx - r, cy - k], [cx - k, cy - r], [cx, cy - r]),
        PathCommand::Close,
    ]
}

// Polygon converted to 4 or N cubic segments (for uniform morph compatibility)
fn regular_polygon_as_cubics(
    cx: f32,
    cy: f32,
    r: f32,
    n: usize,
    rot: f32,
) -> Vec<PathCommand> {
    let mut pts = Vec::with_capacity(n);
    for i in 0..n {
        let a = rot + i as f32 * 2.0 * PI / n as f32;
        pts.push([cx + a.cos() * r, cy + a.sin() * r]);
    }

    let mut cmds = Vec::new();
    cmds.push(PathCommand::MoveTo(pts[0]));
    for i in 0..n {
        let p1 = pts[i];
        let p2 = pts[(i + 1) % n];
        // Straight line as degenerate cubic (so it can smoothly morph into curves)
        let c1 = [p1[0] + (p2[0] - p1[0]) / 3.0, p1[1] + (p2[1] - p1[1]) / 3.0];
        let c2 = [p1[0] + 2.0 * (p2[0] - p1[0]) / 3.0, p1[1] + 2.0 * (p2[1] - p1[1]) / 3.0];
        cmds.push(PathCommand::CubicTo(c1, c2, p2));
    }
    cmds.push(PathCommand::Close);
    cmds
}

fn star_as_cubics(
    cx: f32,
    cy: f32,
    r_outer: f32,
    r_inner: f32,
    points: usize,
    rot: f32,
) -> Vec<PathCommand> {
    let n = points * 2;
    let mut pts = Vec::with_capacity(n);
    for i in 0..n {
        let r = if i % 2 == 0 { r_outer } else { r_inner };
        let a = rot + i as f32 * PI / points as f32;
        pts.push([cx + a.cos() * r, cy + a.sin() * r]);
    }

    let mut cmds = Vec::new();
    cmds.push(PathCommand::MoveTo(pts[0]));
    for i in 0..n {
        let p1 = pts[i];
        let p2 = pts[(i + 1) % n];
        let c1 = [p1[0] + (p2[0] - p1[0]) / 3.0, p1[1] + (p2[1] - p1[1]) / 3.0];
        let c2 = [p1[0] + 2.0 * (p2[0] - p1[0]) / 3.0, p1[1] + 2.0 * (p2[1] - p1[1]) / 3.0];
        cmds.push(PathCommand::CubicTo(c1, c2, p2));
    }
    cmds.push(PathCommand::Close);
    cmds
}

fn cross_as_cubics(
    cx: f32,
    cy: f32,
    r: f32,
    w_ratio: f32,
    rot: f32,
) -> Vec<PathCommand> {
    let w = r * w_ratio;
    // 12 points for cross
    let raw_pts = [
        [-w, -r], [w, -r], [w, -w], [r, -w],
        [r, w], [w, w], [w, r], [-w, r],
        [-w, w], [-r, w], [-r, -w], [-w, -w],
    ];

    let cos_r = rot.cos();
    let sin_r = rot.sin();
    let pts: Vec<[f32; 2]> = raw_pts
        .iter()
        .map(|&[px, py]| [cx + px * cos_r - py * sin_r, cy + px * sin_r + py * cos_r])
        .collect();

    let mut cmds = Vec::new();
    cmds.push(PathCommand::MoveTo(pts[0]));
    for i in 0..pts.len() {
        let p1 = pts[i];
        let p2 = pts[(i + 1) % pts.len()];
        let c1 = [p1[0] + (p2[0] - p1[0]) / 3.0, p1[1] + (p2[1] - p1[1]) / 3.0];
        let c2 = [p1[0] + 2.0 * (p2[0] - p1[0]) / 3.0, p1[1] + 2.0 * (p2[1] - p1[1]) / 3.0];
        cmds.push(PathCommand::CubicTo(c1, c2, p2));
    }
    cmds.push(PathCommand::Close);
    cmds
}

// ----------------------------------------------------------------------------
// Waves / Sine Stripes
// ----------------------------------------------------------------------------
fn eval_waves(params: &WavesSineStripesParams, w: f32, h: f32) -> Vec<GeneratedPrimitive> {
    let mut primitives = Vec::new();
    let bands = params.band_count.max(1);
    let samples = params.samples_per_wave.clamp(8, 64);
    let freq = params.frequency.max(1) as f32;

    for b in 0..bands {
        let frac = (b as f32 + 0.5) / bands as f32;
        let uv = match params.orientation {
            WaveOrientation::Horizontal => (0.5, frac),
            WaveOrientation::Vertical => (frac, 0.5),
            WaveOrientation::Diagonal => (frac, frac),
        };

        let gen_wave_cmds = |amp: f32, phase_shift: f32| -> Vec<PathCommand> {
            let mut pts = Vec::with_capacity(samples as usize + 1);
            for s in 0..=samples {
                let t = s as f32 / samples as f32;
                let angle = t * 2.0 * PI * freq + phase_shift;
                let wave_disp = (angle.sin() + params.harmonic_ratio * (angle * 2.0).sin()) * amp;

                let (x, y) = match params.orientation {
                    WaveOrientation::Horizontal => (t * w, frac * h + wave_disp),
                    WaveOrientation::Vertical => (frac * w + wave_disp, t * h),
                    WaveOrientation::Diagonal => {
                        let base_x = (t + frac) * w * 0.5;
                        let base_y = (t - frac + 1.0) * h * 0.5;
                        (base_x + wave_disp * 0.707, base_y + wave_disp * 0.707)
                    }
                };
                pts.push([x, y]);
            }

            // Convert point chain to cubic splines
            let mut cmds = Vec::new();
            if let Some(&first) = pts.first() {
                cmds.push(PathCommand::MoveTo(first));
                for i in 0..pts.len() - 1 {
                    let p0 = if i > 0 { pts[i - 1] } else { pts[i] };
                    let p1 = pts[i];
                    let p2 = pts[i + 1];
                    let p3 = if i + 2 < pts.len() { pts[i + 2] } else { p2 };

                    // Catmull-Rom to Cubic Bezier conversion
                    let c1 = [
                        p1[0] + (p2[0] - p0[0]) / 6.0,
                        p1[1] + (p2[1] - p0[1]) / 6.0,
                    ];
                    let c2 = [
                        p2[0] - (p3[0] - p1[0]) / 6.0,
                        p2[1] - (p3[1] - p1[1]) / 6.0,
                    ];
                    cmds.push(PathCommand::CubicTo(c1, c2, p2));
                }
            }
            cmds
        };

        let base_cmds = gen_wave_cmds(params.amplitude, 0.0);
        let morph_neg = gen_wave_cmds(params.amplitude * 0.2, -PI * 0.5);
        let morph_pos = gen_wave_cmds(params.amplitude * 1.8, PI * 0.5);

        // Compute midpoint as pivot
        let pivot = match params.orientation {
            WaveOrientation::Horizontal => [w * 0.5, frac * h],
            WaveOrientation::Vertical => [frac * w, h * 0.5],
            WaveOrientation::Diagonal => [w * 0.5, h * 0.5],
        };

        primitives.push(GeneratedPrimitive {
            commands: base_cmds,
            morph_neg: Some(morph_neg),
            morph_pos: Some(morph_pos),
            pivot,
            grid_uv: uv,
            is_closed: false,
            style_override: None,
            wrap_offsets: Vec::new(),
        });
    }

    primitives
}

// ----------------------------------------------------------------------------
// Truchet / Geometric Tiles
// ----------------------------------------------------------------------------
fn eval_truchet(params: &TruchetTilesParams, w: f32, h: f32) -> Vec<GeneratedPrimitive> {
    let mut primitives = Vec::new();
    let cols = params.cols.max(1);
    let rows = params.rows.max(1);
    let cell_w = w / cols as f32;
    let cell_h = h / rows as f32;
    let cr = cell_w.min(cell_h) * params.corner_radius_ratio.clamp(0.2, 0.8);

    for r in 0..rows {
        for c in 0..cols {
            let x0 = c as f32 * cell_w;
            let y0 = r as f32 * cell_h;
            let cx = x0 + cell_w * 0.5;
            let cy = y0 + cell_h * 0.5;
            let u = c as f32 / cols as f32;
            let v = r as f32 / rows as f32;

            // Orientation hash
            let oriented = match params.layout {
                TruchetLayout::Checkerboard => (c + r) % 2 == 1,
                TruchetLayout::DiagonalWave => (c * 2 + r) % 3 == 0,
                TruchetLayout::HashSeed => {
                    let hval = simple_hash(c as u64, r as u64, params.seed);
                    hval % 2 == 1
                }
            };

            match params.variant {
                TruchetVariant::QuarterCircles => {
                    // Two quarter circle arcs in opposite corners
                    let (arc1, arc2) = if !oriented {
                        // Top-left and bottom-right
                        (
                            quarter_circle_arc(x0, y0, cr, 0.0),
                            quarter_circle_arc(x0 + cell_w, y0 + cell_h, cr, PI),
                        )
                    } else {
                        // Top-right and bottom-left
                        (
                            quarter_circle_arc(x0 + cell_w, y0, cr, PI * 0.5),
                            quarter_circle_arc(x0, y0 + cell_h, cr, PI * 1.5),
                        )
                    };

                    primitives.push(GeneratedPrimitive {
                        commands: arc1.clone(),
                        morph_neg: Some(scale_cmds(&arc1, cx, cy, 0.5)),
                        morph_pos: Some(scale_cmds(&arc1, cx, cy, 1.4)),
                        pivot: [cx, cy],
                        grid_uv: (u, v),
                        is_closed: false,
                        style_override: None,
                        wrap_offsets: Vec::new(),
                    });

                    if params.double_arc {
                        primitives.push(GeneratedPrimitive {
                            commands: arc2.clone(),
                            morph_neg: Some(scale_cmds(&arc2, cx, cy, 0.5)),
                            morph_pos: Some(scale_cmds(&arc2, cx, cy, 1.4)),
                            pivot: [cx, cy],
                            grid_uv: (u, v),
                            is_closed: false,
                            style_override: None,
                            wrap_offsets: Vec::new(),
                        });
                    }
                }
                TruchetVariant::DiagonalLines => {
                    let (p1, p2) = if !oriented {
                        ([x0, y0 + cell_h * 0.5], [x0 + cell_w * 0.5, y0])
                    } else {
                        ([x0 + cell_w * 0.5, y0], [x0 + cell_w, y0 + cell_h * 0.5])
                    };
                    let cmds = line_as_cubics(p1, p2);
                    primitives.push(GeneratedPrimitive {
                        commands: cmds.clone(),
                        morph_neg: Some(scale_cmds(&cmds, cx, cy, 0.5)),
                        morph_pos: Some(scale_cmds(&cmds, cx, cy, 1.4)),
                        pivot: [cx, cy],
                        grid_uv: (u, v),
                        is_closed: false,
                        style_override: None,
                        wrap_offsets: Vec::new(),
                    });
                }
                TruchetVariant::TriangleSubdivision => {
                    let pts = if !oriented {
                        vec![[x0, y0], [x0 + cell_w, y0], [x0, y0 + cell_h]]
                    } else {
                        vec![[x0 + cell_w, y0], [x0 + cell_w, y0 + cell_h], [x0, y0 + cell_h]]
                    };
                    let cmds = poly_as_cubics(&pts);
                    primitives.push(GeneratedPrimitive {
                        commands: cmds.clone(),
                        morph_neg: Some(scale_cmds(&cmds, cx, cy, 0.7)),
                        morph_pos: Some(scale_cmds(&cmds, cx, cy, 1.2)),
                        pivot: [cx, cy],
                        grid_uv: (u, v),
                        is_closed: true,
                        style_override: None,
                        wrap_offsets: Vec::new(),
                    });
                }
            }
        }
    }

    primitives
}

fn quarter_circle_arc(cx: f32, cy: f32, r: f32, start_angle: f32) -> Vec<PathCommand> {
    let a1 = start_angle;
    let a2 = start_angle + PI * 0.5;
    let p1 = [cx + a1.cos() * r, cy + a1.sin() * r];
    let p2 = [cx + a2.cos() * r, cy + a2.sin() * r];
    let k = 0.55228475 * r;

    let c1 = [p1[0] - a1.sin() * k, p1[1] + a1.cos() * k];
    let c2 = [p2[0] - a2.sin() * k, p2[1] + a2.cos() * k];

    vec![PathCommand::MoveTo(p1), PathCommand::CubicTo(c1, c2, p2)]
}

fn line_as_cubics(p1: [f32; 2], p2: [f32; 2]) -> Vec<PathCommand> {
    let c1 = [p1[0] + (p2[0] - p1[0]) / 3.0, p1[1] + (p2[1] - p1[1]) / 3.0];
    let c2 = [p1[0] + 2.0 * (p2[0] - p1[0]) / 3.0, p1[1] + 2.0 * (p2[1] - p1[1]) / 3.0];
    vec![PathCommand::MoveTo(p1), PathCommand::CubicTo(c1, c2, p2)]
}

fn poly_as_cubics(pts: &[[f32; 2]]) -> Vec<PathCommand> {
    let mut cmds = Vec::new();
    if pts.is_empty() {
        return cmds;
    }
    cmds.push(PathCommand::MoveTo(pts[0]));
    for i in 0..pts.len() {
        let p1 = pts[i];
        let p2 = pts[(i + 1) % pts.len()];
        let c1 = [p1[0] + (p2[0] - p1[0]) / 3.0, p1[1] + (p2[1] - p1[1]) / 3.0];
        let c2 = [p1[0] + 2.0 * (p2[0] - p1[0]) / 3.0, p1[1] + 2.0 * (p2[1] - p1[1]) / 3.0];
        cmds.push(PathCommand::CubicTo(c1, c2, p2));
    }
    cmds.push(PathCommand::Close);
    cmds
}

fn scale_cmds(cmds: &[PathCommand], cx: f32, cy: f32, factor: f32) -> Vec<PathCommand> {
    cmds.iter()
        .map(|c| match c {
            PathCommand::MoveTo([x, y]) => {
                PathCommand::MoveTo([cx + (x - cx) * factor, cy + (y - cy) * factor])
            }
            PathCommand::LineTo([x, y]) => {
                PathCommand::LineTo([cx + (x - cx) * factor, cy + (y - cy) * factor])
            }
            PathCommand::CubicTo([x1, y1], [x2, y2], [x3, y3]) => PathCommand::CubicTo(
                [cx + (x1 - cx) * factor, cy + (y1 - cy) * factor],
                [cx + (x2 - cx) * factor, cy + (y2 - cy) * factor],
                [cx + (x3 - cx) * factor, cy + (y3 - cy) * factor],
            ),
            PathCommand::Close => PathCommand::Close,
        })
        .collect()
}

// ----------------------------------------------------------------------------
// Isometric / Hexagonal Grid
// ----------------------------------------------------------------------------
fn eval_isometric(
    params: &IsometricHexGridParams,
    w: f32,
    h: f32,
    primary_color: [u8; 4],
    secondary_color: [u8; 4],
) -> Vec<GeneratedPrimitive> {
    let mut primitives = Vec::new();
    let cols = params.cols.max(1);
    let rows = params.rows.max(1);

    // Regular isometric tile spacing
    let dx = w / cols as f32;
    let dy = h / rows as f32;
    let radius = dx.min(dy) * 0.5 * (1.0 - params.inset_ratio);

    for r in 0..rows {
        for c in 0..cols {
            let stagger = if r % 2 == 1 { dx * 0.5 } else { 0.0 };
            let cx = (c as f32 + 0.5) * dx + stagger;
            let cy = (r as f32 + 0.5) * dy;
            let u = (cx / w) % 1.0;
            let v = (cy / h) % 1.0;

            match params.mode {
                TessellationMode::IsometricCubes => {
                    // 3 faces of an isometric cube: Top, Left, Right
                    let (top_cmds, left_cmds, right_cmds) = make_cube_faces(cx, cy, radius);

                    // Colors for 3D depth
                    let top_color = primary_color;
                    let left_color = [
                        (primary_color[0] as f32 * 0.7) as u8,
                        (primary_color[1] as f32 * 0.7) as u8,
                        (primary_color[2] as f32 * 0.7) as u8,
                        primary_color[3],
                    ];
                    let right_color = [
                        (secondary_color[0] as f32 * 0.85) as u8,
                        (secondary_color[1] as f32 * 0.85) as u8,
                        (secondary_color[2] as f32 * 0.85) as u8,
                        secondary_color[3],
                    ];

                    let wrap_offsets = compute_wrap_offsets(cx, cy, radius * 1.5, w, h);

                    let faces = [
                        (top_cmds, top_color),
                        (left_cmds, left_color),
                        (right_cmds, right_color),
                    ];

                    for (cmds, col) in faces {
                        primitives.push(GeneratedPrimitive {
                            commands: cmds.clone(),
                            morph_neg: Some(scale_cmds(&cmds, cx, cy, 0.7)),
                            morph_pos: Some(scale_cmds(&cmds, cx, cy, 1.25)),
                            pivot: [cx, cy],
                            grid_uv: (u, v),
                            is_closed: true,
                            style_override: if params.face_shading {
                                Some(PrimitiveStyleOverride {
                                    fill_color: Some(col),
                                    stroke_color: None,
                                })
                            } else {
                                None
                            },
                            wrap_offsets: wrap_offsets.clone(),
                        });
                    }
                }
                TessellationMode::HexHoneycomb => {
                    let hex = regular_polygon_as_cubics(cx, cy, radius, 6, 0.0);
                    let wrap_offsets = compute_wrap_offsets(cx, cy, radius * 1.5, w, h);

                    primitives.push(GeneratedPrimitive {
                        commands: hex.clone(),
                        morph_neg: Some(scale_cmds(&hex, cx, cy, 0.6)),
                        morph_pos: Some(scale_cmds(&hex, cx, cy, 1.3)),
                        pivot: [cx, cy],
                        grid_uv: (u, v),
                        is_closed: true,
                        style_override: None,
                        wrap_offsets,
                    });
                }
                TessellationMode::TriangularMesh => {
                    let tri = regular_polygon_as_cubics(cx, cy, radius, 3, -PI * 0.5);
                    let wrap_offsets = compute_wrap_offsets(cx, cy, radius * 1.5, w, h);

                    primitives.push(GeneratedPrimitive {
                        commands: tri.clone(),
                        morph_neg: Some(scale_cmds(&tri, cx, cy, 0.6)),
                        morph_pos: Some(scale_cmds(&tri, cx, cy, 1.3)),
                        pivot: [cx, cy],
                        grid_uv: (u, v),
                        is_closed: true,
                        style_override: None,
                        wrap_offsets,
                    });
                }
            }
        }
    }

    primitives
}

fn make_cube_faces(
    cx: f32,
    cy: f32,
    r: f32,
) -> (Vec<PathCommand>, Vec<PathCommand>, Vec<PathCommand>) {
    // 6 outer hexagon vertices
    let mut v = Vec::with_capacity(6);
    for i in 0..6 {
        let a = -PI * 0.5 + i as f32 * PI / 3.0;
        v.push([cx + a.cos() * r, cy + a.sin() * r]);
    }
    let center = [cx, cy];

    // Top face: v[0], v[1], center, v[5]
    let top = poly_as_cubics(&[v[0], v[1], center, v[5]]);
    // Right face: center, v[1], v[2], v[3]
    let right = poly_as_cubics(&[center, v[1], v[2], v[3]]);
    // Left face: center, v[3], v[4], v[5]
    let left = poly_as_cubics(&[center, v[3], v[4], v[5]]);

    (top, left, right)
}

// ----------------------------------------------------------------------------
// Concentric Ripples
// ----------------------------------------------------------------------------
fn eval_ripples(params: &ConcentricRipplesParams, w: f32, h: f32) -> Vec<GeneratedPrimitive> {
    let mut primitives = Vec::new();
    let cx_count = params.center_count_x.max(1);
    let cy_count = params.center_count_y.max(1);
    let rings = params.rings_per_center.max(1);
    let max_radius = w.min(h) * params.max_radius_ratio.clamp(0.1, 1.5);

    let cell_w = w / cx_count as f32;
    let cell_h = h / cy_count as f32;

    for ry in 0..cy_count {
        for rx in 0..cx_count {
            let base_cx = (rx as f32 + 0.5) * cell_w;
            let base_cy = (ry as f32 + 0.5) * cell_h;

            let jx = if params.jitter_amount > 0.0 {
                let hval = simple_hash(rx as u64, ry as u64, params.seed) as f32 / u32::MAX as f32;
                (hval - 0.5) * cell_w * params.jitter_amount
            } else {
                0.0
            };
            let jy = if params.jitter_amount > 0.0 {
                let hval = simple_hash(rx as u64 + 100, ry as u64, params.seed) as f32
                    / u32::MAX as f32;
                (hval - 0.5) * cell_h * params.jitter_amount
            } else {
                0.0
            };

            let center_x = base_cx + jx;
            let center_y = base_cy + jy;
            let u = (center_x / w) % 1.0;
            let v = (center_y / h) % 1.0;

            for ring in 1..=rings {
                let frac = ring as f32 / rings as f32;
                let r = max_radius * frac.powf(params.ring_spacing_exp);

                let base_cmds = circle_cubic(center_x, center_y, r);
                let morph_neg = circle_cubic(center_x, center_y, r * 0.7);
                let morph_pos = circle_cubic(center_x, center_y, r * 1.3);

                let wrap_offsets = compute_wrap_offsets(center_x, center_y, r, w, h);

                primitives.push(GeneratedPrimitive {
                    commands: base_cmds,
                    morph_neg: Some(morph_neg),
                    morph_pos: Some(morph_pos),
                    pivot: [center_x, center_y],
                    grid_uv: (u, v),
                    is_closed: false,
                    style_override: None,
                    wrap_offsets,
                });
            }
        }
    }

    primitives
}

// ----------------------------------------------------------------------------
// Flow Field Dashes
// ----------------------------------------------------------------------------
fn eval_flow_field(params: &FlowFieldDashesParams, w: f32, h: f32) -> Vec<GeneratedPrimitive> {
    let mut primitives = Vec::new();
    let cols = params.cols.max(1);
    let rows = params.rows.max(1);
    let cell_w = w / cols as f32;
    let cell_h = h / rows as f32;
    let len = cell_w.min(cell_h) * params.dash_length_ratio.clamp(0.2, 1.2);

    for r in 0..rows {
        for c in 0..cols {
            let cx = (c as f32 + 0.5) * cell_w;
            let cy = (r as f32 + 0.5) * cell_h;
            let u = cx / w;
            let v = cy / h;

            // Periodic curl angle
            let ang1 = (u * params.freq_x as f32 * 2.0 * PI).sin();
            let ang2 = (v * params.freq_y as f32 * 2.0 * PI).cos();
            let angle = (ang1 + ang2 + params.curl_phase) * params.angle_amplitude_deg.to_radians();

            let half_len = len * 0.5;
            let dx = angle.cos() * half_len;
            let dy = angle.sin() * half_len;

            let p1 = [cx - dx, cy - dy];
            let p2 = [cx + dx, cy + dy];

            let base_cmds = if params.arrow_heads {
                make_arrow_cmds(p1, p2, len * 0.3)
            } else {
                line_as_cubics(p1, p2)
            };

            let morph_neg = scale_cmds(&base_cmds, cx, cy, 0.4);
            let morph_pos = scale_cmds(&base_cmds, cx, cy, 1.5);

            let wrap_offsets = compute_wrap_offsets(cx, cy, half_len * 1.5, w, h);

            primitives.push(GeneratedPrimitive {
                commands: base_cmds,
                morph_neg: Some(morph_neg),
                morph_pos: Some(morph_pos),
                pivot: [cx, cy],
                grid_uv: (u, v),
                is_closed: params.arrow_heads,
                style_override: None,
                wrap_offsets,
            });
        }
    }

    primitives
}

fn make_arrow_cmds(p1: [f32; 2], p2: [f32; 2], head_size: f32) -> Vec<PathCommand> {
    let mut cmds = line_as_cubics(p1, p2);
    let dx = p2[0] - p1[0];
    let dy = p2[1] - p1[1];
    let angle = dy.atan2(dx);

    let a1 = angle + PI * 0.8;
    let a2 = angle - PI * 0.8;
    let h1 = [p2[0] + a1.cos() * head_size, p2[1] + a1.sin() * head_size];
    let h2 = [p2[0] + a2.cos() * head_size, p2[1] + a2.sin() * head_size];

    let wing1 = line_as_cubics(p2, h1);
    let wing2 = line_as_cubics(p2, h2);
    cmds.extend(wing1);
    cmds.extend(wing2);
    cmds
}

// ----------------------------------------------------------------------------
// Utilities
// ----------------------------------------------------------------------------
pub fn compute_wrap_offsets(cx: f32, cy: f32, radius: f32, w: f32, h: f32) -> Vec<[f32; 2]> {
    let mut offsets = Vec::new();

    let cross_left = cx - radius < 0.0;
    let cross_right = cx + radius > w;
    let cross_top = cy - radius < 0.0;
    let cross_bottom = cy + radius > h;

    let dx_list: &[f32] = if cross_left {
        &[w]
    } else if cross_right {
        &[-w]
    } else {
        &[]
    };

    let dy_list: &[f32] = if cross_top {
        &[h]
    } else if cross_bottom {
        &[-h]
    } else {
        &[]
    };

    for &dx in dx_list {
        offsets.push([dx, 0.0]);
    }
    for &dy in dy_list {
        offsets.push([0.0, dy]);
    }
    for &dx in dx_list {
        for &dy in dy_list {
            offsets.push([dx, dy]);
        }
    }

    offsets
}

fn simple_hash(a: u64, b: u64, seed: u64) -> u32 {
    let mut x = a ^ (b << 16) ^ (seed << 32);
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x as u32
}
