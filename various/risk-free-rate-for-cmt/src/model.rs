#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CmtYield {
    pub days: u32,
    pub yield_: f64,
}

#[cfg(test)]
pub const YIELDS_IN_CBOE: [CmtYield; 12] = [
    CmtYield { days: 30, yield_: 0.03 }, // 1 Mo
    CmtYield { days: 60, yield_: 0.02 }, // 2 Mo
    CmtYield { days: 91, yield_: 0.04 }, // 3 Mo
    CmtYield { days: 182, yield_: 0.05 }, // 6 Mo
    CmtYield { days: 365, yield_: 0.08 }, // 1 Yr
    CmtYield { days: 730, yield_: 0.11 }, // 2 Yr
    CmtYield { days: 1095, yield_: 0.22 }, // 3 Yr
    CmtYield { days: 1825, yield_: 0.59 }, // 5 Yr
    CmtYield { days: 2555, yield_: 1.0 }, // 7 Yr
    CmtYield { days: 3650, yield_: 1.37 }, // 10 Yr
    CmtYield { days: 7300, yield_: 2.03 }, // 20 Yr
    CmtYield { days: 10950, yield_: 2.21 }, // 30 Yr
];

#[cfg(test)]
pub const YIELDS_IN_REAL: [CmtYield; 9] = [
    CmtYield { days: 182, yield_: 5.342 }, // 6 Mo
    CmtYield { days: 365, yield_: 4.988 }, // 1 Yr
    CmtYield { days: 730, yield_: 4.421 }, // 2 Yr
    CmtYield { days: 1095, yield_: 4.103 }, // 3 Yr
    CmtYield { days: 1825, yield_: 3.897 }, // 5 Yr
    CmtYield { days: 2555, yield_: 3.929 }, // 7 Yr
    CmtYield { days: 3650, yield_: 3.907 }, // 10 Yr
    CmtYield { days: 7300, yield_: 4.189 }, // 20 Yr
    CmtYield { days: 10950, yield_: 4.01 }, // 30 Yr
];

#[cfg(test)]
pub const YIELDS_IN_INVESTING_COM: [CmtYield; 12] = [
    CmtYield { days: 30, yield_: 5.388 }, // 1 Mo
    CmtYield { days: 60, yield_: 5.412 }, // 2 Mo
    CmtYield { days: 91, yield_: 5.395 }, // 3 Mo
    // CmtYield { days: TODO, yield_: 5.408 }, // 4 Mo
    CmtYield { days: 182, yield_: 5.334 }, // 6 Mo
    CmtYield { days: 365, yield_: 4.963 }, // 1 Yr
    CmtYield { days: 730, yield_: 4.419 }, // 2 Yr
    CmtYield { days: 1095, yield_: 4.100 }, // 3 Yr
    CmtYield { days: 1825, yield_: 3.897 }, // 5 Yr
    CmtYield { days: 2555, yield_: 3.930 }, // 7 Yr
    CmtYield { days: 3650, yield_: 3.914 }, // 10 Yr
    CmtYield { days: 7300, yield_: 4.202 }, // 20 Yr
    CmtYield { days: 10950, yield_: 4.026 }, // 30 Yr
];
