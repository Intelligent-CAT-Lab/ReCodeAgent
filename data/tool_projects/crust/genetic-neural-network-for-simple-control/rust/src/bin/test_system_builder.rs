use Genetic_neural_network_for_simple_control::systems_builder::selectSystem;
use Genetic_neural_network_for_simple_control::pid_controller::{deletePid, PID};
use Genetic_neural_network_for_simple_control::signal_designer::Signal;
#[test]
pub fn testSelectSystem() {
    println!("\x1b[1m=======TEST SELECT SYSTEM STARTED=======\x1b[0m");

    let mut func_ptr: Option<fn(&mut [f32]) -> f32> = None;
    let mut pid = Box::new(PID::default());

    // Select the system
    let _size = selectSystem(&mut func_ptr);

    // Allocate dummy signals and system memory
    pid.signal = Some(Box::new(Signal {
        signal: vec![0.0],
        dt: 0.1,
        length: 1,
    }));

    pid.output = Some(Box::new(Signal {
        signal: vec![0.0],
        dt: 0.1,
        length: 1,
    }));

    pid.dataSystem = Some(Box::new([0.0; 10]));

    deletePid(pid.as_mut());

    println!("\x1b[1m\x1b[32m=======TEST SELECT SYSTEM SUCCESSFUL=======\x1b[0m");
    
}
fn main(){}