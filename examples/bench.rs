use asap_rs::{smooth, statistics::Metrics};
use std::time::Instant;

/// Generate deterministic psuedo-random data without external dependencies
fn generate_test_data(size: usize) -> Vec<f64> {
    let mut data = Vec::with_capacity(size);
    
    // Generate deterministic values using simple hash function
    for i in 0..size {
        // Simple number generator based on index
        // Using prime multipliers for better distribution
        let base = ((i * 15485863) % 32452843) as f64 / 32452843.0 * 200.0 - 100.0;
        
        // Add trend and seasonality
        let trend = i as f64 * 0.01;
        let seasonality = (i as f64 * 0.1).sin() * 20.0;
        
        data.push(base + trend + seasonality);
    }
    
    data
}

fn main() {
    println!("ASAP-RS Benchmark");
    println!("================\n");
    
    // Generate test data
    let data_size = 500000;
    println!("Generating {} data points...", data_size);
    let data = generate_test_data(data_size);
    
    println!("Data size: {}", data.len());
    println!("First 10 data points: {:?}", &data[0..10]);
    
    // Calculate original metrics
    let original_metrics = Metrics::new(data[0..1000].to_vec()); // Sample first 1000 points
    println!("\nOriginal roughness (first 1000 points): {:.4}", original_metrics.roughness());
    println!("Original kurtosis (first 1000 points): {:.4}", original_metrics.kurtosis());
    
    // Test with different resolutions
    let resolutions = [25, 100, 500];
    
    for &resolution in &resolutions {
        // Measure and apply smoothing
        println!("\nApplying ASAP smoothing with resolution {}...", resolution);
        let start = Instant::now();
        let smoothed_data = smooth(&data, resolution);
        let duration = start.elapsed();
        
        println!("Smoothing completed in {:?}", duration);
        println!("Smoothed data size: {}", smoothed_data.len());
        println!("Compression ratio: {:.1}x", data.len() as f64 / smoothed_data.len() as f64);
        println!("First 10 smoothed data points: {:?}", &smoothed_data[0..smoothed_data.len().min(10)]);
    
        // Calculate smoothed metrics
        if !smoothed_data.is_empty() {
            let sample_size = 1000.min(smoothed_data.len());
            let smoothed_metrics = Metrics::new(smoothed_data[0..sample_size].to_vec());
            println!("Smoothed roughness (first {} points): {:.4}", sample_size, smoothed_metrics.roughness());
            println!("Smoothed kurtosis (first {} points): {:.4}", sample_size, smoothed_metrics.kurtosis());
            
            // Calculate roughness reduction
            let roughness_change = (smoothed_metrics.roughness() - original_metrics.roughness()) / 
                                  original_metrics.roughness() * 100.0;
            println!("Roughness change: {:.1}%", roughness_change);
        }
    
        // print some basic stats
        let original_mean = data.iter().sum::<f64>() / data.len() as f64;
        let smoothed_mean = if !smoothed_data.is_empty() {
            smoothed_data.iter().sum::<f64>() / smoothed_data.len() as f64
        } else {
            0.0
        };
    
        println!("Original data mean: {:.2}", original_mean);
        println!("Smoothed data mean: {:.2}", smoothed_mean);
    }
}