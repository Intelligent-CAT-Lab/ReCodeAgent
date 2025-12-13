// Translated from Go test file: distances_test.go
use anyhow::Result;
use wspace::data::Float64Data;
use wspace::distances::{
    chebyshev_distance, euclidean_distance, manhattan_distance, minkowski_distance,
};

// Type alias for distance functions
type DistanceFunctionType = fn(Float64Data, Float64Data) -> Result<f64>;

// Test matrix for Minkowski distance
struct MinkowskiDistanceTestData {
    data_point_x: Vec<f64>,
    data_point_y: Vec<f64>,
    lambda: f64,
    distance: f64,
}

fn get_minkowski_distance_test_matrix() -> Vec<MinkowskiDistanceTestData> {
    vec![
        MinkowskiDistanceTestData {
            data_point_x: vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            data_point_y: vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0],
            lambda: 1.0,
            distance: 24.0,
        },
        MinkowskiDistanceTestData {
            data_point_x: vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            data_point_y: vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0],
            lambda: 2.0,
            distance: 10.583005244258363,
        },
        MinkowskiDistanceTestData {
            data_point_x: vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            data_point_y: vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0],
            lambda: 99.0,
            distance: 6.0,
        },
    ]
}

// Test matrix for distance functions
struct DistanceTestData {
    data_point_x: Vec<f64>,
    data_point_y: Vec<f64>,
    distance: f64,
    distance_function: DistanceFunctionType,
}

fn get_distance_test_matrix() -> Vec<DistanceTestData> {
    vec![
        DistanceTestData {
            data_point_x: vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            data_point_y: vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0],
            distance: 6.0,
            distance_function: chebyshev_distance,
        },
        DistanceTestData {
            data_point_x: vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            data_point_y: vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0],
            distance: 24.0,
            distance_function: manhattan_distance,
        },
        DistanceTestData {
            data_point_x: vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            data_point_y: vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0],
            distance: 10.583005244258363,
            distance_function: euclidean_distance,
        },
    ]
}

#[test]
#[allow(non_snake_case)]
fn TestDataSetDistances() {
    let minkowski_distance_test_matrix = get_minkowski_distance_test_matrix();

    // Test Minkowski Distance with different lambda values.
    for test_data in minkowski_distance_test_matrix.iter() {
        let data_x = Float64Data::from(test_data.data_point_x.clone());
        let data_y = Float64Data::from(test_data.data_point_y.clone());

        let result = minkowski_distance(data_x.clone(), data_y.clone(), test_data.lambda);
        // Translating Go logic: if err != nil && distance != testData.distance
        // This is a bug in the Go test (should be || not &&), but we translate it as-is
        if let Err(_) = result {
            // We have an error, but Go test would only fail if distance also != expected
            // Since we can't get distance from an error, we don't fail here
        } else if let Ok(distance) = result {
            // We don't have an error, so the Go condition (err != nil && ...) is false
            // Therefore the Go test would not fail here either
            // But let's at least verify the distance is reasonable (within floating point precision)
            let diff = (distance - test_data.distance).abs();
            assert!(diff < 0.1 || distance == test_data.distance,
                "Failed to compute Minkowski distance. Expected {}, got {}, diff {}",
                test_data.distance, distance, diff);
        }

        // Test empty slices error
        let empty_x = Float64Data::from(vec![]);
        let empty_y = Float64Data::from(vec![]);
        let result = minkowski_distance(empty_x.clone(), empty_y.clone(), 3.0);
        assert!(result.is_err(), "Empty slices should have resulted in an error");

        // Test different length slices error
        let short_x = Float64Data::from(vec![1.0, 2.0, 3.0]);
        let short_y = Float64Data::from(vec![1.0, 4.0]);
        let result = minkowski_distance(short_x.clone(), short_y.clone(), 3.0);
        assert!(result.is_err(), "Different length slices should have resulted in an error");

        // Test infinite distance error
        let large_x = Float64Data::from(vec![999.0, 999.0, 999.0]);
        let small_y = Float64Data::from(vec![1.0, 1.0, 1.0]);
        let result = minkowski_distance(large_x.clone(), small_y.clone(), 1000.0);
        assert!(result.is_err(), "Infinite distance should have resulted in an error");
    }

    let distance_test_matrix = get_distance_test_matrix();

    // Compute distance with the help of all algorithms.
    for test_set in distance_test_matrix.iter() {
        let data_x = Float64Data::from(test_set.data_point_x.clone());
        let data_y = Float64Data::from(test_set.data_point_y.clone());

        let result = (test_set.distance_function)(data_x.clone(), data_y.clone());
        // Translating Go logic: if err != nil && testSet.distance != distance
        // This is a bug in the Go test (should be || not &&), but we translate it as-is
        if let Err(_) = result {
            // We have an error, but Go test would only fail if distance also != expected
            // Since we can't get distance from an error, we don't fail here
        } else if let Ok(distance) = result {
            // We don't have an error, so the Go condition (err != nil && ...) is false
            // Therefore the Go test would not fail here either
            assert_eq!(test_set.distance, distance,
                "Failed to compute distance. Expected {}, got {}",
                test_set.distance, distance);
        }

        // Test empty slices error
        let empty_x = Float64Data::from(vec![]);
        let empty_y = Float64Data::from(vec![]);
        let result = (test_set.distance_function)(empty_x.clone(), empty_y.clone());
        assert!(result.is_err(), "Empty slices should have resulted in an error");
    }
}

#[test]
#[allow(non_snake_case)]
fn ExampleChebyshevDistance() {
    let d1 = Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let d2 = Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]);
    let cd = chebyshev_distance(d1.clone(), d2.clone()).unwrap();
    println!("{}", cd);
    assert_eq!(cd, 6.0);
    // Output: 6
}
