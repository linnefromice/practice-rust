use statrs::distribution::{Normal, ContinuousCDF};

fn main() {
    println!("Hello, world!");
}

pub struct BlackScholesInput {
    pub s: f64, // spot price
    pub k: f64, // strike price
    pub t: f64, // time to maturity
    pub r: f64, // risk-free rate
    pub sigma: f64, // IV
    pub is_call: bool, // call or put
}

fn black_scholes(
    input: BlackScholesInput,
) -> f64 {
    let BlackScholesInput { s, k, t, r, sigma, is_call } = input;

    // ref
    //   https://jp.mathworks.com/help/symbolic/the-black-scholes-formula-for-call-option-price.html#TheBlackScholesFormulaForCallOptionPriceExample-4
    //   https://www.monte-carlo-note.com/2017/04/python-implied-volatility.html
    let d1_numerator = (s / k).ln() + (r + sigma.powi(2) / 2.0) * t;
    let d1_denominator = sigma * t.sqrt();
    let d1 = d1_numerator / d1_denominator;
    let d2 = d1 - sigma * t.sqrt();
    let norm_dist = Normal::new(0.0, 1.0).unwrap();

    // ref: https://www.nsspirt-cashf2.com/ba/black-shoals/
    if is_call {
        s * norm_dist.cdf(d1) - k * (-r * t).exp() * norm_dist.cdf(d2)
    } else {
        k * (-r * t).exp() * norm_dist.cdf(-d2) - s * norm_dist.cdf(-d1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_scholes() {
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
}
