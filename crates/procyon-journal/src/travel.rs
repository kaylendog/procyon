//! Contains data types for travel-related events, such as hyperspace jumps and station docks.

use serde::{Deserialize, Serialize};

/// An enumeration of travel related events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all_fields = "PascalCase")]
pub enum Travel {
    ApproachBody {},
    Docked {},
    DockingCancelled {},
    DockingDenied {},
    DockingGranted {},
    DockingRequested {},
    DockingTimeout {},
    FSDJump {},
    FSDTarget {},
    LeaveBody {},
    Liftoff {},
    Location {},
    StartJump {},
    SupercruiseEntry {},
    SupercruiseExit {},
    Touchdown {},
    Undocked {},
    NavRoute {},
    NavRouteClear {},
}

/// Event emitted when the commander docks at a station.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Docked {
    /// The name of the station.
    pub station_name: String,
    /// The type of the station.
    pub station_type: String,
    /// The current star system.
    pub star_system: String,
    /// Whether the cockpit is breached.
    #[serde(default)]
    pub cockpit_breach: bool,
    /// The station's controlling faction.
    pub faction: String,
    /// The current state of the faction.
    pub faction_state: String,
    /// The allegiance of this faction.
    pub allegiance: String,
    /// The economy type of this station.
    pub economy: String,
    /// The type of government.
    pub government: String,
    /// The security of this station.
    pub security: String,
}

/// Event emitted when the player cancels a docking request.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingCancelled {
    ///  The name of the station.
    pub station_name: String,
}

/// Event emitted when the station denies a docking request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingDenied {
    /// The name of the station.
    pub station_name: String,
    /// The reason for denial.
    pub reason: DockingDenialReason,
}

/// An enumeration of docking denial reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DockingDenialReason {
    /// The station is full.
    NoSpace,
    /// The station has no appropriate landing pads.
    TooLarge,
    Hostile,
    Offences,
    /// The player is too far from the station.
    Distance,
    ActiveFighter,
    /// No reason specified.
    NoReason,
}

/// Event emitted when a docking request is granted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingGranted {
    /// The name of the station.
    pub station_name: String,
    /// The landing pad number.
    pub landing_pad: u8,
}

/// Event emitted when the player requests docking at a station.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequested {
    /// The name of the station.
    pub station_name: String,
}

/// Event emitted when a docking request times out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockingTimeout {
    /// The name of the station.
    pub station_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerplayState {
    InPrepareRadius,
    Prepared,
    Exploited,
    Contested,
    Controlled,
    Turmoil,
    HomeSystem,
}

/// Event emitted when lifting off from a planet's surface.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Liftoff {
    pub latitude: f64,
    pub longitude: f64,
}

/// Event emitted when entering supercruise from normal space.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseEntry {
    #[serde(rename = "Starsystem")]
    pub star_system: String,
}

/// Event emitted when leaving supercruise for normal space.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseExit {
    #[serde(rename = "Starsystem")]
    pub star_system: String,
    pub body: Option<String>,
    pub body_type: Option<()>,
}

/// Event emitted when landing on a planet's surface.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Touchdown {
    pub latitude: f64,
    pub longitude: f64,
}

/// Event emitted when lifting off from a landing pad in a station, outpost, or settlement.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Undocked {
    /// The name of the station.
    pub station_name: String,
}