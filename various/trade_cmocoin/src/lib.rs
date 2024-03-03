use std::{collections::BTreeMap, time::{SystemTime, UNIX_EPOCH}};

use ring::hmac;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::json;

const PUBLIC_API: &str = "https://api.coin.z.com/public";
const PRIVATE_API: &str = "https://api.coin.z.com/private";

#[derive(Debug)]
pub struct Error(String);

#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(rename = "currentPage")]
    pub current_page: u32,
    pub count: u32,
}

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

fn private_get<T>(path: &str, query_parameters: Vec<(&str, &str)>) -> Result<T, Error>
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
        .query_pairs(query_parameters)
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
    private_get("/v1/account/assets", vec![])
}

#[derive(Debug, Deserialize)]
pub struct ActiveOrdersResponse {
    pub data: ActiveOrdersResponseData,
    pub responsetime: String,
    pub status: u8
}
#[derive(Debug, Deserialize)]
pub struct ActiveOrdersResponseData {
    pub pagination: Pagination,
    pub list: Vec<ActiveOrdersResponseDatum>,
}
#[derive(Debug, Deserialize)]
pub struct ActiveOrdersResponseDatum {
    #[serde(rename = "rootOrderId")]
    pub root_order_id: u128,
    #[serde(rename = "orderId")]
    pub order_id: u128,
    pub symbol: String,
    pub side: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "executionType")]
    pub execution_type: String,
    #[serde(rename = "settleType")]
    pub settle_type: String,
    pub size: String,
    #[serde(rename = "executedSize")]
    pub executed_size: String,
    pub price: String,
    #[serde(rename = "losscutPrice")]
    pub losscut_price: String,
    pub status: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    pub timestamp: String
}
pub fn private_active_orders() -> Result<ActiveOrdersResponse, Error> {
    private_get("/v1/activeOrders", vec![
        ("symbol", "DAI")
    ])
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
        // let res = private_account_assets();
        let res = private_active_orders();
        println!("{:?}", res)
    }
}
