use crate::model::animation::{
    AnimationDirection, ColorOpacityState, EasingFunction, KeyframeStop, MorphState, TransformState,
};
use crate::model::generators::{
    ConcentricRipplesParams, FlowFieldDashesParams, GeneratorKind, IsometricHexGridParams,
    MotifShape, PolkaHalftoneParams, TessellationMode, TruchetLayout, TruchetTilesParams,
    TruchetVariant, WaveOrientation, WavesSineStripesParams,
};
use crate::model::project::{PatternLayer, Project};

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub name: &'static str,
    pub background: [u8; 4],
    pub primary: [u8; 4],
    pub secondary: [u8; 4],
    pub accent: [u8; 4],
}

pub fn get_curated_palettes() -> Vec<ColorPalette> {
    vec![
        ColorPalette {
            name: "Cyber Neon",
            background: [13, 14, 26, 255],
            primary: [0, 240, 255, 255],
            secondary: [255, 0, 128, 255],
            accent: [255, 230, 0, 255],
        },
        ColorPalette {
            name: "Synthwave Sunset",
            background: [26, 11, 46, 255],
            primary: [255, 110, 199, 255],
            secondary: [255, 179, 71, 255],
            accent: [113, 88, 226, 255],
        },
        ColorPalette {
            name: "Deep Ocean",
            background: [8, 20, 36, 255],
            primary: [32, 201, 151, 255],
            secondary: [13, 110, 253, 255],
            accent: [111, 66, 193, 255],
        },
        ColorPalette {
            name: "Forest Moss",
            background: [18, 28, 22, 255],
            primary: [74, 222, 128, 255],
            secondary: [234, 179, 8, 255],
            accent: [34, 197, 94, 255],
        },
        ColorPalette {
            name: "Tokyo Midnight",
            background: [20, 22, 38, 255],
            primary: [168, 85, 247, 255],
            secondary: [236, 72, 153, 255],
            accent: [56, 189, 248, 255],
        },
        ColorPalette {
            name: "Warm Terracotta",
            background: [36, 22, 20, 255],
            primary: [251, 146, 60, 255],
            secondary: [244, 63, 94, 255],
            accent: [253, 224, 71, 255],
        },
        ColorPalette {
            name: "Monochrome Sleek",
            background: [16, 16, 18, 255],
            primary: [240, 240, 245, 255],
            secondary: [140, 140, 150, 255],
            accent: [80, 80, 90, 255],
        },
        ColorPalette {
            name: "Solar Gold",
            background: [24, 20, 15, 255],
            primary: [250, 204, 21, 255],
            secondary: [249, 115, 22, 255],
            accent: [239, 68, 68, 255],
        },
    ]
}

#[derive(Debug, Clone)]
pub struct StarterPreset {
    pub name: &'static str,
    pub description: &'static str,
    pub project_fn: fn() -> Project,
}

pub fn get_curated_presets() -> Vec<StarterPreset> {
    vec![
        StarterPreset {
            name: "Cyberpunk Isometric Matrix",
            description: "Pulsing 3D isometric cubes paired with a drifting neon flow field",
            project_fn: preset_cyber_isometric,
        },
        StarterPreset {
            name: "Synthwave Ocean Ripples",
            description: "Morphing sine waves with spatial phase delay over halftone sun rays",
            project_fn: preset_synthwave_waves,
        },
        StarterPreset {
            name: "Kinetic Truchet Labyrinth",
            description: "Color-cycling quarter-circle geometric tiles with pulsing cross motifs",
            project_fn: preset_kinetic_truchet,
        },
        StarterPreset {
            name: "Bioluminescent Ripple Pond",
            description: "Toroidal concentric ripples expanding over drifting spore points",
            project_fn: preset_bioluminescent_ripples,
        },
        StarterPreset {
            name: "Magnetic Vector Field",
            description: "Oscillating flow field dashes over subtle hexagonal honeycomb mesh",
            project_fn: preset_magnetic_flow,
        },
        StarterPreset {
            name: "Retro Halftone Spin",
            description: "Staggered star motifs with wave-propagated rotation and scale pulses",
            project_fn: preset_retro_halftone,
        },
    ]
}

// ----------------------------------------------------------------------------
// Preset Generators
// ----------------------------------------------------------------------------
fn preset_cyber_isometric() -> Project {
    let mut p = Project {
        name: "Cyberpunk Isometric Matrix".to_string(),
        tile_width: 240.0,
        tile_height: 240.0,
        background_color: [13, 14, 26, 255],
        layers: Vec::new(),
        selected_layer_idx: 0,
        next_layer_id: 3,
    };

    let mut l1 = PatternLayer::new_isometric(1, "Isometric Cubes");
    l1.primary_color = [0, 240, 255, 255];
    l1.secondary_color = [255, 0, 128, 255];
    l1.stroke_width = 1.5;
    l1.animations.transform.enabled = true;
    l1.animations.transform.duration_sec = 4.0;
    l1.animations.transform.phase_freq_x = 1;
    l1.animations.transform.phase_freq_y = 1;
    l1.animations.transform.stops = vec![
        KeyframeStop {
            offset_pct: 0.0,
            value: TransformState { scale_x: 0.85, scale_y: 0.85, rotate_deg: 0.0, skew_x_deg: 0.0 },
        },
        KeyframeStop {
            offset_pct: 50.0,
            value: TransformState { scale_x: 1.15, scale_y: 1.15, rotate_deg: 0.0, skew_x_deg: 0.0 },
        },
        KeyframeStop {
            offset_pct: 100.0,
            value: TransformState { scale_x: 0.85, scale_y: 0.85, rotate_deg: 0.0, skew_x_deg: 0.0 },
        },
    ];

    let mut l2 = PatternLayer::new_flow_field(2, "Drifting Flow Arrows");
    l2.primary_color = [255, 230, 0, 255];
    l2.secondary_color = [255, 0, 128, 255];
    l2.stroke_width = 2.0;
    l2.opacity = 0.7;
    if let GeneratorKind::FlowFieldDashes(ref mut params) = l2.generator {
        params.arrow_heads = true;
        params.cols = 6;
        params.rows = 6;
    }
    l2.animations.drift.enabled = true;
    l2.animations.drift.duration_sec = 8.0;
    l2.animations.drift.tiles_dx = 1;
    l2.animations.drift.tiles_dy = 0;

    p.layers.push(l1);
    p.layers.push(l2);
    p
}

fn preset_synthwave_waves() -> Project {
    let mut p = Project {
        name: "Synthwave Ocean Ripples".to_string(),
        tile_width: 240.0,
        tile_height: 240.0,
        background_color: [26, 11, 46, 255],
        layers: Vec::new(),
        selected_layer_idx: 1,
        next_layer_id: 3,
    };

    let mut l1 = PatternLayer::new_polka(1, "Halftone Horizon");
    l1.primary_color = [255, 179, 71, 160];
    l1.secondary_color = [255, 110, 199, 160];
    if let GeneratorKind::PolkaHalftone(ref mut params) = l1.generator {
        params.cols = 10;
        params.rows = 10;
        params.shape = MotifShape::Circle;
        params.size_modulation_amp = 0.4;
    }
    l1.animations.color_opacity.enabled = true;
    l1.animations.color_opacity.duration_sec = 4.0;
    l1.animations.color_opacity.direction = AnimationDirection::Alternate;

    let mut l2 = PatternLayer::new_waves(2, "Morphing Sine Ribbons");
    l2.primary_color = [255, 110, 199, 255];
    l2.secondary_color = [113, 88, 226, 255];
    l2.stroke_width = 3.5;
    if let GeneratorKind::WavesSineStripes(ref mut params) = l2.generator {
        params.band_count = 8;
        params.amplitude = 18.0;
        params.frequency = 2;
    }
    l2.animations.path_morph.enabled = true;
    l2.animations.path_morph.duration_sec = 6.0;
    l2.animations.path_morph.direction = AnimationDirection::Alternate;

    p.layers.push(l1);
    p.layers.push(l2);
    p
}

fn preset_kinetic_truchet() -> Project {
    let mut p = Project {
        name: "Kinetic Truchet Labyrinth".to_string(),
        tile_width: 240.0,
        tile_height: 240.0,
        background_color: [16, 16, 22, 255],
        layers: Vec::new(),
        selected_layer_idx: 0,
        next_layer_id: 3,
    };

    let mut l1 = PatternLayer::new_truchet(1, "Quarter Circles");
    l1.primary_color = [50, 220, 160, 255];
    l1.secondary_color = [20, 120, 200, 255];
    l1.stroke_width = 5.0;
    if let GeneratorKind::TruchetTiles(ref mut params) = l1.generator {
        params.cols = 6;
        params.rows = 6;
        params.layout = TruchetLayout::Checkerboard;
        params.double_arc = true;
    }
    l1.animations.color_opacity.enabled = true;
    l1.animations.color_opacity.duration_sec = 4.0;
    l1.animations.color_opacity.stops = vec![
        KeyframeStop {
            offset_pct: 0.0,
            value: ColorOpacityState { color: [50, 220, 160, 255], opacity: 1.0 },
        },
        KeyframeStop {
            offset_pct: 50.0,
            value: ColorOpacityState { color: [236, 72, 153, 255], opacity: 1.0 },
        },
        KeyframeStop {
            offset_pct: 100.0,
            value: ColorOpacityState { color: [50, 220, 160, 255], opacity: 1.0 },
        },
    ];

    let mut l2 = PatternLayer::new_polka(2, "Cross Grid");
    l2.primary_color = [240, 240, 245, 200];
    l2.secondary_color = [100, 100, 120, 200];
    l2.stroke_width = 1.0;
    if let GeneratorKind::PolkaHalftone(ref mut params) = l2.generator {
        params.cols = 6;
        params.rows = 6;
        params.shape = MotifShape::Cross;
        params.base_radius_ratio = 0.2;
    }
    l2.animations.transform.enabled = true;
    l2.animations.transform.duration_sec = 4.0;
    l2.animations.transform.stops = vec![
        KeyframeStop {
            offset_pct: 0.0,
            value: TransformState { scale_x: 0.8, scale_y: 0.8, rotate_deg: 0.0, skew_x_deg: 0.0 },
        },
        KeyframeStop {
            offset_pct: 50.0,
            value: TransformState { scale_x: 1.3, scale_y: 1.3, rotate_deg: 90.0, skew_x_deg: 0.0 },
        },
        KeyframeStop {
            offset_pct: 100.0,
            value: TransformState { scale_x: 0.8, scale_y: 0.8, rotate_deg: 180.0, skew_x_deg: 0.0 },
        },
    ];

    p.layers.push(l1);
    p.layers.push(l2);
    p
}

fn preset_bioluminescent_ripples() -> Project {
    let mut p = Project {
        name: "Bioluminescent Ripple Pond".to_string(),
        tile_width: 240.0,
        tile_height: 240.0,
        background_color: [8, 20, 36, 255],
        layers: Vec::new(),
        selected_layer_idx: 0,
        next_layer_id: 3,
    };

    let mut l1 = PatternLayer::new_ripples(1, "Toroidal Water Rings");
    l1.primary_color = [0, 240, 255, 255];
    l1.secondary_color = [32, 201, 151, 255];
    l1.stroke_width = 2.5;
    if let GeneratorKind::ConcentricRipples(ref mut params) = l1.generator {
        params.center_count_x = 2;
        params.center_count_y = 2;
        params.rings_per_center = 6;
        params.max_radius_ratio = 0.85;
    }
    l1.animations.path_morph.enabled = true;
    l1.animations.path_morph.duration_sec = 5.0;
    l1.animations.path_morph.direction = AnimationDirection::Alternate;

    let mut l2 = PatternLayer::new_polka(2, "Drifting Luminescent Dots");
    l2.primary_color = [111, 66, 193, 220];
    l2.secondary_color = [0, 240, 255, 220];
    if let GeneratorKind::PolkaHalftone(ref mut params) = l2.generator {
        params.cols = 12;
        params.rows = 12;
        params.shape = MotifShape::Circle;
        params.base_radius_ratio = 0.15;
    }
    l2.animations.drift.enabled = true;
    l2.animations.drift.duration_sec = 10.0;
    l2.animations.drift.tiles_dx = 1;
    l2.animations.drift.tiles_dy = 1;

    p.layers.push(l1);
    p.layers.push(l2);
    p
}

fn preset_magnetic_flow() -> Project {
    let mut p = Project {
        name: "Magnetic Vector Field".to_string(),
        tile_width: 240.0,
        tile_height: 240.0,
        background_color: [20, 22, 38, 255],
        layers: Vec::new(),
        selected_layer_idx: 1,
        next_layer_id: 3,
    };

    let mut l1 = PatternLayer::new_isometric(1, "Hex Background Grid");
    l1.primary_color = [56, 189, 248, 80];
    l1.secondary_color = [168, 85, 247, 80];
    l1.filled = false;
    l1.stroked = true;
    l1.stroke_width = 1.0;
    if let GeneratorKind::IsometricHexGrid(ref mut params) = l1.generator {
        params.mode = TessellationMode::HexHoneycomb;
        params.cols = 6;
        params.rows = 6;
    }

    let mut l2 = PatternLayer::new_flow_field(2, "Oscillating Vector Dashes");
    l2.primary_color = [236, 72, 153, 255];
    l2.secondary_color = [168, 85, 247, 255];
    l2.stroke_width = 3.0;
    if let GeneratorKind::FlowFieldDashes(ref mut params) = l2.generator {
        params.cols = 8;
        params.rows = 8;
        params.angle_amplitude_deg = 120.0;
    }
    l2.animations.transform.enabled = true;
    l2.animations.transform.duration_sec = 4.0;
    l2.animations.transform.direction = AnimationDirection::Alternate;
    l2.animations.transform.stops = vec![
        KeyframeStop {
            offset_pct: 0.0,
            value: TransformState { scale_x: 0.7, scale_y: 0.7, rotate_deg: -30.0, skew_x_deg: 0.0 },
        },
        KeyframeStop {
            offset_pct: 50.0,
            value: TransformState { scale_x: 1.3, scale_y: 1.3, rotate_deg: 30.0, skew_x_deg: 0.0 },
        },
        KeyframeStop {
            offset_pct: 100.0,
            value: TransformState { scale_x: 0.7, scale_y: 0.7, rotate_deg: -30.0, skew_x_deg: 0.0 },
        },
    ];

    p.layers.push(l1);
    p.layers.push(l2);
    p
}

fn preset_retro_halftone() -> Project {
    let mut p = Project {
        name: "Retro Halftone Spin".to_string(),
        tile_width: 240.0,
        tile_height: 240.0,
        background_color: [24, 20, 15, 255],
        layers: Vec::new(),
        selected_layer_idx: 0,
        next_layer_id: 2,
    };

    let mut l1 = PatternLayer::new_polka(1, "Staggered Stars");
    l1.primary_color = [250, 204, 21, 255];
    l1.secondary_color = [249, 115, 22, 255];
    l1.filled = true;
    l1.stroked = true;
    l1.stroke_width = 1.5;
    if let GeneratorKind::PolkaHalftone(ref mut params) = l1.generator {
        params.cols = 6;
        params.rows = 6;
        params.shape = MotifShape::Star;
        params.stagger_odd_rows = true;
        params.base_radius_ratio = 0.35;
    }
    l1.animations.transform.enabled = true;
    l1.animations.transform.duration_sec = 5.0;
    l1.animations.transform.phase_freq_x = 2;
    l1.animations.transform.phase_freq_y = 1;
    l1.animations.transform.stops = vec![
        KeyframeStop {
            offset_pct: 0.0,
            value: TransformState { scale_x: 0.6, scale_y: 0.6, rotate_deg: 0.0, skew_x_deg: 0.0 },
        },
        KeyframeStop {
            offset_pct: 50.0,
            value: TransformState { scale_x: 1.3, scale_y: 1.3, rotate_deg: 180.0, skew_x_deg: 0.0 },
        },
        KeyframeStop {
            offset_pct: 100.0,
            value: TransformState { scale_x: 0.6, scale_y: 0.6, rotate_deg: 360.0, skew_x_deg: 0.0 },
        },
    ];

    p.layers.push(l1);
    p
}
