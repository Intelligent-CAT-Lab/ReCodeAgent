use inversion_list::inversion_list::*;

#[test]
fn test() {
    // Original test checks that creating a set with capacity=5
    // but having a value >= 5 fails (EINVAL in C).
    let a = [1, 2, 3, 5, 7, 8, 9, 0, 2];

    match InversionList::new(5, &a) {
        Ok(_) => {
            panic!("Expected an error due to value >= capacity=5, but got Ok()");
        }
        Err(InversionListError::ValueOutOfRange(val, cap)) => {
            println!(
                "Got expected ValueOutOfRange({}, {}). Test passed.",
                val, cap
            );
        }
        Err(e) => {
            panic!("Expected ValueOutOfRange error, got: {:?}", e);
        }
    }
}
fn main(){}