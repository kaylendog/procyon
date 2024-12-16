//! Provides data types for working with Elite Dangerous's journal files.

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use chrono::{DateTime, Utc};
use combat::Combat;
use exploration::Exploration;
use serde::{Deserialize, Serialize};

pub mod carrier;
pub mod combat;
pub mod exploration;
pub mod misc;
pub mod odyssey;
pub mod powerplay;
pub mod squadron;
pub mod startup;
pub mod station;
pub mod trade;
pub mod travel;

use carrier::Carrier;
use misc::Misc;
use odyssey::Odyssey;
use powerplay::Powerplay;
use squadron::Squadron;
use startup::Startup;
use station::Station;
use trade::Trade;
use travel::Travel;
/// A log entry in the Elite Dangerous game journal.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    /// The time at which this event occured.
    pub timestamp: DateTime<Utc>,
    /// The event's payload.
    #[serde(flatten)]
    pub payload: Event,
}

/// An event type recorded in the game log.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Event {
    /// The file header, written at the beginning of each log file.
    #[serde(rename_all = "camelCase")]
    Fileheader {
        part: i64,
        language: String,
        #[serde(rename = "Odyssey")]
        odyssey: bool,
        gameversion: String,
        build: String,
    },
    /// Startup events, such as loading the game and identifying the commander.
    Startup(Startup),
    /// Travel events, such as jumping to a new system or docking at a station.
    Travel(Travel),
    /// Combat events, such as taking damage or destroying a ship.
    Combat(Combat),
    /// Exploration events.
    Exploration(Exploration),
    /// Trade events.
    Trade(Trade),
    /// Station events.
    Station(Station),
    /// Powerplay events.
    Powerplay(Powerplay),
    Squadron(Squadron),
    Carrier(Carrier),
    Odyssey(Odyssey),
    Misc(Misc),
}

/// A reader for Elite Dangerous journal files.
#[derive(Debug)]
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
