pub extern crate chrono;
pub extern crate serde_json;

pub mod inventory;
pub mod lifecycle;
pub mod location;
pub mod statistics;

use chrono::{DateTime, Utc};

use lifecycle::LifecycleEvent;

use serde::{Deserialize, Serialize};


/// An event type recorded in the game log.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum Event {
    /// A lifecycle event.
    Lifecycle(LifecycleEvent),
}

/// A log entry in the Elite Dangerous game logs.
#[derive(Deserialize)]
pub struct Entry {
    /// The time at which this event occured.
    pub timestamp: DateTime<Utc>,
    /// The event's payload.
    #[serde(flatten)]
    pub payload: Event,
}

/// Event containing game and language information.
#[derive(Deserialize)]
pub struct Fileheader {
    pub part: usize,
    pub language: String,
    #[serde(rename = "Odyssey")]
    pub odyssey: bool,
    pub gameversion: String,
    pub build: String,
}

/// Event containing commander information.
#[derive(Deserialize)]
pub struct Commander {
    #[serde(rename = "fid")]
    pub fid: String,
    #[serde(rename = "Name")]
    pub name: String,
}

/// Event containing the commander's rank information.
#[derive(Deserialize)]
pub struct Rank {
    #[serde(rename = "Combat")]
    pub combat: u8,
    #[serde(rename = "Trade")]
    pub trade: u8,
    #[serde(rename = "Explore")]
    pub explore: u8,
    #[serde(rename = "Soldier")]
    pub soldier: u8,
    #[serde(rename = "Exobiologist")]
    pub exobiologist: u8,
    #[serde(rename = "Empire")]
    pub empire: u8,
    #[serde(rename = "Federation")]
    pub federation: u8,
    #[serde(rename = "CQC")]
    pub cqc: u8,
}

/// Event containing the commander's progress information.
#[derive(Deserialize)]
pub struct Progress {
    #[serde(rename = "Combat")]
    pub combat: u8,
    #[serde(rename = "Trade")]
    pub trade: u8,
    #[serde(rename = "Explore")]
    pub explore: u8,
    #[serde(rename = "Soldier")]
    pub soldier: u8,
    #[serde(rename = "Exobiologist")]
    pub exobiologist: u8,
    #[serde(rename = "Empire")]
    pub empire: u8,
    #[serde(rename = "Federation")]
    pub federation: u8,
    #[serde(rename = "CQC")]
    pub cqc: u8,
}

/// Event containing the commander's reputation information.
#[derive(Deserialize)]
pub struct Reputation {
    #[serde(rename = "Empire", default)]
    pub empire: f32,
    #[serde(rename = "Federation", default)]
    pub federation: f32,
    #[serde(rename = "CQC", default)]
    pub cqc: f32,
}

/// Event containing the commander's engineering progress.
#[derive(Deserialize)]
pub struct EngineerProgress {
    #[serde(rename = "Engineer")]
    pub engineer: String,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u32,
    #[serde(rename = "Progress")]
    pub progress: UnlockState,
    #[serde(rename = "RankProgress")]
    pub rank_progress: u8,
    #[serde(rename = "Rank")]
    pub rank: usize,
}

/// The unlock state of a given engineer.
#[derive(Deserialize)]
pub enum UnlockState {
    Unlocked,
    Known,
    Invited,
}

/// Event containing information when the game is loaded.
#[derive(Deserialize)]
pub struct LoadGame {
    #[serde(rename = "FID")]
    pub fid: String,
    #[serde(rename = "Commander")]
    pub commander: String,
    #[serde(rename = "Horizons")]
    pub horizons: bool,
    #[serde(rename = "Odyssey")]
    pub odyssey: bool,
    #[serde(rename = "ShipID")]
    pub ship: Ship,
    #[serde(rename = "ShipName")]
    pub ship_name: String,
    #[serde(rename = "ShipIdent")]
    pub ship_identifier: String,
    #[serde(rename = "FuelLevel")]
    pub fuel_level: u8,
    #[serde(rename = "FuelCapacity")]
    pub fuel_capacity: u8,
    #[serde(rename = "GameMode")]
    pub gamemode: Gamemode,
    #[serde(rename = "Credits")]
    pub credits: u64,
    #[serde(rename = "Loan")]
    pub loan: u32,
    pub language: String,
    #[serde(rename = "gameversion")]
    pub game_version: String,
    pub build: String,
}

/// An enumeration of ship types.
#[derive(Deserialize)]
pub enum Ship {
    Python = 6,
}

/// An enumeration of gamemode types.
#[derive(Deserialize)]
pub enum Gamemode {
    Solo,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReceiveText {
    pub from: String,
    pub message: String,
    #[serde(rename = "Message_Localised")]
    pub message_localised: String,
    pub channel: String,
}
