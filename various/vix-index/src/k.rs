pub struct ParamF {
    strike_price: f64,
    call_price: f64,
    put_price: f64,
    risk_free_rate: f64,
    time_to_expiration: f64,
}
pub fn calculate_f(p: ParamF) -> f64 {
    p.strike_price + (p.risk_free_rate * p.time_to_expiration).exp() * (p.call_price - p.put_price)
}

pub fn find_closest_less_than_f(list: Vec<f64>, f: f64) -> Option<usize> {
    let mut closest_idx = None;
    let mut closest_val = None;
    
    for (idx, value) in list.iter().enumerate() {
        if closest_val.is_none() || (f > *value && (f - value) < closest_val.unwrap()) {
            closest_val = Some(*value);
            closest_idx = Some(idx);
        }
    }

    closest_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://cdn.cboe.com/api/global/us_indices/governance/VIX_Methodology.pdf

    #[test]
    fn test_f_for_near_term() {
        let t1 = 34484.0 / 525600.0;
        let r1 = 0.031664;
        assert_eq!(
            calculate_f(ParamF {
                strike_price: 1965.0,
                call_price: 21.05,
                put_price: 23.15,
                risk_free_rate: r1,
                time_to_expiration: t1,
            }),
            1962.8956328456716 // 1962.89996
        );
    }

    #[test]
    fn test_f_for_next_term() {
        let t2 = 44954.0 / 525600.0;
        let r2 = 0.028797;
        assert_eq!(
            calculate_f(ParamF {
                strike_price: 1960.0,
                call_price: 27.30,
                put_price: 24.90,
                risk_free_rate: r2,
                time_to_expiration: t2,
            }),
            1962.4059184285786 // 1962.40006
        );
    }

    #[test]
    fn test_find_closest_less_than_f_1() {
        let val = 1962.40006;
        let list = vec![
            1940.0,
            1945.0,
            1950.0,
            1955.0,
            1960.0,
            1965.0,
            1970.0,
            1975.0,
            1980.0
        ];
        let idx = find_closest_less_than_f(list.clone(), val);
        assert_eq!(*list.get(idx.unwrap()).unwrap(), 1960.0);
    }
}