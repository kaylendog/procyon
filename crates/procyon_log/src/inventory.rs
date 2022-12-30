use serde::{Deserialize, Serialize};

/// Event containing the commander's materials inventory.
#[derive(Deserialize)]
pub struct Materials {
    pub raw: Vec<Material>,
    pub encoded: Vec<Material>,
    pub manufactured: Vec<Material>,
}

/// A material owned by the commander.
#[derive(Deserialize)]
pub struct Material {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Name_Localised")]
    pub localized_name: Option<String>,
    #[serde(rename = "Count")]
    pub count: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipLocker {
    pub items: Vec<()>,
    pub components: Vec<()>,
    pub consumables: Vec<()>,
    pub data: Vec<()>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Loadout {
    pub ship: String,
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    pub ship_name: String,
    pub ship_ident: String,
    pub hull_value: i64,
    pub modules_value: i64,
    pub hull_health: f64,
    pub unladen_mass: f64,
    pub cargo_capacity: i64,
    pub max_jump_range: f64,
    pub fuel_capacity: FuelCapacity,
    pub rebuy: i64,
    pub modules: Vec<Module>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuelCapacity {
    pub main: f64,
    pub reserve: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Module {
    pub slot: String,
    pub item: String,
    pub on: bool,
    pub priority: i64,
    pub health: f64,
    pub value: Option<i64>,
    pub ammo_in_clip: Option<i64>,
    pub ammo_in_hopper: Option<i64>,
    pub engineering: Option<Engineering>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Engineering {
    pub engineer: String,
    #[serde(rename = "EngineerID")]
    pub engineer_id: i64,
    #[serde(rename = "BlueprintID")]
    pub blueprint_id: i64,
    pub blueprint_name: String,
    pub level: i64,
    pub quality: f64,
    pub modifiers: Vec<Modifier>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Modifier {
    pub label: String,
    pub value: f64,
    pub original_value: f64,
    pub less_is_good: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cargo {
    pub vessel: String,
    pub count: i64,
    pub inventory: Vec<()>,
}
