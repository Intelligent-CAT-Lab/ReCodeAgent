#[derive(Clone)]
pub struct Particle {
    pub x_coordinate: f64,
    pub y_coordinate: f64,
    pub radius: f64,
    pub next: Option<Box<Particle>>,
    pub idx: i32,
}
#[derive(Clone, Copy)]
pub struct ParticleProperties {
    pub mass: f64,
    pub kn: f64,
    pub ks: f64,
}
#[derive(Clone, Copy)]
pub struct Vector {
    pub x_component: f64,
    pub y_component: f64,
}
#[derive(Clone, Copy)]
pub struct Contact {
    pub p1_idx: usize,
    pub p2_idx: usize,
    pub overlap: f64,
}