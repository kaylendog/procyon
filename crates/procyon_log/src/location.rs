use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f64,
    pub docked: bool,
    pub taxi: bool,
    pub multicrew: bool,
    pub star_system: String,
    pub system_address: i64,
    pub star_pos: Vec<f64>,
    pub system_allegiance: String,
    pub system_economy: String,
    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localised: String,
    pub system_second_economy: String,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: String,
    pub system_government: String,
    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: String,
    pub system_security: String,
    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: String,
    pub population: i64,
    pub body: String,
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    pub body_type: String,
    pub factions: Vec<Faction>,
    pub system_faction: SystemFaction,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub my_reputation: f64,
    #[serde(default)]
    pub active_states: Vec<ActiveState>,
    #[serde(default)]
    pub recovering_states: Vec<RecoveringState>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActiveState {
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecoveringState {
    pub state: String,
    pub trend: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemFaction {
    pub name: String,
    pub faction_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FSDTarget {
    pub name: String,
    pub system_address: i64,
    pub star_class: String,
    pub remaining_jumps_in_route: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartJump {
    pub jump_type: String,
    pub star_system: String,
    pub system_address: i64,
    pub star_class: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum JumpType {
    #[serde(rename = "HyperSpace")]
    Hyperspace { star_system: String, system_address: i64, star_class: String },
    #[default]
    Supercruise,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FSDJump {
    pub taxi: bool,
    pub multicrew: bool,
    pub star_system: String,
    pub system_address: i64,
    pub star_pos: Vec<f64>,
    pub system_allegiance: String,
    pub system_economy: String,
    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localised: String,
    pub system_second_economy: String,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: String,
    pub system_government: String,
    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: String,
    pub system_security: String,
    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: String,
    pub population: i64,
    pub body: String,
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    pub body_type: String,
    pub jump_dist: f64,
    pub fuel_used: f64,
    pub fuel_level: f64,
    pub factions: Vec<Faction>,
    pub system_faction: SystemFaction,
}
