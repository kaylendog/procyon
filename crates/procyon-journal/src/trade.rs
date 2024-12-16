use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
pub enum Trade {
    AsteroidCracked,
    BuyTradeData,
    CollectCargo,
    EjectCargo,
    MarketBuy,
    MarketSell,
    MiningRefined,
}
