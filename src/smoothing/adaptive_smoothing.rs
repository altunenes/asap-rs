use crate::statistics::Metrics;
use crate::utils::ACF;
use super::sma::sma;

pub fn smooth(data: &[f64], resolution: usize) -> Vec<f64> {
    let mut data = data.to_vec();
    
    // Ignore the last value if it's NaN
    if data.last().map_or(false, |&x| x.is_nan()) {
        data.pop();
    }

    if data.len() >= 2 * resolution {
        let window = data.len() / resolution;
        data = sma(&data, window, window);
    }

    let mut acf = ACF::new(data.clone(), (data.len() / 10).max(1));
    let peaks = acf.find_peaks();
    let metrics = Metrics::new(data.clone());
    let original_kurt = metrics.kurtosis();
    let mut min_obj = metrics.roughness();
    let mut window_size = 1;
    let mut lb = 1;
    let mut largest_feasible = usize::MAX; // Changed from -1 to usize::MAX
    let mut tail = data.len() / 10; // Changed from i32 to usize (from js)

    for (i, &w) in peaks.iter().rev().enumerate() {
        if w < lb || w == 1 {
            break;
        } else if (1.0 - acf.correlations[w]).sqrt() * window_size as f64 >
                  (1.0 - acf.correlations[window_size]).sqrt() * w as f64 {
            continue;
        }

        let smoothed = sma(&data, w, 1);
        let metrics = Metrics::new(smoothed);
        let roughness = metrics.roughness();

        if metrics.kurtosis() >= original_kurt {
            if roughness < min_obj {
                min_obj = roughness;
                window_size = w;
            }
            lb = ((w as f64 * ((acf.max_acf - 1.0) / (acf.correlations[w] - 1.0)).sqrt()).round() as usize).max(lb);
            if largest_feasible == usize::MAX {
                largest_feasible = i;
            }
        }
    }

    if largest_feasible != usize::MAX {
        if largest_feasible < peaks.len().saturating_sub(2) {
            tail = peaks[largest_feasible + 1];
        }
        lb = lb.max(peaks[largest_feasible] + 1);
    }

    window_size = binary_search(lb, tail, &data, min_obj, original_kurt, window_size);

    sma(&data, window_size, 1)
}

fn binary_search(mut head: usize, mut tail: usize, data: &[f64], mut min_obj: f64, original_kurt: f64, mut window_size: usize) -> usize {
    while head <= tail {
        let w = (head + tail) / 2;
        let smoothed = sma(data, w, 1);
        let metrics = Metrics::new(smoothed);
        if metrics.kurtosis() >= original_kurt {
            // Search second half if feasible
            let roughness = metrics.roughness();
            if roughness < min_obj {
                window_size = w;
                min_obj = roughness;
            }
            head = w + 1;
        } else {
            // Search first half
            tail = w.saturating_sub(1);
        }
    }
    window_size
}