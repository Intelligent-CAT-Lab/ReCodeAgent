use crate::systems_builder;
// pid_controller.rs
use std::fs::File;
use crate::signal_designer::Signal;
#[derive(Default)]
pub struct PID {
    // constants not changed in calculations
    pub Kp: f32,
    pub Ki: f32,
    pub Kd: f32,
    pub tauI: f32,
    pub tauD: f32,
    // values used in process of the calculations
    pub prevError: f32,
    pub proportiError: f32,
    pub integralError: f32,
    pub differenError: f32,
    // input values
    pub signal: Option<Box<Signal>>,  // Assuming ownership
    pub func_system: Option<fn(&mut [f32]) -> f32>,
    // memory of output
    pub output: Option<Box<Signal>>,  // Assuming ownership
    pub dataSystem: Option<Box<[f32]>>,
    pub sizeDataSystem: i32,
    // the limits
    pub limMaxInt: f32,
    pub limMinInt: f32,
    pub limMax: f32,
    pub limMin: f32,
    // the fit value of pid run
    pub fit: f32,
    // the check for steady rise pid
    pub steadyRiseCheck: i32,
    // signal oscillation counter
    pub maxCounter: i32,
}
pub fn createNewPidController(pid: &mut PID) {
    unimplemented!();
}
pub fn deletePid(pid: &mut PID) {
    unimplemented!();
}
pub fn makeSimulationOfSignal(pid: &mut PID, csv_file: &mut File, csv: i32) {
    unimplemented!();
}
pub fn resetOutputMemoryPid(pid: &mut PID) {
    unimplemented!();
}