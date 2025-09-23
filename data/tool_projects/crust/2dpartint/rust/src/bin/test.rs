use twoDPartInt::data::{Particle, ParticleProperties, Vector, Contact};
use twoDPartInt::functions::{compute_forces, compute_acceleration, compute_velocity, displace_particle};
const TOLERANCE: f64 = 0.00005;
fn assert_approx_eq(result: f64, expected: f64, test_description: &str) {
if (result - expected).abs() > TOLERANCE {
panic!("Test: '{}' FAILED! {} did not equal {}", test_description, result, expected);
} else {
println!("Test: '{}' PASSED!", test_description);
}
}
#[test]
fn test_compute_forces_one_contact() {
let size = 2;
let contacts_size = 1;
let properties = [
ParticleProperties { mass: 0.049, kn: 247435.829652697, ks: 19033.5253578998 },
ParticleProperties { mass: 0.049, kn: 247435.829652697, ks: 19033.5253578998 },
];
let particles = [
Particle { x_coordinate: 24.9999428493601, y_coordinate: 25.0, radius: 50.0, next: None, idx: 0 },
Particle { x_coordinate: 24.7762980060664, y_coordinate: 74.6253249615872, radius: 50.0, next: None, idx: 1 },
];
let velocities = [
Vector { x_component: -0.00159733057834793, y_component: 0.0 },
Vector { x_component: -4.64544001147225, y_component: -6.36971454859269 },
];
let mut normal_forces = vec![0.0, 53.2634277334533, 53.2634277334533, 0.0];
let mut tangent_forces = vec![0.0, 2.0526062679878, 2.0526062679878, 0.0];
let dt = 0.000025;
let contacts = [Contact { p1_idx: 0, p2_idx: 1, overlap: 42.9554 }];
let mut resultant_forces = vec![Vector { x_component: 0.0, y_component: 0.0 }; size];
compute_forces(dt, size, contacts_size, &particles, &properties, &contacts, &velocities, &mut normal_forces, &mut tangent_forces, &mut resultant_forces);
assert_approx_eq(resultant_forces[0].x_component, -3.858892943, "test_compute_forces_one_contact - resultant_forces[P1].x_component");
assert_approx_eq(resultant_forces[0].y_component, -93.03497946, "test_compute_forces_one_contact - resultant_forces[P1].y_component");
assert_approx_eq(normal_forces[2], 92.53595901628790, "test_compute_forces_one_contact - normal_forces[P1 <- P2]");
assert_approx_eq(tangent_forces[2], 4.275960624, "test_compute_forces_one_contact - tangent_forces[P1 <- P2]");
assert_approx_eq(resultant_forces[1].x_component, 3.858892943, "test_compute_forces_one_contact - resultant_forces[P2].x_component");
assert_approx_eq(resultant_forces[1].y_component, 92.07359946, "test_compute_forces_one_contact - resultant_forces[P2].y_component");
assert_approx_eq(normal_forces[1], 92.53595901628790, "test_compute_forces_one_contact - normal_forces[P2 <- P1]");
assert_approx_eq(tangent_forces[1], 4.275960624, "test_compute_forces_one_contact - tangent_forces[P2 <- P1]");
}
#[test]
fn test_compute_forces_multiple_contacts() {
let size = 9;
let contacts_size = 3;
let properties = [
ParticleProperties { mass: 0.049, kn: 247435.829652697, ks: 19033.5253578998 }; 9
];
let particles = [
Particle { x_coordinate: 24.9996682317, y_coordinate: 25.0, radius: 50.0, next: None, idx: 0 },
Particle { x_coordinate: 24.3329247490, y_coordinate: 74.1788246441, radius: 50.0, next: None, idx: 1 },
Particle { x_coordinate: 20.8181703235, y_coordinate: 122.9449251651, radius: 50.0, next: None, idx: 2 },
Particle { x_coordinate: 16.8606509998, y_coordinate: 172.4918861911, radius: 50.0, next: None, idx: 3 },
Particle { x_coordinate: 75.0003317683, y_coordinate: 25.0, radius: 50.0, next: None, idx: 4 },
Particle { x_coordinate: 75.6670752510, y_coordinate: 74.1788246441, radius: 50.0, next: None, idx: 5 },
Particle { x_coordinate: 79.1818296765, y_coordinate: 122.9449251651, radius: 50.0, next: None, idx: 6 },
Particle { x_coordinate: 83.1393490002, y_coordinate: 172.4918861911, radius: 50.0, next: None, idx: 7 },
Particle { x_coordinate: 50.0, y_coordinate: 75.7713467697, radius: 50.0, next: None, idx: 8 },
];
let velocities = [
Vector { x_component: -0.00728767793878, y_component: 0.0 },
Vector { x_component: -10.44576385514790, y_component: -9.61529833378254 },
Vector { x_component: -22.72352179173930, y_component: -6.11340796404052 },
Vector { x_component: -22.22735951506210, y_component: -4.22438903830371 },
Vector { x_component: 0.00728767793878, y_component: 0.0 },
Vector { x_component: 10.44576385514790, y_component: -9.61529833378254 },
Vector { x_component: 22.72352179173930, y_component: -6.11340796404052 },
Vector { x_component: 22.22735951506210, y_component: -4.22438903830371 },
Vector { x_component: -0.00000000000001, y_component: -259.77483981875700 },
];
let mut normal_forces = vec![
0.0, 143.165149, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
201.763569, 0.0, 297.810731, 0.0, 0.0, 0.0, 0.0, 0.0, 6205.981479,
0.0, 297.810731, 0.0, 81.040751, 0.0, 0.0, 0.0, 0.0, 0.0,
0.0, 0.0, 81.040751, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
0.0, 0.0, 0.0, 0.0, 0.0, 143.165149, 0.0, 0.0, 0.0,
0.0, 0.0, 0.0, 0.0, 143.165149, 0.0, 297.810731, 0.0, 6205.981479,
0.0, 0.0, 0.0, 0.0, 0.0, 297.810731, 0.0, 81.040751, 0.0,
0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 81.040751, 0.0, 0.0,
0.0, 6205.981479, 0.0, 0.0, 0.0, 6205.981479, 0.0, 0.0, 0.0,
];
let mut tangent_forces = vec![
0.0, 7.77476, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
12.803354, 0.0, 61.408871, 0.0, 0.0, 0.0, 0.0, 0.0, -553.7956,
0.0, 61.40887, 0.0, 46.788899128, 0.0, 0.0, 0.0, 0.0, 0.0,
0.0, 0.0, 46.788899, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
0.0, 0.0, 0.0, 0.0, 0.0, -7.77476, 0.0, 0.0, 0.0,
0.0, 0.0, 0.0, 0.0, -7.7747607, 0.0, -61.408871, 0.0, 553.7956,
0.0, 0.0, 0.0, 0.0, 0.0, -61.4089, 0.0, -46.78890, 0.0,
0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -46.788899, 0.0, 0.0,
0.0, -553.79557, 0.0, 0.0, 0.0, 553.7956, 0.0, 0.0, 0.0,
];
let dt = 0.000025;
let contacts = [
Contact { p1_idx: 1, p2_idx: 0, overlap: 49.18334413 },
Contact { p1_idx: 1, p2_idx: 2, overlap: 48.89259718 },
Contact { p1_idx: 1, p2_idx: 8, overlap: 25.71643207 },
];
let mut resultant_forces = vec![Vector { x_component: 0.0, y_component: 0.0 }; size];
compute_forces(dt, size, contacts_size, &particles, &properties, &contacts, &velocities, &mut normal_forces, &mut tangent_forces, &mut resultant_forces);
assert_approx_eq(resultant_forces[1].x_component, -6221.088058, "test_compute_forces_multiple_contacts - resultant_forces[P2].x_component");
assert_approx_eq(resultant_forces[1].y_component, -1131.303630, "test_compute_forces_multiple_contacts - resultant_forces[P2].y_component");
assert_approx_eq(normal_forces[1], 201.7636, "test_compute_forces_multiple_contacts - normal_forces[P2 <- P1]");
assert_approx_eq(tangent_forces[1], 12.80335446, "test_compute_forces_multiple_contacts - tangent_forces[P2 <- P1]");
assert_approx_eq(normal_forces[19], 270.7447, "test_compute_forces_multiple_contacts - normal_forces[P2 <- P3]");
assert_approx_eq(tangent_forces[19], 67.11619734, "test_compute_forces_multiple_contacts - tangent_forces[P2 <- P3]");
assert_approx_eq(normal_forces[73], 6237.3175, "test_compute_forces_multiple_contacts - normal_forces[P2 <- P9]");
assert_approx_eq(tangent_forces[73], -672.91035983, "test_compute_forces_multiple_contacts - tangent_forces[P2 <- P9]");
assert_approx_eq(resultant_forces[0].x_component, -14.30076729, "test_compute_forces_multiple_contacts - resultant_forces[P1].x_component");
assert_approx_eq(resultant_forces[0].y_component, -261.0604892, "test_compute_forces_multiple_contacts - resultant_forces[P1].y_component");
assert_approx_eq(normal_forces[9], 260.3620, "test_compute_forces_multiple_contacts - normal_forces[P1 <- P2]");
assert_approx_eq(tangent_forces[9], 17.83194748, "test_compute_forces_multiple_contacts - tangent_forces[P1 <- P2]");
assert_approx_eq(resultant_forces[2].x_component, 47.47945664, "test_compute_forces_multiple_contacts - resultant_forces[P3].x_component");
assert_approx_eq(resultant_forces[2].y_component, 274.3883165, "test_compute_forces_multiple_contacts - resultant_forces[P3].y_component");
assert_approx_eq(normal_forces[11], 270.7447, "test_compute_forces_multiple_contacts - normal_forces[P3 <- P2]");
assert_approx_eq(tangent_forces[11], 67.11619348, "test_compute_forces_multiple_contacts - tangent_forces[P3 <- P2]");
assert_approx_eq(resultant_forces[8].x_component, 6183.675611, "test_compute_forces_multiple_contacts - resultant_forces[P9].x_component");
assert_approx_eq(resultant_forces[8].y_component, 1057.391839, "test_compute_forces_multiple_contacts - resultant_forces[P9].y_component");
assert_approx_eq(normal_forces[17], 6237.3175, "test_compute_forces_multiple_contacts - normal_forces[P9 <- P2]");
assert_approx_eq(tangent_forces[17], -672.91036059, "test_compute_forces_multiple_contacts - tangent_forces[P9 <- P2]");
}
#[test]
fn test_compute_acceleration_one_element() {
let size = 1;
let forces = [Vector { x_component: 30.0, y_component: 30.0 }];
let particle_properties = [ParticleProperties { mass: 3.0, kn: 0.0, ks: 0.0 }];
let mut accelerations = vec![Vector { x_component: 0.0, y_component: 0.0 }; size];
compute_acceleration(0, &particle_properties, &forces, &mut accelerations);
assert_approx_eq(accelerations[0].x_component, 10.0, "test_compute_acceleration_one_element - x_component");
assert_approx_eq(accelerations[0].y_component, 10.0, "test_compute_acceleration_one_element - y_component");
}
#[test]
fn test_compute_acceleration_multiple_elements() {
let size = 3;
let forces = [
Vector { x_component: -12.58, y_component: -15.896 },
Vector { x_component: 13.945, y_component: -200.826 },
Vector { x_component: -543.62, y_component: -0.62 },
];
let particle_properties = [
ParticleProperties { mass: 0.367, kn: 0.0, ks: 0.0 },
ParticleProperties { mass: 3.967, kn: 0.0, ks: 0.0 },
ParticleProperties { mass: 0.52, kn: 0.0, ks: 0.0 },
];
let mut accelerations = vec![Vector { x_component: 0.0, y_component: 0.0 }; size];
for i in 0..size {
compute_acceleration(i, &particle_properties, &forces, &mut accelerations);
}
let expected = [
Vector { x_component: -34.2779, y_component: -43.31335149863761 },
Vector { x_component: 3.5152508192588856, y_component: -50.6241 },
Vector { x_component: -1045.4231, y_component: -1.1923 },
];
for i in 0..size {
assert_approx_eq(accelerations[i].x_component, expected[i].x_component, &format!("test_compute_acceleration_multiple_elements - x_component {}", i));
assert_approx_eq(accelerations[i].y_component, expected[i].y_component, &format!("test_compute_acceleration_multiple_elements - y_component {}", i));
}
}
#[test]
fn test_compute_velocity_one_element() {
let size = 1;
let accelerations = [Vector { x_component: 42.53, y_component: -631.431 }];
let dt = 0.00025;
let mut velocities = vec![Vector { x_component: 0.0, y_component: 0.0 }; size];
compute_velocity(dt, 0, &accelerations, &mut velocities);
assert_approx_eq(velocities[0].x_component, 0.010632500000000001, "test_compute_velocity_one_element - x_component");
assert_approx_eq(velocities[0].y_component, -0.15785775000000002, "test_compute_velocity_one_element - y_component");
}
#[test]
fn test_compute_velocity_multiple_elements() {
let size = 3;
let accelerations = [
Vector { x_component: -10.59, y_component: 162.35 },
Vector { x_component: -53223.12, y_component: 4212.124 },
Vector { x_component: 532521.2124124, y_component: 12142.512124 },
];
let dt = 0.00025;
let mut velocities = vec![
Vector { x_component: 5.332, y_component: 2.123 },
Vector { x_component: 7.12, y_component: 8.96 },
Vector { x_component: 61.52, y_component: 1293.123 },
];
for i in 0..size {
compute_velocity(dt, i, &accelerations, &mut velocities);
}
let expected = [
Vector { x_component: 5.3293525, y_component: 2.1635875 },
Vector { x_component: -6.18578, y_component: 10.013031 },
Vector { x_component: 194.6503031031, y_component: 1296.158628031 },
];
for i in 0..size {
assert_approx_eq(velocities[i].x_component, expected[i].x_component, &format!("test_compute_velocity_multiple_elements - x_component {}", i));
assert_approx_eq(velocities[i].y_component, expected[i].y_component, &format!("test_compute_velocity_multiple_elements - y_component {}", i));
}
}
#[test]
fn test_displace_particles_one_element() {
let size = 1;
let mut particles = vec![Particle { x_coordinate: 0.0, y_coordinate: 100.0, radius: 0.0, next: None, idx: 0 }];
let displacements = [Vector { x_component: 0.05, y_component: -0.05 }];
displace_particle(0, &displacements, &mut particles);
assert_approx_eq(particles[0].x_coordinate, 50.0, "test_displace_particles_one_element - x_coordinate");
assert_approx_eq(particles[0].y_coordinate, 50.0, "test_displace_particles_one_element - y_coordinate");
}
#[test]
fn test_displace_particles_multiple_elements() {
let size = 3;
let mut particles = vec![
Particle { x_coordinate: 0.0, y_coordinate: 100.0, radius: 0.0, next: None, idx: 0 },
Particle { x_coordinate: 111.0, y_coordinate: 210.0, radius: 0.0, next: None, idx: 1 },
Particle { x_coordinate: 10.0, y_coordinate: -30.0, radius: 0.0, next: None, idx: 2 },
];
let displacements = [
Vector { x_component: 0.0, y_component: 0.0 },
Vector { x_component: 0.015, y_component: -0.033 },
Vector { x_component: -0.015, y_component: 0.03 },
];
for i in 0..size {
displace_particle(i, &displacements, &mut particles);
}
let expected = [
Particle { x_coordinate: 0.0, y_coordinate: 100.0, radius: 0.0, next: None, idx: 0 },
Particle { x_coordinate: 126.0, y_coordinate: 177.0, radius: 0.0, next: None, idx: 1 },
Particle { x_coordinate: -5.0, y_coordinate: 0.0, radius: 0.0, next: None, idx: 2 },
];
for i in 0..size {
assert_approx_eq(particles[i].x_coordinate, expected[i].x_coordinate, &format!("test_displace_particles_multiple_elements - x_coordinate {}", i));
assert_approx_eq(particles[i].y_coordinate, expected[i].y_coordinate, &format!("test_displace_particles_multiple_elements - y_coordinate {}", i));
}
}
fn main() {}