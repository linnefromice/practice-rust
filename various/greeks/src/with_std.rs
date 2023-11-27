use statrs::distribution::{Normal, ContinuousCDF};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SeekIvParam {
    pub s: f64, // spot price
    pub k: f64, // strike price
    pub t: f64, // time to maturity
    pub r: f64, // risk-free rate
    pub is_call: bool, // call or put
    // param to seek
    pub market_price: f64,
    pub initial_sigma: f64,
    pub tolerance: f64,
    pub attempt_count: u64,
}
fn seek_implied_volatility(param: SeekIvParam) -> (f64, u64) {
    let SeekIvParam { market_price, initial_sigma, tolerance, attempt_count, s, k, t, r, is_call } = param;
    let mut sigma = initial_sigma;
    let mut attempt = 0;

    for _ in 0..attempt_count {
        attempt += 1;
        let theoretical_price = black_scholes(BlackScholesInput { s, k, t, r, sigma, is_call });
        let vega = vega(VegaInput { s, k, t, r, sigma });
        let diff = theoretical_price - market_price;
        if diff.abs() < tolerance {
            return (sigma, attempt);
        }
        sigma = sigma - diff / vega;
    }
    panic!("Implied volatility not found");
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlackScholesInput {
    pub s: f64, // spot price
    pub k: f64, // strike price
    pub t: f64, // time to maturity
    pub r: f64, // risk-free rate
    pub sigma: f64, // IV
    pub is_call: bool, // call or put
}
fn black_scholes(input: BlackScholesInput) -> f64 {
    let BlackScholesInput { s, k, t, r, sigma, is_call } = input;

    // ref
    //   https://jp.mathworks.com/help/symbolic/the-black-scholes-formula-for-call-option-price.html#TheBlackScholesFormulaForCallOptionPriceExample-4
    //   https://www.monte-carlo-note.com/2017/04/python-implied-volatility.html
    let d1 = d1_of_black_scholes(s, k, t, r, sigma);
    let d2 = d1 - sigma * t.sqrt();
    let norm_dist = Normal::new(0.0, 1.0).unwrap();

    // ref: https://www.nsspirt-cashf2.com/ba/black-shoals/
    if is_call {
        s * norm_dist.cdf(d1) - k * (-r * t).exp() * norm_dist.cdf(d2)
    } else {
        k * (-r * t).exp() * norm_dist.cdf(-d2) - s * norm_dist.cdf(-d1)
    }
}
fn d1_of_black_scholes(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    let numerator = (s / k).ln() + (r + sigma.powi(2) / 2.0) * t;
    let denominator = sigma * t.sqrt();
    numerator / denominator
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VegaInput {
    pub s: f64, // spot price
    pub k: f64, // strike price
    pub t: f64, // time to maturity
    pub r: f64, // risk-free rate
    pub sigma: f64, // IV
}
fn vega(input: VegaInput) -> f64 {
    let VegaInput { s, k, t, r, sigma} = input;
    let d1 = d1_of_black_scholes(s, k, t, r, sigma);
    let norm_dist = Normal::new(0.0, 1.0).unwrap();
    s * norm_dist.cdf(d1) * t.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_scholes_1() {
        // https://jp.mathworks.com/help/symbolic/the-black-scholes-formula-for-call-option-price.html#TheBlackScholesFormulaForCallOptionPriceExample-4
        assert_eq!(
            black_scholes(
                BlackScholesInput {
                    s: 100.0,
                    k: 95.0,
                    t: 3.0/12.0,
                    r: 0.01,
                    sigma: 0.5,
                    is_call: true
                }
            ),
            12.527923392521458
        );
    }

    #[test]
    fn test_block_scholes_2() {
        // https://www.monte-carlo-note.com/2017/04/python-implied-volatility.html
        assert_eq!(
            black_scholes(
                BlackScholesInput {
                    s: 100.0,
                    k: 100.0,
                    t: 1.0,
                    r: 0.05,
                    sigma: 0.45234036,
                    is_call: true
                }
            ),
            19.999999993347053
        );
    }

    #[test]
    fn test_seek_implied_volatility_1() {
        // https://www.optiontrading.dr-harv.com/post7695/
        assert_eq!(
            seek_implied_volatility(
                SeekIvParam {
                    s: 27500.0,
                    k: 26500.0,
                    t: 24.74/280.0,
                    r: -0.03,
                    is_call: false,
                    market_price: 230.0,
                    initial_sigma: 0.5,
                    tolerance: 0.000001,
                    attempt_count: 100
                }
            ),
            (0.18100252565432273, 37)
        );
    }

    #[test]
    fn test_seek_implied_volatility_2() {
        // https://www.monte-carlo-note.com/2017/04/python-implied-volatility.html
        assert_eq!(
            seek_implied_volatility(
                SeekIvParam {
                    s: 100.0,
                    k: 100.0,
                    t: 1.0,
                    r: 0.05,
                    is_call: true,
                    market_price: 20.0,
                    initial_sigma: 0.2,
                    tolerance: 0.000001,
                    attempt_count: 100
                }
            ),
            (0.4523403406867319, 19)
        );
    }
}
