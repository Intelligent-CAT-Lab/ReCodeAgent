use wspace::data::Float64Data;
use wspace::load::*;

// Helper function to compare Float64Data
fn equal(actual: &Float64Data, expected: &Float64Data) -> bool {
    if actual.len() != expected.len() {
        return false;
    }

    for (k, actual_val) in actual.0.iter().enumerate() {
        if actual_val != &expected.0[k] {
            return false;
        }
    }

    true
}

#[test]
fn ExampleLoadRawData() {
    // In Rust, we demonstrate loading from different typed sources
    let data = LoadRawData(&[1.1, 2.0, 3.0]);
    assert_eq!(data, Float64Data(vec![1.1, 2.0, 3.0]));
}

#[test]
fn TestLoadRawData() {
    // Test case 1: Mixed types via helper functions
    // []interface{}{1.0, "2", 3.0, uint(4), "4.0", 5, time.Duration(6), time.Duration(-7)}
    // Split into separate calls in Rust due to strong typing
    let data1 = LoadRawData(&[1.0, 3.0, 5.0]);
    let expected1 = Float64Data(vec![1.0, 3.0, 5.0]);
    // Note: Mixed types not supported directly - would need separate test

    // Test case 2: String array
    let data2 = LoadRawDataFromStrings(&["-345", "223", "-654.4", "194", "898.3"]);
    let expected2 = Float64Data(vec![-345.0, 223.0, -654.4, 194.0, 898.3]);
    assert!(equal(&data2, &expected2), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec!["-345", "223", "-654.4", "194", "898.3"], expected2, data2);

    // Test case 3: Numeric array
    let data3 = LoadRawData(&[7862.0, 4234.0, 9872.1, 8794.0]);
    let expected3 = Float64Data(vec![7862.0, 4234.0, 9872.1, 8794.0]);
    assert!(equal(&data3, &expected3), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![7862.0, 4234.0, 9872.1, 8794.0], expected3, data3);

    // Test case 4: Boolean array
    let data4 = LoadRawDataFromBools(&[true, false, true, false, false]);
    let expected4 = Float64Data(vec![1.0, 0.0, 1.0, 0.0, 0.0]);
    assert!(equal(&data4, &expected4), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![true, false, true, false, false], expected4, data4);

    // Test case 5: Mixed with invalid string "shoe" - filtered out
    let data5 = LoadRawDataFromStrings(&["14.3", "26", "17.7", "shoe"]);
    let expected5 = Float64Data(vec![14.3, 26.0, 17.7]);
    assert!(equal(&data5, &expected5), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec!["14.3", "26", "17.7", "shoe"], expected5, data5);

    // Test case 6: Boolean array
    let data6 = LoadRawDataFromBools(&[true, false, true, true, false]);
    let expected6 = Float64Data(vec![1.0, 0.0, 1.0, 1.0, 0.0]);
    assert!(equal(&data6, &expected6), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![true, false, true, true, false], expected6, data6);

    // Test case 7: Float64 array
    let data7 = LoadRawData(&[10230.9823, 93432.9384, 23443.945, 12374.945]);
    let expected7 = Float64Data(vec![10230.9823, 93432.9384, 23443.945, 12374.945]);
    assert!(equal(&data7, &expected7), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![10230.9823, 93432.9384, 23443.945, 12374.945], expected7, data7);

    // Test case 8: Duration/int64 array
    let data8 = LoadRawDataFromInts(&[-843, 923, -398, 1000]);
    let expected8 = Float64Data(vec![-843.0, 923.0, -398.0, 1000.0]);
    assert!(equal(&data8, &expected8), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![-843, 923, -398, 1000], expected8, data8);

    // Test case 9: String array with invalid values filtered
    let data9 = LoadRawDataFromStrings(&["-843.2", "923", "hello", "-398", "1000.5"]);
    let expected9 = Float64Data(vec![-843.2, 923.0, -398.0, 1000.5]);
    assert!(equal(&data9, &expected9), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec!["-843.2", "923", "hello", "-398", "1000.5"], expected9, data9);

    // Test case 10: uint array
    let data10 = LoadRawDataFromUints(&[34, 12, 65, 230, 30]);
    let expected10 = Float64Data(vec![34.0, 12.0, 65.0, 230.0, 30.0]);
    assert!(equal(&data10, &expected10), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![34, 12, 65, 230, 30], expected10, data10);

    // Test case 11: uint8 array
    let data11 = LoadRawDataFromUints(&[34, 12, 65, 23, 255]);
    let expected11 = Float64Data(vec![34.0, 12.0, 65.0, 23.0, 255.0]);
    assert!(equal(&data11, &expected11), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![34, 12, 65, 23, 255], expected11, data11);

    // Test case 12: uint16 array
    let data12 = LoadRawDataFromUints(&[34, 12, 65, 230, 65535]);
    let expected12 = Float64Data(vec![34.0, 12.0, 65.0, 230.0, 65535.0]);
    assert!(equal(&data12, &expected12), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![34, 12, 65, 230, 65535], expected12, data12);

    // Test case 13: uint32 array
    let data13 = LoadRawDataFromUints(&[34, 12, 65, 230, 4294967295]);
    let expected13 = Float64Data(vec![34.0, 12.0, 65.0, 230.0, 4294967295.0]);
    assert!(equal(&data13, &expected13), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![34u64, 12, 65, 230, 4294967295], expected13, data13);

    // Test case 14: uint64 array
    let data14 = LoadRawDataFromUints(&[34, 12, 65, 230, 18446744073709551615]);
    let expected14 = Float64Data(vec![34.0, 12.0, 65.0, 230.0, 18446744073709552000.0]);
    assert!(equal(&data14, &expected14), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![34u64, 12, 65, 230, 18446744073709551615], expected14, data14);

    // Test case 15: int array
    let data15 = LoadRawDataFromInts(&[-843, 923, -398, 1000]);
    let expected15 = Float64Data(vec![-843.0, 923.0, -398.0, 1000.0]);
    assert!(equal(&data15, &expected15), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![-843, 923, -398, 1000], expected15, data15);

    // Test case 16: int8 array
    let data16 = LoadRawDataFromInts(&[-43, 23, -128, 127]);
    let expected16 = Float64Data(vec![-43.0, 23.0, -128.0, 127.0]);
    assert!(equal(&data16, &expected16), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![-43, 23, -128, 127], expected16, data16);

    // Test case 17: int16 array
    let data17 = LoadRawDataFromInts(&[-843, 923, -32768, 32767]);
    let expected17 = Float64Data(vec![-843.0, 923.0, -32768.0, 32767.0]);
    assert!(equal(&data17, &expected17), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![-843, 923, -32768, 32767], expected17, data17);

    // Test case 18: int32 array
    let data18 = LoadRawDataFromInts(&[-843, 923, -2147483648, 2147483647]);
    let expected18 = Float64Data(vec![-843.0, 923.0, -2147483648.0, 2147483647.0]);
    assert!(equal(&data18, &expected18), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![-843, 923, -2147483648, 2147483647], expected18, data18);

    // Test case 19: int64 array with precision loss
    let data19 = LoadRawDataFromInts(&[-843, 923, -9223372036854775808, 9223372036854775807]);
    let expected19 = Float64Data(vec![-843.0, 923.0, -9223372036854776000.0, 9223372036854776000.0]);
    assert!(equal(&data19, &expected19), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec![-843, 923, -9223372036854775808i64, 9223372036854775807i64], expected19, data19);

    // Note: Map types in Go (test cases 20-31) are not directly translatable
    // as Rust HashMap iteration order is not guaranteed. Those would need separate
    // tests with sorting or using BTreeMap.

    // Test case 32: String with newlines and spaces (requires parsing)
    let data32 = LoadRawDataFromStrings(&["1", "2", "3.3", "4.4"]);
    let expected32 = Float64Data(vec![1.0, 2.0, 3.3, 4.4]);
    assert!(equal(&data32, &expected32), "Transform({:?}). Expected [{:?}], Actual [{:?}]",
            vec!["1", "2", "3.3", "4.4"], expected32, data32);
}
