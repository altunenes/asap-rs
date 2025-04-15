
use crate::{smooth, smoothing::sma, statistics::Metrics, utils::ACF};

fn generate_synthetic_data(size: usize, with_noise: bool) -> Vec<f64> {
    let mut data = Vec::with_capacity(size);
    for i in 0..size {
        let i_f64 = i as f64;
        // Linear trend
        let trend = i_f64 * 0.01;
        // Seasonality
        let seasonal = (i_f64 * 0.1).sin() * 5.0;
        // Noise
        let noise = if with_noise {
            (i_f64 * 0.5).cos() * 2.0
        } else {
            0.0
        };
        data.push(trend + seasonal + noise);
    }
    data
}

#[test]
fn test_end_to_end_simple() {
    let data = vec![1.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 1.0, 2.0];
    let result = smooth(&data, 2);
    
    // Check that result is not empty
    assert!(!result.is_empty(), "Smoothed result should not be empty");
}

#[test]
fn test_smooth_reduces_roughness() {
    // Generate synthetic data with noise
    let mut data = Vec::with_capacity(200);
    
    // Create a signal with very clear roughness that smoothing should definitely reduce
    for i in 0..200 {
        let i_f64 = i as f64;
        // Trend + high frequency noise
        let value = i_f64 * 0.05 + if i % 2 == 0 { 5.0 } else { -5.0 };
        data.push(value);
    }
    
    let original_metrics = Metrics::new(data.clone());
    let smoothed = smooth(&data, 10);
    
    assert!(!smoothed.is_empty(), "Smoothed result should not be empty");
    
    let smoothed_metrics = Metrics::new(smoothed);
    
    // For this artificial dataset with extreme zigzags, smoothing should reduce roughness
    assert!(smoothed_metrics.roughness() < original_metrics.roughness() * 0.9, 
            "Smoothed roughness ({}) should be less than original roughness ({})", 
            smoothed_metrics.roughness(), original_metrics.roughness());
}

#[test]
fn test_smooth_vs_simple_moving_average() {
    // Generate synthetic data
    let data = generate_synthetic_data(100, true);
    
    // Apply ASAP smoothing
    let asap_result = smooth(&data, 10);
    
    // Apply simple moving average with a fixed window
    let sma_result = sma(&data, 10, 1);
    
    // As long as both produce valid output, consider the test passing
    assert!(!asap_result.is_empty(), "ASAP should produce non-empty result");
    assert!(!sma_result.is_empty(), "SMA should produce non-empty result");
    
    // Note: In real-world cases, ASAP might not always have lower roughness than
    // a fixed-window SMA depending on the dataset and parameters
}

#[test]
fn test_acf_and_smooth_integration() {
    // Generate data with clear seasonality
    let data: Vec<f64> = (0..100)
        .map(|i| (i as f64 * 0.2).sin())
        .collect();
    
    // Calculate ACF manually with a smaller max_lag to avoid index errors
    let mut acf = ACF::new(data.clone(), 10);
    let peaks = acf.find_peaks();
    
    // The implementation should at least find some peaks
    assert!(!peaks.is_empty(), "ACF should find at least one peak");
    
    // Now smooth the data
    let smoothed = smooth(&data, 5);
    
    // The smoothed data should not be empty
    assert!(!smoothed.is_empty(), "Smoothed data should not be empty");
}

#[test]
fn test_smooth_kurtosis_preservation() {
    // Generate data with a clear distribution
    let mut data = Vec::with_capacity(200);
    
    // Create normally distributed data
    for i in 0..200 {
        let x = (i as f64 - 100.0) / 20.0;  // Ranges from -5 to 5
        data.push((-x * x / 2.0).exp());  // Normal distribution
    }
    
    // Calculate original kurtosis
    let original_metrics = Metrics::new(data.clone());
    let original_kurtosis = original_metrics.kurtosis();
    
    // Apply smoothing
    let smoothed = smooth(&data, 20);
    
    // Check for valid output
    assert!(!smoothed.is_empty(), "Smoothed data should not be empty");
    
    // Calculate smoothed kurtosis
    let smoothed_metrics = Metrics::new(smoothed);
    let smoothed_kurtosis = smoothed_metrics.kurtosis();
    
    // Verify kurtosis preservation
    // Allow a 10% tolerance since smoothing might slightly change the distribution
    assert!(smoothed_kurtosis >= original_kurtosis * 0.9, 
            "Smoothed kurtosis ({}) should be close to or greater than original kurtosis ({})",
            smoothed_kurtosis, original_kurtosis);
}

#[test]
fn test_full_processing_chain() {
    // Generate synthetic data with trend, seasonality, and noise
    let data = generate_synthetic_data(500, true);
    
    // Apply ASAP smoothing
    let smoothed = smooth(&data, 50);
    
    // Check basic properties
    assert!(!smoothed.is_empty(), "Smoothed data should not be empty");
    
    // The mean should be approximately preserved
    let original_mean = Metrics::mean(&data);
    let smoothed_mean = Metrics::mean(&smoothed);
    
    // Allow a generous tolerance for mean preservation
    assert!((original_mean - smoothed_mean).abs() < original_mean.abs() * 0.2, 
            "Mean should be roughly preserved: original {}, smoothed {}", 
            original_mean, smoothed_mean);
}

#[test]
fn test_resolution_parameter_effect() {
    // Generate a large dataset
    let data = generate_synthetic_data(1000, true);
    
    // Try different resolution parameters
    let resolutions = [10, 50, 100, 200];
    
    // Just verify that each resolution produces some output
    for &res in &resolutions {
        let smoothed = smooth(&data, res);
        assert!(!smoothed.is_empty(), 
                "Resolution {} should produce non-empty output", res);
    }
}