// Test suite for util module functions
use wspace::util::float64ToInt;

#[test]
fn TestFloat64ToInt() {
    let m = float64ToInt(234.0234);
    if m != 234 {
        panic!("{:#x} != {:#x}", m, 234);
    }

    let m = float64ToInt(-234.0234);
    if m != -234 {
        panic!("{:#x} != {:#x}", m, -234);
    }

    let m = float64ToInt(1.0);
    if m != 1 {
        panic!("{:#x} != {:#x}", m, 1);
    }
}
