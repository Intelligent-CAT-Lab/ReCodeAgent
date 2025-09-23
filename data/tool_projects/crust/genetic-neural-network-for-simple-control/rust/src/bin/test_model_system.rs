use Genetic_neural_network_for_simple_control::model_system::{
    createNNSystem, 
    createDeNormalization, 
    makeSimulationOfSignalNN, 
    cleanNNSystem
};
use Genetic_neural_network_for_simple_control::neural_network::{
    fillMatrixesNN,
    NNInput
};
use Genetic_neural_network_for_simple_control::population::{
    createInputPop, 
    createStructure, 
    clearPopulation, 
    Pop, 
    InputPop
};
use Genetic_neural_network_for_simple_control::model_system::{
    SystemNN
};

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn create_system_neural_network_input(input: &mut NNInput, check: i32) {
    input.neuronsSize = if check == 1 {
        vec![1, 3, 2, 1]
    } else {
        vec![1, 5, 5, 1]
    };

    input.layerNumber = 4;
    input.layerType = vec![0; input.layerNumber];
    input.sdNumber = 0;
}

#[test]
pub fn testSystemCreate()  {
    println!("\x1b[1m=======TEST SYSTEM NN CREATE STARTED=======\x1b[0m");

    let mut input = NNInput::default();
    let mut system_nn = Box::new(SystemNN::default());

    create_system_neural_network_input(&mut input, 1);
    createNNSystem(&mut system_nn, &mut input);

    system_nn.max_sys = 50.0;
    system_nn.min_sys = -50.0;

    createDeNormalization(&mut system_nn);

    let mut input_pop = Box::new(InputPop::default());
    let mut pop = Box::new(Pop::default());
    let count = system_nn.neural_network.as_ref().countOfValues;
    let size = vec![1, count];
    let max = vec![1.0; count];
    let min = vec![0.0; count];

    let mut csv_file = BufWriter::new( File::create("./input/data_nn.csv").unwrap() );
    
    writeln!(csv_file, "CV,RV");
    

    createInputPop(&mut input_pop, &max, &min, &size.iter().map(|&x| x as i32).collect::<Vec<i32>>());
    createStructure(&mut input_pop, &mut pop);

    fillMatrixesNN(system_nn.neural_network.as_mut(), &pop.pop[0]);
    makeSimulationOfSignalNN(&mut system_nn, &mut csv_file, true);

    let mut csv_file2 = File::create("./input/data_nn_fit.csv").expect("Failed to create fit file");
    writeln!(csv_file2, "best").unwrap();
    writeln!(csv_file2, "{}", system_nn.fit).unwrap();


    cleanNNSystem(&mut system_nn.as_mut());
    clearPopulation(&mut pop.as_mut());

    println!("\x1b[1m\x1b[32m=======TEST SYSTEM NN CREATE SUCCESSFUL=======\x1b[0m");
    
}
fn main(){}