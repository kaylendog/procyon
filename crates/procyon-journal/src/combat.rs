//! Events relating to combat, bounties, and ship damage.

use serde::{Deserialize, Serialize};

/// Enumeration of combat events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CombatEvent {
    Bounty,
    CapShipBond,
    Died,
    // Died,
    EscapeInterdiction,
    FactionKillBond,
    FighterDestroyed,
    HeatDamage,
    HeatWarning,
    HullDamage,
    Interdicted,
    Interdiction,
    PVPKill,
    ShieldState,
    ShipTargetted,
    SRVDestroyed,
    UnderAttack,
}

/// Event emitted when the player is awarded a bounty for a kill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bounty {
    pub rewards: Vec<BountyReward>,
    pub target: String,
    pub total_reward: i64,
    pub victim_faction: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BountyReward {
    pub faction: String,
    pub reward: i64,
}

/// Event emitted when the player is rewarded for a capital ship combat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CapShipBond {
    pub reward: i64,
    pub awarding_faction: String,
    pub victim_faction: String,
}

/// Event emitted when the player is killed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum Died {
    /// The player was killed by a single player.
    Single { killer_name: String, killer_ship: String, killer_rank: String },
    /// The player was killed by a member of a wing.
    Wing(Vec<WingKiller>),
}

/// A member of the wing that was responsbile for killing the player.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WingKiller {
    name: String,
    ship: String,
    rank: String,
}

/// Event emitted when the player escapes interdiction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EscapeInterdiction {
    pub interdictor: String,
    pub is_player: bool,
}

/// Event emitted when the player is rewarded for taking part in a combat zone.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionKillBond {
    pub reward: i64,
    pub awarding_faction: String,
    pub victim_faction: String,
}

/// Event emitted when the player's ship takes hull damage.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HullDamage {
    pub health: f64,
}

/// Event emitted when the player is interdicted.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Interdicted {
    pub submitted: bool,
    pub interdictor: String,
    pub is_player: bool,
    pub faction: String,
}

/// Event emitted when the player interdicts another ship.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Interdiction {
    pub success: bool,
    pub interdicted: String,
    pub is_player: bool,
    pub combat_rank: i64,
}

/// Event emitted when the player kills another player.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PVPKill {
    pub victim: String,
    pub combat_rank: u8,
}

/// Event emitted when the player's shields are disabled in combat, or recharged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShieldState {
    pub shields_up: bool,
}
