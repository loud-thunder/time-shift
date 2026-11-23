use super::animation::LayerAnimations;
use super::generators::{
    ConcentricRipplesParams, FlowFieldDashesParams, GeneratorKind, IsometricHexGridParams,
    PolkaHalftoneParams, TruchetTilesParams, WavesSineStripesParams,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PatternLayer {
    pub id: u64,
    pub name: String,
    pub visible: bool,
    pub opacity: f32, // 0.0 .. 1.0
    pub primary_color: [u8; 4],
    pub secondary_color: [u8; 4],
    pub stroke_width: f32,
    pub filled: bool,
    pub stroked: bool,
    pub generator: GeneratorKind,
    pub animations: LayerAnimations,
}

impl PatternLayer {
    pub fn new_polka(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            visible: true,
            opacity: 1.0,
            primary_color: [14, 82, 196, 255],
            secondary_color: [220, 38, 38, 255],
            stroke_width: 2.0,
            filled: true,
            stroked: false,
            generator: GeneratorKind::PolkaHalftone(PolkaHalftoneParams::default()),
            animations: LayerAnimations::default(),
        }
    }

    pub fn new_waves(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            visible: true,
            opacity: 0.9,
            primary_color: [15, 23, 42, 255],
            secondary_color: [14, 165, 233, 255],
            stroke_width: 3.0,
            filled: false,
            stroked: true,
            generator: GeneratorKind::WavesSineStripes(WavesSineStripesParams::default()),
            animations: LayerAnimations::default(),
        }
    }

    pub fn new_truchet(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            visible: true,
            opacity: 1.0,
            primary_color: [20, 83, 45, 255],
            secondary_color: [2, 132, 199, 255],
            stroke_width: 4.0,
            filled: false,
            stroked: true,
            generator: GeneratorKind::TruchetTiles(TruchetTilesParams::default()),
            animations: LayerAnimations::default(),
        }
    }

    pub fn new_isometric(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            visible: true,
            opacity: 1.0,
            primary_color: [24, 24, 27, 255],
            secondary_color: [99, 102, 241, 255],
            stroke_width: 1.5,
            filled: true,
            stroked: true,
            generator: GeneratorKind::IsometricHexGrid(IsometricHexGridParams::default()),
            animations: LayerAnimations::default(),
        }
    }

    pub fn new_ripples(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            visible: true,
            opacity: 0.8,
            primary_color: [15, 44, 89, 255],
            secondary_color: [13, 148, 136, 255],
            stroke_width: 2.5,
            filled: false,
            stroked: true,
            generator: GeneratorKind::ConcentricRipples(ConcentricRipplesParams::default()),
            animations: LayerAnimations::default(),
        }
    }

    pub fn new_flow_field(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            visible: true,
            opacity: 0.85,
            primary_color: [194, 65, 12, 255],
            secondary_color: [180, 83, 9, 255],
            stroke_width: 2.0,
            filled: false,
            stroked: true,
            generator: GeneratorKind::FlowFieldDashes(FlowFieldDashesParams::default()),
            animations: LayerAnimations::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub name: String,
    pub tile_width: f32,
    pub tile_height: f32,
    pub background_color: [u8; 4],
    pub layers: Vec<PatternLayer>,
    pub selected_layer_idx: usize,
    pub next_layer_id: u64,
}

impl Default for Project {
    fn default() -> Self {
        let mut proj = Self {
            name: "Time-Shift Pattern".to_string(),
            tile_width: 240.0,
            tile_height: 240.0,
            background_color: [255, 255, 255, 255],
            layers: Vec::new(),
            selected_layer_idx: 0,
            next_layer_id: 1,
        };

        // Add 2 default layers with subtle animations
        let mut l1 = PatternLayer::new_polka(1, "Halftone Grid");
        l1.animations.transform.enabled = true;
        l1.animations.transform.duration_sec = 4.0;
        proj.layers.push(l1);

        let mut l2 = PatternLayer::new_waves(2, "Sine Waves");
        l2.animations.path_morph.enabled = true;
        l2.animations.path_morph.duration_sec = 5.0;
        proj.layers.push(l2);

        proj.next_layer_id = 3;
        proj
    }
}

impl Project {
    pub fn selected_layer(&self) -> Option<&PatternLayer> {
        self.layers.get(self.selected_layer_idx)
    }

    pub fn selected_layer_mut(&mut self) -> Option<&mut PatternLayer> {
        self.layers.get_mut(self.selected_layer_idx)
    }
}
