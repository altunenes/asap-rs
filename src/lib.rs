// src/lib.rs

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

pub mod fft;
pub mod statistics;
pub mod utils;
pub mod smoothing;

#[cfg(test)]
pub mod tests;

pub use smoothing::adaptive_smoothing::smooth;