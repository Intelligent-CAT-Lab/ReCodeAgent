use Genetic_neural_network_for_simple_control::genetic_operations::{selbest, selrand, selturn, crosov, mutx};
use Genetic_neural_network_for_simple_control::population::{createInputPop, createStructure, clearPopulation, Pop, InputPop};

pub fn createPopulation(population: &mut Pop, size: &[i32]){
    let mut input : InputPop = InputPop::default();
    let min = vec![0.0, 50.0, -20.0];
    let max = vec![10.0, 100.0, -10.0];
    createInputPop(&mut input, &min, &max, size);
    createStructure(&mut input, population);
}

#[test]
pub fn testSelbest()  {
    println!("\x1b[1m=======TEST SELBEST STARTED=======\x1b[0m");

    let mut new_population_higher = Pop::default();
    let mut new_population_lower = Pop::default();
    let mut population = Pop::default();

    let fit = vec![-1.0, 15.0, -3.0];
    let selects = vec![2, 2];
    let size = vec![3, 3];
    let fit_length = 3;
    let selects_length = 2;

    createPopulation(&mut population, &size);

    selbest(&fit, &population, &mut new_population_higher, &selects, 0);
    selbest(&fit, &population, &mut new_population_lower, &selects , 1);

    assert_eq!(new_population_higher.rows, 4);
    assert_eq!(new_population_lower.rows, 4);

    clearPopulation(&mut population);
    clearPopulation(&mut new_population_higher);
    clearPopulation(&mut new_population_lower);

    println!("\x1b[1m\x1b[32m=======TEST SELBEST SUCCESSFUL=======\x1b[0m");
    
}

#[test]
pub fn testCrossov() {
    println!("\x1b[1m=======TEST CROSSOV STARTED=======\x1b[0m");

    let mut population = Box::new(Pop::default());
    let size = vec![2, 9];
    let row1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let row2 = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0];
    let row1_check = vec![1.0, 2.0, 30.0, 40.0, 50.0, 6.0, 7.0, 80.0, 90.0];
    let row2_check = vec![10.0, 20.0, 3.0, 4.0, 5.0, 60.0, 70.0, 8.0, 9.0];

    createPopulation(&mut population, &size);
    population.pop[0] = row1.clone();
    population.pop[1] = row2.clone();

    let mut selects = vec![2, 5, 7];
    crosov(&mut population, &mut selects, 3);

    assert_eq!(population.pop[0], row1_check);
    assert_eq!(population.pop[1], row2_check);

    clearPopulation(&mut population);

    println!("\x1b[1m\x1b[32m=======TEST CROSSOV SUCCESSFUL=======\x1b[0m");
    
}

#[test]
pub fn testMutx()  {
    println!("\x1b[1m=======TEST MUTX STARTED=======\x1b[0m");

    let mut population = Box::new(Pop::default());
    let size = vec![2, 9];
    createPopulation(&mut population, &size);

    mutx(&mut population, 0.3);

    clearPopulation(&mut population);

    println!("\x1b[1m\x1b[32m=======TEST MUTX SUCCESSFUL=======\x1b[0m");
    
}

#[test]
pub fn testSelrand() {
    println!("\x1b[1m=======TEST SELRAND STARTED=======\x1b[0m");

    let mut population = Pop::default();
    let mut new_population = Pop::default();
    let size = vec![5, 3];
    let rows = 10;

    createPopulation(&mut population, &size);
    selrand(&mut population, &mut new_population, rows);

    assert_eq!(new_population.rows, rows as usize);

    clearPopulation(&mut population);
    clearPopulation(&mut new_population);

    println!("\x1b[1m\x1b[32m=======TEST SELRAND SUCCESSFUL=======\x1b[0m");
    
}

#[test]
pub fn testSelturn() {
    println!("\x1b[1m=======TEST SELTURN STARTED=======\x1b[0m");

    let mut population = Pop::default();
    let mut new_population = Pop::default();
    let size = vec![3, 3];
    let rows = 10;
    let fit = vec![0.9, 0.3, 0.5];

    createPopulation(&mut population, &size);
    selturn(&population, &fit, &mut new_population, rows);

    assert_eq!(new_population.rows, rows as usize);

    clearPopulation(&mut population);
    clearPopulation(&mut new_population);

    println!("\x1b[1m\x1b[32m=======TEST SELTURN SUCCESSFUL=======\x1b[0m");
    
}
pub fn main(){}