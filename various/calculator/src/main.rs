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

fn value_at_risk(prices: &[f64], confidence_level: f64) -> f64 {
    let returns = logarithmic_returns(prices);
    let num_returns = returns.len() as f64;
    let data_point = num_returns * (1.0 - confidence_level);
    let ceiled_data_point = data_point.ceil();

    let mut sorted_returns = returns.clone();
    sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if data_point == ceiled_data_point {
        sorted_returns[(ceiled_data_point as usize) - 1]
    } else {
        // ci = confidence interval
        let ci_inside_closest_to_dp = ceiled_data_point as usize;
        let ci_outside_closest_to_dp = ci_inside_closest_to_dp - 1;
        let weight = data_point.fract();
        sorted_returns[ci_outside_closest_to_dp - 1] * (1.0 - weight) + sorted_returns[ci_inside_closest_to_dp - 1] * weight
    }
}

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

    #[test]
    fn test_value_at_risk_exist_data_matching_datapoint() {
        let prices: Vec<f64> = (1..=101).map(|x| x as f64).collect();
        assert_eq!(value_at_risk(&prices, 0.95), 0.010362787035546658);
        assert_eq!(value_at_risk(&prices, 0.99), 0.009950330853168092);
    }
    #[test]
    fn test_value_at_risk_no_data_matching_datapoint() {
        let prices: Vec<f64> = (1..=365).map(|x| x as f64).collect();
        assert_eq!(value_at_risk(&prices, 0.95), 0.002879360839919127);
        assert_eq!(value_at_risk(&prices, 0.99), 0.002763506313518348);
    }
}
