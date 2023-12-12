use serde::Deserialize;

use k::{calculate_f, ParamF, find_closest_less_than_f};
use options::{convert_data_to_options, select_target_puts, select_target_calls};
use variance::variance_per_term;
use vix::{ParamVix, calculate_vix, ParamVixPerTerm};

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

    // Calculate Variance
    println!("near");
    let variance_1 = calculate_variance_per_term(near_data, near_atm_strike, r1, t1);
    println!("variance_1: {}", variance_1);

    println!("next");
    let variance_2 = calculate_variance_per_term(next_data, next_atm_strike, r2, t2);
    println!("variance_2: {}", variance_2);

    // Calculate VIX
    let vix = calculate_vix(ParamVix {
        near: ParamVixPerTerm {
            variance: variance_1,
            t: t1,
            minites_until_t: 34484.0,
        },
        next: ParamVixPerTerm {
            variance: variance_2,
            t: t2,
            minites_until_t: 44954.0,
        },
    });
    println!("vix: {}", vix);
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
    // println!("calls: {:?}", calls);
    // println!("puts: {:?}", puts);

    // [3] Calculate Variance
    let (k_0_from_calls, calls) = calls.split_first().unwrap();
    let (k_0_from_puts, puts) = puts.split_first().unwrap();
    // println!("k_0_from_calls: {:?}", k_0_from_calls);
    // println!("k_0_from_puts: {:?}", k_0_from_puts);
    assert!(k_0_from_calls.strike_price == *k_0);
    assert!(k_0_from_puts.strike_price == *k_0);

    let k_0_option = variance::Option {
        strike_price: *k_0,
        // NOTE: calculate mid from call and ask
        bid: (k_0_from_calls.bid + k_0_from_calls.ask) / 2.0,
        ask: (k_0_from_puts.bid + k_0_from_puts.ask) / 2.0,
    };
    let variance_options = [
        puts.iter().rev().map(|op| variance::Option {
            strike_price: op.strike_price,
            bid: op.bid,
            ask: op.ask,
        }).collect::<Vec<variance::Option>>(),
        vec![k_0_option],
        calls.iter().map(|op| variance::Option {
            strike_price: op.strike_price,
            bid: op.bid,
            ask: op.ask,
        }).collect::<Vec<variance::Option>>(),
    ].concat();
    
    variance_per_term(variance::ParamVariance {
        options: variance_options,
        forward_price: f,
        k_0: *k_0,
        risk_free_rate,
        time_to_expiration,
    })
}
