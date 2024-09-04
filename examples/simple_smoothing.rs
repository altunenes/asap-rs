use asap_rs::smooth;

fn main() {
    // Example data
    let data = [1.0,3.2,2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    let resolution = 2;

    // Apply smoothing
    let smoothed_data = smooth(&data, resolution);

    println!("Original data: {:?}", data);
    println!("Smoothed data: {:?}", smoothed_data);
}