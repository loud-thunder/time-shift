#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingFunction {
    Linear,
    EaseInOut,
    EaseIn,
    EaseOut,
    CubicBezier(i32, i32, i32, i32), // quantized fixed point or we can store f32 / custom
}

impl EasingFunction {
    pub fn all() -> &'static [EasingFunction] {
        &[
            EasingFunction::Linear,
            EasingFunction::EaseInOut,
            EasingFunction::EaseIn,
            EasingFunction::EaseOut,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            EasingFunction::Linear => "linear",
            EasingFunction::EaseInOut => "ease-in-out",
            EasingFunction::EaseIn => "ease-in",
            EasingFunction::EaseOut => "ease-out",
            EasingFunction::CubicBezier(..) => "cubic-bezier",
        }
    }

    pub fn css_value(&self) -> String {
        match self {
            EasingFunction::Linear => "linear".to_string(),
            EasingFunction::EaseInOut => "ease-in-out".to_string(),
            EasingFunction::EaseIn => "ease-in".to_string(),
            EasingFunction::EaseOut => "ease-out".to_string(),
            EasingFunction::CubicBezier(p1, p2, p3, p4) => {
                format!(
                    "cubic-bezier({:.2}, {:.2}, {:.2}, {:.2})",
                    *p1 as f32 / 100.0,
                    *p2 as f32 / 100.0,
                    *p3 as f32 / 100.0,
                    *p4 as f32 / 100.0
                )
            }
        }
    }

    pub fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => t * (2.0 - t),
            EasingFunction::CubicBezier(_p1, _p2, _p3, _p4) => {
                // Smooth approximation for general S-curve
                t * t * (3.0 - 2.0 * t)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

impl AnimationDirection {
    pub fn all() -> &'static [AnimationDirection] {
        &[
            AnimationDirection::Normal,
            AnimationDirection::Reverse,
            AnimationDirection::Alternate,
            AnimationDirection::AlternateReverse,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            AnimationDirection::Normal => "normal",
            AnimationDirection::Reverse => "reverse",
            AnimationDirection::Alternate => "alternate",
            AnimationDirection::AlternateReverse => "alternate-reverse",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyframeStop<T: Clone + Copy> {
    pub offset_pct: f32, // 0.0 .. 100.0
    pub value: T,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformState {
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotate_deg: f32,
    pub skew_x_deg: f32,
}

impl Default for TransformState {
    fn default() -> Self {
        Self {
            scale_x: 1.0,
            scale_y: 1.0,
            rotate_deg: 0.0,
            skew_x_deg: 0.0,
        }
    }
}

impl TransformState {
    pub fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        Self {
            scale_x: a.scale_x + (b.scale_x - a.scale_x) * t,
            scale_y: a.scale_y + (b.scale_y - a.scale_y) * t,
            rotate_deg: a.rotate_deg + (b.rotate_deg - a.rotate_deg) * t,
            skew_x_deg: a.skew_x_deg + (b.skew_x_deg - a.skew_x_deg) * t,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorOpacityState {
    pub color: [u8; 4],
    pub opacity: f32,
}

impl Default for ColorOpacityState {
    fn default() -> Self {
        Self {
            color: [255, 255, 255, 255],
            opacity: 1.0,
        }
    }
}

impl ColorOpacityState {
    pub fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        let r = (a.color[0] as f32 + (b.color[0] as f32 - a.color[0] as f32) * t).clamp(0.0, 255.0) as u8;
        let g = (a.color[1] as f32 + (b.color[1] as f32 - a.color[1] as f32) * t).clamp(0.0, 255.0) as u8;
        let bl = (a.color[2] as f32 + (b.color[2] as f32 - a.color[2] as f32) * t).clamp(0.0, 255.0) as u8;
        let al = (a.color[3] as f32 + (b.color[3] as f32 - a.color[3] as f32) * t).clamp(0.0, 255.0) as u8;
        Self {
            color: [r, g, bl, al],
            opacity: (a.opacity + (b.opacity - a.opacity) * t).clamp(0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MorphState {
    pub morph_factor: f32, // -1.0 .. 1.0
}

impl Default for MorphState {
    fn default() -> Self {
        Self { morph_factor: 0.0 }
    }
}

impl MorphState {
    pub fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        Self {
            morph_factor: a.morph_factor + (b.morph_factor - a.morph_factor) * t,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DriftTrack {
    pub enabled: bool,
    pub duration_sec: f32,
    pub easing: EasingFunction,
    pub direction: AnimationDirection,
    pub tiles_dx: i32, // integer multiple of tile width
    pub tiles_dy: i32, // integer multiple of tile height
    pub stops: Vec<KeyframeStop<(f32, f32)>>,
}

impl Default for DriftTrack {
    fn default() -> Self {
        Self {
            enabled: false,
            duration_sec: 10.0,
            easing: EasingFunction::Linear,
            direction: AnimationDirection::Normal,
            tiles_dx: 1,
            tiles_dy: 0,
            stops: vec![
                KeyframeStop { offset_pct: 0.0, value: (0.0, 0.0) },
                KeyframeStop { offset_pct: 100.0, value: (1.0, 0.0) },
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransformTrack {
    pub enabled: bool,
    pub duration_sec: f32,
    pub easing: EasingFunction,
    pub direction: AnimationDirection,
    pub phase_freq_x: i32,
    pub phase_freq_y: i32,
    pub stops: Vec<KeyframeStop<TransformState>>,
}

impl Default for TransformTrack {
    fn default() -> Self {
        Self {
            enabled: false,
            duration_sec: 4.0,
            easing: EasingFunction::EaseInOut,
            direction: AnimationDirection::Alternate,
            phase_freq_x: 1,
            phase_freq_y: 1,
            stops: vec![
                KeyframeStop {
                    offset_pct: 0.0,
                    value: TransformState { scale_x: 0.8, scale_y: 0.8, rotate_deg: 0.0, skew_x_deg: 0.0 },
                },
                KeyframeStop {
                    offset_pct: 50.0,
                    value: TransformState { scale_x: 1.2, scale_y: 1.2, rotate_deg: 45.0, skew_x_deg: 0.0 },
                },
                KeyframeStop {
                    offset_pct: 100.0,
                    value: TransformState { scale_x: 0.8, scale_y: 0.8, rotate_deg: 90.0, skew_x_deg: 0.0 },
                },
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorOpacityTrack {
    pub enabled: bool,
    pub duration_sec: f32,
    pub easing: EasingFunction,
    pub direction: AnimationDirection,
    pub phase_freq_x: i32,
    pub phase_freq_y: i32,
    pub stops: Vec<KeyframeStop<ColorOpacityState>>,
}

impl Default for ColorOpacityTrack {
    fn default() -> Self {
        Self {
            enabled: false,
            duration_sec: 5.0,
            easing: EasingFunction::EaseInOut,
            direction: AnimationDirection::Alternate,
            phase_freq_x: 1,
            phase_freq_y: 0,
            stops: vec![
                KeyframeStop {
                    offset_pct: 0.0,
                    value: ColorOpacityState { color: [100, 200, 255, 255], opacity: 0.4 },
                },
                KeyframeStop {
                    offset_pct: 50.0,
                    value: ColorOpacityState { color: [255, 100, 200, 255], opacity: 1.0 },
                },
                KeyframeStop {
                    offset_pct: 100.0,
                    value: ColorOpacityState { color: [100, 200, 255, 255], opacity: 0.4 },
                },
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathMorphTrack {
    pub enabled: bool,
    pub duration_sec: f32,
    pub easing: EasingFunction,
    pub direction: AnimationDirection,
    pub phase_freq_x: i32,
    pub phase_freq_y: i32,
    pub stops: Vec<KeyframeStop<MorphState>>,
}

impl Default for PathMorphTrack {
    fn default() -> Self {
        Self {
            enabled: false,
            duration_sec: 4.0,
            easing: EasingFunction::EaseInOut,
            direction: AnimationDirection::Alternate,
            phase_freq_x: 1,
            phase_freq_y: 1,
            stops: vec![
                KeyframeStop { offset_pct: 0.0, value: MorphState { morph_factor: -0.6 } },
                KeyframeStop { offset_pct: 50.0, value: MorphState { morph_factor: 0.6 } },
                KeyframeStop { offset_pct: 100.0, value: MorphState { morph_factor: -0.6 } },
            ],
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayerAnimations {
    pub drift: DriftTrack,
    pub transform: TransformTrack,
    pub color_opacity: ColorOpacityTrack,
    pub path_morph: PathMorphTrack,
}
