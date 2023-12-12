use k::{calculate_f, ParamF};
use serde::Deserialize;

use crate::{options::{convert_data_to_options, select_target_puts, select_target_calls}, k::find_closest_less_than_f};

mod k;
mod options;
mod variance;
mod vix;

#[derive(Debug, Deserialize)]
pub struct Datum {
    pub strike_price: f64,
    pub call_bid: f64,
    pub call_ask: f64,
    pub put_bid: f64,
    pub put_ask: f64,
}

fn read_data(path: &str) -> Vec<Datum> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    reader.deserialize().collect::<Result<Vec<Datum>, csv::Error>>().unwrap()
}

fn main() {
    let near_data = read_data("resources/near-term.csv");
    let next_data = read_data("resources/next-term.csv");

    // Other input parameters
    //// near
    let r1 = 0.00031664;
    let t1 = 34484.0 / 525600.0; // 0.0656088
    //// next
    let r2 = 0.00028797;
    let t2 = 44954.0 / 525600.0; // 0.0855289

    // strike price at which the absolute difference between the call price and the put price is smallest
    // NOTE: be supposed to search
    let near_atm_strike = 1965.0;
    let next_atm_strike = 1960.0;

    println!("near");
    let _ = calculate_variance_per_term(near_data, near_atm_strike, r1, t1);

    println!("next");
    let _ = calculate_variance_per_term(next_data, next_atm_strike, r2, t2);
    
    // [3] Calculate Variance -> TODO

    // [4] Calculate VIX -> TODO
}

fn calculate_variance_per_term(data: Vec<Datum>, atm_strike: f64, risk_free_rate: f64, time_to_expiration: f64) -> f64 {
    // [1] Calculate F, K_0
    let datum_for_f = data.iter().find(|d| d.strike_price == atm_strike).unwrap();
    let f = calculate_f(ParamF {
        strike_price: datum_for_f.strike_price,
        call_price: (datum_for_f.call_bid + datum_for_f.call_ask) / 2.0,
        put_price: (datum_for_f.put_bid + datum_for_f.put_ask) / 2.0,
        risk_free_rate,
        time_to_expiration,
    });

    let strike_prices = data.iter().map(|d| d.strike_price).collect::<Vec<f64>>();
    let k_0_idx = find_closest_less_than_f(f, strike_prices.clone()).unwrap();
    let k_0 = strike_prices.get(k_0_idx).unwrap();

    println!("f: {}", f);
    println!("k_0: {}", k_0);

    // [2] Select Options to calculate
    let options = convert_data_to_options(data);

    let calls = select_target_calls(*k_0, options.clone());
    let puts = select_target_puts(*k_0, options.clone());

    println!("calls: {:?}", calls.iter().map(|op| op.strike_price).collect::<Vec<f64>>());
    println!("puts: {:?}", puts.iter().map(|op| op.strike_price).collect::<Vec<f64>>());

    0.0
}