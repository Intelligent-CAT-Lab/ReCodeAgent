use Genetic_neural_network_for_simple_control::sort::quickSort;

#[test]
pub fn testQuicksort() {
    println!("\x1b[1m=======TEST QUICK SORT STARTED=======\x1b[0m");

    let length = 5;
    let mut fit = vec![9.6, 3.4, 10.0, 4.6, 0.4];
    let mut result: Vec<i32> = (0..length).collect();
    let result_correct = vec![4, 1, 3, 0, 2];

    quickSort(&mut fit, &mut result, length);

    for i in 0..length {
        assert_eq!(result[i as usize], result_correct[i as usize], "Mismatch at index {}: expected {}, got {}", i, result_correct[i as usize], result[i as usize]);
    }

    println!("\x1b[1m\x1b[32m=======TEST QUICK SORT SUCCESSFUL=======\x1b[0m");
    
}
pub fn main(){}