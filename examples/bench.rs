use asap_rs::smooth;
use rand::Rng;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();
    let data: Vec<f64> = (0..500000)
        .map(|_| rng.gen_range(-100.0..100.0))
        .collect();
    let data: Vec<f64> = data
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            x + i as f64 * 0.01 // Trend
                + (i as f64 * 0.1).sin() * 20.0 // Seasonality
        })
        .collect();
    let resolution = 25; 
    println!("Data size: {}", data.len());
    println!("First 10 data points: {:?}", &data[0..10]);
    // Measure and apply smoothing
    let start = Instant::now();
    let smoothed_data = smooth(&data, resolution);
    let duration = start.elapsed();
    println!("Smoothing completed in {:?}", duration);
    println!("Smoothed data size: {}", smoothed_data.len());
    println!("First 10 smoothed data points: {:?}", &smoothed_data[0..10]);

    // print some basic stats
    let original_mean = data.iter().sum::<f64>() / data.len() as f64;
    let smoothed_mean = smoothed_data.iter().sum::<f64>() / smoothed_data.len() as f64;

    println!("Original data mean: {:.2}", original_mean);
    println!("Smoothed data mean: {:.2}", smoothed_mean);
}