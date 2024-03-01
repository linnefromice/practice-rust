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
}
