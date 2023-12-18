pub struct MonotonicCubicSpline {
    m_x: Vec<f64>,
    m_y: Vec<f64>,
    m_m: Vec<f64>
}
impl MonotonicCubicSpline {
    pub fn new(x : &Vec<f64>, y : &Vec<f64>) -> MonotonicCubicSpline {

        assert!(x.len() == y.len() && x.len() >= 2 && y.len() >= 2, "Must have at least 2 control points.");

        let n = x.len();

        let mut secants = vec![0.0 ; n - 1];
        let mut slopes  = vec![0.0 ; n];

        for i in 0..(n-1) {
            let h = *x.get(i + 1).unwrap() - *x.get(i).unwrap();
            assert!(h > 0.0, "Control points must be monotonically increasing.");
            secants[i] = (*y.get(i + 1).unwrap() - *y.get(i).unwrap()) / h;

        }

        slopes[0] = secants[0];
        for i in 1..(n-1) {
            slopes[i] = (secants[i - 1] + secants[i]) * 0.5;
        }
        slopes[n - 1] = secants[n - 2];

        for i in 0..(n-1) {
            if secants[i] == 0.0 {
                slopes[i] = 0.0;
                slopes[i + 1] = 0.0;
            } else {
                let alpha = slopes[i] / secants[i];
                let beta = slopes[i + 1] / secants[i];
                let h = alpha.hypot(beta);
                if h > 9.0 {
                    let t = 3.0 / h;
                    slopes[i] = t * alpha * secants[i];
                    slopes[i + 1] = t * beta * secants[i];
                }
            }
        }

        let spline = MonotonicCubicSpline {
            m_x: x.clone(),
            m_y: y.clone(),
            m_m: slopes
        };
        return spline;
    }

    fn hermite(point: f64, x : (f64, f64), y: (f64, f64), m: (f64, f64)) -> f64 {
        let h: f64 = x.1 - x.0;
        let t = (point - x.0) / h;
        return (y.0 * (1.0 + 2.0 * t) + h * m.0 * t) * (1.0 - t) * (1.0 - t)
            + (y.1 * (3.0 - 2.0 * t) + h * m.1 * (t - 1.0)) * t * t;
    }

    pub fn interpolate(&mut self, point : f64) -> f64 {
        let n = self.m_x.len();

        if point <= *self.m_x.get(0).unwrap() {
            return *self.m_y.get(0).unwrap();
        }
        if point >= *self.m_x.get(n - 1).unwrap() {
            return *self.m_y.get(n - 1).unwrap();
        }

        let mut i = 0;
        while point >= *self.m_x.get(i + 1).unwrap() {
            i += 1;
            if point == *self.m_x.get(i).unwrap() {
                return *self.m_y.get(i).unwrap();
            }
        }
        return MonotonicCubicSpline::hermite(
            point,
            (*self.m_x.get(i).unwrap(), *self.m_x.get(i+1).unwrap()),
            (*self.m_y.get(i).unwrap(), *self.m_y.get(i+1).unwrap()),
            (*self.m_m.get(i).unwrap(), *self.m_m.get(i+1).unwrap())
        );
    }

    fn partial(x: Vec<f64>, y: Vec<f64>) -> impl Fn(f64) -> f64 {
        move |p| {
            let mut spline = MonotonicCubicSpline::new(&x, &y);
            spline.interpolate(p)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CmtYield;

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
        let mut spline = MonotonicCubicSpline::new(
            &YIELDS_IN_CBOE.iter().map(|v| v.days as f64).collect(),
            &YIELDS_IN_CBOE.iter().map(|v| v.yield_).collect()
        );
        let r1 = spline.interpolate(t1 as f64);
        assert_eq!(r1, 0.031664)
    }

    #[test]
    fn test_cboe_r2() {
        let t2 = 31;
        let mut spline = MonotonicCubicSpline::new(
            &YIELDS_IN_CBOE.iter().map(|v| v.days as f64).collect(),
            &YIELDS_IN_CBOE.iter().map(|v| v.yield_).collect()
        );
        let r2 = spline.interpolate(t2 as f64);
        assert_eq!(r2, 0.028797)
    }
}