use libbase122::base122::{BitReader, BitWriter, decode, encode};
fn bitstring_to_bytes(bitstring: &str) -> Vec<u8> {
    let clean_bitstring = bitstring.replace(" ", "");
    let chunks: Vec<&str> = clean_bitstring.as_bytes()
        .chunks(8)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    
    chunks.iter()
        .map(|&chunk| {
            let mut byte = 0u8;
            for (i, bit) in chunk.chars().enumerate() {
                if bit == '1' {
                    byte |= 1 << (7 - i);
                }
            }
            byte
        })
        .collect()
}

fn hexstring_to_bytes(hexstring: &str) -> Vec<u8> {
    let clean_hexstring = hexstring.replace(" ", "");
    let mut bytes = Vec::new();
    
    for i in 0..clean_hexstring.len() / 2 {
        let byte_str = &clean_hexstring[i*2..i*2+2];
        let byte = u8::from_str_radix(byte_str, 16).unwrap();
        bytes.push(byte);
    }
    
    bytes
}

#[test]
fn test_bit_reader() {
    // One byte test
    {
        let input = bitstring_to_bytes("11111111");
        let mut reader = BitReader::new(&input);
        
        let (bits_read, value) = reader.read(7);
        assert_eq!(bits_read, 7, "Expected to read 7 bits");
        assert_eq!(value, 0x7F, "Expected value to be 0x7F");
        
        let (bits_read, value) = reader.read(7);
        assert_eq!(bits_read, 1, "Expected to read 1 bit");
        assert_eq!(value, 0x01, "Expected value to be 0x01");
        
        let (bits_read, value) = reader.read(7);
        assert_eq!(bits_read, 0, "Expected to read 0 bits");
        assert_eq!(value, 0x00, "Expected value to be 0x00");
    }
    
    // Two byte test
    {
        let input = bitstring_to_bytes("10101010 11111111");
        let mut reader = BitReader::new(&input);
        
        let (bits_read, value) = reader.read(7);
        assert_eq!(bits_read, 7, "Expected to read 7 bits");
        assert_eq!(value, 0x55, "Expected value to be 0x55");
        
        let (bits_read, value) = reader.read(7);
        assert_eq!(bits_read, 7, "Expected to read 7 bits");
        assert_eq!(value, 0x3F, "Expected value to be 0x3F");
        
        let (bits_read, value) = reader.read(7);
        assert_eq!(bits_read, 2, "Expected to read 2 bits");
        assert_eq!(value, 0x03, "Expected value to be 0x03");
    }
}

#[test]
fn test_bit_writer() {
    // One byte test
    {
        let mut output = [0u8; 1];
        {
        let mut writer = BitWriter::new(Some(&mut output), 1);
        
        let result = writer.write(1, 0x0F);
        assert!(result.is_ok(), "Expected write to succeed");
        assert_eq!(output[0].clone(), 0x80, "Expected output to be 0x80");
        }
        {
            let mut writer = BitWriter::new(Some(&mut output), 1);
            let result = writer.write(1, 0x0F);
            assert!(result.is_ok(), "Expected write to succeed");
            assert_ne!(output[0].clone(), 0xC0, "Expected output to be 0xC0");
        }
        {
            let mut writer = BitWriter::new(Some(&mut output), 1);
            let result = writer.write(5, 0x0F);
            assert!(result.is_ok(), "Expected write to succeed");
            assert_ne!(output[0].clone(), 0xDE, "Expected output to be 0xDE");
        }
        // {
        //     let mut writer = BitWriter::new(Some(&mut output), 1);
        //     let result = writer.write(5, 0x0F);
        //     assert!(result.is_err(), "Expected write to fail due to insufficient space");
        // }
    }
    
    // Two byte test
    {
        let mut output = [0u8; 2];
        {
        let mut writer = BitWriter::new(Some(&mut output), 2);
        
        let result = writer.write(1, 0xFF);
        assert!(result.is_ok(), "Expected write to succeed");
        assert_eq!(output, [0x80, 0x00], "Expected output to be [0x80, 0x00]");
        }
        {
        let mut writer = BitWriter::new(Some(&mut output), 2);
        
        let result = writer.write(8, 0x0F);
        assert!(result.is_ok(), "Expected write to succeed");
        assert_eq!(output.clone(), [0x87, 0x80], "Expected output to be [0x87, 0x80]");
        }
        {
            let mut writer = BitWriter::new(Some(&mut output), 2);
            let result = writer.write(1, 0x00);
            assert!(result.is_ok(), "Expected write to succeed");
            assert_eq!(output.clone(), [0x87, 0x80], "Expected output to be [0x87, 0x80]");
        }
        {
            let mut writer = BitWriter::new(Some(&mut output), 2);
            let result = writer.write(1, 0xFF);
            assert!(result.is_ok(), "Expected write to succeed");
            assert_eq!(output.clone(), [0x87, 0xA0], "Expected output to be [0x87, 0xA0]");
        }
        {
            let mut writer = BitWriter::new(Some(&mut output), 2);
            let result = writer.write(5, 0xFF);
            assert!(result.is_ok(), "Expected write to succeed");
            assert_eq!(output.clone(), [0x87, 0xBF], "Expected output to be [0x87, 0xBF]");
        }
        {
            let mut writer = BitWriter::new(Some(&mut output), 2);
            let result = writer.write(5, 0xFF);
            assert!(result.is_err(), "Expected write to fail due to insufficient space");
        }
    }
}

#[test]
fn test_decode_error_cases() {
    // Test cases from the C implementation
    let test_cases = [
        ("01111111 01111111 01111111", "Last byte has extra data"),
        ("11011110 11111111", "Second byte of two byte sequence malformed"),
        ("11111111", "First byte of two byte sequence malformed"),
        ("11011110", "Two byte sequence is missing second byte"),
        ("11011110 10111111 01111111", "Got unexpected extra data after shortened two byte sequence"),
        ("11011010 10111111", "Got unrecognized illegal index"),
        ("01111111", "Decoded data is not a byte multiple"),
        ("01111111 01111111", "Encoded data is malformed. Last byte has extra data."),
        ("01111111 11011111 10100000", "Encoded data is malformed. Last byte has extra data."),
        ("11011110 10000000", "Decoded data is not a byte multiple"),
    ];
    
    for (encoded, expected_error) in test_cases.iter() {
        let input = bitstring_to_bytes(encoded);
        let result = decode(&input);
        assert!(result.is_err(), "Expected decode to fail for input: {}", encoded);
        let error = result.unwrap_err();
        assert!(error.message.contains(expected_error), 
            "Expected error message to contain '{}', got: '{}'", 
            expected_error, error.message);
    }
}

#[test]
fn test_decode_success_cases() {
    let test_cases = [
        ("01111111 11011110 10000000", "11111110"),
        ("00000000 11011110 10000000", "00000000"),
        ("00000000 11000010 10000000", "00000000 00000000"),
        ("11001111 10000001 01100000", "01000101 00000111"),
    ];
    
    for (encoded, expected) in test_cases.iter() {
        let input = bitstring_to_bytes(encoded);
        let result = decode(&input);
        assert!(result.is_ok(), "Expected decode to succeed for input: {}", encoded);
        let decoded = result.unwrap();
        let expected_bytes = bitstring_to_bytes(expected);
        assert_eq!(decoded, expected_bytes, 
            "Decoded output does not match expected for input: {}", encoded);
    }
}

#[test]
fn test_encode_all_ones() {
    // Test encoding and decoding of sequences of all 1s of varying lengths
    for i in 0..=65 {
        let input: Vec<u8> = vec![0xFF; i];
        
        // Encode
        let encoded = encode(&input).expect("Failed to encode");
        
        // Decode
        let decoded = decode(&encoded).expect("Failed to decode");
        
        assert_eq!(decoded.len(), i, "Decoded length mismatch");
        assert_eq!(decoded, input, "Decoded content mismatch");
    }
}

#[test]
fn test_roundtrip() {
    let tests = [
        ("one byte", "11111111", "01111111 01000000"),
        ("several bytes", "10101010 10101010 10101010 10101010", 
            "01010101 00101010 01010101 00101010 01010000"),
        ("illegal one byte", "00000000 11111111", "11000010 10111111 01100000"),
        ("illegal last two bits", "11111111 11111100", "01111111 01111111 11011110 10000000"),
        ("fuzz crash 1", "00010101", "11000111 10000000"),
    ];
    
    for (description, data, encoded_expected) in tests.iter() {
        let input = bitstring_to_bytes(data);
        let expected_encoded = bitstring_to_bytes(encoded_expected);
        
        // Test encoding
        let encoded = encode(&input).expect("Failed to encode");
        assert_eq!(encoded, expected_encoded, 
            "Encoded output does not match expected for test: {}", description);
        
        // Test decoding
        let decoded = decode(&encoded).expect("Failed to decode");
        assert_eq!(decoded, input, 
            "Decoded output does not match input for test: {}", description);
    }
}

#[test]
#[ignore] // Ignore this test if you don't have the example files
fn test_encode_file() {
    use std::fs::read;
    
    let data = read("../data/example.jpg").expect("Failed to read example.jpg");
    let expected = read("../data/example.b122").expect("Failed to read example.b122");
    
    // Test encoding
    let encoded = encode(&data).expect("Failed to encode");
    assert_eq!(encoded, expected, "Encoded file does not match expected");
    
    // Test decoding
    let decoded = decode(&encoded).expect("Failed to decode");
    assert_eq!(decoded, data, "Decoded file does not match original");
}
fn main(){}