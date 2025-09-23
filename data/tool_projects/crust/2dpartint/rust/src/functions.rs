use crate::data;
pub fn compute_velocity(
    dt: f64,
    particle_index: usize,
    accelerations: &[data::Vector],
    velocities: &mut [data::Vector],
) {
    unimplemented!()
}
pub fn compute_forces(
    dt: f64,
    particles_size: usize,
    contacts_size: usize,
    particles: &[data::Particle],
    properties: &[data::ParticleProperties],
    contacts: &[data::Contact],
    velocities: &[data::Vector],
    normal_forces: &mut [f64],
    tangent_forces: &mut [f64],
    forces: &mut [data::Vector],
) {
    unimplemented!()
}
pub fn compute_distance(p1: &data::Particle, p2: &data::Particle) -> f64 {
    unimplemented!()
}
pub fn apply_gravity(
    size: usize,
    particles_properties: &[data::ParticleProperties],
    forces: &mut [data::Vector],
) {
    unimplemented!()
}
pub fn compute_overlap(p1: &data::Particle, p2: &data::Particle) -> f64 {
    unimplemented!()
}
pub fn fix_displacement(
    particle_index: usize,
    velocities: &mut [data::Vector],
    particles: &mut [data::Particle],
) {
    unimplemented!()
}
pub fn compute_acceleration(
    particle_index: usize,
    particles_properties: &[data::ParticleProperties],
    forces: &[data::Vector],
    accelerations: &mut [data::Vector],
) {
    unimplemented!()
}
pub fn compute_displacement(
    dt: f64,
    particle_index: usize,
    velocities: &[data::Vector],
    displacements: &mut [data::Vector],
) {
    unimplemented!()
}
pub fn size_triangular_matrix(n: usize) -> usize {
    unimplemented!()
}
pub fn displace_particle(
    particle_index: usize,
    displacements: &[data::Vector],
    particles: &mut [data::Particle],
) {
    unimplemented!()
}
pub fn collide_two_particles(
    dt: f64,
    distance: f64, 
    p1: &data::Particle,
    p2: &data::Particle,
    velocity_p1: &data::Vector,
    velocity_p2: &data::Vector,
    properties_p1: &data::ParticleProperties,
    properties_p2: &data::ParticleProperties,
    previous_normal: f64, 
    previous_tangent: f64, 
    forces_p2: &data::Vector,
) {
    unimplemented!()
}