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
        let mut acf = ACF {
            mean: Metrics::mean(&values),
            values,
            correlations: vec![0.0; max_lag],
            corr_thresh: 0.2,
            max_acf: 0.0,
        };
        acf.calculate();
        acf
    }

    fn calculate(&mut self) {
        // Padding to the closest power of 2
        let len = 2_usize.pow((self.values.len() as f64).log2().ceil() as u32);
        let mut fft_real = vec![0.0; len];
        let mut fft_imag = vec![0.0; len];

        for (i, &value) in self.values.iter().enumerate() {
            fft_real[i] = value - self.mean;
        }

        // F_R(f) = FFT(X)
        transform(&mut fft_real, &mut fft_imag).unwrap();

        // S(f) = F_R(f)F_R*(f)
        for i in 0..fft_real.len() {
            fft_real[i] = fft_real[i].powi(2) + fft_imag[i].powi(2);
            fft_imag[i] = 0.0;
        }

        // R(t) = IFFT(S(f))
        inverse_transform(&mut fft_real, &mut fft_imag).unwrap();

        for i in 1..self.correlations.len() {
            self.correlations[i] = fft_real[i] / fft_real[0];
        }
    }

    pub fn find_peaks(&mut self) -> Vec<usize> {
        let mut peak_indices = Vec::new();

        if self.correlations.len() > 1 {
            let mut positive = self.correlations[1] > self.correlations[0];
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
        if peak_indices.len() <= 1 {
            peak_indices.extend(2..self.correlations.len());
        }

        peak_indices
    }
}