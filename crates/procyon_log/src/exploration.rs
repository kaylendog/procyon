use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExplorationEvent {
    Scan(Box<Scan>),
    MaterialCollected(MaterialInfo),
    MaterialDiscarded(MaterialInfo),
    MaterialDiscovered(MaterialDiscovered),
    BuyExplorationData(BuyExplorationData),
    SellExplorationData(SellExplorationData),
    Screenshot(Screenshot),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Scan {
    pub body_name: String,
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f64,
    pub tidal_lock: bool,
    pub terraform_state: String,
    pub planet_class: String,
    pub atmosphere: String,
    pub volcanism: String,
    #[serde(rename = "MassEM")]
    pub mass_em: f64,
    pub radius: f64,
    pub surface_gravity: f64,
    pub surface_temperature: f64,
    pub surface_pressure: f64,
    pub landable: bool,
    #[serde(flatten)]
    pub orbital_parametesr: Option<OrbitalParameters>,
    #[serde(flatten)]
    pub ring_properties: Option<RingProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrbitalParameters {
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub periapsis: f64,
    pub orbital_period: f64,
    pub rotation_period: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RingProperties {
    pub name: String,
    pub ring_class: String,
    pub mass_mt: String,
    pub inner_rad: f64,
    pub outer_rad: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialInfo {
    pub category: String,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialDiscovered {
    pub category: String,
    pub name: String,
    pub discovery_number: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuyExplorationData {
    pub system: String,
    pub cost: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SellExplorationData {
    pub systems: Vec<String>,
    pub discovered: Vec<String>,
    pub base_value: i64,
    pub bonus: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Screenshot {
    pub filename: String,
    pub width: i64,
    pub height: i64,
    pub system: String,
    pub body: String,
}
