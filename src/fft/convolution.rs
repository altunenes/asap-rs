use super::transform::{transform, inverse_transform};

pub fn convolve_real(x: &[f64], y: &[f64]) -> Result<Vec<f64>, String> {
    if x.len() != y.len() {
        return Err("Mismatched lengths".to_string());
    }

    let n = x.len();
    let mut out = vec![0.0; n];
    let zeros_in = vec![0.0; n];
    let mut zeros_out = vec![0.0; n];

    convolve_complex(x, &zeros_in, y, &zeros_in, &mut out, &mut zeros_out)?;

    Ok(out)
}

pub fn convolve_complex(
    xreal: &[f64], ximag: &[f64], 
    yreal: &[f64], yimag: &[f64], 
    outreal: &mut [f64], outimag: &mut [f64]
) -> Result<(), String> {
    let n = xreal.len();
    if n != ximag.len() || n != yreal.len() || n != yimag.len() || 
       n != outreal.len() || n != outimag.len() {
        return Err("Mismatched lengths".to_string());
    }

    let mut xr = xreal.to_vec();
    let mut xi = ximag.to_vec();
    let mut yr = yreal.to_vec();
    let mut yi = yimag.to_vec();

    transform(&mut xr, &mut xi)?;
    transform(&mut yr, &mut yi)?;

    for i in 0..n {
        let temp = xr[i] * yr[i] - xi[i] * yi[i];
        xi[i] = xi[i] * yr[i] + xr[i] * yi[i];
        xr[i] = temp;
    }

    inverse_transform(&mut xr, &mut xi)?;

    for i in 0..n {
        outreal[i] = xr[i] / n as f64;
        outimag[i] = xi[i] / n as f64;
    }

    Ok(())
}