use serde::{de::DeserializeOwned, Deserialize};

const PUBLIC_API: &str = "https://api.coin.z.com/public";
const _PRIVATE_API: &str = "https://api.coin.z.com/private";

#[derive(Debug)]
struct Error(String);

fn get<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
    let body = ureq::get(url).call().map_err(|e| Error(e.to_string()))?;
    body.into_json().map_err(|e| Error(e.to_string()))
}

#[derive(Debug, Deserialize)]
struct StatusResponse {
    data: StatusResponseStatus,
    responsetime: String,
    status: u8
}
#[derive(Debug, Deserialize)]
struct StatusResponseStatus {
    status: String,
}
pub fn public_status() -> Result<StatusResponse, Error> {
    let url = format!("{}/v1/status", PUBLIC_API);
    get(&url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_status() {
        let res = public_status();
        assert!(res.is_ok());
        let val = res.unwrap();
        assert_eq!(val.status, 0);
    }
}
