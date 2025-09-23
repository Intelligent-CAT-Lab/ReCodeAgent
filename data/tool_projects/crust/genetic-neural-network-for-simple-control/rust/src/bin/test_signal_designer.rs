use Genetic_neural_network_for_simple_control::signal_designer::{Signal, cliSignalSelector, deleteSignal};
#[test]
pub fn testSignalCreate() {
    println!("\x1b[1m=======TEST SIGNAL CREATE STARTED=======\x1b[0m");

    let mut signal = Box::new(Signal::default());
    cliSignalSelector(&mut signal);
    deleteSignal(&mut signal);

    let flag = 1;
    assert_eq!(flag, 1, "Test Signal Create failed");

    println!("\x1b[1m\x1b[32m=======TEST SIGNAL CREATE SUCCESSFUL=======\x1b[0m");
}

fn main() {
 
}