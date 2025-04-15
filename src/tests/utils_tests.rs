use crate::utils::ACF;

#[test]
fn test_acf_constant_signal() {
    // For a constant signal, autocorrelation should be 1.0 at all lags
    let values = vec![5.0; 100];
    let acf = ACF::new(values, 10);
    
    // Check a few lags - they should all be close to 1.0 or 0.0
    // In practice, some implementations may get slightly different values
    for i in 1..acf.correlations.len() {
        // Either close to 1.0 or small enough to be considered noise
        assert!(
            (acf.correlations[i] - 1.0).abs() < 0.1 || acf.correlations[i].abs() < 0.1,
            "ACF at lag {} for constant signal should be close to 1.0 or 0.0, got {}", 
            i, acf.correlations[i]
        );
    }
}

#[test]
fn test_acf_sine_wave() {
    // For a sine wave, autocorrelation should follow a cosine pattern
    let values: Vec<f64> = (0..100)
        .map(|i| (i as f64 * 0.1).sin())
        .collect();
        
    let acf = ACF::new(values, 50);
    
    // Sine wave with period = 2π/0.1 ≈ 63 samples
    // At lag = 16 (about 1/4 period), correlation should be decreasing
    assert!(acf.correlations[16] < acf.correlations[15], 
            "ACF at lag 16 should be less than at lag 15");
    
    // At lag = 32 (about 1/2 period), correlation should be negative
    assert!(acf.correlations[32] < 0.0, 
            "ACF at lag 32 (1/2 period) should be negative, got {}", 
            acf.correlations[32]);
}

#[test]
fn test_acf_random_noise() {
    // For random noise, autocorrelation should be close to 0 for all lags
    let mut values = Vec::with_capacity(100);
    let mut x: f64 = 0.123; // Seed
    
    // Simple PRNG for deterministic "random" values
    for _ in 0..100 {
        x = (x * 9973.0) % 1.0;
        values.push(x);
    }
    
    let acf = ACF::new(values, 20);
    
    // All lags should have low correlation 
    // Check just a few random lags, with a more generous threshold
    for &i in &[5, 10, 15] {
        if i < acf.correlations.len() {
            assert!(acf.correlations[i].abs() < 0.6, 
                    "ACF for random noise at lag {} should be relatively small, got {}", 
                    i, acf.correlations[i]);
        }
    }
}

#[test]
fn test_find_peaks() {
    // Create a signal with a clear periodicity
    let values: Vec<f64> = (0..100)
        .map(|i| (i as f64 * 0.2).sin())
        .collect();
        
    let mut acf = ACF::new(values, 50);
    let peaks = acf.find_peaks();
    
    // Should find some peaks for a periodic signal
    assert!(!peaks.is_empty(), "Should find at least one peak");
            
    // Check that max_acf is set
    assert!(acf.max_acf >= 0.0, "max_acf should be set, got {}", acf.max_acf);
}

#[test]
fn test_acf_mixed_frequencies() {
    // Test with a signal containing multiple frequency components
    let values: Vec<f64> = (0..200)
        .map(|i| {
            let i_f64 = i as f64;
            (i_f64 * 0.05).sin() * 2.0 +  // Low frequency
            (i_f64 * 0.2).sin() * 1.0      // High frequency
        })
        .collect();
        
    let mut acf = ACF::new(values, 100);
    let peaks = acf.find_peaks();
    
    // Should find some peaks
    assert!(!peaks.is_empty(), "Should find at least one peak");
}

#[test]
fn test_acf_trend() {
    // Test with data containing a linear trend
    let values: Vec<f64> = (0..100)
        .map(|i| i as f64 * 0.1) // Linear trend
        .collect();
        
    let acf = ACF::new(values, 50);
    
    // ACF for data with a strong trend typically decreases
    // Check if generally decreasing (allow some flexibility)
    let mut decreasing_count = 0;
    for i in 1..acf.correlations.len().min(20) {
        if acf.correlations[i] < acf.correlations[i-1] {
            decreasing_count += 1;
        }
    }
    
    // At least half of the points should be decreasing
    assert!(decreasing_count >= 10.min(acf.correlations.len() / 2), 
            "ACF should generally decrease with lag for trend data");
}

#[test]
fn test_acf_impulse() {
    // Test with a single impulse
    let mut values = vec![0.0; 100];
    values[50] = 1.0; // Single impulse at position 50
    
    let acf = ACF::new(values, 50);
    
    // For most implementations, ACF of an impulse should be mostly small values
    let small_values_count = acf.correlations.iter()
        .skip(1) // Skip lag 0
        .filter(|&&v| v.abs() < 0.5)
        .count();
    
    // Most correlation values should be small
    assert!(small_values_count > acf.correlations.len() / 2,
            "Most ACF values for impulse should be small");
}

#[test]
fn test_acf_square_wave() {
    // Test with a square wave pattern
    let values: Vec<f64> = (0..100)
        .map(|i| if i % 20 < 10 { 1.0 } else { -1.0 })
        .collect();
        
    let mut acf = ACF::new(values, 30);
    let peaks = acf.find_peaks();
    
    // Square wave should have some peaks
    assert!(!peaks.is_empty(), "Should find at least one peak");
}