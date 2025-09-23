// genetic_operations.rs
use crate::population::{Pop, InputPop};
use crate::sort::quickSort;
use crate::general_math::createRandomFloat;
/// Creates a new input population structure from a given population.
/// Used internally by other selection functions.
fn create_input_pop_custom(input_pop: &mut InputPop, rows: i32, population: &Pop) {
    unimplemented!();
}
/// Selects the best individuals based on fitness.
pub fn selbest(
    fit: &[f32],
    population: &Pop,
    new_population: &mut Pop,
    selects: &[i32],
    way: i32,
) {
    unimplemented!();
}
/// Selects individuals randomly from the population.
pub fn selrand(population: &mut Pop, new_population: &mut Pop, rows: i32) {
    unimplemented!();
}
/// Tournament selection between two randomly chosen individuals.
pub fn selturn(population: &Pop, fit: &[f32], new_population: &mut Pop, rows: i32) {
    unimplemented!();
}
/// Crossover operation exchanging genes between adjacent individuals.
pub fn crosov(population: &mut Pop, selects: &mut Vec<i32>, selects_length: i32) {
    unimplemented!();
}
/// Mutation function that randomly alters values with a given probability.
pub fn mutx(population: &mut Pop, chance: f32) {
    unimplemented!();
}