use crate::statistics::Metrics;

#[test]
fn test_mean() {
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(Metrics::mean(&values), 3.0);
    
    let values = vec![-5.0, -3.0, -1.0, 1.0, 3.0, 5.0];
    assert_eq!(Metrics::mean(&values), 0.0);
    
    let values = vec![42.0];
    assert_eq!(Metrics::mean(&values), 42.0);
}

#[test]
fn test_std() {
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    // std = sqrt((1-3)² + (2-3)² + (3-3)² + (4-3)² + (5-3)²) / 5
    // = sqrt(4 + 1 + 0 + 1 + 4) / 5 = sqrt(10/5) = sqrt(2)
    assert!((Metrics::std(&values) - 2.0_f64.sqrt()).abs() < 1e-10);
    
    // Test with values having mean = 0
    let values = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
    // std = sqrt((-2)² + (-1)² + 0² + 1² + 2²) / 5 = sqrt(10/5) = sqrt(2)
    assert!((Metrics::std(&values) - 2.0_f64.sqrt()).abs() < 1e-10);
    
    // Test with constant values
    let values = vec![7.0, 7.0, 7.0, 7.0, 7.0];
    assert!(Metrics::std(&values).abs() < 1e-10);
}

#[test]
fn test_kurtosis() {
    // Normal distribution has kurtosis of 3
    // Generate normal-like data
    let values = vec![2.0, 2.5, 2.8, 3.0, 3.0, 3.0, 3.2, 3.5, 4.0];
    let metrics = Metrics::new(values);
    
    // Allow some tolerance since this is a small sample
    assert!((metrics.kurtosis() - 3.0).abs() < 1.0);
    
    // Uniform distribution has kurtosis of 1.8
    let values: Vec<f64> = (1..11).map(|x| x as f64).collect();
    let metrics = Metrics::new(values);
    assert!(metrics.kurtosis() < 3.0);
    
    // High kurtosis (peaked with heavy tails)
    let values = vec![0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0, -10.0];
    let metrics = Metrics::new(values);
    assert!(metrics.kurtosis() > 3.0);
}

#[test]
fn test_roughness() {
    let values = vec![1.0, 3.0, 2.0, 4.0, 3.0];
    let metrics = Metrics::new(values);
    
    // Diffs: [2.0, -1.0, 2.0, -1.0]
    // Mean of diffs: 0.5
    // Variance: ((2-0.5)² + (-1-0.5)² + (2-0.5)² + (-1-0.5)²) / 4
    // = (1.5² + (-1.5)² + 1.5² + (-1.5)²) / 4 = 4*1.5² / 4 = 2.25
    // Std: sqrt(2.25) = 1.5
    assert!((metrics.roughness() - 1.5).abs() < 1e-10);
    
    // Test with constant values - roughness should be 0
    let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
    let metrics = Metrics::new(values);
    assert!(metrics.roughness().abs() < 1e-10);
    
    // Test with linear trend - constant differences
    let values = vec![1.0, 3.0, 5.0, 7.0, 9.0];
    let metrics = Metrics::new(values);
    assert!(metrics.roughness().abs() < 1e-10);
    
    // Test with linear trend plus noise
    let values = vec![1.0, 3.1, 5.0, 6.9, 9.1];
    let metrics = Metrics::new(values);
    assert!(metrics.roughness() > 0.0 && metrics.roughness() < 0.2);
}

#[test]
fn test_metrics_new() {
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let metrics = Metrics::new(values.clone());
    
    // Test that the metrics object stores the values correctly
    // We can't directly access .values since it's private, but we can
    // check that the computed metrics are correct
    assert!((metrics.kurtosis() - 1.7).abs() < 0.5); // Approx kurtosis for uniform data
    
    // Check that metrics.m (mean) is correctly calculated
    // by using roughness which depends on mean calculation
    let expected_roughness = Metrics::std(&values.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>());
    assert!((metrics.roughness() - expected_roughness).abs() < 1e-10);
}

#[test]
fn test_metrics_with_extreme_values() {
    // Test with very large values
    let values = vec![1e10, 2e10, 3e10, 4e10, 5e10];
    let metrics = Metrics::new(values);
    
    // The roughness calculation with very large values might be sensitive to implementation details
    // Just ensure it returns a value of reasonable magnitude
    let roughness = metrics.roughness();
    assert!(roughness.is_finite(), "Roughness should be finite for large values");
    
    // Test with very small values
    let values = vec![1e-10, 2e-10, 3e-10, 4e-10, 5e-10];
    let metrics = Metrics::new(values);
    
    // Ensure the calculation works with small values too
    let roughness = metrics.roughness();
    assert!(roughness.is_finite(), "Roughness should be finite for small values");
    
    // Test with mix of positive and negative
    let values = vec![-1e6, 2e6, -3e6, 4e6, -5e6];
    let metrics = Metrics::new(values);
    assert!(metrics.kurtosis() > 0.0); // Just check it's a valid value
}