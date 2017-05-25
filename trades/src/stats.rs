
/// Trade statistics in single time-frame for both sides (asks and bids).
pub struct TradeStats {
    /// Trade stats on ask side.
    pub asks: BookSideStats,

    /// Trade stats on bid side.
    pub bids: BookSideStats,
}

/// Trade side statistics in single time-frame for single side (asks or bids).
pub struct BookSideStats {
    /// Weighted average of rates.
    pub avg_rate: RateStats,

    pub min_rate: RateStats,
    pub max_rate: RateStats,
}

/// Rate statistics.
pub struct RateStats {
    /// Trade rate.
    pub rate: f64,

    /// Rate trade volume.
    pub volume: f64,
}
