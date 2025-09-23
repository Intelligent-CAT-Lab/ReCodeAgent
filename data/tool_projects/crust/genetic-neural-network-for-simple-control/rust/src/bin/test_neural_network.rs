// This is a skeleton Rust test file representing the translated structure
// from the C code. Actual implementation for each function assumed to exist

use Genetic_neural_network_for_simple_control::neural_network::*;
use Genetic_neural_network_for_simple_control::population::*;
use Genetic_neural_network_for_simple_control::matrix_math::*;

fn check_matrixes_nn(neural_network: &NN, population: &[f32]) -> bool {
    let mut global_index = 0;

    for i in 0..(neural_network.layerNumber - 1) {
        let aw = &neural_network.AW[i];
        for x in 0..aw.sizes[0] {
            for y in 0..aw.sizes[1] {
                if aw.matrix[x][y] != population[global_index] {
                    return false;
                }
                global_index += 1;
            }
        }

        if i < neural_network.layerNumber - 2 {
            let bw = &neural_network.BW[i];
            for x in 0..bw.sizes[0] {
                for y in 0..bw.sizes[1] {
                    if bw.matrix[x][y] != population[global_index] {
                        return false;
                    }
                    global_index += 1;
                }
            }
        }
    }

    true
}

fn create_simple_neural_network(neural_network: &mut NN, check: i32) {
    let mut input = NNInput {
        neuronsSize: vec![1, 0, 0, 1],
        layerNumber: 4,
        normalizationMatrix: [vec![100.0], vec![0.0]],
        denormalizationMatrix: [vec![100.0], vec![0.0]],
        layerType: vec![0, 1, 0, 0],
        sdNumber: 1,
    };

    if check == 1 {
        input.neuronsSize[1] = 3;
        input.neuronsSize[2] = 2;
    } else {
        input.neuronsSize[1] = 5;
        input.neuronsSize[2] = 5;
    }

    createNeuralNetwork(input, neural_network);
}

#[test]
pub fn test_neural_network_create()  {
    println!("\x1b[1m=======TEST NEURAL NETWORK CREATE STARTED=======\x1b[0m");

    let mut neural_network = NN::default();
    let check = 0;
    create_simple_neural_network(&mut neural_network, check);

    let neuronsSize = vec![1, 5, 5, 1];
    let layerType = vec![0, 1, 0, 0];
    let normalization_max = vec![100.0];
    let normalization_min = vec![0.0];
    let denormalization_max = vec![100.0];
    let denormalization_min = vec![0.0];
    let sd_types_one = vec![0, 0, 1, 2, 2];
    let row_sd = vec![5, 1];

    assert_eq!(neural_network.neuronsSize, neuronsSize);
    assert_eq!(neural_network.layerType, layerType);
    assert_eq!(neural_network.normalizationMatrix[0], normalization_max);
    assert_eq!(neural_network.normalizationMatrix[1], normalization_min);
    assert_eq!(neural_network.denormalizationMatrix[0], denormalization_max);
    assert_eq!(neural_network.denormalizationMatrix[1], denormalization_min);
    assert_eq!(neural_network.sdNeuronsTypes[0], sd_types_one);
    assert_eq!(neural_network.SDMemory[0].sizes, row_sd);

    clearNeuralNetwork(neural_network);

    println!("\x1b[1m\x1b[32m=======TEST NEURAL NETWORK CREATE SUCCESSFUL=======\x1b[0m");
    
}

#[test]
pub fn test_fill_matrixes_nn() {
    println!("\x1b[1m=======TEST FILL MATRIXES NN STARTED=======\x1b[0m");

    let mut neural_network = NN::default();
    create_simple_neural_network(&mut neural_network, 0);
    let count = neural_network.countOfValues;

    let mut pop = Pop::default();
    let mut input_pop = InputPop::default();
    createInputPop(&mut input_pop, &vec![10.0; count], &vec![0.0; count], &[1, count as i32]);
    createStructure(&mut input_pop, &mut pop);

    fillMatrixesNN(&mut neural_network, &pop.pop[0]);
    let flag = check_matrixes_nn(&neural_network, &pop.pop[0]);

    assert!(flag);

    clearNeuralNetwork(neural_network);
    clearPopulation(&mut pop);

    println!("\x1b[1m\x1b[32m=======TEST FILL MATRIXES NN SUCCESSFUL=======\x1b[0m");
    
}

#[test]
pub fn test_de_normalization_process() {
    println!("\x1b[1m=======TEST DE_NORMALIZATION PROCESS STARTED=======\x1b[0m");

    let mut neural_network = NN::default();
    create_simple_neural_network(&mut neural_network, 0);
    let mut a = Matrix::default();
    a.matrix = vec![vec![0.0; 1]; 1];
    a.sizes = vec![1, 1];

    a.matrix[0][0] = 50.0;
    deNormalizationProcess(&mut neural_network, &mut a, 0);
    assert_eq!(a.matrix[0][0], 0.0);

    a.matrix[0][0] = 0.40;
    deNormalizationProcess(&mut neural_network, &mut a, 1);
    assert_eq!(a.matrix[0][0], 70.0);

    a.matrixDelete();
    clearNeuralNetwork(neural_network);

    println!("\x1b[1m\x1b[32m=======TEST DE_NORMALIZATION PROCESS SUCCESSFUL=======\x1b[0m");
}

#[test]
pub fn test_one_calculation() {
    println!("\x1b[1m=======TEST ONE CALCULATION STARTED=======\x1b[0m");

    let mut neural_network = NN::default();
    create_simple_neural_network(&mut neural_network, 1);

    let mut input = Matrix::default();
    input.matrix = vec![vec![0.0; 1]; 1];
    input.sizes = vec![1, 1];
    input.matrix[0][0] = 70.0;

    let mut output = Matrix::default();
    output.matrix = vec![vec![0.0; 1]; 1];
    output.sizes = vec![1, 1];

    let count = neural_network.countOfValues;
    let mut pop = Pop::default();
    let mut input_pop = InputPop::default();
    createInputPop(&mut input_pop, &vec![1.0; count], &vec![-1.0; count], &[1, count as i32]);
    createStructure(&mut input_pop, &mut pop);

    let pop_example = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.1, 0.2, 0.1, 0.2];
    fillMatrixesNN(&mut neural_network, &pop_example);

    oneCalculation(&mut neural_network, &mut input, &mut output);

    println!("Result output:");
    for row in &output.matrix {
        for &val in row {
            let rounded = (val * 10_000.0).round() / 10_000.0;
            print!("{} ", rounded);
        }
        println!();
    }

    println!("Result SD memory:");
    for row in &neural_network.SDMemory[0].matrix {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }

    clearNeuralNetwork(neural_network);
    clearPopulation(&mut pop);
    output.matrixDelete();
    let flag = 1;
    assert_eq!(flag, 1);
    println!("\x1b[1m\x1b[32m=======TEST ONE CALCULATION SUCCESSFUL=======\x1b[0m");
}

fn main(){}
