//! Common data structures, such as economy and government types.

use serde::{Deserialize, Serialize};

/// An enumeration of economy types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EconomyType {
    #[serde(rename = "$economy_Extraction;")]
    Extraction,
    #[serde(rename = "$economy_Agri;")]
    Agriculture,
}

/// An enumeration of government types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovernmentType {
    #[serde(rename = "$government_Anarchy;")]
    Anarchy,
    #[serde(rename = "$government_Democracy;")]
    Democracy,
}

/// Star system position coordinates in lightyears from Sol.
pub type StarPos = [f64; 3];
