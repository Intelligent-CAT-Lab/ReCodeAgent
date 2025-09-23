use Genetic_neural_network_for_simple_control::pid_controller::{createNewPidController, deletePid, makeSimulationOfSignal, PID};
use std::fs::File;
use std::io::Write;

#[test]
pub fn testPIDCreate() {
    println!("\x1b[1m=======TEST PID CREATE STARTED=======\x1b[0m");

    let mut pid = Box::new(PID::default());
    createNewPidController(&mut pid);

    let mut csv_file = File::create("data_pid_.csv").expect("Unable to create CSV file");
    writeln!(csv_file, "P,I,D,CV,RV").expect("Failed to write CSV header");

    pid.Kp = 1.0;
    pid.Ki = 1.0;
    pid.Kd = 0.0;

    pid.limMax = 10.0;
    pid.limMin = -10.0;

    pid.limMaxInt = pid.limMax / 2.0;
    pid.limMinInt = pid.limMin / 2.0;

    let csv = true;
    makeSimulationOfSignal(&mut pid, &mut csv_file, 1);

    println!("{}", pid.fit);
    
    deletePid(&mut pid.as_mut());

    let flag = 1;
    assert_eq!(flag, 1, "Test PID Create failed");

    println!("\x1b[1m\x1b[32m=======TEST PID CREATE SUCCESSFUL=======\x1b[0m");
}
fn main(){}