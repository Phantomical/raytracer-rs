#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct RaymarchOptions {
    /// The maximum distance out to which a ray will be traced
    pub max_distance: f64,
    /// The minumum distance that a ray must traverse before intersecting
    pub min_distance: f64,
    /// The distance that the ray must be within to be considered
    /// to have hit the surface
    pub intersect_distance: f64,
}

impl Default for RaymarchOptions {
    fn default() -> Self {
        return Self {
            max_distance: 1.0e6,
            min_distance: 1.0e-6,
            intersect_distance: 1.0e-8,
        };
    }
}
