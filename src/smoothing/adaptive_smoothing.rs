//! Adaptive smoothing implementation for time series data.
//!
//! This module provides the core implementation of the ASAP algorithm that automatically
//! determines the optimal window size for smoothing time series data to highlight important
//! patterns while reducing noise.
//!
//! ## Algorithm Details
//!
//! The core ASAP algorithm works as follows:
//!
//! 1. For time series with strong periodicity, use autocorrelation to identify candidate
//!    window sizes aligned with the periods in the data
//!
//! 2. For each candidate window size, apply a simple moving average and evaluate:
//!    - The roughness of the result (standard deviation of differences)
//!    - Whether kurtosis is preserved (to maintain the shape of the distribution)
//!
//! 3. Select the window size that minimizes roughness while preserving kurtosis
//!
//! 4. For data without strong periodicity, use binary search to find the optimal window size
//!
//! ## Pattern-Specific Handling
//!
//! This implementation includes specialized detection and handling for different types of patterns:
//!
//! ### Zigzag Patterns
//!
//! Zigzag patterns (rapidly alternating high-low values) present a special challenge for smoothing
//! algorithms because:
//!
//! - They have extremely high roughness due to constant direction changes
//! - Any smoothing dramatically alters their kurtosis
//! - The strict kurtosis preservation constraint would prevent any effective smoothing
//!
//! The implementation detects zigzag patterns by analyzing:
//!
//! - The roughness relative to the data range
//! - The percentage of points that alternate direction
//!
//! For detected zigzag patterns, the algorithm:
//!
//! - Uses a relaxed kurtosis constraint (allowing up to 40% reduction instead of 10%)
//! - Prioritizes a window size of 2, which is mathematically optimal for alternating patterns
//! - Ensures the resulting smoothed data effectively reduces the zigzag while preserving the mean
//!
//! ### Small Datasets
//!
//! For small datasets, the implementation uses a simplified approach with direct testing of
//! a few window sizes rather than the full autocorrelation analysis, to avoid over-smoothing.


use crate::statistics::Metrics;
use crate::utils::ACF;
use super::sma::sma;

pub fn smooth(data: &[f64], resolution: usize) -> Vec<f64> {
    // Return empty result for empty input
    if data.is_empty() {
        return Vec::new();
    }
    
    let mut data = data.to_vec();
    
    // Ignore the last value if it's NaN
    if data.last().map_or(false, |&x| x.is_nan()) {
        data.pop();
    }
    
    // If data is empty after removing NaN, return empty result
    if data.is_empty() {
        return Vec::new();
    }

    // For very small datasets relative to resolution, avoid over-smoothing
    if data.len() <= resolution * 2 {
        return data;
    }

    // Only perform preaggregation if we have significantly more points than resolution
    let target_min_points = resolution.max(10);
    
    if data.len() > target_min_points * 10 {
        // Calculate window size to achieve roughly target_min_points * 2
        // This ensures we don't overly compress the data initially
        let window = (data.len() / (target_min_points * 2)).max(1);
        let slide = window; // Non-overlapping windows
        data = sma(&data, window, slide);
    }

    // If we're down to very few points after preaggregation, just return
    if data.len() <= 2 {
        return data;
    }

    // Calculate metrics for original (potentially preaggregated) data
    let metrics = Metrics::new(data.clone());
    let original_kurt = metrics.kurtosis();
    let original_roughness = metrics.roughness();
    
    // NEW: Check for zigzag pattern
    let is_zigzag_pattern = detect_zigzag_pattern(&data, original_roughness);
    
    // Adjust kurtosis constraint based on data characteristics
    // For zigzag patterns, we'll be more aggressive in smoothing
    let kurtosis_threshold = if is_zigzag_pattern {
        // Allow up to 40% reduction in kurtosis for zigzag patterns
        original_kurt * 0.6
    } else {
        // Regular case - allow up to 10% reduction
        original_kurt * 0.9
    };

    // For small datasets, use a simple approach
    if data.len() < 20 {
        // Try a few simple window sizes and pick the best one
        let candidates = [2, 3, 5, 7];
        let mut best_window = 1;
        let mut min_roughness = original_roughness;
        
        for &w in &candidates {
            if w >= data.len() {
                continue;
            }
            
            let smoothed = sma(&data, w, 1);
            if smoothed.is_empty() {
                continue;
            }
            
            let metrics = Metrics::new(smoothed);
            let roughness = metrics.roughness();
            
            // Only consider if kurtosis constraint is satisfied
            if metrics.kurtosis() >= kurtosis_threshold && roughness < min_roughness {
                min_roughness = roughness;
                best_window = w;
            }
        }
        
        // If we found a better window, use it; otherwise return original
        if best_window > 1 {
            return sma(&data, best_window, 1);
        }
        
        // Special case for zigzag patterns - if nothing worked above but it's a zigzag,
        // force a window size of 2 to smooth it out
        if is_zigzag_pattern {
            return sma(&data, 2, 1);
        }
        
        return data;
    }

    // ==== For larger datasets, use the ASAP algorithm with improvements ====
    
    // Calculate ACF with a safe max_lag (avoid empty data issues)
    let max_lag = (data.len() / 10).max(1).min(50); // Cap at 50 to avoid excessive computation
    let mut acf = ACF::new(data.clone(), max_lag);
    let peaks = acf.find_peaks();
    
    // If no peaks found, use a reasonable default window
    if peaks.is_empty() {
        // ==== FIXED: More balanced default window size ====
        let window_size = (data.len() / 10).max(2).min(data.len() / 2);
        let smoothed = sma(&data, window_size, 1);
        
        // Check if this improves roughness while preserving kurtosis
        let metrics = Metrics::new(smoothed.clone());
        if metrics.kurtosis() >= kurtosis_threshold && metrics.roughness() < original_roughness {
            return smoothed;
        }
        
        // If zigzag, try with window size 2 (which specifically handles alternating patterns)
        if is_zigzag_pattern {
            let smoothed = sma(&data, 2, 1);
            let metrics = Metrics::new(smoothed.clone());
            if metrics.roughness() < original_roughness {
                return smoothed;
            }
        }
        
        // If not, return original data
        return data;
    }
    
    // Initialize search variables
    let mut min_obj = original_roughness;
    let mut window_size = 1; // Start with original data (no smoothing)
    let mut lb = 1;
    let mut largest_feasible = usize::MAX;
    let mut tail = (data.len() / 10).min(peaks[peaks.len() - 1]);

    // Search peaks from largest to smallest with better constraints
    for (i, &w) in peaks.iter().rev().enumerate() {
        // Skip if window size is too large (prevents over-smoothing)
        if w > data.len() / 2 {
            continue;
        }
        
        if w < lb || w == 1 {
            break;
        } else if (1.0 - acf.correlations[w]).sqrt() * window_size as f64 >
                  (1.0 - acf.correlations[window_size]).sqrt() * w as f64 {
            continue;
        }

        let smoothed = sma(&data, w, 1);
        if smoothed.len() < 2 { // Skip if smoothed result is too small
            continue;
        }
        
        let metrics = Metrics::new(smoothed);
        let roughness = metrics.roughness();

        // Use the adjusted kurtosis threshold
        if metrics.kurtosis() >= kurtosis_threshold {
            if roughness < min_obj {
                min_obj = roughness;
                window_size = w;
            }
            // Update lower bound for future searches
            let acf_ratio = ((acf.max_acf - 1.0) / (acf.correlations[w] - 1.0)).sqrt();
            if acf_ratio.is_finite() && !acf_ratio.is_nan() {
                lb = ((w as f64 * acf_ratio).round() as usize).max(lb);
            }
            if largest_feasible == usize::MAX {
                largest_feasible = i;
            }
        }
    }

    // For zigzag patterns, if we haven't found a good window yet, force a window of 2
    if window_size == 1 && is_zigzag_pattern {
        window_size = 2;
    }

    // If we found a feasible window, refine with binary search
    if window_size > 1 {
        if largest_feasible != usize::MAX && largest_feasible < peaks.len().saturating_sub(2) {
            tail = peaks[largest_feasible + 1];
        }
        lb = lb.max(if largest_feasible != usize::MAX { peaks[largest_feasible] + 1 } else { 1 });
        
        // Ensure binary search range is reasonable
        if lb < tail && lb < data.len() / 2 {
            window_size = binary_search(lb, tail, &data, min_obj, kurtosis_threshold, window_size, is_zigzag_pattern);
        }
    } else {
        // If no feasible window found in peak search, try a conservative default
        let default_window = (data.len() / 20).max(2).min(data.len() / 3);
        let smoothed = sma(&data, default_window, 1);
        
        if !smoothed.is_empty() {
            let metrics = Metrics::new(smoothed.clone());
            if metrics.kurtosis() >= kurtosis_threshold && metrics.roughness() < original_roughness {
                return smoothed;
            }
        }
        
        // If still no improvement but it's a zigzag pattern, try window size 2
        if is_zigzag_pattern {
            let smoothed = sma(&data, 2, 1);
            if !smoothed.is_empty() {
                let metrics = Metrics::new(smoothed.clone());
                if metrics.roughness() < original_roughness {
                    return smoothed;
                }
            }
        }
        
        // If still no improvement, return original data
        return data;
    }

    // Ensure window_size is reasonable and won't over-smooth
    window_size = window_size.max(2).min(data.len() / 3);
    
    // Final smoothing with selected window
    let smoothed = sma(&data, window_size, 1);

    // Calculate metrics for original and smoothed data to compare roughness
    let original_roughness = Metrics::new(data.clone()).roughness();
    let smoothed_roughness = Metrics::new(smoothed.clone()).roughness();

    // If smoothing made roughness worse or result is too small, return original data
    if smoothed.len() < 2 || smoothed_roughness >= original_roughness * 1.1 {
        return data;
    }

    smoothed
}

// Helper function to detect zigzag patterns
fn detect_zigzag_pattern(data: &[f64], roughness: f64) -> bool {
    // If data is too short, it's not meaningful to check
    if data.len() < 4 {
        return false;
    }
    
    // Zigzag patterns have very high roughness relative to the range of values
    let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let data_range = max_val - min_val;
    
    // If range is effectively zero, it's not a zigzag
    if data_range < 1e-10 {
        return false;
    }
    
    // Check for alternating pattern (values go up, down, up, down...)
    let mut alternating_count = 0;
    for i in 1..data.len()-1 {
        let prev_diff = data[i] - data[i-1];
        let next_diff = data[i+1] - data[i];
        if prev_diff * next_diff < 0.0 {  // Different signs
            alternating_count += 1;
        }
    }
    
    let alternating_ratio = alternating_count as f64 / (data.len() - 2) as f64;
    
    // Zigzag patterns have:
    // 1. High roughness relative to data range
    // 2. High percentage of alternating points
    roughness > data_range * 0.5 && alternating_ratio > 0.7
}

fn binary_search(mut head: usize, mut tail: usize, data: &[f64], mut min_obj: f64, kurtosis_threshold: f64, mut window_size: usize, is_zigzag: bool) -> usize {
    // Handle edge cases
    if data.is_empty() || head > data.len() || tail > data.len() {
        return window_size;
    }
    
    head = head.min(data.len() / 2); // Prevent window sizes larger than half the data
    tail = tail.min(data.len() / 2);
    
    // Skip search if head > tail
    if head > tail {
        return window_size;
    }

    // For zigzag patterns, prefer window size 2 if it works
    if is_zigzag && head <= 2 && 2 <= tail {
        let smoothed = sma(data, 2, 1);
        if !smoothed.is_empty() {
            let metrics = Metrics::new(smoothed);
            if metrics.roughness() < min_obj {
                return 2;
            }
        }
    }
    
    while head <= tail {
        let w = (head + tail) / 2;
        let smoothed = sma(data, w, 1);
        
        // Skip if smoothed is empty or too small
        if smoothed.len() < 2 {
            break;
        }
        
        let metrics = Metrics::new(smoothed);
        // Use the adjusted kurtosis threshold
        if metrics.kurtosis() >= kurtosis_threshold {
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