use std::f64::consts::PI;
use super::convolution::convolve_complex;

pub fn transform(real: &mut [f64], imag: &mut [f64]) -> Result<(), String> {
    if real.len() != imag.len() {
        return Err("Mismatched lengths".to_string());
    }

    let n = real.len();
    if n == 0 {
        return Ok(());
    } else if n & (n - 1) == 0 { 
        transform_radix2(real, imag)
    } else {
        transform_bluestein(real, imag)
    }
}


pub fn inverse_transform(real: &mut [f64], imag: &mut [f64]) -> Result<(), String> {
    // Swap real and imaginary parts
    transform(imag, real)
}

fn transform_radix2(real: &mut [f64], imag: &mut [f64]) -> Result<(), String> {
    let n = real.len();
    
    if n == 1 {  // Trivial transform
        return Ok(());
    }

    // Compute levels = floor(log2(n))
    let mut levels = 0;
    for i in 0..32 {
        if (1 << i) == n {
            levels = i;
            break;
        }
    }
    if levels == 0 {
        return Err("Length is not a power of 2".to_string());
    }

    // Trigonometric tables
    let mut cos_table = vec![0.0; n / 2];
    let mut sin_table = vec![0.0; n / 2];
    for i in 0..n/2 {
        cos_table[i] = (2.0 * PI * i as f64 / n as f64).cos();
        sin_table[i] = (2.0 * PI * i as f64 / n as f64).sin();
    }

    // Bit-reversed addressing permutation
    for i in 0..n {
        let j = reverse_bits(i, levels);
        if j > i {
            real.swap(i, j);
            imag.swap(i, j);
        }
    }

    // Cooley-Tukey decimation-in-time radix-2 FFT
    let mut size = 2;
    while size <= n {
        let halfsize = size / 2;
        let tablestep = n / size;
        for i in (0..n).step_by(size) {
            for j in i..i+halfsize {
                let k = (j - i) * tablestep;
                let tpre =  real[j+halfsize] * cos_table[k] + imag[j+halfsize] * sin_table[k];
                let tpim = -real[j+halfsize] * sin_table[k] + imag[j+halfsize] * cos_table[k];
                real[j + halfsize] = real[j] - tpre;
                imag[j + halfsize] = imag[j] - tpim;
                real[j] += tpre;
                imag[j] += tpim;
            }
        }
        size *= 2;
    }

    Ok(())
}

fn reverse_bits(mut x: usize, bits: u32) -> usize {
    let mut y = 0;
    for _ in 0..bits {
        y = (y << 1) | (x & 1);
        x >>= 1;
    }
    y
}

fn transform_bluestein(real: &mut [f64], imag: &mut [f64]) -> Result<(), String> {
    let n = real.len();
    
    // Find a power-of-2 convolution length m such that m >= n * 2 + 1
    let mut m = 1;
    while m < n * 2 + 1 {
        m *= 2;
    }

    // Trigonometric tables
    let mut cos_table = vec![0.0; n];
    let mut sin_table = vec![0.0; n];
    for i in 0..n {
        let j = (i * i) % (n * 2);  // This is more accurate than j = i * i
        cos_table[i] = (PI * j as f64 / n as f64).cos();
        sin_table[i] = (PI * j as f64 / n as f64).sin();
    }

    // Temporary vectors and preprocessing
    let mut areal = vec![0.0; m];
    let mut aimag = vec![0.0; m];
    for i in 0..n {
        areal[i] =  real[i] * cos_table[i] + imag[i] * sin_table[i];
        aimag[i] = -real[i] * sin_table[i] + imag[i] * cos_table[i];
    }
    // Note: areal and aimag are already zero-initialized for i >= n

    let mut breal = vec![0.0; m];
    let mut bimag = vec![0.0; m];
    breal[0] = cos_table[0];
    bimag[0] = sin_table[0];
    for i in 1..n {
        breal[i] = cos_table[i];
        bimag[i] = sin_table[i];
        breal[m - i] = cos_table[i];
        bimag[m - i] = sin_table[i];
    }
    // Note: breal and bimag are already zero-initialized for n <= i <= m - n

    // Convolution
    let mut creal = vec![0.0; m];
    let mut cimag = vec![0.0; m];
    convolve_complex(&areal, &aimag, &breal, &bimag, &mut creal, &mut cimag)?;

    // Postprocessing
    for i in 0..n {
        real[i] =  creal[i] * cos_table[i] + cimag[i] * sin_table[i];
        imag[i] = -creal[i] * sin_table[i] + cimag[i] * cos_table[i];
    }

    Ok(())
}