use crate::smoothing::{sma, smooth};
use crate::statistics::Metrics;

#[test]
fn test_sma_basic() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = sma(&data, 3, 3);
    
    // we should get 2 results:
    // [1,2,3] -> 2.0 and [4,5] -> 4.5
    assert_eq!(result.len(), 2, "Expected 2 values from SMA");
    assert!((result[0] - 2.0).abs() < 1e-10, "First value should be 2.0, got {}", result[0]);
    assert!((result[1] - 4.5).abs() < 1e-10, "Second value should be 4.5, got {}", result[1]);
}

#[test]
fn test_sma_slide_one() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = sma(&data, 3, 1);
    
    // Expected: [(1+2+3)/3, (2+3+4)/3, (3+4+5)/3] = [2.0, 3.0, 4.0] 
    assert_eq!(result.len(), 3);
    assert!((result[0] - 2.0).abs() < 1e-10);
    assert!((result[1] - 3.0).abs() < 1e-10);
    assert!((result[2] - 4.0).abs() < 1e-10);
}

#[test]
fn test_sma_with_nan() {
    let data = vec![1.0, f64::NAN, 3.0, 4.0];
    let result = sma(&data, 2, 2);
    
    // NaN should be treated as 0
    // Expected: [(1+0)/2, (3+4)/2] = [0.5, 3.5]
    assert_eq!(result.len(), 2);
    assert!((result[0] - 0.5).abs() < 1e-10, "First value should be 0.5, got {}", result[0]);
    assert!((result[1] - 3.5).abs() < 1e-10, "Second value should be 3.5, got {}", result[1]);
}

#[test]
fn test_sma_window_equals_slide() {
    // Test when window size equals slide size (no overlap)
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let result = sma(&data, 2, 2);
    
    // Expected: [(1+2)/2, (3+4)/2, (5+6)/2] = [1.5, 3.5, 5.5]
    assert_eq!(result.len(), 3);
    assert!((result[0] - 1.5).abs() < 1e-10);
    assert!((result[1] - 3.5).abs() < 1e-10);
    assert!((result[2] - 5.5).abs() < 1e-10);
}

#[test]
fn test_sma_window_greater_than_data() {
    // Test when window size is greater than data length
    let data = vec![1.0, 2.0, 3.0];
    let result = sma(&data, 5, 1);
    
    // No complete windows, should return empty
    assert!(result.is_empty());
}

#[test]
fn test_sma_last_incomplete_window() {
    // Test handling of the last incomplete window
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = sma(&data, 3, 3);
    
    // First window: (1+2+3)/3 = 2.0
    // Second window is incomplete (4+5)/2 = 4.5 and should be included
    assert_eq!(result.len(), 2);
    assert!((result[0] - 2.0).abs() < 1e-10);
    assert!((result[1] - 4.5).abs() < 1e-10);
}

#[test]
fn test_smooth_small_dataset() {
    let data = vec![1.0, 3.0, 2.0, 4.0, 3.0, 5.0, 4.0, 2.0];
    let result = smooth(&data, 2);
    
    // Just check that it runs and returns appropriate size
    assert!(!result.is_empty());
    assert!(result.len() <= data.len());
}

#[test]
fn test_smooth_empty() {
    let data: Vec<f64> = vec![];
    let result = smooth(&data, 2);
    
    // Should return empty vec
    assert!(result.is_empty());
}

#[test]
fn test_smooth_single_element() {
    let data = vec![42.0];
    let result = smooth(&data, 2);
    
    // Should return something (either same element or empty is acceptable)
    // Just verify it doesn't crash
    assert!(result.len() <= 1);
    if !result.is_empty() {
        assert_eq!(result[0], 42.0);
    }
}

#[test]
fn test_smooth_preserves_mean() {
    // Test that smoothing approximately preserves the mean value
    let data: Vec<f64> = (1..101).map(|x| x as f64).collect();
    let original_mean = Metrics::mean(&data);
    let smoothed = smooth(&data, 10);
    
    // Skip test if smoothed is empty
    if smoothed.is_empty() {
        return;
    }
    
    let smoothed_mean = Metrics::mean(&smoothed);
    
    // Allow a reasonable tolerance
    assert!((original_mean - smoothed_mean).abs() < original_mean * 0.1,
            "Means should be close: original {}, smoothed {}", 
            original_mean, smoothed_mean);
}

#[test]
fn test_smooth_reduces_roughness() {
    // Create data with artificial high roughness
    let mut data = Vec::with_capacity(100);
    for i in 0..100 {
        // Zigzag pattern to ensure high roughness
        data.push(if i % 2 == 0 { 10.0 } else { 0.0 });
    }
    
    let original_metrics = Metrics::new(data.clone());
    let smoothed = smooth(&data, 10);
    
    // Skip test if smoothed is empty
    if smoothed.is_empty() {
        return;
    }
    
    let smoothed_metrics = Metrics::new(smoothed);
    
    // For this artificial zigzag data, smoothing should definitely reduce roughness
    assert!(smoothed_metrics.roughness() < original_metrics.roughness(),
            "Smoothed roughness ({}) should be less than original roughness ({})", 
            smoothed_metrics.roughness(), original_metrics.roughness());
}

#[test]
fn test_smooth_with_nan_values() {
    // Test that smoothing handles NaN values gracefully
    let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    data.push(f64::NAN); // Add NaN at the end
    
    let result = smooth(&data, 2);
    
    // Verify it produces some output without crashing
    assert!(!result.is_empty());
    
    // Verify no NaN values in result
    for (i, &val) in result.iter().enumerate() {
        assert!(!val.is_nan(), "Result contains NaN at position {}", i);
    }
}

#[test]
fn test_smooth_seasonal_data() {
    // Create seasonal data
    let data: Vec<f64> = (0..200)
        .map(|i| {
            let i_f64 = i as f64;
            // Seasonal component + some noise to ensure roughness can be reduced
            (i_f64 * 0.1).sin() * 10.0 + (i_f64 * 0.6).sin() * 2.0
        })
        .collect();
    
    let result = smooth(&data, 10);
    
    // Verify it produces some output
    assert!(!result.is_empty());
}

#[test]
fn test_smooth_with_different_resolutions() {
    // Test smoothing with different resolution parameters
    let data: Vec<f64> = (0..100).map(|i| (i as f64 * 0.1).sin() + i as f64 * 0.01).collect();
    
    // Try different resolutions - just make sure each produces some output
    for &res in &[5, 10, 20] {
        let smoothed = smooth(&data, res);
        assert!(!smoothed.is_empty(), 
                "Resolution {} should produce non-empty output", res);
    }
}