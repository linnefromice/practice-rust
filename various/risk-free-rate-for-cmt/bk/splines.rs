#[cfg(test)]
mod tests {
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