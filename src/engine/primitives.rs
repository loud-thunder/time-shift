#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathCommand {
    MoveTo([f32; 2]),
    LineTo([f32; 2]),
    CubicTo([f32; 2], [f32; 2], [f32; 2]),
    Close,
}

impl PathCommand {
    pub fn translate(&self, dx: f32, dy: f32) -> Self {
        match self {
            PathCommand::MoveTo([x, y]) => PathCommand::MoveTo([x + dx, y + dy]),
            PathCommand::LineTo([x, y]) => PathCommand::LineTo([x + dx, y + dy]),
            PathCommand::CubicTo([x1, y1], [x2, y2], [x3, y3]) => {
                PathCommand::CubicTo([x1 + dx, y1 + dy], [x2 + dx, y2 + dy], [x3 + dx, y3 + dy])
            }
            PathCommand::Close => PathCommand::Close,
        }
    }

    pub fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        match (a, b) {
            (PathCommand::MoveTo([ax, ay]), PathCommand::MoveTo([bx, by])) => {
                PathCommand::MoveTo([ax + (bx - ax) * t, ay + (by - ay) * t])
            }
            (PathCommand::LineTo([ax, ay]), PathCommand::LineTo([bx, by])) => {
                PathCommand::LineTo([ax + (bx - ax) * t, ay + (by - ay) * t])
            }
            (
                PathCommand::CubicTo([ax1, ay1], [ax2, ay2], [ax3, ay3]),
                PathCommand::CubicTo([bx1, by1], [bx2, by2], [bx3, by3]),
            ) => PathCommand::CubicTo(
                [ax1 + (bx1 - ax1) * t, ay1 + (by1 - ay1) * t],
                [ax2 + (bx2 - ax2) * t, ay2 + (by2 - ay2) * t],
                [ax3 + (bx3 - ax3) * t, ay3 + (by3 - ay3) * t],
            ),
            (PathCommand::Close, PathCommand::Close) => PathCommand::Close,
            _ => *a, // Mismatched command fallback
        }
    }

    pub fn to_svg_string(&self) -> String {
        match self {
            PathCommand::MoveTo([x, y]) => format!("M{:.2} {:.2}", x, y),
            PathCommand::LineTo([x, y]) => format!("L{:.2} {:.2}", x, y),
            PathCommand::CubicTo([x1, y1], [x2, y2], [x3, y3]) => {
                format!("C{:.2} {:.2} {:.2} {:.2} {:.2} {:.2}", x1, y1, x2, y2, x3, y3)
            }
            PathCommand::Close => "Z".to_string(),
        }
    }
}

pub fn commands_to_svg_d(cmds: &[PathCommand]) -> String {
    cmds.iter()
        .map(|c| c.to_svg_string())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn lerp_commands(a: &[PathCommand], b: &[PathCommand], t: f32) -> Vec<PathCommand> {
    if a.len() != b.len() {
        return a.to_vec();
    }
    a.iter()
        .zip(b.iter())
        .map(|(ca, cb)| PathCommand::lerp(ca, cb, t))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveStyleOverride {
    pub fill_color: Option<[u8; 4]>,
    pub stroke_color: Option<[u8; 4]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedPrimitive {
    pub commands: Vec<PathCommand>,
    // Pre-evaluated morph commands at -1.0, 0.0, 1.0 or custom morph factors
    pub morph_neg: Option<Vec<PathCommand>>,
    pub morph_pos: Option<Vec<PathCommand>>,
    pub pivot: [f32; 2],
    pub grid_uv: (f32, f32), // normalized 0..1 coordinate on the tile lattice
    pub is_closed: bool,
    pub style_override: Option<PrimitiveStyleOverride>,
    pub wrap_offsets: Vec<[f32; 2]>, // Toroidal ghost offsets (e.g., [W, 0], [-W, 0], etc.)
}

impl GeneratedPrimitive {
    pub fn get_commands_at_morph(&self, factor: f32) -> Vec<PathCommand> {
        let factor = factor.clamp(-1.0, 1.0);
        if factor > 0.0 {
            if let Some(pos) = &self.morph_pos {
                return lerp_commands(&self.commands, pos, factor);
            }
        } else if factor < 0.0 {
            if let Some(neg) = &self.morph_neg {
                return lerp_commands(&self.commands, neg, -factor);
            }
        }
        self.commands.clone()
    }
}
