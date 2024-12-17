use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Station {
    #[serde(rename = "StationName")]
    name: String,
    #[serde(rename = "StationType")]
    kind: String,
    #[serde(rename = "MarketID")]
    market_id: i64,
    #[serde(rename = "StationFaction")]
    faction: StationFaction,
    #[serde(rename = "StationGovernment")]
    government: String,
    #[serde(rename = "StationGovernment_Localised")]
    government_localised: Option<String>,
    #[serde(rename = "StationAllegiance")]
    allegiance: String,
    #[serde(rename = "StationServices")]
    services: Vec<String>,
    #[serde(rename = "StationEconomy")]
    economy: String,
    #[serde(rename = "StationEconomy_Localised")]
    economy_localised: String,
    #[serde(rename = "StationEconomies")]
    economies: Vec<StationEconomy>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationFaction {
    pub name: String,
    pub faction_state: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationEconomy {
    pub name: String,
    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    pub proportion: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
    pub name: String,
    pub faction_state: String,
    pub government: String,
    pub influence: f64,
    pub allegiance: String,
    pub happiness: String,
    #[serde(rename = "Happiness_Localised")]
    pub happiness_localised: String,
    #[serde(rename = "MyReputation")]
    pub commander_reputation: f64,
    #[serde(default)]
    pub recovering_states: Vec<RecoveringState>,
    #[serde(default)]
    pub active_states: Vec<ActiveState>,
    #[serde(default)]
    pub pending_states: Vec<PendingState>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecoveringState {
    pub state: String,
    pub trend: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActiveState {
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PendingState {
    pub state: String,
    pub trend: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemFaction {
    pub name: String,
    pub faction_state: Option<String>,
}
