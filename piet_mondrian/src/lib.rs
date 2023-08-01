mod algorithm;
pub use algorithm::Rectangle;
pub use rand::{self, rngs::StdRng, thread_rng, SeedableRng};
pub use svg::save;

pub struct Mondrian {
    /// Canvas size
    pub size: f32,
    /// Graphic width
    pub width: f32,
    /// Available colors
    pub colors: Vec<String>,
    /// Color Weights
    pub color_weights: Vec<u32>,
    /// Grid size
    pub grid_round: f32,
    /// Line width
    pub line_width: f32,
    /// Random seed
    pub rng: StdRng,
}

impl Default for Mondrian {
    fn default() -> Self {
        let colors = vec!["#D40920", "#1356A2", "#F7D842", "#F2F5F1"];
        Self {
            grid_round: 0.05,
            line_width: 1.0,
            colors: colors.iter().map(|&s| String::from(s)).collect(),
            color_weights: vec![1, 1, 1, 9],
            size: 100.0,
            width: 1.6,
            rng: StdRng::seed_from_u64(42),
        }
    }
}
