use crate::statistics::Metrics;
use crate::fft::{transform, inverse_transform};

pub struct ACF {
    mean: f64,
    values: Vec<f64>,
    pub correlations: Vec<f64>,
    corr_thresh: f64,
    pub max_acf: f64,
}

impl ACF {
    pub fn new(values: Vec<f64>, max_lag: usize) -> Self {
        // Handle empty values case
        if values.is_empty() {
            return ACF {
                mean: 0.0,
                values,
                correlations: vec![0.0], // Only lag 0
                corr_thresh: 0.2,
                max_acf: 0.0,
            };
        }
        
        // For non-empty arrays, ensure max_lag is not larger than the data length - 1
        let adjusted_max_lag = max_lag.min(values.len() - 1);
        
        let mut acf = ACF {
            mean: Metrics::mean(&values),
            values,
            correlations: vec![0.0; adjusted_max_lag + 1], // +1 to include lag 0
            corr_thresh: 0.2,
            max_acf: 0.0,
        };
        
        // Skip calculation if max_lag is 0
        if adjusted_max_lag > 0 {
            acf.calculate();
        }
        
        acf
    }

    fn calculate(&mut self) {
        // Skip calculation if values is empty
        if self.values.is_empty() {
            return;
        }
        
        // Padding to the closest power of 2
        let len = 2_usize.pow((self.values.len() as f64).log2().ceil() as u32);
        let mut fft_real = vec![0.0; len];
        let mut fft_imag = vec![0.0; len];

        for (i, &value) in self.values.iter().enumerate() {
            fft_real[i] = value - self.mean;
        }

        // F_R(f) = FFT(X)
        if let Ok(()) = transform(&mut fft_real, &mut fft_imag) {
            // S(f) = F_R(f)F_R*(f)
            for i in 0..fft_real.len() {
                fft_real[i] = fft_real[i].powi(2) + fft_imag[i].powi(2);
                fft_imag[i] = 0.0;
            }

            // R(t) = IFFT(S(f))
            if let Ok(()) = inverse_transform(&mut fft_real, &mut fft_imag) {
                // Ensure correlations[0] is not zero to avoid division by zero
                if fft_real[0].abs() > 1e-10 {
                    // Fill correlations array (only up to correlations.len())
                    for i in 1..self.correlations.len() {
                        self.correlations[i] = fft_real[i] / fft_real[0];
                    }
                }
            }
        }
    }

    pub fn find_peaks(&mut self) -> Vec<usize> {
        let mut peak_indices = Vec::new();

        if self.correlations.len() > 2 {  // Need at least 3 elements (lag 0, 1, 2)
            // Start at correlations[1] and check against correlations[0]
            let mut positive = self.correlations.get(1).map_or(false, |&v| v > self.correlations[0]);
            let mut max = 1;

            for i in 2..self.correlations.len() {
                if !positive && self.correlations[i] > self.correlations[i - 1] {
                    max = i;
                    positive = !positive;
                } else if positive && self.correlations[i] > self.correlations[max] {
                    max = i;
                } else if positive && self.correlations[i] < self.correlations[i - 1] {
                    if max > 1 && self.correlations[max] > self.corr_thresh {
                        peak_indices.push(max);
                        if self.correlations[max] > self.max_acf {
                            self.max_acf = self.correlations[max];
                        }
                    }
                    positive = !positive;
                }
            }
        }

        // If there is no autocorrelation peak within the MAX_WINDOW boundary,
        // try windows from the largest to the smallest
        if peak_indices.len() <= 1 && self.correlations.len() > 2 {
            peak_indices.extend(2..self.correlations.len());
        }

        peak_indices
    }
}