use std::{fmt, collections::HashMap};

use serde::{Deserializer, de::{Visitor, MapAccess}};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotValue {
    pub jsonrpc: String,
    pub id: String,
    pub result: _ResultX,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct _ResultX {
    #[serde(deserialize_with = "from_ticks")]
    pub ticks: HashMap<String, Tick>
}
pub fn from_ticks<'de, D>(deserializer: D) -> Result<HashMap<String, Tick>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(CustomVisitor)
}

struct CustomVisitor;
impl<'de> Visitor<'de> for CustomVisitor {
    type Value = HashMap<String, Tick>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>
    {
        let mut result = HashMap::new();
        while let Some((k, v)) = map.next_entry::<i64, Tick>()? {
            result.insert(k.to_string(), v);
        }
        Ok(result)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tick {
    pub index: Option<i64>,
    pub liquidity_gross: String,
    pub liquidity_net: String,
    pub fee_growth_outside_0x128: String,
    pub fee_growth_outside_1x128: String,
    pub initialized: bool,
}

fn main() {
    let file = std::fs::File::open("v3pool.json").unwrap();
    let reader = std::io::BufReader::new(file);
    let data: SnapshotValue = serde_json::from_reader(reader).unwrap();
    println!("{:?}", data);
}
