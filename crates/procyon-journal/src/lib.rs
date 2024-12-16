//! Provides data types for working with Elite Dangerous's journal files.

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use chrono::{DateTime, Utc};

pub mod combat;
pub mod common;
pub mod exploration;
pub mod lifecycle;
pub mod travel;

use serde::{Deserialize, Serialize};

/// A log entry in the Elite Dangerous game journal.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    /// The time at which this event occured.
    pub timestamp: DateTime<Utc>,
    /// The event's payload.
    #[serde(flatten)]
    pub payload: Event,
}

/// An event type recorded in the game log. Contents are heap allocated since
/// their size is significant.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
pub enum Event {
    /// Cargo event.
    Cargo,
    /// Commander event.
    Commander,
    /// Engineer progress event.
    EngineerProgress,
    /// File header event.
    Fileheader,
    /// FSD jump event.
    FSDJump,
    /// FSD target event.
    FSDTarget,
    /// FSS signal discovered event.
    FSSSignalDiscovered,
    /// Fuel scoop event.
    FuelScoop,
    /// Load game event.
    LoadGame,
    /// Loadout event.
    Loadout,
    /// Location event.
    Location,
    /// Market event.
    Market,
    /// Materials event.
    Materials,
    /// Missions event.
    Missions,
    /// Module buy event.
    ModuleBuy,
    /// Module retrieve event.
    ModuleRetrieve,
    /// Module store event.
    ModuleStore,
    /// Music event.
    Music,
    /// Navigation route event.
    NavRoute,
    /// Navigation route clear event.
    NavRouteClear,
    /// NPC crew paid wage event.
    NpcCrewPaidWage,
    /// Outfitting event.
    Outfitting,
    /// Progress event.
    Progress,
    /// Rank event.
    Rank,
    /// Receive text event.
    ReceiveText,
    /// Reputation event.
    Reputation,
    /// Scan event.
    Scan,
    /// Scan barycentre event.
    ScanBaryCentre,
    /// Sell drones event.
    SellDrones,
    /// Ship locker event.
    ShipLocker,
    /// Shipyard event.
    Shipyard,
    /// Shipyard swap event.
    ShipyardSwap,
    /// Shutdown event.
    Shutdown,
    /// Start jump event.
    StartJump,
    /// Statistics event.
    Statistics,
    /// Stored modules event.
    StoredModules,
    /// Stored ships event.
    StoredShips,
    /// Undocked event.
    Undocked,
    /// Module info event.
    ModuleInfo,
    /// Jet cone boost event.
    JetConeBoost,
}

/// A reader for Elite Dangerous journal files.
pub struct JournalReader {
    /// The inner lines iterator.
    inner: Lines<BufReader<File>>,
}

impl JournalReader {
    /// Create a new journal reader from a file.
    pub fn new(inner: File) -> Self {
        Self { inner: BufReader::new(inner).lines() }
    }
}

impl Iterator for JournalReader {
    type Item = Result<Entry, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = match self.inner.next()? {
            Ok(line) => line,
            Err(e) => return Some(Err(e.into())),
        };
        let entry: Entry = match serde_json::from_str(&line) {
            Ok(entry) => entry,
            Err(e) => return Some(Err(e.into())),
        };
        Some(Ok(entry))
    }
}
