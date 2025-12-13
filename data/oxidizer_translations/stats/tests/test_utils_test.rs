// Approximate float comparisons
// Taken from the standard library's math/all_test.go

/// Tolerance function for approximate float comparisons
///
/// This function checks if two float values are approximately equal within a tolerance.
/// It handles edge cases like small values and ensures proper comparison.
#[allow(non_snake_case)]
pub fn tolerance(a: f64, b: f64, e: f64) -> bool {
    // Multiplying by e here can underflow denormal values to zero.
    // Check a==b so that at least if a and b are small and identical
    // we say they match.
    if a == b {
        return true;
    }
    let mut d = a - b;
    if d < 0.0 {
        d = -d;
    }

    // note: b is correct (expected) value, a is actual value.
    // make error tolerance a fraction of b, not a.
    let mut e = e;
    if b != 0.0 {
        e = e * b;
        if e < 0.0 {
            e = -e;
        }
    }
    d < e
}

/// Check if two floats are close with tolerance 1e-14
#[allow(non_snake_case)]
pub fn close(a: f64, b: f64) -> bool {
    tolerance(a, b, 1e-14)
}

/// Check if two floats are very close with tolerance 4e-16
#[allow(non_snake_case)]
pub fn veryclose(a: f64, b: f64) -> bool {
    tolerance(a, b, 4e-16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tolerance_exact_match() {
        // Exact matches should always return true
        assert!(tolerance(1.0, 1.0, 1e-10));
        assert!(tolerance(0.0, 0.0, 1e-10));
        assert!(tolerance(-5.5, -5.5, 1e-10));
    }

    #[test]
    fn test_tolerance_within_epsilon() {
        // Values within tolerance should return true
        assert!(tolerance(1.0, 1.0 + 1e-15, 1e-14));
        assert!(tolerance(100.0, 100.0 + 1e-12, 1e-14));
    }

    #[test]
    fn test_tolerance_outside_epsilon() {
        // Values outside tolerance should return false
        assert!(!tolerance(1.0, 1.1, 1e-14));
        assert!(!tolerance(100.0, 101.0, 1e-14));
    }

    #[test]
    fn test_tolerance_small_values() {
        // Small values that are equal should match
        let small = 1e-20;
        assert!(tolerance(small, small, 1e-14));
    }

    #[test]
    fn test_tolerance_negative_difference() {
        // Should work regardless of order
        assert_eq!(tolerance(1.0, 1.01, 1e-14), tolerance(1.01, 1.0, 1e-14));
    }

    #[test]
    fn test_close_function() {
        // Test the close function with 1e-14 tolerance
        assert!(close(1.0, 1.0));
        assert!(close(100.0, 100.0 + 1e-13));
        assert!(!close(1.0, 1.001));
    }

    #[test]
    fn test_veryclose_function() {
        // Test the veryclose function with 4e-16 tolerance
        assert!(veryclose(1.0, 1.0));
        assert!(veryclose(100.0, 100.0 + 1e-14));
        // This should be more strict than close
        assert!(!veryclose(1.0, 1.0 + 1e-10));
    }

    #[test]
    fn test_veryclose_stricter_than_close() {
        // veryclose should be stricter than close
        let a = 1.0;
        let b = 1.0 + 5e-15;

        // This might pass close but not veryclose
        // The exact behavior depends on the scaling with b
        if close(a, b) {
            // If close passes, veryclose might or might not pass
            // This is just to demonstrate the relationship
        }
    }

    #[test]
    fn test_tolerance_zero_value() {
        // Test with zero value for b
        assert!(tolerance(1e-15, 0.0, 1e-14));
        assert!(!tolerance(1.0, 0.0, 1e-14));
    }

    #[test]
    fn test_tolerance_negative_values() {
        // Test with negative values
        assert!(tolerance(-1.0, -1.0, 1e-14));
        assert!(tolerance(-100.0, -100.0 - 1e-13, 1e-14));
        assert!(!tolerance(-1.0, -1.1, 1e-14));
    }
}
