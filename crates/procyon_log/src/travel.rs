//! Contains data types for travel-related events, such as hyperspace jumps and station docks.

use serde::{Deserialize, Serialize};

/// An enumeration of travel related events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum TravelEvent {
    /// Event emitted when the commander docks at a station.
    Docked(Box<Docked>),
    /// Event emitted when the commander cancels a docking request.
    DockingCancelled,
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
