//! # ASAP-RS
//! 
//! `asap-rs` is a Rust implementation of Automatic Smoothing for Attention Prioritization (ASAP),
//! an algorithm for automatically smoothing time series data to improve visualization.
//! 
//! The algorithm is based on the paper ["ASAP: Prioritizing Attention via Time Series Smoothing"](https://arxiv.org/pdf/1703.00983.pdf)
//! by Kexin Rong, Peter Bailis, et al. from Stanford University.
//! 
//! ## Features
//! 
//! - Automatically determines optimal smoothing window size based on data characteristics
//! - Maintains important patterns and anomalies while reducing noise
//! - Preserves kurtosis to ensure the overall distribution shape is maintained
//! - Optimizes for reduced visual roughness
//! - Special handling for different data patterns (trending, periodic, zigzag)
//! 
//! ## Usage
//! 
//! ```
//! use asap_rs::smooth;
//! 
//! fn main() {
//!     // Example time series data
//!     let data = vec![1.0, 2.0, 1.5, 3.0, 2.5, 4.0, 3.5, 5.0, 4.5, 6.0];
//!     
//!     // Apply ASAP smoothing with target resolution of 5 points
//!     let smoothed_data = smooth(&data, 5);
//!     
//!     println!("Original data: {:?}", data);
//!     println!("Smoothed data: {:?}", smoothed_data);
//! }
//! ```
//!
//! ## Algorithm Overview
//!
//! ASAP automatically finds the optimal window size for smoothing by:
//!
//! 1. Computing the roughness (standard deviation of differences between consecutive points)
//! 2. Ensuring that kurtosis (measure of "tailedness" of distribution) is preserved
//! 3. Using autocorrelation to identify potential window sizes aligned with patterns in the data
//! 4. Selecting the window size that minimizes roughness while preserving key statistical properties
//!
//! ### Pattern-Specific Processing
//!
//! ASAP-RS includes specialized handling for different types of time series patterns:
//!
//! - **Periodic data**: Windows aligned with the period are prioritized to preserve seasonality
//! - **Trend data**: Appropriate windows are selected to maintain the trend while removing noise
//! - **Zigzag patterns**: Special detection and handling for highly alternating data patterns
//!
//! For zigzag patterns (rapidly alternating high-low values), ASAP uses a relaxed kurtosis 
//! constraint and prioritizes a window size of 2, which is mathematically optimal for smoothing
//! such patterns while preserving their mean value.
//!
//! ## Resolution Parameter
//!
//! The `resolution` parameter controls the trade-off between smoothing and detail preservation:
//!
//! - Lower values (e.g., 2-5) apply more aggressive smoothing, highlighting major trends
//! - Higher values (e.g., 50+) preserve more detail, suitable for dense visualizations
//!
//! The parameter roughly corresponds to the desired number of points in the output, though
//! ASAP may return more or fewer points based on what best preserves important patterns.


pub mod fft;
pub mod statistics;
pub mod utils;
pub mod smoothing;

#[cfg(test)]
pub mod tests;

pub use smoothing::adaptive_smoothing::smooth;