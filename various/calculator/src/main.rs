fn main() {
    println!("Hello, world!");
}

fn logarithmic_return(prev_price: f64, current_price: f64) -> f64 {
    (current_price / prev_price).ln()
}

fn logarithmic_returns(prices: &[f64]) -> Vec<f64> {
    let mut returns = Vec::new();
    for i in 1..prices.len() {
        returns.push(logarithmic_return(prices[i - 1], prices[i]));
    }
    returns
}

// fn value_at_risk

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn test_logarithmic_return() {
        assert_eq!(logarithmic_return(271967285010812.0, 265340213436227.0), -0.024668956490624585);
        assert_eq!(logarithmic_return(999955000000000000.0, 1000004000000000000.0), 0.00004900100453034797);
    }

    #[test]
    fn test_logarithmic_returns() {
        let prices = &[
            271967285010812.0,
            265340213436227.0,
            261144846968439.0,
            265260589450498.0,
        ];
        let res = logarithmic_returns(prices);
        assert_eq!(res.len(), 3);
        assert_eq!(res[0], logarithmic_return(prices[0], prices[1]));
        assert_eq!(res[1], logarithmic_return(prices[1], prices[2]));
        assert_eq!(res[2], logarithmic_return(prices[2], prices[3]));
    }
}
