//! Tests for FFT module functionality

use crate::fft::{transform, inverse_transform, convolve_real, convolve_complex};

#[test]
fn test_fft_identity() {
    // Test that FFT followed by inverse FFT returns the original signal
    let mut real = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut imag = vec![0.0; 8];
    let original_real = real.clone();
    
    // Forward transform
    transform(&mut real, &mut imag).unwrap();
    
    // Inverse transform
    inverse_transform(&mut real, &mut imag).unwrap();
    
    // Scale the result manually by 1/n
    let n = real.len() as f64;
    for i in 0..real.len() {
        real[i] /= n;
        imag[i] /= n;
    }
    
    // Check that we get back approximately the original
    for i in 0..real.len() {
        assert!((real[i] - original_real[i]).abs() < 1e-10, 
                "Element at index {} differs: {} vs {}", i, real[i], original_real[i]);
    }
}

#[test]
fn test_fft_known_transform() {
    // Test against a known transform pair
    // DC component (constant signal) should have energy only in the first bin
    let mut real = vec![1.0, 1.0, 1.0, 1.0];
    let mut imag = vec![0.0; 4];
    
    transform(&mut real, &mut imag).unwrap();
    
    // DC component should be the sum of the input
    assert!((real[0] - 4.0).abs() < 1e-10, "DC component mismatch: {} vs 4.0", real[0]);
    assert!(imag[0].abs() < 1e-10, "DC component imag should be 0, got {}", imag[0]);
    
    // All other bins should be approximately zero
    for i in 1..real.len() {
        assert!(real[i].abs() < 1e-10, "Bin {} real part should be 0, got {}", i, real[i]);
        assert!(imag[i].abs() < 1e-10, "Bin {} imag part should be 0, got {}", i, imag[i]);
    }
}

#[test]
fn test_sine_wave_transform() {
    // Test with a sine wave (should have energy in frequency bin k and N-k)
    let n = 16;
    let mut real = vec![0.0; n];
    let mut imag = vec![0.0; n];
    
    // Generate a sine wave with exactly 2 cycles
    for i in 0..n {
        real[i] = (2.0 * std::f64::consts::PI * 2.0 * i as f64 / n as f64).sin();
    }
    
    transform(&mut real, &mut imag).unwrap();
    
    // Energy should be primarily in bins 2 and N-2
    let epsilon = 1e-10;
    
    // All bins except 2 and N-2 should be close to zero
    for i in 0..n {
        if i != 2 && i != n-2 {
            assert!(real[i].abs() < epsilon, "Bin {} real part should be 0, got {}", i, real[i]);
            assert!(imag[i].abs() < epsilon, "Bin {} imag part should be 0, got {}", i, imag[i]);
        }
    }
    
    // Bins 2 and N-2 should have significant values
    assert!(real[2].abs() < epsilon, "Bin 2 real part should be 0, got {}", real[2]);
    assert!(imag[2].abs() > 1.0, "Bin 2 imag part should be significant, got {}", imag[2]);
    
    assert!(real[n-2].abs() < epsilon, "Bin N-2 real part should be 0, got {}", real[n-2]);
    assert!(imag[n-2].abs() > 1.0, "Bin N-2 imag part should be significant, got {}", imag[n-2]);
}

#[test]
fn test_convolution() {
    // Test convolution with a simple case
    let x = vec![1.0, 2.0, 3.0, 4.0];
    let y = vec![0.5, 0.5, 0.0, 0.0];
    
    let result = convolve_real(&x, &y).unwrap();
    
    // Expected values from circular convolution
    // [1.0*0.5 + 4.0*0.5, 2.0*0.5 + 1.0*0.5, 3.0*0.5 + 2.0*0.5, 4.0*0.5 + 3.0*0.5]
    // [0.5 + 2.0, 1.0 + 0.5, 1.5 + 1.0, 2.0 + 1.5] = [2.5, 1.5, 2.5, 3.5]
    assert_eq!(result.len(), 4);
    assert!((result[0] - 2.5).abs() < 1e-10, "Expected 2.5, got {}", result[0]);
    assert!((result[1] - 1.5).abs() < 1e-10, "Expected 1.5, got {}", result[1]);
    assert!((result[2] - 2.5).abs() < 1e-10, "Expected 2.5, got {}", result[2]);
    assert!((result[3] - 3.5).abs() < 1e-10, "Expected 3.5, got {}", result[3]);
}

#[test]
fn test_complex_convolution() {
    // Test complex convolution
    let xreal = vec![1.0, 0.0, 0.0, 0.0];
    let ximag = vec![0.0, 0.0, 0.0, 0.0];
    let yreal = vec![1.0, 2.0, 3.0, 4.0];
    let yimag = vec![0.0, 1.0, 0.0, -1.0];
    
    let mut outreal = vec![0.0; 4];
    let mut outimag = vec![0.0; 4];
    
    convolve_complex(&xreal, &ximag, &yreal, &yimag, &mut outreal, &mut outimag).unwrap();
    
    // Complex impulse convolution should equal the second signal
    for i in 0..4 {
        assert!((outreal[i] - yreal[i]).abs() < 1e-10, 
                "Real part at index {}: {} vs {}", i, outreal[i], yreal[i]);
        assert!((outimag[i] - yimag[i]).abs() < 1e-10, 
                "Imag part at index {}: {} vs {}", i, outimag[i], yimag[i]);
    }
}

#[test]
fn test_non_power_of_two() {
    // Test that the FFT works for non-power-of-two lengths
    let mut real = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut imag = vec![0.0; 5];
    let original = real.clone();
    
    // Forward transform (should use Bluestein algorithm https://ccrma.stanford.edu/~jos/st/Bluestein_s_FFT_Algorithm.html)
    transform(&mut real, &mut imag).unwrap();
    
    // Inverse transform
    inverse_transform(&mut real, &mut imag).unwrap();
    
    // Scale the result manually
    let n = real.len() as f64;
    for i in 0..real.len() {
        real[i] /= n;
        imag[i] /= n;
    }
    
    // Check that we get back approximately the original
    for i in 0..real.len() {
        assert!((real[i] - original[i]).abs() < 1e-10);
    }
}

#[test]
fn test_error_handling() {
    // Test error handling for mismatched lengths
    let mut real = vec![1.0, 2.0];
    let mut imag = vec![0.0, 0.0, 0.0];
    
    let result = transform(&mut real, &mut imag);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Mismatched lengths");
    
    // Test error handling for other functions
    let x = vec![1.0, 2.0];
    let y = vec![1.0, 2.0, 3.0];
    let result = convolve_real(&x, &y);
    assert!(result.is_err());
}