use libm17::encode::SYMBOL_LIST;
use libm17::math::{golay24_encode, golay24_sdecode, soft_bit_xor, DECODE_MATRIX, ENCODE_MATRIX};

use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec;
#[test]
fn soft_logic_xor() {
    assert_eq!(0x0000, soft_bit_xor(0x0000, 0x0000));
    assert_eq!(0x7FFE, soft_bit_xor(0x0000, 0x7FFF));
    assert_eq!(0xFFFE, soft_bit_xor(0x0000, 0xFFFF));
    assert_eq!(0x7FFE, soft_bit_xor(0x7FFF, 0x0000));
    assert_eq!(0x7FFE, soft_bit_xor(0x7FFF, 0x7FFF));
    assert_eq!(0x7FFF, soft_bit_xor(0x7FFF, 0xFFFF));
    assert_eq!(0xFFFE, soft_bit_xor(0xFFFF, 0x0000));
    assert_eq!(0x7FFF, soft_bit_xor(0xFFFF, 0x7FFF));
    assert_eq!(0x0000, soft_bit_xor(0xFFFF, 0xFFFF));
}

fn symbol_to_soft_dibit(symb_in: f32) -> [u16; 2] {
    let mut dibit = [0u16; 2];

    // First bit
    if symb_in >= (SYMBOL_LIST[2] as f32) {
        dibit[0] = 0x0000;
    } else if symb_in >= (SYMBOL_LIST[1] as f32) {
        dibit[0] =
            0x7FFF - ((symb_in * 0xFFFF as f32) / (SYMBOL_LIST[2] - SYMBOL_LIST[1]) as f32) as u16;
    } else {
        dibit[0] = 0xFFFF;
    }

    // Second bit
    if symb_in >= SYMBOL_LIST[3] as f32 {
        dibit[1] = 0xFFFF;
    } else if symb_in >= SYMBOL_LIST[2] as f32 {
        let out = (-0xFFFF as f32) / ((SYMBOL_LIST[3] - SYMBOL_LIST[2]) as f32)
            * (SYMBOL_LIST[2] as f32)
            + symb_in * (0xFFFF as f32) / ((SYMBOL_LIST[3] - SYMBOL_LIST[2]) as f32);
        dibit[1] = out as u16;
    } else if symb_in >= SYMBOL_LIST[1] as f32 {
        dibit[1] = 0x0000;
    } else if symb_in >= SYMBOL_LIST[0] as f32 {
        dibit[1] = (0xFFFF as f32 / (SYMBOL_LIST[1] - SYMBOL_LIST[0]) as f32
            * SYMBOL_LIST[1] as f32
            - symb_in * 0xFFFF as f32 / (SYMBOL_LIST[1] - SYMBOL_LIST[0]) as f32)
            as u16;
    } else {
        dibit[1] = 0xFFFF;
    }

    dibit
}

fn apply_errors(vect: &mut [u16; 24], start_pos: u8, end_pos: u8, num_errs: u8, sum_errs: f32) {
    if end_pos < start_pos {
        panic!("Invalid bit range");
    }

    let num_bits = end_pos - start_pos + 1;
    if num_errs > num_bits || num_bits > 24 || num_errs > 24 || sum_errs > num_errs as f32 {
        panic!("Impossible combination of error value and number of bits");
    }

    let val = (0xFFFF as f32 * sum_errs / num_errs as f32).round() as u16;
    let mut err_loc: u32 = 0;
    let mut rng = rand::thread_rng();

    for _ in 0..num_errs {
        let mut bit_pos;
        loop {
            bit_pos = start_pos + (rng.gen::<u8>() % num_bits);
            if err_loc & (1 << bit_pos) == 0 {
                break;
            }
        }
        vect[bit_pos as usize] ^= val;
        err_loc |= 1 << bit_pos;
    }
}
#[test]
fn symbol_to_dibit() {
    let test_values = [30.0, 4.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -3.0, -4.0, -30.0];
    let expected_results = [
        [0x0000, 0xFFFF],
        [0x0000, 0xFFFF],
        [0x0000, 0xFFFF],
        [0x0000, 0x7FFF],
        [0x0000, 0x0000],
        [0x7FFF, 0x0000],
        [0xFFFE, 0x0000],
        [0xFFFF, 0x7FFF],
        [0xFFFF, 0xFFFF],
        [0xFFFF, 0xFFFF],
        [0xFFFF, 0xFFFF],
    ];

    for (i, &val) in test_values.iter().enumerate() {
        let dibit = symbol_to_soft_dibit(val);
        assert_eq!(dibit[0], expected_results[i][0]);
        assert_eq!(dibit[1], expected_results[i][1]);
    }
}
#[test]
fn golay_encode() {
    let mut data: u16 = 0x0800;
    for i in (1..ENCODE_MATRIX.len()).rev() {
        assert_eq!(
            ((data << 12) | ENCODE_MATRIX[i]) as u32,
            golay24_encode(data)
        );
        data >>= 1;
    }

    data = 0x0D78;
    assert_eq!(0x0D7880F, golay24_encode(data));
}
#[test]
fn test_golay_soft_decode() {
    let test_params = [
        ("flipped_parity_1", 0, 11, 1, 1.0, true),
        ("erased_parity_1", 0, 11, 1, 0.5, true),
        ("flipped_parity_2", 0, 11, 2, 2.0, true),
        ("erased_parity_2", 0, 11, 2, 1.0, true),
        ("flipped_parity_3", 0, 11, 3, 3.0, true),
        ("erased_parity_3", 0, 11, 3, 1.5, true),
        ("erased_parity_3_5", 0, 11, 7, 3.5, false),
        ("erased_parity_5", 0, 11, 5, 2.5, false),
        ("flipped_parity_5", 0, 11, 5, 5.0, false),
        ("flipped_data_1", 12, 23, 1, 1.0, false),
        ("erased_data_1", 12, 23, 1, 0.5, true),
        ("flipped_data_2", 12, 23, 2, 2.0, true),
        ("erased_data_2", 12, 23, 2, 1.0, true),
        ("flipped_data_3", 12, 23, 3, 3.0, true),
        ("erased_data_3", 12, 23, 3, 1.5, true),
        ("erased_data_3_5", 12, 23, 7, 3.5, true),
        ("erased_data_5", 12, 23, 5, 2.5, true),
        ("flipped_data_5", 12, 23, 5, 5.0, false),
    ];

    for (name, start_pos, end_pos, num_errs, sum_errs, is_equal) in test_params.iter() {
        let mut vector: [u16; 24] = [0; 24];

        for i in 0..24 {
            vector[23 - i] = (((0x0D7880F >> i) & 1) * 0xFFFF) as u16;
        }

        for _ in 0..1000 {
            apply_errors(&mut vector, *start_pos, *end_pos, *num_errs, *sum_errs);
            let result = golay24_sdecode(&vector);
            if *is_equal {
                assert_eq!(0x0D78, result, "Test case: {}", name);
            } else {
                assert_ne!(0x0D78, result, "Test case: {}", name);
            }
            for i in 0..24 {
                vector[23 - i] = (((0x0D7880F >> i) & 1) * 0xFFFF) as u16;
            }
        }
    }
}
#[test]
fn golay_soft_decode_flipped_parity_4() {
    let mut vector: [u16; 24] = [0; 24];
    for i in 0..24 {
        vector[23 - i] = (((0x0D7880F >> i) & 1) * 0xFFFF) as u16;
    }

    vector[6] ^= 0xFFFF;
    vector[7] ^= 0xFFFF;
    vector[8] ^= 0xFFFF;
    vector[11] ^= 0xFFFF;

    assert_ne!(0x0D78, golay24_sdecode(&vector));
    for i in 0..24 {
        vector[23 - i] = (((0x0D7880F >> i) & 1) * 0xFFFF) as u16;
    }

    vector[6] ^= 0xFFFF;
    vector[7] ^= 0xFFFF;
    vector[8] ^= 0xFFFF;
    vector[9] ^= 0xFFFF;
    assert_eq!(0x0D78, golay24_sdecode(&vector));
    for i in 0..24 {
        vector[23 - i] = (((0x0D7880F >> i) & 1) * 0xFFFF) as u16;
    }
}

//4 errors is exactly half the hamming distance, so due to rounding etc., results may vary
//therefore we run 2 tests here to prove that
#[test]
fn golay_soft_decode_flipped_data_4() {
    let mut vector: [u16; 24] = [0; 24];
    for i in 0..24 {
        vector[23 - i] = (((0x0D7880F >> i) & 1) * 0xFFFF) as u16;
    }

    vector[12] ^= 0xFFFF;
    vector[13] ^= 0xFFFF;
    vector[16] ^= 0xFFFF;
    vector[22] ^= 0xFFFF;
    assert_ne!(0x0D78, golay24_sdecode(&vector));
    for i in 0..24 {
        vector[23 - i] = (((0x0D7880F >> i) & 1) * 0xFFFF) as u16;
    }

    vector[14] ^= 0xFFFF;
    vector[16] ^= 0xFFFF;
    vector[17] ^= 0xFFFF;
    vector[20] ^= 0xFFFF;
    assert_eq!(0x0D78, golay24_sdecode(&vector));
    for i in 0..24 {
        vector[23 - i] = (((0x0D7880F >> i) & 1) * 0xFFFF) as u16;
    }
}
#[test]
fn golay_soft_decode_corrupt_data_4_5() {
    let mut vector: [u16; 24] = [0; 24];
    for i in 0..24 {
        vector[23 - i] = (((0x0D7880F >> i) & 1) * 0xFFFF) as u16;
    }

    apply_errors(&mut vector, 12, 23, 12, 4.5);
    assert_eq!(0x0D78, golay24_sdecode(&vector));
    for i in 0..24 {
        vector[23 - i] = (((0x0D7880F >> i) & 1) * 0xFFFF) as u16;
    }
}

fn main() {}
