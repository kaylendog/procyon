//! Provides data types for working with Elite Dagnerous's log files.

pub extern crate chrono;
pub extern crate serde_json;

pub mod lifecycle;
pub mod travel;

use chrono::{DateTime, Utc};

use lifecycle::LifecycleEvent;

use serde::{Deserialize, Serialize};
use travel::TravelEvent;

/// A log entry in the Elite Dangerous game logs.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    /// The time at which this event occured.
    pub timestamp: DateTime<Utc>,
    /// The event's payload.
    #[serde(flatten)]
    pub payload: Event,
}

/// An event type recorded in the game log.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Event {
    /// A lifecycle event.
    Lifecycle(LifecycleEvent),
    /// A travel event.
    Travel(TravelEvent),
}
