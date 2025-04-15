use asap_rs::{smooth, statistics::Metrics};

fn main() {
    println!("ASAP-RS: Automatic Smoothing for Attention Prioritization");
    println!("=======================================================\n");
    
    // Example 1: Basic smoothing of a simple dataset
    println!("Example 1: Basic Smoothing");
    println!("-------------------------");
    
    // Example data with fluctuations
    let data = [1.0, 3.2, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    println!("Original data: {:?}", data);
    
    // Calculate roughness of original data
    let original_metrics = Metrics::new(data.to_vec());
    println!("Original roughness: {:.4}", original_metrics.roughness());
    println!("Original kurtosis: {:.4}", original_metrics.kurtosis());
    
    // Apply ASAP smoothing with different resolutions
    for &resolution in &[2, 5] {
        let smoothed_data = smooth(&data, resolution);
        
        // Calculate metrics for smoothed data
        let smoothed_metrics = Metrics::new(smoothed_data.clone());
        println!("\nSmoothed data (resolution={}): {:?}", resolution, smoothed_data);
        println!("Smoothed roughness: {:.4}", smoothed_metrics.roughness());
        println!("Smoothed kurtosis: {:.4}", smoothed_metrics.kurtosis());
        
        let roughness_change = (smoothed_metrics.roughness() - original_metrics.roughness()) / 
                              original_metrics.roughness() * 100.0;
        println!("Roughness change: {:.1}%", roughness_change);
    }
    
    println!("\n\nExample 2: Zigzag Pattern Smoothing");
    println!("----------------------------------");
    
    // Create a zigzag pattern (high roughness)
    let zigzag: Vec<f64> = (0..20).map(|i| if i % 2 == 0 { 10.0 } else { 0.0 }).collect();
    println!("Original zigzag data: {:?}", zigzag);
    
    // Calculate roughness of zigzag data
    let original_metrics = Metrics::new(zigzag.clone());
    println!("Original roughness: {:.4}", original_metrics.roughness());
    
    // Apply ASAP smoothing
    let smoothed_zigzag = smooth(&zigzag, 5);
    
    // Calculate metrics for smoothed data
    let smoothed_metrics = Metrics::new(smoothed_zigzag.clone());
    println!("\nSmoothed zigzag data: {:?}", smoothed_zigzag);
    println!("Smoothed roughness: {:.4}", smoothed_metrics.roughness());
    
    let roughness_reduction = (original_metrics.roughness() - smoothed_metrics.roughness()) / 
                             original_metrics.roughness() * 100.0;
    println!("Roughness reduction: {:.1}%", roughness_reduction);
    
    println!("\n\nExample 3: Seasonal Data");
    println!("------------------------");
    
    // Create seasonal data with trend
    let seasonal: Vec<f64> = (0..50).map(|i| {
        let i_f64 = i as f64;
        // Base trend
        let trend = i_f64 * 0.1;
        // Seasonal component (period = 10)
        let seasonal = (i_f64 * 0.628).sin() * 5.0;
        // Noise component
        let noise = ((i_f64 * 1.5).cos() + (i_f64 * 3.7).sin()) * 0.5;
        
        trend + seasonal + noise
    }).collect();
    
    println!("First 10 points of seasonal data: {:?}", &seasonal[0..10]);
    
    // Calculate metrics of original seasonal data
    let original_metrics = Metrics::new(seasonal.clone());
    println!("Original roughness: {:.4}", original_metrics.roughness());
    println!("Original kurtosis: {:.4}", original_metrics.kurtosis());
    
    // Apply ASAP smoothing with different resolutions
    for &resolution in &[5, 10, 20] {
        let smoothed = smooth(&seasonal, resolution);
        
        // Calculate metrics for smoothed data
        let smoothed_metrics = Metrics::new(smoothed.clone());
        println!("\nSmoothed with resolution={} (length: {})", resolution, smoothed.len());
        println!("First 10 points: {:?}", &smoothed.iter().take(10).collect::<Vec<_>>());
        println!("Smoothed roughness: {:.4}", smoothed_metrics.roughness());
        println!("Smoothed kurtosis: {:.4}", smoothed_metrics.kurtosis());
        
        let roughness_change = (smoothed_metrics.roughness() - original_metrics.roughness()) / 
                              original_metrics.roughness() * 100.0;
        println!("Roughness change: {:.1}%", roughness_change);
    }
}