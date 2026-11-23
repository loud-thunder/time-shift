#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotifShape {
    Circle,
    Square,
    Cross,
    Star,
    Diamond,
}

impl MotifShape {
    pub fn all() -> &'static [MotifShape] {
        &[
            MotifShape::Circle,
            MotifShape::Square,
            MotifShape::Cross,
            MotifShape::Star,
            MotifShape::Diamond,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            MotifShape::Circle => "Circle",
            MotifShape::Square => "Square",
            MotifShape::Cross => "Cross",
            MotifShape::Star => "Star",
            MotifShape::Diamond => "Diamond",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PolkaHalftoneParams {
    pub cols: u32,
    pub rows: u32,
    pub shape: MotifShape,
    pub base_radius_ratio: f32, // fraction of cell size (0.05 .. 0.5)
    pub stagger_odd_rows: bool,
    pub size_modulation_amp: f32, // 0.0 .. 0.5
    pub size_freq_x: i32,
    pub size_freq_y: i32,
    pub base_rotation_deg: f32,
}

impl Default for PolkaHalftoneParams {
    fn default() -> Self {
        Self {
            cols: 8,
            rows: 8,
            shape: MotifShape::Circle,
            base_radius_ratio: 0.3,
            stagger_odd_rows: false,
            size_modulation_amp: 0.0,
            size_freq_x: 1,
            size_freq_y: 1,
            base_rotation_deg: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveOrientation {
    Horizontal,
    Vertical,
    Diagonal,
}

impl WaveOrientation {
    pub fn all() -> &'static [WaveOrientation] {
        &[
            WaveOrientation::Horizontal,
            WaveOrientation::Vertical,
            WaveOrientation::Diagonal,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            WaveOrientation::Horizontal => "Horizontal",
            WaveOrientation::Vertical => "Vertical",
            WaveOrientation::Diagonal => "Diagonal",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WavesSineStripesParams {
    pub band_count: u32,
    pub orientation: WaveOrientation,
    pub amplitude: f32,
    pub frequency: u32, // integer periods per tile for seamlessness
    pub harmonic_ratio: f32,
    pub ribbon_thickness: f32,
    pub samples_per_wave: u32,
}

impl Default for WavesSineStripesParams {
    fn default() -> Self {
        Self {
            band_count: 6,
            orientation: WaveOrientation::Horizontal,
            amplitude: 15.0,
            frequency: 2,
            harmonic_ratio: 0.0,
            ribbon_thickness: 8.0,
            samples_per_wave: 32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruchetVariant {
    QuarterCircles,
    DiagonalLines,
    TriangleSubdivision,
}

impl TruchetVariant {
    pub fn all() -> &'static [TruchetVariant] {
        &[
            TruchetVariant::QuarterCircles,
            TruchetVariant::DiagonalLines,
            TruchetVariant::TriangleSubdivision,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            TruchetVariant::QuarterCircles => "Quarter Circles",
            TruchetVariant::DiagonalLines => "Diagonal Lines",
            TruchetVariant::TriangleSubdivision => "Triangles",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruchetLayout {
    HashSeed,
    Checkerboard,
    DiagonalWave,
}

impl TruchetLayout {
    pub fn all() -> &'static [TruchetLayout] {
        &[
            TruchetLayout::HashSeed,
            TruchetLayout::Checkerboard,
            TruchetLayout::DiagonalWave,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            TruchetLayout::HashSeed => "Hash Seed",
            TruchetLayout::Checkerboard => "Checkerboard",
            TruchetLayout::DiagonalWave => "Diagonal Wave",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TruchetTilesParams {
    pub cols: u32,
    pub rows: u32,
    pub variant: TruchetVariant,
    pub layout: TruchetLayout,
    pub seed: u64,
    pub double_arc: bool,
    pub corner_radius_ratio: f32, // 0.2 .. 0.8
}

impl Default for TruchetTilesParams {
    fn default() -> Self {
        Self {
            cols: 6,
            rows: 6,
            variant: TruchetVariant::QuarterCircles,
            layout: TruchetLayout::HashSeed,
            seed: 42,
            double_arc: true,
            corner_radius_ratio: 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TessellationMode {
    IsometricCubes,
    HexHoneycomb,
    TriangularMesh,
}

impl TessellationMode {
    pub fn all() -> &'static [TessellationMode] {
        &[
            TessellationMode::IsometricCubes,
            TessellationMode::HexHoneycomb,
            TessellationMode::TriangularMesh,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            TessellationMode::IsometricCubes => "Isometric Cubes",
            TessellationMode::HexHoneycomb => "Hex Honeycomb",
            TessellationMode::TriangularMesh => "Triangular Mesh",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IsometricHexGridParams {
    pub cols: u32,
    pub rows: u32,
    pub mode: TessellationMode,
    pub inset_ratio: f32, // 0.0 .. 0.3
    pub face_shading: bool,
}

impl Default for IsometricHexGridParams {
    fn default() -> Self {
        Self {
            cols: 5,
            rows: 5,
            mode: TessellationMode::IsometricCubes,
            inset_ratio: 0.05,
            face_shading: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConcentricRipplesParams {
    pub center_count_x: u32,
    pub center_count_y: u32,
    pub rings_per_center: u32,
    pub max_radius_ratio: f32, // 0.2 .. 1.2
    pub ring_spacing_exp: f32, // 1.0 = linear, >1.0 = accelerating
    pub jitter_amount: f32,
    pub seed: u64,
}

impl Default for ConcentricRipplesParams {
    fn default() -> Self {
        Self {
            center_count_x: 2,
            center_count_y: 2,
            rings_per_center: 5,
            max_radius_ratio: 0.7,
            ring_spacing_exp: 1.0,
            jitter_amount: 0.0,
            seed: 777,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowFieldDashesParams {
    pub cols: u32,
    pub rows: u32,
    pub dash_length_ratio: f32, // fraction of cell size
    pub freq_x: i32,
    pub freq_y: i32,
    pub angle_amplitude_deg: f32,
    pub curl_phase: f32,
    pub arrow_heads: bool,
    pub rounded_caps: bool,
}

impl Default for FlowFieldDashesParams {
    fn default() -> Self {
        Self {
            cols: 8,
            rows: 8,
            dash_length_ratio: 0.6,
            freq_x: 1,
            freq_y: 1,
            angle_amplitude_deg: 90.0,
            curl_phase: 0.0,
            arrow_heads: false,
            rounded_caps: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GeneratorKind {
    PolkaHalftone(PolkaHalftoneParams),
    WavesSineStripes(WavesSineStripesParams),
    TruchetTiles(TruchetTilesParams),
    IsometricHexGrid(IsometricHexGridParams),
    ConcentricRipples(ConcentricRipplesParams),
    FlowFieldDashes(FlowFieldDashesParams),
}

impl GeneratorKind {
    pub fn name(&self) -> &'static str {
        match self {
            GeneratorKind::PolkaHalftone(_) => "Polka Dots / Halftone Grid",
            GeneratorKind::WavesSineStripes(_) => "Waves / Sine Stripes",
            GeneratorKind::TruchetTiles(_) => "Truchet Tiles",
            GeneratorKind::IsometricHexGrid(_) => "Isometric / Hex Grid",
            GeneratorKind::ConcentricRipples(_) => "Concentric Ripples",
            GeneratorKind::FlowFieldDashes(_) => "Flow Field Dashes",
        }
    }

    pub fn type_id(&self) -> u32 {
        match self {
            GeneratorKind::PolkaHalftone(_) => 0,
            GeneratorKind::WavesSineStripes(_) => 1,
            GeneratorKind::TruchetTiles(_) => 2,
            GeneratorKind::IsometricHexGrid(_) => 3,
            GeneratorKind::ConcentricRipples(_) => 4,
            GeneratorKind::FlowFieldDashes(_) => 5,
        }
    }
}
