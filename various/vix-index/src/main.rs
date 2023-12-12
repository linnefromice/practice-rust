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

    // [1] Calculate F, K_0
    let near_param_f = near_data.iter().find(|d| d.strike_price == near_atm_strike).unwrap();
    let next_param_f = next_data.iter().find(|d| d.strike_price == next_atm_strike).unwrap();

    let near_f = calculate_f(ParamF {
        strike_price: near_param_f.strike_price,
        call_price: (near_param_f.call_bid + near_param_f.call_ask) / 2.0,
        put_price: (near_param_f.put_bid + near_param_f.put_ask) / 2.0,
        risk_free_rate: r1,
        time_to_expiration: t1,
    });
    let next_f = calculate_f(ParamF {
        strike_price: next_param_f.strike_price,
        call_price: (next_param_f.call_bid + next_param_f.call_ask) / 2.0,
        put_price: (next_param_f.put_bid + next_param_f.put_ask) / 2.0,
        risk_free_rate: r2,
        time_to_expiration: t2,
    });

    let near_strike_prices = near_data.iter().map(|d| d.strike_price).collect::<Vec<f64>>();
    let near_k_0_idx = find_closest_less_than_f(near_f, near_strike_prices.clone()).unwrap();
    let near_k_0 = near_strike_prices.get(near_k_0_idx).unwrap();

    let next_strike_prices = next_data.iter().map(|d| d.strike_price).collect::<Vec<f64>>();
    let next_k_0_idx = find_closest_less_than_f(next_f, next_strike_prices.clone()).unwrap();
    let next_k_0 = next_strike_prices.get(next_k_0_idx).unwrap();

    println!("near_f: {}", near_f);
    println!("near_k_0: {}", near_k_0);
    println!("next_f: {}", next_f);
    println!("next_k_0: {}", next_k_0);

    // [2] Select Options to calculate
    let near_options = convert_data_to_options(near_data);
    let next_options = convert_data_to_options(next_data);

    let near_calls = select_target_calls(*near_k_0, near_options.clone());
    let near_puts = select_target_puts(*near_k_0, near_options.clone());
    println!("near_calls: {:?}", near_calls.iter().map(|op| op.strike_price).collect::<Vec<f64>>());
    println!("near_puts: {:?}", near_puts.iter().map(|op| op.strike_price).collect::<Vec<f64>>());

    let next_calls = select_target_calls(*next_k_0, next_options.clone());
    let next_puts = select_target_puts(*next_k_0, next_options.clone());
    println!("next_calls: {:?}", next_calls.iter().map(|op| op.strike_price).collect::<Vec<f64>>());
    println!("next_puts: {:?}", next_puts.iter().map(|op| op.strike_price).collect::<Vec<f64>>());

    // [3] Calculate Variance -> TODO

    // [4] Calculate VIX -> TODO
}