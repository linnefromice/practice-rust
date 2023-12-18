use nalgebra::{DMatrix, DVector};

// Function to calculate BEY from CMTi using natural cubic spline interpolation
fn calculate_bey(cmt_yields: &[CmtYield], t: f64) -> Option<f64> {
    let n = cmt_yields.len();

    if n < 2 || t < cmt_yields[0].days as f64 || t > cmt_yields[n - 1].days as f64 {
        return None; // Invalid input or out of range
    }

    // Initialize data matrices and vectors
    let mut x = DMatrix::zeros(n, n);
    let mut y = DVector::zeros(n);
    let mut h = DVector::zeros(n - 1);
    let mut alpha = DVector::zeros(n - 1);
    let mut l = DVector::zeros(n);
    let mut mu = DVector::zeros(n - 1);
    let mut z = DVector::zeros(n);

    // Populate the x matrix
    for i in 0..n {
        x[(i, i)] = 1.0;
    }

    for i in 0..n - 1 {
        let hi = (cmt_yields[i + 1].days - cmt_yields[i].days) as f64;
        h[i] = hi;
        alpha[i] = (6.0 / hi) * (cmt_yields[i + 1].yield_ - cmt_yields[i].yield_);
    }

    for i in 1..n - 1 {
        let factor1 = h[i - 1] / (h[i - 1] + h[i]);
        let factor2 = 1.0 - factor1;

        l[i] = factor1 * l[i - 1] + 2.0;
        mu[i - 1] = factor2 / l[i];
    }

    for i in 1..n - 1 {
        z[i] = alpha[i] - (h[i - 1] / h[i]) * z[i - 1];
    }

    // Calculate the coefficients
    let mut c = DVector::zeros(n);
    let mut b = DVector::zeros(n - 1);
    let mut d = DVector::zeros(n - 1);

    c[n - 1] = 0.0;

    for i in (0..n - 1).rev() {
        c[i] = z[i] - mu[i] * c[i + 1];
        b[i] = (cmt_yields[i + 1].yield_ - cmt_yields[i].yield_) / h[i] - h[i] * (c[i + 1] + 2.0 * c[i]) / 6.0;
        d[i] = (c[i + 1] - c[i]) / (6.0 * h[i]);
    }

    // Find the interval containing t
    let mut interval = None;
    for i in 0..n - 1 {
        if t >= cmt_yields[i].days as f64 && t <= cmt_yields[i + 1].days as f64 {
            interval = Some(i);
            break;
        }
    }

    if let Some(i) = interval {
        let ti = t - cmt_yields[i].days as f64;
        let bey = cmt_yields[i].yield_
            + b[i] * ti
            + c[i] * ti.powi(2) / 2.0
            + d[i] * ti.powi(3) / 6.0;
        return Some(bey * 2.0); // Multiply by 2 to get BEY
    }

    None // t was not within any interval
}
