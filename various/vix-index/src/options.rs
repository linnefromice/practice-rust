#[derive(Copy, Clone, Debug)]
pub struct Option {
    pub strike_price: f64,
    pub bid: f64,
    pub ask: f64,
    pub is_call: bool,
}
pub fn select_target_calls(k_0: f64, options: Vec<Option>) -> Vec<Option> {
    // Check call option & ATM, OTM
    let mut targets = options.iter().filter(|op| op.is_call && op.strike_price >= k_0).cloned().collect::<Vec<Option>>();
    // Sort by strike price asc to process from ATM
    targets.sort_by(|a, b| a.strike_price.partial_cmp(&b.strike_price).unwrap());

    non_zero_bid_rule(&mut targets)
}
pub fn select_target_puts(k_0: f64, options: Vec<Option>) -> Vec<Option> {
    // Check put option & ATM, OTM
    let mut targets = options.iter().filter(|op| !op.is_call && op.strike_price <= k_0).cloned().collect::<Vec<Option>>();
    // Sort by strike price desc to process from ATM
    targets.sort_by(|a, b| b.strike_price.partial_cmp(&a.strike_price).unwrap());

    non_zero_bid_rule(&mut targets)
}
fn non_zero_bid_rule(options: &mut Vec<Option>) -> Vec<Option> {
    // Non-Zero bid rule
    let mut result = vec![];
    let mut is_last_bid_zero = false;
    for target in options {
        if target.bid > 0.0 {
            result.push(*target);
            is_last_bid_zero = false;
        } else {
            if is_last_bid_zero {
                break;
            } else {
                is_last_bid_zero = true;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

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
    fn convert_data_to_options(data: Vec<Datum>) -> Vec<super::Option> {
        let mut options = vec![];
        for datum in data {
            options.push(super::Option {
                strike_price: datum.strike_price,
                bid: datum.call_bid,
                ask: datum.call_ask,
                is_call: true,
            });
            options.push(super::Option {
                strike_price: datum.strike_price,
                bid: datum.put_bid,
                ask: datum.put_ask,
                is_call: false,
            });
        }
        options
    }

    #[test]
    fn test_select_target_calls_in_near_term() {
        let data = read_data("resources/near-term.csv");
        let options = convert_data_to_options(data);

        let selected = select_target_calls(1960.0, options);
        let selected_strike_prices = selected.iter().map(|op| op.strike_price).collect::<Vec<f64>>();
        assert_eq!(selected_strike_prices, vec![1960.0, 1965.0, 1970.0, 1975.0, 1980.0, 1985.0, 1990.0, 1995.0, 2000.0, 2005.0, 2010.0, 2015.0, 2020.0, 2025.0, 2030.0, 2035.0, 2040.0, 2045.0, 2050.0, 2055.0, 2060.0, 2065.0, 2070.0, 2075.0, 2080.0, 2085.0, 2090.0, 2095.0, 2100.0, 2125.0]);
    }

    #[test]
    fn test_select_target_puts_in_near_term() {
        let data = read_data("resources/near-term.csv");
        let options = convert_data_to_options(data);

        let selected = select_target_puts(1960.0, options);
        let mut selected_strike_prices = selected.iter().map(|op| op.strike_price).collect::<Vec<f64>>();
        selected_strike_prices.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        assert_eq!(selected_strike_prices, vec![1370.0, 1375.0, 1380.0, 1385.0, 1390.0, 1395.0, 1400.0, 1410.0, 1420.0, 1425.0, 1430.0, 1435.0, 1440.0, 1445.0, 1450.0, 1455.0, 1460.0, 1465.0, 1470.0, 1475.0, 1480.0, 1485.0, 1490.0, 1495.0, 1500.0, 1505.0, 1510.0, 1515.0, 1520.0, 1525.0, 1530.0, 1535.0, 1540.0, 1545.0, 1550.0, 1555.0, 1560.0, 1565.0, 1570.0, 1575.0, 1580.0, 1585.0, 1590.0, 1595.0, 1600.0, 1605.0, 1610.0, 1615.0, 1620.0, 1625.0, 1630.0, 1635.0, 1640.0, 1645.0, 1650.0, 1655.0, 1660.0, 1665.0, 1670.0, 1675.0, 1680.0, 1685.0, 1690.0, 1695.0, 1700.0, 1705.0, 1710.0, 1715.0, 1720.0, 1725.0, 1730.0, 1735.0, 1740.0, 1745.0, 1750.0, 1755.0, 1760.0, 1765.0, 1770.0, 1775.0, 1780.0, 1785.0, 1790.0, 1795.0, 1800.0, 1805.0, 1810.0, 1815.0, 1820.0, 1825.0, 1830.0, 1835.0, 1840.0, 1845.0, 1850.0, 1855.0, 1860.0, 1865.0, 1870.0, 1875.0, 1880.0, 1885.0, 1890.0, 1895.0, 1900.0, 1905.0, 1910.0, 1915.0, 1920.0, 1925.0, 1930.0, 1935.0, 1940.0, 1945.0, 1950.0, 1955.0, 1960.0]);
    }

    #[test]
    fn test_select_target_calls_in_next_term() {
        let data = read_data("resources/next-term.csv");
        let options = convert_data_to_options(data);

        let selected = select_target_calls(1960.0, options);
        let selected_strike_prices = selected.iter().map(|op| op.strike_price).collect::<Vec<f64>>();
        assert_eq!(selected_strike_prices, vec![1960.0, 1965.0, 1970.0, 1975.0, 1980.0, 1985.0, 1990.0, 1995.0, 2000.0, 2005.0, 2010.0, 2015.0, 2020.0, 2025.0, 2030.0, 2035.0, 2040.0, 2045.0, 2050.0, 2060.0, 2070.0, 2075.0, 2100.0, 2125.0, 2150.0, 2200.0]);
    }

    #[test]
    fn test_select_target_puts_in_next_term() {
        let data = read_data("resources/next-term.csv");
        let options = convert_data_to_options(data);

        let selected = select_target_puts(1960.0, options);
        let mut selected_strike_prices = selected.iter().map(|op| op.strike_price).collect::<Vec<f64>>();
        selected_strike_prices.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        assert_eq!(selected_strike_prices, vec![1275.0, 1325.0, 1350.0, 1375.0, 1400.0, 1425.0, 1450.0, 1475.0, 1500.0, 1510.0, 1520.0, 1525.0, 1530.0, 1540.0, 1550.0, 1555.0, 1560.0, 1565.0, 1570.0, 1575.0, 1580.0, 1585.0, 1590.0, 1595.0, 1600.0, 1605.0, 1610.0, 1615.0, 1620.0, 1625.0, 1630.0, 1635.0, 1640.0, 1645.0, 1650.0, 1655.0, 1660.0, 1665.0, 1670.0, 1675.0, 1680.0, 1685.0, 1690.0, 1695.0, 1700.0, 1705.0, 1710.0, 1715.0, 1720.0, 1725.0, 1730.0, 1735.0, 1740.0, 1745.0, 1750.0, 1755.0, 1760.0, 1765.0, 1770.0, 1775.0, 1780.0, 1785.0, 1790.0, 1795.0, 1800.0, 1805.0, 1810.0, 1815.0, 1820.0, 1825.0, 1830.0, 1835.0, 1840.0, 1845.0, 1850.0, 1855.0, 1860.0, 1865.0, 1870.0, 1875.0, 1880.0, 1885.0, 1890.0, 1895.0, 1900.0, 1905.0, 1910.0, 1915.0, 1920.0, 1925.0, 1930.0, 1935.0, 1940.0, 1945.0, 1950.0, 1955.0, 1960.0]);
    }
}
