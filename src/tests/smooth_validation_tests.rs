use crate::{smooth, statistics::Metrics};

/// Tests for validating the fixed ASAP implementation
#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_small_dataset_not_oversmoothed() {
        // Test with a small dataset (10 points) and low resolution (2)
        // This should not collapse to a single point
        let data = vec![1.0, 3.2, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let smoothed = smooth(&data, 2);
        
        // Should have at least 2 points (not over-smoothed)
        assert!(smoothed.len() >= 2, 
            "Small dataset was over-smoothed: {} points -> {} points", 
            data.len(), smoothed.len());
        
        // Original metrics
        let original_metrics = Metrics::new(data.clone());
        
        // If smoothing occurred, roughness should decrease
        if smoothed.len() < data.len() {
            let smoothed_metrics = Metrics::new(smoothed);
            assert!(smoothed_metrics.roughness() <= original_metrics.roughness(),
                "Smoothing increased roughness from {} to {}", 
                original_metrics.roughness(), smoothed_metrics.roughness());
        }
    }
    
    #[test]
    fn test_zigzag_pattern_smoothing() {
        // Test with a zigzag pattern that has high roughness
        let data: Vec<f64> = (0..50).map(|i| if i % 2 == 0 { 10.0 } else { 0.0 }).collect();
        let original_metrics = Metrics::new(data.clone());
        
        // Apply smoothing
        let smoothed = smooth(&data, 10);
        
        // Should reduce the size somewhat but not collapse entirely
        assert!(smoothed.len() >= 5, 
            "Zigzag pattern over-smoothed: {} -> {} points", 
            data.len(), smoothed.len());
        
        // Should significantly reduce roughness
        let smoothed_metrics = Metrics::new(smoothed);
        assert!(smoothed_metrics.roughness() < original_metrics.roughness() * 0.8,
            "Smoothing didn't reduce roughness enough: {} -> {}", 
            original_metrics.roughness(), smoothed_metrics.roughness());
    }
    
    #[test]
    fn test_seasonal_data_preserves_patterns() {
        // Create seasonal data with a clear pattern
        let data: Vec<f64> = (0..100).map(|i| {
            let i_f64 = i as f64;
            // Seasonal pattern with period = 20
            (i_f64 * 0.314).sin() * 5.0
        }).collect();
        
        let original_metrics = Metrics::new(data.clone());
        
        // Apply smoothing
        let smoothed = smooth(&data, 20);
        
        // Should have a reasonable number of points
        assert!(smoothed.len() >= 10, 
            "Seasonal data over-smoothed: {} -> {} points", 
            data.len(), smoothed.len());
        
        // Should preserve kurtosis (within 10%)
        let smoothed_metrics = Metrics::new(smoothed);
        assert!(smoothed_metrics.kurtosis() >= original_metrics.kurtosis() * 0.9,
            "Smoothing reduced kurtosis too much: {} -> {}", 
            original_metrics.kurtosis(), smoothed_metrics.kurtosis());
    }
    
    #[test]
    fn test_roughness_always_decreases() {
        // Test with various datasets that smoothing always decreases roughness
        let datasets = vec![
            // Simple linear trend
            (0..50).map(|i| i as f64 * 0.5).collect::<Vec<f64>>(),
            
            // Sine wave
            (0..50).map(|i| (i as f64 * 0.1).sin() * 10.0).collect::<Vec<f64>>(),
            
            // Random-like data (deterministic)
            (0..50).map(|i| ((i * 7919) % 104729) as f64 / 104729.0 * 100.0).collect::<Vec<f64>>(),
            
            // Zigzag
            (0..50).map(|i| if i % 2 == 0 { 5.0 } else { -5.0 }).collect::<Vec<f64>>()
        ];
        
        for (i, data) in datasets.iter().enumerate() {
            let original_metrics = Metrics::new(data.clone());
            
            // Skip if data has zero roughness
            if original_metrics.roughness() < 1e-10 {
                continue;
            }
            
            let smoothed = smooth(data, 10);
            
            // If any smoothing occurred
            if smoothed.len() < data.len() {
                let smoothed_metrics = Metrics::new(smoothed);
                assert!(smoothed_metrics.roughness() <= original_metrics.roughness(),
                    "Dataset {} - Smoothing increased roughness: {} -> {}", 
                    i, original_metrics.roughness(), smoothed_metrics.roughness());
            }
        }
    }
    
    #[test]
    fn test_various_resolutions() {
        // Test that different resolutions produce reasonable results
        let data: Vec<f64> = (0..200).map(|i| {
            let i_f64 = i as f64;
            // Trend + seasonal + noise
            i_f64 * 0.1 + (i_f64 * 0.05).sin() * 10.0 + ((i * 104729) % 15485863) as f64 / 15485863.0 * 5.0
        }).collect();
        
        let resolutions = [5, 10, 20, 50, 100];
        
        for &resolution in &resolutions {
            let smoothed = smooth(&data, resolution);
            
            // Output size should be related to resolution (roughly)
            assert!(smoothed.len() > 0, 
                "Resolution {} produced empty result", resolution);
                
            // Higher resolutions should produce more points
            if resolution > 10 {
                let lower_res_smoothed = smooth(&data, resolution / 2);
                assert!(smoothed.len() >= lower_res_smoothed.len(),
                    "Higher resolution ({}) produced fewer points than lower resolution ({}): {} vs {}", 
                    resolution, resolution/2, smoothed.len(), lower_res_smoothed.len());
            }
        }
    }
    
    #[test]
    fn test_mean_preservation() {
        // Test that smoothing approximately preserves the mean
        let data: Vec<f64> = (0..100).map(|i| i as f64 + ((i * 104729) % 15485863) as f64 / 15485863.0 * 10.0).collect();
        
        let original_mean = Metrics::mean(&data);
        let smoothed = smooth(&data, 10);
        
        if !smoothed.is_empty() {
            let smoothed_mean = Metrics::mean(&smoothed);
            
            // Mean should be preserved within 5%
            let percent_diff = ((smoothed_mean - original_mean) / original_mean).abs() * 100.0;
            assert!(percent_diff < 5.0,
                "Smoothing changed mean by {:.2}%: {} -> {}", 
                percent_diff, original_mean, smoothed_mean);
        }
    }
    
    #[test]
    fn test_extreme_resolution_values() {
        // Test with extreme resolution values
        let data: Vec<f64> = (0..100).map(|i| i as f64).collect();
        
        // Very low resolution
        let smoothed_low = smooth(&data, 1);
        assert!(!smoothed_low.is_empty(), "Resolution=1 produced empty result");
        
        // Very high resolution (higher than data length)
        let smoothed_high = smooth(&data, 200);
        assert_eq!(smoothed_high, data, 
            "Resolution > data.len() should return original data");
    }
    
    #[test]
    fn test_empty_and_single_element() {
        // Test with empty input
        let empty: Vec<f64> = vec![];
        let smoothed_empty = smooth(&empty, 10);
        assert!(smoothed_empty.is_empty(), "Empty input should produce empty output");
        
        // Test with single element
        let single = vec![42.0];
        let smoothed_single = smooth(&single, 10);
        assert_eq!(smoothed_single, single, "Single element should remain unchanged");
    }
    
    #[test]
    fn test_nan_handling() {
        // Test with NaN values
        let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        data.push(f64::NAN);
        
        let smoothed = smooth(&data, 2);
        
        // Result should not contain NaN
        for (i, &val) in smoothed.iter().enumerate() {
            assert!(!val.is_nan(), "Smoothed result contains NaN at position {}", i);
        }
    }
    
    #[test]
    fn test_large_dataset_performance() {
        // Generate a large dataset
        let data: Vec<f64> = (0..10000).map(|i| (i as f64 * 0.01).sin() * 10.0 + i as f64 * 0.1).collect();
        
        // Measure performance
        let start = std::time::Instant::now();
        let smoothed = smooth(&data, 100);
        let duration = start.elapsed();
        
        // Just a basic check that it completes in a reasonable time
        assert!(duration.as_secs() < 2, "Smoothing took too long: {:?}", duration);
        assert!(!smoothed.is_empty(), "Large dataset produced empty result");
    }
}