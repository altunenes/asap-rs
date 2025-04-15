//! This example demonstrates how ASAP handles different types of time series patterns

use asap_rs::{smooth, statistics::Metrics};
use std::f64::consts::PI;

fn main() {
    println!("ASAP-RS Pattern Comparison");
    println!("==========================\n");

    // Test ASAP on several different pattern types
    compare_linear_trend();
    compare_periodic();
    compare_zigzag();
    compare_random_walk();
    compare_outliers();
    compare_mixed_patterns();
}

// Helper function to print metrics and results
fn print_results(_name: &str, data: &[f64], smoothed: &[f64]) {
    let original_metrics = Metrics::new(data.to_vec());
    let smoothed_metrics = if !smoothed.is_empty() {
        Metrics::new(smoothed.to_vec())
    } else {
        return;
    };
    
    let roughness_change = (smoothed_metrics.roughness() - original_metrics.roughness()) / 
                           original_metrics.roughness() * 100.0;
    let kurtosis_change = (smoothed_metrics.kurtosis() - original_metrics.kurtosis()) / 
                          original_metrics.kurtosis() * 100.0;
    
    println!("Compression: {:.1}x ({} → {} points)", 
             data.len() as f64 / smoothed.len() as f64,
             data.len(), smoothed.len());
    println!("Roughness: {:.4} → {:.4} ({:.1}% change)", 
             original_metrics.roughness(), smoothed_metrics.roughness(), roughness_change);
    println!("Kurtosis: {:.4} → {:.4} ({:.1}% change)", 
             original_metrics.kurtosis(), smoothed_metrics.kurtosis(), kurtosis_change);
    println!();
}

// 1. Linear trend with noise
fn compare_linear_trend() {
    println!("\n=== LINEAR TREND WITH NOISE ===");
    println!("A steadily increasing series with random fluctuations");
    
    // Generate data with linear trend and noise
    let data: Vec<f64> = (0..100).map(|i| {
        let trend = i as f64 * 0.5;  // Linear trend
        let noise = ((i * 16807) % 2147483647) as f64 / 2147483647.0 * 5.0 - 2.5;
        trend + noise
    }).collect();
    
    println!("First 10 points: {:?}", &data[0..10]);
    
    // Apply ASAP smoothing
    let smoothed = smooth(&data, 20);
    println!("Smoothed (first 10): {:?}", &smoothed[0..smoothed.len().min(10)]);
    
    print_results("Linear trend", &data, &smoothed);
    println!("→ ASAP effectively removes noise while preserving the underlying trend");
}

// 2. Periodic pattern
fn compare_periodic() {
    println!("\n=== PERIODIC PATTERN ===");
    println!("A sine wave with added noise");
    
    // Generate periodic data with noise
    let data: Vec<f64> = (0..120).map(|i| {
        let i_f64 = i as f64;
        // Sine wave with period of 20 points
        let periodic = (i_f64 * PI / 10.0).sin() * 10.0;
        // Noise component
        let noise = ((i * 48271) % 2147483647) as f64 / 2147483647.0 * 4.0 - 2.0;
        periodic + noise
    }).collect();
    
    println!("First 10 points: {:?}", &data[0..10]);
    
    // Apply ASAP smoothing
    let smoothed = smooth(&data, 20);
    println!("Smoothed (first 10): {:?}", &smoothed[0..smoothed.len().min(10)]);
    
    print_results("Periodic", &data, &smoothed);
    println!("→ ASAP identifies the periodic nature and smooths accordingly");
}

// 3. Zigzag pattern
fn compare_zigzag() {
    println!("\n=== ZIGZAG PATTERN ===");
    println!("A rapidly alternating pattern with high roughness");
    
    // Generate zigzag pattern
    let data: Vec<f64> = (0..50).map(|i| if i % 2 == 0 { 10.0 } else { 0.0 }).collect();
    
    println!("First 10 points: {:?}", &data[0..10]);
    
    // Apply ASAP smoothing
    let smoothed = smooth(&data, 10);
    println!("Smoothed (first 10): {:?}", &smoothed[0..smoothed.len().min(10)]);
    
    print_results("Zigzag", &data, &smoothed);
    println!("→ ASAP detects the zigzag pattern and applies specialized smoothing");
}

// 4. Random walk
fn compare_random_walk() {
    println!("\n=== RANDOM WALK ===");
    println!("Each point depends on the previous point plus random change");
    
    // Generate random walk data
    let mut data = Vec::with_capacity(100);
    let mut value = 50.0;
    
    for i in 0..100 {
        // Random step between -1 and 1
        let step = ((i * 69621) % 2147483647) as f64 / 2147483647.0 * 2.0 - 1.0;
        value += step;
        data.push(value);
    }
    
    println!("First 10 points: {:?}", &data[0..10]);
    
    // Apply ASAP smoothing
    let smoothed = smooth(&data, 20);
    println!("Smoothed (first 10): {:?}", &smoothed[0..smoothed.len().min(10)]);
    
    print_results("Random walk", &data, &smoothed);
    println!("→ ASAP reduces local fluctuations while preserving the overall path");
}

// 5. Data with outliers
fn compare_outliers() {
    println!("\n=== DATA WITH OUTLIERS ===");
    println!("Normal data with occasional extreme values");
    
    // Generate data with outliers
    let data: Vec<f64> = (0..100).map(|i| {
        let base = (i as f64 * 0.2).sin() * 5.0;  // Base pattern
        
        // Add occasional outliers
        if i % 20 == 0 {
            base + 20.0
        } else if i % 33 == 0 {
            base - 15.0
        } else {
            base
        }
    }).collect();
    
    println!("First 10 points: {:?}", &data[0..10]);
    
    // Apply ASAP smoothing
    let smoothed = smooth(&data, 20);
    println!("Smoothed (first 10): {:?}", &smoothed[0..smoothed.len().min(10)]);
    
    print_results("Outliers", &data, &smoothed);
    println!("→ ASAP's kurtosis preservation ensures outliers remain visible");
}

// 6. Mixed complex pattern
fn compare_mixed_patterns() {
    println!("\n=== MIXED COMPLEX PATTERN ===");
    println!("Combination of trend, seasonality, and noise");
    
    // Generate complex pattern with multiple components
    let data: Vec<f64> = (0..200).map(|i| {
        let i_f64 = i as f64;
        
        // Linear trend
        let trend = i_f64 * 0.05;
        
        // Multiple seasonal components
        let daily = (i_f64 * 2.0 * PI / 20.0).sin() * 5.0;  // "Daily" cycle
        let weekly = (i_f64 * 2.0 * PI / 140.0).sin() * 10.0;  // "Weekly" cycle
        
        // Noise
        let noise = ((i * 16807) % 2147483647) as f64 / 2147483647.0 * 3.0 - 1.5;
        
        trend + daily + weekly + noise
    }).collect();
    
    println!("First 10 points: {:?}", &data[0..10]);
    
    // Try different resolutions
    for &resolution in &[10, 50] {
        println!("\nResolution = {}", resolution);
        let smoothed = smooth(&data, resolution);
        println!("Smoothed (first 10): {:?}", &smoothed[0..smoothed.len().min(10)]);
        
        print_results(&format!("Mixed (res={})", resolution), &data, &smoothed);
    }
    
    println!("  Note: Very low resolutions (10) can be inappropriate for complex patterns, resulting in increased roughness.");
}