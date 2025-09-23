// input_toolbox.rs
use crate::model_system::SystemNN;
/// Input preparation for NN type one.
pub fn typeOne(data: &mut [f32]) {
    unimplemented!();
}
/// Input preparation for NN type two.
pub fn typeTwo(data: &mut [f32]) {
    unimplemented!();
}
/// Constructs input arrays for type one networks.
fn makeInputDataSystem(system_nn: &mut SystemNN, size: usize, full: usize) {
    unimplemented!();
}
/// Constructs input arrays for type two networks.
fn makeInputDataSystemTwo(system_nn: &mut SystemNN, size: usize, full: usize) {
    unimplemented!();
}
/// Lets the user select an input-preparation function and sets up the corresponding system config.
pub fn selectInputNNFunction(func_ptr: &mut Option<fn(&mut [f32])>, system_nn: &mut SystemNN) {
    unimplemented!();
}