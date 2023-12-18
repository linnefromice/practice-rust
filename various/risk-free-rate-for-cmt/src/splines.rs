#[cfg(test)]
mod tests {
    use crate::CmtYield;

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

    fn generate_keys() -> Vec<Key<f64, f64>> {
        YIELDS_IN_CBOE.iter()
            .map(|v| Key::new(
                v.days as f64,
                v.yield_,
                 splines::Interpolation::CatmullRom
            ))
            .collect::<Vec<Key>>()
    }

    // #[test]
    // fn test_cboe_r1() {
    //     let t1 = 24;
    //     let mut spline = MonotonicCubicSpline::new(
    //         &YIELDS_IN_CBOE.iter().map(|v| v.days as f64).collect(),
    //         &YIELDS_IN_CBOE.iter().map(|v| v.yield_).collect()
    //     );
    //     let r1 = spline.interpolate(t1 as f64);
    //     assert_eq!(r1, 0.031664)
    // }

    // #[test]
    // fn test_cboe_r2() {
    //     let t2 = 31;
    //     let mut spline = MonotonicCubicSpline::new(
    //         &YIELDS_IN_CBOE.iter().map(|v| v.days as f64).collect(),
    //         &YIELDS_IN_CBOE.iter().map(|v| v.yield_).collect()
    //     );
    //     let r2 = spline.interpolate(t2 as f64);
    //     assert_eq!(r2, 0.028797)
    // }
}