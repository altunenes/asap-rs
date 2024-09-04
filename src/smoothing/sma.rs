pub fn sma(data: &[f64], range: usize, slide: usize) -> Vec<f64> {
    let mut window_start = 0;
    let mut sum = 0.0;
    let mut count = 0;
    let mut values = Vec::new();

    for (i, &value) in data.iter().enumerate() {
        let value = if value.is_nan() { 0.0 } else { value };
        if i - window_start >= range {
            values.push(sum / count as f64);
            let old_start = window_start;
            while window_start < data.len() && window_start - old_start < slide {
                sum -= data[window_start];
                count -= 1;
                window_start += 1;
            }
        }
        sum += value;
        count += 1;
    }

    if count == range {
        values.push(sum / count as f64);
    }

    values
}
