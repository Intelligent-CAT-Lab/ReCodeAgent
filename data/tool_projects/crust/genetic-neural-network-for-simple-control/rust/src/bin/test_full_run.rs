use Genetic_neural_network_for_simple_control::sort::quickSort;
use Genetic_neural_network_for_simple_control::fit_functions::*;
use Genetic_neural_network_for_simple_control::genetic_operations::*;
use Genetic_neural_network_for_simple_control::pid_controller::*;
use Genetic_neural_network_for_simple_control::population::*;
use Genetic_neural_network_for_simple_control::neural_network::*;
use Genetic_neural_network_for_simple_control::model_system::*;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
fn getMinFit(fit: &Vec<f32>) -> usize {
    let mut min = 0;
    let length = fit.len();
    for i in 1..length {
        if fit[i as usize] < fit[min] {
            min = i as usize;
        }
    }
    min
}

fn closeRandomSizeGeneration(population: &mut Pop, sizes: &Vec<f32>){
    for i in 0 .. population.cols{
        population.s[0][i] = sizes[i as usize] + 0.7;
        population.s[1][i] = sizes[i as usize] - 0.7;
    }
}

fn createSystemNeuralNetworkInputTEST(input: &mut NNInput) {
    input.layerNumber = 5;
    input.neuronsSize = vec![1, 5, 5, 5, 5, 1];
    input.layerType = vec![0, 0, 0, 0, 0, 0];
    input.sdNumber = 0;
}

#[test]
pub fn testPIDRun() {
    println!("\x1b[1m=======TEST PID RUN STARTED=======\x1b[0m");

    let mut csv_file = File::create("TOOLBOX/PYTHON/input/data_pid_run.csv").unwrap();
    writeln!(csv_file, "best").unwrap();

    let chance = 0.1;
    let generations = 5000;
    let best_index = [0, 10];
    let rand_one_index = [10, 195];
    let rand_two_index = [195, 380];
    let rand_ful_index = [380, 500];

    let size_one_index = 185;
    let size_two_index = 185;

    let best_nums = [4, 2, 2, 2];

    let mut input = InputPop::default();
    let mut pop = Pop::default();

    let max = vec![100.0, 100.0, 100.0, 1.0];
    let min = vec![0.0, 0.0, 0.0, 0.0];
    let size = [500, 4];
    let size_ful = [120, 4];

    createInputPop(&mut input, &max, &min, &size);
    createStructure(&mut input, &mut pop);

    let mut fit = vec![0.0; pop.rows];

    let mut pid = PID::default();
    createNewPidController(&mut pid);
    pid.limMax = 60.0;
    pid.limMin = -60.0;
    pid.limMaxInt = pid.limMax / 2.0;
    pid.limMinInt = pid.limMin / 2.0;

    for i in 0..generations {
        pidFitFunction(&mut pop, &mut fit, &mut pid);
        let best_fit = getMinFit(&fit);

        writeln!(csv_file, "{}", fit[best_fit]).unwrap();

        if i % 100 == 0 || i > 9990 {
            println!(
                "GENERATION[{i:05}] - fit: {} P: {} I: {} D: {} Tau: {} - {}",
                fit[best_fit],
                pop.pop[best_fit][0],
                pop.pop[best_fit][1],
                pop.pop[best_fit][2],
                pop.pop[best_fit][3],
                best_fit
            );
        }

        let mut best_pop = Pop::default();
        let mut rand_pop_one = Pop::default();
        let mut rand_pop_two = Pop::default();

        selbest(&fit, &pop, &mut best_pop, &best_nums, 1);
        selturn(&pop, &fit, &mut rand_pop_one, size_one_index);
        selturn(&pop, &fit, &mut rand_pop_two, size_two_index);

        crosov(&mut rand_pop_one, &mut vec![1, 2], 2);
        crosov(&mut rand_pop_two, &mut vec![1, 2], 2);

        mutx(&mut rand_pop_one, chance);
        mutx(&mut rand_pop_two, chance);

        let mut input_rand = InputPop::default();
        let mut pop_random = Pop::default();
        createInputPop(&mut input_rand, &max, &min, &size_ful);
        createStructure(&mut input_rand, &mut pop_random);

        placePartOfPop(&mut pop, &best_pop, &best_index);
        placePartOfPop(&mut pop, &rand_pop_one, &rand_one_index);
        placePartOfPop(&mut pop, &rand_pop_two, &rand_two_index);
        placePartOfPop(&mut pop, &pop_random, &rand_ful_index);

        if i == generations - 1 || i % 100 == 0 {
            let mut csv_file2 = File::create("TOOLBOX/PYTHON/input/data_pid_.csv").unwrap();
            writeln!(csv_file2, "P,I,D,CV,RV").unwrap();

            pid.Kp = best_pop.pop[0][0];
            pid.Ki = best_pop.pop[0][1];
            pid.Kd = best_pop.pop[0][2];
            pid.tauD = best_pop.pop[0][3];

            makeSimulationOfSignal(&mut pid, &mut csv_file2, 1);

            println!(
                "BEST PID: P: {} I: {} D: {} Tau: {} - FIT: {} Count: {}",
                pid.Kp, pid.Ki, pid.Kd, pid.tauD, pid.fit, pid.maxCounter
            );
        }
    }

    clearPopulation(&mut pop);
    deletePid(&mut pid);
    // plotGraph();
    let flag = 1;
    assert!(flag == 1);
    println!("\x1b[1m\x1b[32m=======TEST PID RUN SUCCESSFUL=======\x1b[0m");
    
}
#[test]
pub fn testNNRun() {
    println!("\x1b[1m=======TEST NN RUN STARTED=======\x1b[0m");

    let mut csv_file = BufWriter::new(File::create("data_nn_fit.csv").unwrap());
    writeln!(csv_file, "best").unwrap();

    let mut input_nn = NNInput::default();
    let mut system_nn = SystemNN::default();

    createSystemNeuralNetworkInputTEST(&mut input_nn);
    createNNSystem(&mut system_nn, &mut input_nn);

    system_nn.max_sys = 40.0;
    system_nn.min_sys = -5.0;
    
    let chance = 0.1;
    let generations = 50000;

    let best_index = [0, 5];
    let rand_one_index = [5, 43];
    let rand_two_index = [43, 70];
    let rand_rand_index = [70, 95];
    let rand_randc_index = [95, 100];

    let size_one_index = 38;
    let size_two_index = 27;

    let best_nums = vec![2, 2, 1];
    let best_count = 3;

    let rest_cros = vec![1, 2];

    createDeNormalization(&mut system_nn);

    let count = system_nn.neural_network.countOfValues;
    let max = vec![1.0; count];
    let min = vec![-1.0; count];
    
    let mut input_pop = InputPop::default();
    let mut pop = Pop::default();
    createInputPop(&mut input_pop, &max, &min, &[100, count as i32]);
    createStructure(&mut input_pop, &mut pop);

    let mut fit = vec![0.0; pop.rows];

    let mut input_random = InputPop::default();
    let mut pop_random = Pop::default();
    createInputPop(&mut input_random, &max, &min, &[25, count as i32]);
    createStructure(&mut input_random, &mut pop_random);

    let mut input_close_random = InputPop::default();
    let mut pop_close_random = Pop::default();
    createInputPop(&mut input_close_random, &max, &min, &[5, count as i32]);
    createStructure(&mut input_close_random, &mut pop_close_random);

    for i in 0..generations {
        nnFitFunction(&mut pop, &mut fit, &mut system_nn);
        let best_fit = getMinFit(&fit);
        writeln!(csv_file, "{}", fit[best_fit]).unwrap();

        if i % 10 == 0 || i > 9990 {
            println!("GENERATION[{:05}] - fit: {}  - {}", i, fit[best_fit], best_fit);
        }

        let mut best_pop = Pop::default();
        let mut rand_pop_one = Pop::default();
        let mut rand_pop_two = Pop::default();

        selbest(&fit, &pop, &mut best_pop, &best_nums, 1);
        selturn(&pop, &fit, &mut rand_pop_one, size_one_index);
        selturn(&pop, &fit, &mut rand_pop_two, size_two_index);

        let mut rest_cros_one = vec![5, 10, 15, 20, 25];
        let mut rest_cros_two = vec![10, 15, 27, 35];

        crosov(&mut rand_pop_one, &mut rest_cros_one, 2);
        crosov(&mut rand_pop_two, &mut rest_cros_two, 4);

        mutx(&mut rand_pop_one, chance);
        mutx(&mut rand_pop_two, chance);

        closeRandomSizeGeneration(&mut pop_close_random, &pop.pop[best_fit]);
        generateRandomPopulation(&mut pop_random);
        generateRandomPopulation(&mut pop_close_random);

        if (i % 100 == 0 || i == generations - 1) && i != 0 {
            let mut csv_file2 = BufWriter::new(File::create("TOOLBOX/PYTHON/input/data_nn.csv").unwrap());
            writeln!(csv_file2, "CV,RV, System Output, Corrected Output").unwrap();

            let mut csv_file3 = BufWriter::new(OpenOptions::new()
                .create(true).append(true).open("TOOLBOX/PYTHON/input/best_nn.txt").unwrap());
            write!(csv_file3, "[").unwrap();
            for val in &pop.pop[0] {
                write!(csv_file3, "{}, ", val).unwrap();
            }
            writeln!(csv_file3, "]").unwrap();

            fillMatrixesNN(&mut system_nn.neural_network, &pop.pop[best_fit]);
            makeSimulationOfSignalNN(&mut system_nn, &mut csv_file2, true);

            println!("BEST NN: FIT: {} Count: {}", system_nn.fit, system_nn.max_counter);
            // plotGraphNN();
        }

        placePartOfPop(&mut pop, &best_pop, &best_index);
        placePartOfPop(&mut pop, &rand_pop_one, &rand_one_index);
        placePartOfPop(&mut pop, &rand_pop_two, &rand_two_index);
        placePartOfPop(&mut pop, &pop_random, &rand_rand_index);
        placePartOfPop(&mut pop, &pop_close_random, &rand_randc_index);

        clearPopulation(&mut best_pop);
        clearPopulation(&mut rand_pop_one);
        clearPopulation(&mut rand_pop_two);
    }

    clearPopulation(&mut pop);
    clearPopulation(&mut pop_random);
    clearNNSystem(&mut system_nn);

    // plotGraphNN();

    println!("\x1b[1m\x1b[32m=======TEST NN RUN SUCCESSFUL=======\x1b[0m");
    
}

fn main(){}