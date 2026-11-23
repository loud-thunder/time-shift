use super::primitives::PathCommand;
use crate::model::animation::{
    AnimationDirection, ColorOpacityState, ColorOpacityTrack, EasingFunction, KeyframeStop,
    LayerAnimations, MorphState, PathMorphTrack, TransformState, TransformTrack,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EvaluatedElementState {
    pub transform: TransformState,
    pub color_opacity: ColorOpacityState,
    pub morph_factor: f32,
}

pub fn evaluate_element_state(
    anims: &LayerAnimations,
    time_sec: f32,
    grid_uv: (f32, f32),
    default_color: [u8; 4],
    default_opacity: f32,
) -> EvaluatedElementState {
    let transform = evaluate_transform(&anims.transform, time_sec, grid_uv);

    let color_opacity = evaluate_color_opacity(
        &anims.color_opacity,
        time_sec,
        grid_uv,
        default_color,
        default_opacity,
    );

    let morph_factor = evaluate_path_morph(&anims.path_morph, time_sec, grid_uv);

    EvaluatedElementState {
        transform,
        color_opacity,
        morph_factor,
    }
}

pub fn transform_commands(
    cmds: &[PathCommand],
    pivot: [f32; 2],
    tf: &TransformState,
) -> Vec<PathCommand> {
    let rad = tf.rotate_deg.to_radians();
    let cos_r = rad.cos();
    let sin_r = rad.sin();
    let skew_tan = tf.skew_x_deg.to_radians().tan();

    let transform_pt = |p: [f32; 2]| -> [f32; 2] {
        // 1. Center relative to pivot
        let mut x = p[0] - pivot[0];
        let mut y = p[1] - pivot[1];

        // 2. Skew X
        x += y * skew_tan;

        // 3. Scale
        x *= tf.scale_x;
        y *= tf.scale_y;

        // 4. Rotate
        let rx = x * cos_r - y * sin_r;
        let ry = x * sin_r + y * cos_r;

        // 5. Restore pivot
        [rx + pivot[0], ry + pivot[1]]
    };

    cmds.iter()
        .map(|c| match c {
            PathCommand::MoveTo(p) => PathCommand::MoveTo(transform_pt(*p)),
            PathCommand::LineTo(p) => PathCommand::LineTo(transform_pt(*p)),
            PathCommand::CubicTo(p1, p2, p3) => PathCommand::CubicTo(
                transform_pt(*p1),
                transform_pt(*p2),
                transform_pt(*p3),
            ),
            PathCommand::Close => PathCommand::Close,
        })
        .collect()
}

// ----------------------------------------------------------------------------
// Track Evaluators
// ----------------------------------------------------------------------------
fn evaluate_transform(
    track: &TransformTrack,
    time_sec: f32,
    grid_uv: (f32, f32),
) -> TransformState {
    if !track.enabled || track.duration_sec <= 0.0 {
        return TransformState::default();
    }

    let phase = compute_spatial_phase(grid_uv, track.phase_freq_x, track.phase_freq_y);
    let delayed_time = time_sec + phase * track.duration_sec;
    let progress = compute_progress(
        delayed_time,
        track.duration_sec,
        track.direction,
        track.easing,
    );

    interpolate_stops(
        &track.stops,
        progress,
        TransformState::default(),
        TransformState::lerp,
    )
}

fn evaluate_color_opacity(
    track: &ColorOpacityTrack,
    time_sec: f32,
    grid_uv: (f32, f32),
    default_color: [u8; 4],
    default_opacity: f32,
) -> ColorOpacityState {
    if !track.enabled || track.duration_sec <= 0.0 {
        return ColorOpacityState {
            color: default_color,
            opacity: default_opacity,
        };
    }

    let phase = compute_spatial_phase(grid_uv, track.phase_freq_x, track.phase_freq_y);
    let delayed_time = time_sec + phase * track.duration_sec;
    let progress = compute_progress(
        delayed_time,
        track.duration_sec,
        track.direction,
        track.easing,
    );

    let default_val = ColorOpacityState {
        color: default_color,
        opacity: default_opacity,
    };
    interpolate_stops(&track.stops, progress, default_val, ColorOpacityState::lerp)
}

fn evaluate_path_morph(
    track: &PathMorphTrack,
    time_sec: f32,
    grid_uv: (f32, f32),
) -> f32 {
    if !track.enabled || track.duration_sec <= 0.0 {
        return 0.0;
    }

    let phase = compute_spatial_phase(grid_uv, track.phase_freq_x, track.phase_freq_y);
    let delayed_time = time_sec + phase * track.duration_sec;
    let progress = compute_progress(
        delayed_time,
        track.duration_sec,
        track.direction,
        track.easing,
    );

    let morph_state = interpolate_stops(
        &track.stops,
        progress,
        MorphState::default(),
        MorphState::lerp,
    );
    morph_state.morph_factor
}

// ----------------------------------------------------------------------------
// Interpolation & Phase helpers
// ----------------------------------------------------------------------------
pub fn compute_spatial_phase(grid_uv: (f32, f32), freq_x: i32, freq_y: i32) -> f32 {
    let mut phase = grid_uv.0 * freq_x as f32 + grid_uv.1 * freq_y as f32;
    phase = phase.fract();
    if phase < 0.0 {
        phase += 1.0;
    }
    phase
}

pub fn compute_progress(
    time_sec: f32,
    duration_sec: f32,
    direction: AnimationDirection,
    easing: EasingFunction,
) -> f32 {
    let t = (time_sec / duration_sec).fract();
    let norm_t = if t < 0.0 { t + 1.0 } else { t };

    let directional_t = match direction {
        AnimationDirection::Normal => norm_t,
        AnimationDirection::Reverse => 1.0 - norm_t,
        AnimationDirection::Alternate => {
            let total_cycles = (time_sec / duration_sec).floor() as i64;
            if total_cycles % 2 != 0 {
                1.0 - norm_t
            } else {
                norm_t
            }
        }
        AnimationDirection::AlternateReverse => {
            let total_cycles = (time_sec / duration_sec).floor() as i64;
            if total_cycles % 2 != 0 {
                norm_t
            } else {
                1.0 - norm_t
            }
        }
    };

    easing.evaluate(directional_t)
}

fn interpolate_stops<T: Clone + Copy, F: Fn(&T, &T, f32) -> T>(
    stops: &[KeyframeStop<T>],
    progress: f32,
    fallback: T,
    lerp_fn: F,
) -> T {
    if stops.is_empty() {
        return fallback;
    }
    if stops.len() == 1 {
        return stops[0].value;
    }

    let target_pct = (progress * 100.0).clamp(0.0, 100.0);

    if target_pct <= stops[0].offset_pct {
        return stops[0].value;
    }
    if target_pct >= stops.last().unwrap().offset_pct {
        return stops.last().unwrap().value;
    }

    for i in 0..stops.len() - 1 {
        let s0 = &stops[i];
        let s1 = &stops[i + 1];
        if target_pct >= s0.offset_pct && target_pct <= s1.offset_pct {
            let range = s1.offset_pct - s0.offset_pct;
            let t = if range > 0.0001 {
                (target_pct - s0.offset_pct) / range
            } else {
                0.0
            };
            return lerp_fn(&s0.value, &s1.value, t);
        }
    }

    stops.last().unwrap().value
}
