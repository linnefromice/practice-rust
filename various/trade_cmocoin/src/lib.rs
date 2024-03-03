use std::time::{SystemTime, UNIX_EPOCH};

use ring::hmac;
use serde::{de::DeserializeOwned, Deserialize};

const PUBLIC_API: &str = "https://api.coin.z.com/public";
const PRIVATE_API: &str = "https://api.coin.z.com/private";

#[derive(Debug)]
pub struct Error(String);

fn get<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
    let body = ureq::get(url).call().map_err(|e| Error(e.to_string()))?;
    body.into_json().map_err(|e| Error(e.to_string()))
}

#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    pub data: StatusResponseData,
    pub responsetime: String,
    pub status: u8
}
#[derive(Debug, Deserialize)]
pub struct StatusResponseData {
    pub status: String,
}
pub fn public_status() -> Result<StatusResponse, Error> {
    let url = format!("{}/v1/status", PUBLIC_API);
    get(&url)
}

#[derive(Debug, Deserialize)]
pub struct TickerResponse {
    pub data: Vec<TickerResponseData>,
    pub responsetime: String,
    pub status: u8
}
#[derive(Debug, Deserialize)]
pub struct TickerResponseData {
    pub ask: String,
    pub bid: String,
    pub high: String,
    pub last: String,
    pub low: String,
    pub symbol: String,
    pub timestamp: String,
    pub volume: String
}
pub fn public_ticker() -> Result<TickerResponse, Error> {
    let url = format!("{}/v1/ticker?symbol=DAI", PUBLIC_API);
    get(&url)
}

#[derive(Debug, Deserialize)]
pub struct OrderbooksResponse {
    pub data: OrderbooksResponseData,
    pub responsetime: String,
    pub status: u8
}
#[derive(Debug, Deserialize)]
pub struct OrderbooksResponseData {
    pub asks: Vec<Orderbook>,
    pub bids: Vec<Orderbook>,
    pub symbol: String
}
#[derive(Debug, Deserialize)]
pub struct Orderbook {
    pub price: String,
    pub size: String,
}
pub fn public_orderbooks() -> Result<OrderbooksResponse, Error> {
    let url = format!("{}/v1/orderbooks?symbol=DAI", PUBLIC_API);
    get(&url)
}

fn secrets() -> (String, String) {
    dotenv::dotenv().ok();

    let key = std::env::var("API_KEY").expect("API_KEY is not set");
    let secret = std::env::var("API_SECRET").expect("API_SECRET is not set");
    (key, secret)
}
fn get_timestamp() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

    since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1_000_000
}

fn private_call<T>(path: &str) -> Result<T, Error>
where
    T : std::fmt::Debug + DeserializeOwned,
{
    let (api_key, api_secret) = secrets();
    let timestamp = get_timestamp();
    let method = "GET";
    let url = format!("{}{}", PRIVATE_API, path);

    let text = format!("{}{}{}", timestamp, method, path);
    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, api_secret.as_bytes());
    let sign = hex::encode(hmac::sign(&signed_key, text.as_bytes()).as_ref());

    let body = ureq::get(&url)
        .set("API-KEY", &api_key)
        .set("API-TIMESTAMP", &timestamp.to_string())
        .set("API-SIGN", &sign)
        .call().map_err(|e| Error(e.to_string()))?;
    body.into_json().map_err(|e| Error(e.to_string()))
}

#[derive(Debug, Deserialize)]
pub struct AccountAssetsResponse {
    pub data: Vec<AccountAssetsResponseData>,
    pub responsetime: String,
    pub status: u8
}
#[derive(Debug, Deserialize)]
pub struct AccountAssetsResponseData {
    pub amount: String,
    pub available: String,
    #[serde(rename = "conversionRate")]
    pub conversion_rate: String,
    pub symbol: String
}
pub fn private_account_assets() -> Result<AccountAssetsResponse, Error> {
    private_call("/v1/account/assets")
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_public_status() {
    //     let res = public_status();
    //     assert!(res.is_ok());
    //     let val = res.unwrap();
    //     assert_eq!(val.status, 0);
    // }

    // #[test]
    // fn test_public_ticker() {
    //     let res = public_ticker();
    //     assert!(res.is_ok());
    //     let val = res.unwrap();
    //     assert_eq!(val.status, 0);
    // }

    // #[test]
    // fn test_public_orderbooks() {
    //     let res = public_orderbooks();
    //     assert!(res.is_ok());
    //     let val = res.unwrap();
    //     assert_eq!(val.status, 0);
    // }

    #[test]
    fn verification() {
        let res = private_account_assets();
        println!("{:?}", res)
    }
}
