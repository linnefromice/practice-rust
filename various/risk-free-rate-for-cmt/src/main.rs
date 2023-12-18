mod cubic_spline;

#[derive(Copy, Clone, Debug, PartialEq)]
struct CmtYield {
    pub days: u32,
    pub yield_: f64,
}

fn calculate_risk_free_rate(cmt_yields: Vec<CmtYield>, t: u32) -> f64 {
    // CMT Yields should be a map of days to yield, e.g. , 30 -? 0.02 (2% for 1 month)
    // t is the target number of days for whitch we want to find the yield

    let mut local_cmt_yields = cmt_yields.clone();
    local_cmt_yields.sort_by(|a, b| a.days.cmp(&b.days));
    let first_yield = local_cmt_yields.first().expect("No yields provided");

    let bet_y = if t < first_yield.days {
        // Extrapolate for t < t1
        extrapolate_yield(local_cmt_yields.clone(), t, first_yield)
    } else {
        // Interpolate for t1 <= t <= tN
        interpolate_yield(local_cmt_yields, t)
    };

    compounded_apy_rate_from_bey_r(bet_y)
}

// Convert BEY to APY and then to continuously compounded rate
fn compounded_apy_rate_from_bey_r(bey_r: f64) -> f64 {
    let apy_t = ((1.0 + bey_r / 2.0) as f64).powf(2.0) - 1.0;
    (1.0 + apy_t).ln()
}

fn interpolate_yield(cmt_yields: Vec<CmtYield>, t: u32) -> f64 {
    // Basic linear interpolation for simplicity
    for window in cmt_yields.windows(2) {
        if let [t_i_yield, t_next_yield] = window {
            let CmtYield { days: t_i, yield_: y_i } = t_i_yield.clone();
            let CmtYield { days: t_next, yield_: y_next } = t_next_yield.clone();
            if t >= t_i && t <= t_next {
                return y_i + (y_next - y_i) * (t as f64 - t_i as f64) / (t_next as f64 - t_i as f64);
            }
        }
    }

    panic!("Interpolation failed: t not in range");
}

// fn extrapolate_yield(cmt_yields: &BTreeMap<u32, f64>, keys: &[u32], t: u32, t1: u32, cmt1: f64) -> f64 {
fn extrapolate_yield(cmt_yields: Vec<CmtYield>, t: u32, cmt_yield_1: &CmtYield) -> f64 {
    // Basic linear extrapolation
    let cmt_yield_2 = cmt_yields.iter().find(|&&v| v.days > cmt_yield_1.days && v.yield_ >= cmt_yield_1.yield_).unwrap_or(&cmt_yield_1);

    let CmtYield { days: t1, yield_: cmt1 } = *cmt_yield_1;
    let CmtYield { days: t2, yield_: cmt2 } = *cmt_yield_2;

    let m0_lower = (cmt2 - cmt1) / (t2 as f64 - t1 as f64);
    let b_lower = cmt1 - m0_lower * t1 as f64;

    m0_lower * t as f64 + b_lower
}

fn main() {
    let cmt_yields = vec![
        CmtYield { days: 30, yield_: 0.02 },
        CmtYield { days: 60, yield_: 0.021 },
        CmtYield { days: 91, yield_: 0.022 },
        CmtYield { days: 182, yield_: 0.023 },
        // ... add other maturities
    ];

    let target_days = 45; // Example: find yield for 45 days
    let risk_free_rate = calculate_risk_free_rate(cmt_yields, target_days);

    println!("Risk-free rate for {} days: {}", target_days, risk_free_rate);
}

#[cfg(test)]
mod tests {
    use super::*;

    const YIELDS_IN_CBOE: [CmtYield; 12] = [
        CmtYield { days: 30, yield_: 0.03 }, // 1 Mo
        CmtYield { days: 60, yield_: 0.02 }, // 2 Mo
        CmtYield { days: 91, yield_: 0.04 }, // 3 Mo
        CmtYield { days: 182, yield_: 0.05 }, // 6 Mo
        CmtYield { days: 365, yield_: 0.08 }, // 1 Yr
        CmtYield { days: 730, yield_: 0.11 }, // 2 Yr
        CmtYield { days: 1095, yield_: 0.22 }, // 3 Yr
        CmtYield { days: 1825, yield_: 0.59 }, // 5 Yr
        CmtYield { days: 2555, yield_: 1.0 }, // 7 Yr
        CmtYield { days: 3650, yield_: 1.37 }, // 10 Yr
        CmtYield { days: 7300, yield_: 2.03 }, // 20 Yr
        CmtYield { days: 10950, yield_: 2.21 }, // 30 Yr
    ];

    #[test]
    fn test_cboe_r1() {
        let t1 = 24;
        let r1 = calculate_risk_free_rate(YIELDS_IN_CBOE.to_vec(), t1);
        assert_eq!(r1, 0.031664)
    }

    #[test]
    fn test_cboe_r2() {
        let t2 = 31;
        let r2 = calculate_risk_free_rate(YIELDS_IN_CBOE.to_vec(), t2);
        assert_eq!(r2, 0.028797)
    }

    #[test]
    fn test_lower_r() {
        let cmd_yields = YIELDS_IN_CBOE.to_vec();
        let cmd_yield = cmd_yields.first().unwrap();
        assert_eq!(compounded_apy_rate_from_bey_r(cmd_yield.yield_), 0.02977722498750104)
    }

    #[test]
    fn test_upper_r() {
        let cmd_yields = YIELDS_IN_CBOE.to_vec();
        let cmd_yield = cmd_yields.last().unwrap();
        assert_eq!(compounded_apy_rate_from_bey_r(cmd_yield.yield_), 1.4886309342686894)
    }
}