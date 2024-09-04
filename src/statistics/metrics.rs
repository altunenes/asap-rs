pub struct Metrics {
    len: usize,
    values: Vec<f64>,
    m: f64,
}

impl Metrics {
    pub fn new(values: Vec<f64>) -> Self {
        let len = values.len();
        let m = Self::mean(&values);
        Metrics { len, values, m }
    }

    pub fn mean(values: &[f64]) -> f64 {
        let sum: f64 = values.iter().sum();
        sum / values.len() as f64
    }

    pub fn std(values: &[f64]) -> f64 {
        let m = Self::mean(values);
        let variance: f64 = values.iter()
            .map(|&x| (x - m).powi(2))
            .sum::<f64>() / values.len() as f64;
        variance.sqrt()
    }

    pub fn kurtosis(&self) -> f64 {
        let (u4, variance): (f64, f64) = self.values.iter()
            .map(|&x| {
                let diff = x - self.m;
                (diff.powi(4), diff.powi(2))
            })
            .fold((0.0, 0.0), |(u4_acc, var_acc), (u4, var)| {
                (u4_acc + u4, var_acc + var)
            });
        
        self.len as f64 * u4 / variance.powi(2)
    }

    pub fn roughness(&self) -> f64 {
        Self::std(&self.diffs())
    }

    fn diffs(&self) -> Vec<f64> {
        self.values.windows(2)
            .map(|w| w[1] - w[0])
            .collect()
    }
}