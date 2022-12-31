//! Contains events relating to game lifecycle.

use serde::{Deserialize, Serialize};

/// An enumeration of lifecycle event types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum LifecycleEvent {
    ClearSavedGame(ClearSavedGame),
    NewCommander(NewCommander),
    LoadGame(LoadGame),
    Progress(Progress),
}

/// Event fired when a save game is cleared.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClearSavedGame {
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
    #[serde(rename = "GameMode", flatten)]
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
        group_name: String,
    },
}

/// Event containing the progress of the commander to the next rank for each faction.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Progress {
    pub combat: i64,
    pub trade: i64,
    pub explore: i64,
    pub empire: i64,
    pub federation: i64,
    #[serde(rename = "CQC")]
    pub cqc: i64,
}

/// Event containing the rank of each commander
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rank {
    pub combat: i64,
    pub trade: i64,
    pub explore: i64,
    pub empire: i64,
    pub federation: i64,
    #[serde(rename = "CQC")]
    pub cqc: i64,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lifecycle::{ClearSavedGame, Gamemode, LifecycleEvent, LoadGame},
        Entry, Event,
    };

    use super::NewCommander;

    #[test]
    fn deserialize_clear_saved_game() {
        let entry =
            r#"{ "timestamp":"2016-06-10T14:32:03Z", "event":"ClearSavedGame", "Name":"HRC1" }"#;
        let entry: Entry = serde_json::from_str(entry).unwrap();
        assert_eq!(
            entry.payload,
            Event::Lifecycle(LifecycleEvent::ClearSavedGame(ClearSavedGame {
                name: "HRC1".to_string()
            }))
        );
    }

    #[test]
    fn deserialize_load_game() {
        let entry = r#"
			{ 
				"timestamp": "2016-06-10T14:32:03Z",
				"event": "LoadGame",
				"Commander": "HRC1",
				"Ship": "CobraMkIII",
				"ShipID": 1,
				"GameMode": "Group",
				"Group": "Mobius",
				"Credits": 600120,
				"Loan": 0 
			}"#;
        let entry: Entry = serde_json::from_str(entry).unwrap();
        assert_eq!(
            entry.payload,
            Event::Lifecycle(LifecycleEvent::LoadGame(LoadGame {
                commander: "HRC1".to_string(),
                ship: "CobraMkIII".to_string(),
                ship_id: 1,
                gamemode: Gamemode::Group { group_name: "Mobius".to_string() },
                credits: 600120,
                loan: 0,
                start_dead: false,
                start_landed: false
            }))
        );
    }

    #[test]
    fn deserialize_new_commander() {
        let entry = r#"{ 
			"timestamp": "2016-06-10T14:32:03Z",
			"event": "NewCommander",
			"Name": "HRC1",
			"Package": "ImperialBountyHunter"
		}"#;
        let entry: Entry = serde_json::from_str(entry).unwrap();
        assert_eq!(
            entry.payload,
            Event::Lifecycle(LifecycleEvent::NewCommander(NewCommander {
                name: "HRC1".to_string(),
                package: "ImperialBountyHunter".to_string(),
            }))
        )
    }
}
