use crate::model::animation::{
    AnimationDirection, ColorOpacityState, KeyframeStop, TransformState,
};
use crate::model::generators::{
    GeneratorKind, MotifShape, TessellationMode, TruchetLayout,
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
            name: "Minimalist Ink",
            background: [255, 255, 255, 255],
            primary: [24, 24, 27, 255],
            secondary: [100, 116, 139, 255],
            accent: [225, 29, 72, 255],
        },
        ColorPalette {
            name: "Nordic Clean",
            background: [250, 250, 252, 255],
            primary: [15, 23, 42, 255],
            secondary: [71, 85, 105, 255],
            accent: [14, 165, 233, 255],
        },
        ColorPalette {
            name: "Bauhaus Studio",
            background: [255, 255, 255, 255],
            primary: [14, 82, 196, 255],
            secondary: [220, 38, 38, 255],
            accent: [245, 158, 11, 255],
        },
        ColorPalette {
            name: "Botanical Garden",
            background: [253, 253, 248, 255],
            primary: [20, 83, 45, 255],
            secondary: [46, 125, 50, 255],
            accent: [217, 119, 6, 255],
        },
        ColorPalette {
            name: "Ocean Breeze",
            background: [255, 255, 255, 255],
            primary: [15, 44, 89, 255],
            secondary: [13, 148, 136, 255],
            accent: [244, 63, 94, 255],
        },
        ColorPalette {
            name: "Solarized Light",
            background: [253, 246, 227, 255],
            primary: [101, 123, 131, 255],
            secondary: [38, 139, 210, 255],
            accent: [211, 54, 130, 255],
        },
        ColorPalette {
            name: "Architectural Drafting",
            background: [255, 255, 255, 255],
            primary: [2, 132, 199, 255],
            secondary: [30, 41, 59, 255],
            accent: [234, 88, 12, 255],
        },
        ColorPalette {
            name: "Warm Terracotta Light",
            background: [255, 251, 247, 255],
            primary: [154, 52, 18, 255],
            secondary: [194, 65, 12, 255],
            accent: [180, 83, 9, 255],
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
            description: "Pulsing 3D isometric cubes paired with animated flow field arrows",
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
            description: "Toroidal concentric ripples expanding over luminescent dot lattices",
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
        background_color: [255, 255, 255, 255],
        layers: Vec::new(),
        selected_layer_idx: 0,
        next_layer_id: 3,
    };

    let mut l1 = PatternLayer::new_isometric(1, "Isometric Cubes");
    l1.primary_color = [14, 82, 196, 255];
    l1.secondary_color = [220, 38, 38, 255];
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

    let mut l2 = PatternLayer::new_flow_field(2, "Flow Field Arrows");
    l2.primary_color = [15, 23, 42, 255];
    l2.secondary_color = [2, 132, 199, 255];
    l2.stroke_width = 2.0;
    l2.opacity = 0.85;
    if let GeneratorKind::FlowFieldDashes(ref mut params) = l2.generator {
        params.arrow_heads = true;
        params.cols = 6;
        params.rows = 6;
    }
    l2.animations.transform.enabled = true;
    l2.animations.transform.duration_sec = 4.0;
    l2.animations.transform.phase_freq_x = 1;
    l2.animations.transform.phase_freq_y = 1;

    p.layers.push(l1);
    p.layers.push(l2);
    p
}

fn preset_synthwave_waves() -> Project {
    let mut p = Project {
        name: "Synthwave Ocean Ripples".to_string(),
        tile_width: 240.0,
        tile_height: 240.0,
        background_color: [254, 250, 255, 255],
        layers: Vec::new(),
        selected_layer_idx: 1,
        next_layer_id: 3,
    };

    let mut l1 = PatternLayer::new_polka(1, "Halftone Horizon");
    l1.primary_color = [219, 39, 119, 160];
    l1.secondary_color = [147, 51, 234, 160];
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
    l2.primary_color = [79, 70, 229, 255];
    l2.secondary_color = [225, 29, 72, 255];
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
        background_color: [252, 252, 253, 255],
        layers: Vec::new(),
        selected_layer_idx: 0,
        next_layer_id: 3,
    };

    let mut l1 = PatternLayer::new_truchet(1, "Quarter Circles");
    l1.primary_color = [15, 118, 110, 255];
    l1.secondary_color = [29, 78, 216, 255];
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
            value: ColorOpacityState { color: [15, 118, 110, 255], opacity: 1.0 },
        },
        KeyframeStop {
            offset_pct: 50.0,
            value: ColorOpacityState { color: [180, 83, 9, 255], opacity: 1.0 },
        },
        KeyframeStop {
            offset_pct: 100.0,
            value: ColorOpacityState { color: [15, 118, 110, 255], opacity: 1.0 },
        },
    ];

    let mut l2 = PatternLayer::new_polka(2, "Cross Grid");
    l2.primary_color = [30, 41, 59, 220];
    l2.secondary_color = [100, 116, 139, 220];
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
        background_color: [253, 253, 248, 255],
        layers: Vec::new(),
        selected_layer_idx: 0,
        next_layer_id: 3,
    };

    let mut l1 = PatternLayer::new_ripples(1, "Toroidal Water Rings");
    l1.primary_color = [20, 83, 45, 255];
    l1.secondary_color = [13, 148, 136, 255];
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

    let mut l2 = PatternLayer::new_polka(2, "Luminescent Dot Field");
    l2.primary_color = [30, 64, 175, 200];
    l2.secondary_color = [217, 119, 6, 200];
    if let GeneratorKind::PolkaHalftone(ref mut params) = l2.generator {
        params.cols = 12;
        params.rows = 12;
        params.shape = MotifShape::Circle;
        params.base_radius_ratio = 0.15;
    }
    l2.animations.color_opacity.enabled = true;
    l2.animations.color_opacity.duration_sec = 4.0;
    l2.animations.color_opacity.direction = AnimationDirection::Alternate;

    p.layers.push(l1);
    p.layers.push(l2);
    p
}

fn preset_magnetic_flow() -> Project {
    let mut p = Project {
        name: "Magnetic Vector Field".to_string(),
        tile_width: 240.0,
        tile_height: 240.0,
        background_color: [255, 255, 255, 255],
        layers: Vec::new(),
        selected_layer_idx: 1,
        next_layer_id: 3,
    };

    let mut l1 = PatternLayer::new_isometric(1, "Hex Background Grid");
    l1.primary_color = [100, 116, 139, 100];
    l1.secondary_color = [148, 163, 184, 100];
    l1.filled = false;
    l1.stroked = true;
    l1.stroke_width = 1.0;
    if let GeneratorKind::IsometricHexGrid(ref mut params) = l1.generator {
        params.mode = TessellationMode::HexHoneycomb;
        params.cols = 6;
        params.rows = 6;
    }

    let mut l2 = PatternLayer::new_flow_field(2, "Oscillating Vector Dashes");
    l2.primary_color = [225, 29, 72, 255];
    l2.secondary_color = [99, 102, 241, 255];
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
        background_color: [255, 251, 245, 255],
        layers: Vec::new(),
        selected_layer_idx: 0,
        next_layer_id: 2,
    };

    let mut l1 = PatternLayer::new_polka(1, "Staggered Stars");
    l1.primary_color = [194, 65, 12, 255];
    l1.secondary_color = [217, 119, 6, 255];
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
