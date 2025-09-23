use Genetic_neural_network_for_simple_control::population::{createInputPop, createStructure, clearPopulation, Pop, InputPop};

#[test]
pub fn testPopulation() {
    println!("\x1b[1m=======TEST POPULATION STARTED=======\x1b[0m");

    let mut input = InputPop::default();
    let mut pop = Pop::default();

    let max = vec![10.0, 100.0, -10.0];
    let min = vec![0.0, 50.0, -20.0];
    let size = vec![3, 3];

    createInputPop(&mut input, &max, &min, &size);
    createStructure(&mut input, &mut pop);

    let mut flag = true;

    for i in 0..pop.cols {
        println!("{} - {}", pop.s[0][i], pop.s[1][i]);
        assert!(pop.s[0][i] - max[i] < f32::EPSILON || pop.s[1][i] - min[i] < f32::EPSILON);
        
    }

    println!("Following population created:");
    for i in 0..pop.rows {
        for j in 0..pop.cols {
            print!("{} ", pop.pop[i][j]);
            if pop.pop[i][j] > pop.s[0][j] || pop.pop[i][j] < pop.s[1][j] {
                flag = false;
            }
        }
        println!();
    }

    clearPopulation(&mut pop);


    println!("\x1b[1m\x1b[32m=======TEST POPULATION SUCCESSFUL=======\x1b[0m");
    
}
fn main(){}