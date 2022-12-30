//! Contains events relating to game lifecycle.

use serde::{Deserialize, Serialize};

/// Shim boolean to allow literal defaults.
const fn dfalse() -> bool {
    false
}

/// An enumeration of lifecycle event types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum LifecycleEvent {
    ClearSaveGame(ClearSaveGame),
    NewCommander(NewCommander),
    LoadGame(LoadGame),
}

/// Event fired when a save game is cleared.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClearSaveGame {
    /// The commander name.
    pub name: String,
}

/// Event fired when creating a new commander.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NewCommander {
    /// The name of the commander.
    pub name: String,
    /// The selected starter package.
    pub package: String,
}

/// Event emitted when loading from the main menu into game.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadGame {
    /// The commander name.
    pub commander: String,
    /// The current ship type.
    pub ship: String,
    /// The ship ID number.
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    /// Whether the current ship is landed.
    #[serde(default)]
    pub start_landed: bool,
    /// Whether the commander is dead when loading in.
    #[serde(default)]
    pub start_dead: bool,
    /// The gamemode being loaded into.
    #[serde(rename = "GameMode")]
    pub gamemode: Gamemode,
    /// The commander's current credit balance.
    pub credits: i64,
    /// The commander's current loan.
    pub loan: i64,
}

/// An enumeration of gamemode types.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "GameMode")]
pub enum Gamemode {
    Open,
    #[default]
    Solo,
    Group {
        #[serde(rename = "Group")]
        group: String,
    },
}

#[cfg(test)]
mod tests {
    use crate::Entry;

    #[test]
    fn deserialize_load_game() {
        let entry = r#"{ "timestamp":"2016-06-10T14:32:03Z", "event":"LoadGame", "Commander":"HRC1", "Ship":"CobraMkIII", "ShipID":1, "GameMode":"Group", "Group":"Mobius", "Credits":600120, "Loan":0 }"#;
        let _event: Entry = serde_json::from_str(entry).unwrap();
    }
}
