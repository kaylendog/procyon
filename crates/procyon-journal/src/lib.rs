//! A library for parsing and reading Elite Dangerous journal files.

use serde::{Deserialize, Serialize};

pub mod event;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Backpack {
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "event")]
    pub event: String,
    // TODO: item types
    pub items: Vec<()>,
    pub components: Vec<()>,
    pub consumables: Vec<()>,
    pub data: Vec<()>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cargo {
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "event")]
    pub event: String,
    pub vessel: String,
    pub count: i64,
    pub inventory: Vec<()>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Market {
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "event")]
    pub event: String,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    pub station_name: String,
    pub station_type: String,
    pub star_system: String,
    pub items: Vec<MarketItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketItem {
    #[serde(rename = "id")]
    pub id: i64,
    pub name: String,
    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    pub category: String,
    #[serde(rename = "Category_Localised")]
    pub category_localised: String,
    pub buy_price: i64,
    pub sell_price: i64,
    pub mean_price: i64,
    pub stock_bracket: i64,
    pub demand_bracket: i64,
    pub stock: i64,
    pub demand: i64,
    pub consumer: bool,
    pub producer: bool,
    pub rare: bool,
}

pub struct ModuleInfo {
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Module {
    pub slot: String,
    pub item: String,
    pub power: f64,
    pub priority: Option<i64>,
}

pub struct RouteInfo {
    pub route: Vec<RouteEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RouteEntry {
    pub star_system: String,
    pub system_address: i64,
    pub star_pos: (f64, f64, f64),
    pub star_class: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Outfitting {
    pub market_id: i64,
    pub station_name: String,
    pub star_system: String,
    pub horizons: bool,
    pub items: Vec<OutfittingItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OutfittingItem {
    #[serde(rename = "id")]
    pub id: i64,
    pub name: String,
    pub buy_price: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipLocker {
    pub items: Vec<()>,
    pub components: Vec<()>,
    pub consumables: Vec<()>,
    pub data: Vec<()>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Shipyard {
    pub market_id: i64,
    pub station_name: String,
    pub star_system: String,
    pub horizons: bool,
    #[serde(rename = "AllowCobraMkIV")]
    pub allow_cobra_mk_iv: bool,
    pub price_list: Vec<PriceList>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PriceList {
    #[serde(rename = "id")]
    pub id: i64,
    pub ship_type: String,
    pub ship_price: i64,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
    pub flags: i64,
}

pub mod prelude {
    //! Prelude module for easy importing of all event types.

    pub use crate::event::{Entry, Event, JournalReader};

    pub use crate::event::combat::*;
    pub use crate::event::exploration::*;
    pub use crate::event::misc::*;
    pub use crate::event::odyssey::*;
    pub use crate::event::powerplay::*;
    pub use crate::event::squadron::*;
    pub use crate::event::startup::*;
    pub use crate::event::station::*;
    pub use crate::event::trade::*;
    pub use crate::event::travel::*;
}
