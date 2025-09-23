use fft::fft::fft_inplace;
use fft::fft::FftComplex;

#[test]
fn test_fft_inplace() {
    // Initialize the input data
    let mut data = [
        FftComplex { real: 1.0, imag: 0.0 },
        FftComplex { real: -1.0, imag: 0.0 },
        FftComplex { real: 1.0, imag: 0.0 },
        FftComplex { real: -1.0, imag: 0.0 },
        FftComplex { real: 1.0, imag: 0.0 },
        FftComplex { real: -1.0, imag: 0.0 },
        FftComplex { real: 1.0, imag: 0.0 },
        FftComplex { real: -1.0, imag: 0.0 },
    ];

    // Expected output after fft_inplace
    let expected_output = [
        FftComplex { real: 0.0, imag: 0.0 },
        FftComplex { real: 0.0, imag: 0.0 },
        FftComplex { real: 0.0, imag: 0.0 },
        FftComplex { real: 0.0, imag: 0.0 },
        FftComplex { real: 8.0, imag: 0.0 },
        FftComplex { real: 0.0, imag: 0.0 },
        FftComplex { real: 0.0, imag: 0.0 },
        FftComplex { real: 0.0, imag: 0.0 },
    ];

    // Call the fft_inplace function
    fft_inplace(&mut data, 3);

    // Assert that the output matches the expected output
    for (i, &output) in data.iter().enumerate() {
        assert_eq!(output.real, expected_output[i].real, "Mismatch at index {}: real part", i);
        assert_eq!(output.imag, expected_output[i].imag, "Mismatch at index {}: imag part", i);
    }
}

fn main() {}