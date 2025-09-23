use crate::data;
pub fn fill_grid(
    num_particles: usize,
    x_squares: i32,
    y_squares: i32,
    square_length: f64,
    particles: &[data::Particle],
    grid: &Vec<&mut Vec<&data::Particle>>,
    grid_lasts: &Vec<&mut Vec<&mut data::Particle>>,
) {
    unimplemented!()
}
pub fn compute_contacts(
    grid: &Vec<&Vec<&data::Particle>>,
    x_squares: i32,
    y_squares: i32,
    square_length: f64,
    contacts: &mut [data::Contact],
) -> usize {
    unimplemented!()
}
pub fn find_col(x: f64, x_squares: i32, square_length: f64) -> i32 {
    unimplemented!()
}
pub fn find_row(y: f64, y_squares: i32, square_length: f64) -> i32 {
    unimplemented!()
}
pub fn find_square(x: f64, y: f64, x_squares: i32, y_squares: i32, square_length: f64) -> i32 {
    unimplemented!()
}