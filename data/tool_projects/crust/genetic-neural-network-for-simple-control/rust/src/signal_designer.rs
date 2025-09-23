// signal_designer.rs
#[derive(Default)]
pub struct Signal {
    pub signal: Vec<f32>, // the values of the signal at each time point
    pub dt: f32,          // the sampling time for the signal
    pub length: i32,      // length of the signal in seconds
}
pub fn deleteSignal(signal: &mut Signal) {
    unimplemented!();
}
pub fn cliSignalSelector(signal: &mut Signal) {
    unimplemented!();
}
fn selectStepSignal(signal: &mut Signal) {
    unimplemented!();
}
fn selectCustomASignal(signal: &mut Signal) {
    unimplemented!();
}