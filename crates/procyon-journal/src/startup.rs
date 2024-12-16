use serde::{Deserialize, Serialize};

/// An enumeration of the possible startup events found in the game journal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Startup {
    /// Represents cargo-related data during startup.
    ///
    /// Contains the vessel name and its inventory.
    Cargo {
        /// The name of the vessel.
        vessel: String,
        /// The inventory of the vessel.
        inventory: Vec<CargoItem>,
    },
    /// Clears the saved game for a specific player.
    ///
    /// Contains the player name and their unique Frontier ID (FID).
    ClearSavedGame {
        /// The name of the commander.
        name: String,
        /// The unique Frontier ID of the commander.
        #[serde(rename = "FID")]
        fid: String,
    },
    /// Represents the commander data during startup.
    ///
    /// Includes the commander's name and unique Frontier ID (FID).
    Commander {
        /// The name of the commander.
        name: String,
        /// The unique Frontier ID of the commander.
        #[serde(rename = "FID")]
        fid: String,
    },
    /// Represents the loading of a saved game.
    ///
    /// Contains various details about the game state and the ship being loaded.
    LoadGame {
        /// The name of the commander.
        commander: String,
        /// The unique Frontier ID of the commander.
        #[serde(rename = "FID")]
        fid: String,
        /// Whether the game uses Horizons expansion.
        horizons: bool,
        /// The details of the ship being used.
        #[serde(flatten)]
        ship: Ship,
        /// The current fuel level of the ship.
        fuel_level: f64,
        /// The maximum fuel capacity of the ship.
        fuel_capacity: i64,
        /// The mode of the game (e.g., solo, open).
        game_mode: String,
        /// The current credits of the commander.
        credits: i64,
        /// The current loan amount.
        loan: i64,
    },
    /// Represents the ship's loadout during startup.
    ///
    /// Contains details about the ship and its modules.
    Loadout {
        /// The details of the ship.
        #[serde(flatten)]
        ship: Ship,
        /// The value of the ship's hull.
        hull_value: Option<f64>,
        /// The value of the ship's modules.
        modules_value: Option<f64>,
        /// The health of the ship's hull.
        hull_health: f64,
        /// The unladen mass of the ship.
        unladen_mass: f64,
        /// The fuel capacity of the ship.
        fuel_capacity: FuelCapacity,
        /// The cargo capacity of the ship.
        cargo_capacity: f64,
        /// The maximum jump range of the ship.
        max_jump_range: f64,
        /// The rebuy cost of the ship.
        rebuy: f64,
        /// Whether the ship is flagged as hot (wanted).
        hot: Option<bool>,
        /// The modules installed on the ship.
        modules: Vec<Module>,
    },
    /// Represents the materials available during startup.
    ///
    /// Includes raw, manufactured, and encoded materials.
    Materials {
        /// A list of raw materials.
        raw: Vec<Material>,
        /// A list of manufactured materials.
        manufactured: Vec<Material>,
        /// A list of encoded materials.
        encoded: Vec<Material>,
    },
    /// Represents the missions status during startup.
    ///
    /// Contains lists of active, failed, and completed missions.
    Missions {
        /// A list of active missions.
        active: Vec<Mission>,
        /// A list of failed missions.
        failed: Vec<Mission>,
        /// A list of completed missions.
        complete: Vec<Mission>,
    },
    /// Represents the creation of a new commander.
    ///
    /// Contains the commander's name, unique Frontier ID, and their package.
    NewCommander {
        /// The name of the new commander.
        name: String,
        /// The unique Frontier ID of the commander.
        #[serde(rename = "FID")]
        fid: String,
        /// The package chosen by the commander.
        package: String,
    },
    /// Represents passenger data during startup.
    ///
    /// Contains a list of passengers.
    Passengers(Vec<Passenger>),
    /// Represents powerplay-related data during startup.
    ///
    /// Includes details about the power and its statistics.
    Powerplay {
        /// The name of the power.
        power: String,
        /// The rank of the commander in the power.
        rank: i64,
        /// The merits earned by the commander.
        merits: i64,
        /// The votes contributed by the commander.
        votes: i64,
        /// The time pledged to the power (in seconds).
        time_pledged: i64,
    },
    /// Represents progress data during startup.
    ///
    /// Contains details about progress in various areas.
    Progress {
        /// The combat rank progress.
        combat: i64,
        /// The trade rank progress.
        trade: i64,
        /// The exploration rank progress.
        explore: i64,
        /// The Empire rank progress.
        empire: i64,
        /// The Federation rank progress.
        federation: i64,
        /// The CQC (Close Quarter Combat) rank progress.
        #[serde(rename = "CQC")]
        cqc: i64,
    },
    /// Represents rank data during startup.
    ///
    /// Contains details about ranks in various areas.
    Rank {
        /// The combat rank.
        combat: i64,
        /// The trade rank.
        trade: i64,
        /// The exploration rank.
        explore: i64,
        /// The Empire rank.
        empire: i64,
        /// The Federation rank.
        federation: i64,
        /// The CQC (Close Quarter Combat) rank.
        #[serde(rename = "CQC")]
        cqc: i64,
    },
    /// Represents reputation data during startup.
    ///
    /// Contains details about reputation with various factions.
    Reputation {
        /// The reputation with the Empire faction.
        empire: f32,
        /// The reputation with the Federation faction.
        federation: f32,
        /// The reputation with independent factions.
        independent: f32,
        /// The reputation with the Alliance faction.
        alliance: f32,
    },
    /// Represents detailed statistics during startup.
    ///
    /// Boxed to keep the overall size of the enum reasonable.
    Statistics(Box<Statistics>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CargoItem {
    pub name: String,
    pub count: i64,
    pub stolen: i64,
    #[serde(rename = "MissionID")]
    pub mission_id: Option<i64>,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ship {
    #[serde(rename = "Ship")]
    kind: String,
    #[serde(rename = "ShipID")]
    id: i64,
    #[serde(rename = "ShipName")]
    name: String,
    #[serde(rename = "ShipIdent")]
    ident: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuelCapacity {
    pub main: f64,
    pub reserve: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Module {
    pub slot: String,
    pub item: String,
    pub on: bool,
    pub priority: u8,
    pub health: f64,
    pub value: f64,
    pub ammo_in_clip: Option<u32>,
    pub ammo_in_hopper: Option<u32>,
    pub engineering: Option<Engineering>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Engineering {
    pub engineer_id: u32,
    pub engineer: String,
    pub blueprint_id: u32,
    pub blueprint_name: String,
    pub level: u8,
    pub quality: f64,
    pub experimental_effect: Option<String>,
    pub modifications: Vec<Modification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Modification {
    pub label: String,
    pub value: Option<f64>,
    pub original_value: f64,
    pub less_is_good: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Material {
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mission {
    #[serde(rename = "MissionID")]
    pub mission_id: i64,
    pub name: String,
    pub passenger_mission: bool,
    pub expires: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Passenger {
    pub mission_id: u32,
    #[serde(rename = "type")]
    pub kind: String,
    pub vip: bool,
    pub wanted: bool,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Statistics {
    /// The bank account-related statistics.
    #[serde(rename = "Bank_Account")]
    pub bank_account: BankAccountStatistics,
    /// The combat-related statistics.
    pub combat: CombatStatistics,
    /// The crime-related statistics.
    pub crime: CrimeStatistics,
    /// The smuggling-related statistics.
    pub smuggling: SmugglingStatistics,
    /// The trading-related statistics.
    pub trading: TradingStatistics,
    /// The mining-related statistics.
    pub mining: MiningStatistics,
    /// The exploration-related statistics.
    pub exploration: ExplorationStatistics,
    /// The passenger-related statistics.
    pub passengers: PassengersStatistics,
    /// The search and rescue-related statistics.
    #[serde(rename = "Search_And_Rescue")]
    pub search_and_rescue: SearchAndRescueStatistics,
    /// The crafting-related statistics.
    pub crafting: CraftingStatistics,
    /// The crew-related statistics.
    pub crew: CrewStatistics,
    /// The multicrew-related statistics.
    pub multicrew: MulticrewStatistics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BankAccountStatistics {
    #[serde(rename = "Current_Wealth")]
    pub current_wealth: i64,
    #[serde(rename = "Spent_On_Ships")]
    pub spent_on_ships: i64,
    #[serde(rename = "Spent_On_Outfitting")]
    pub spent_on_outfitting: i64,
    #[serde(rename = "Spent_On_Repairs")]
    pub spent_on_repairs: i64,
    #[serde(rename = "Spent_On_Fuel")]
    pub spent_on_fuel: i64,
    #[serde(rename = "Spent_On_Ammo_Consumables")]
    pub spent_on_ammo_consumables: i64,
    #[serde(rename = "Insurance_Claims")]
    pub insurance_claims: i64,
    #[serde(rename = "Spent_On_Insurance")]
    pub spent_on_insurance: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CombatStatistics {
    #[serde(rename = "Bounties_Claimed")]
    pub bounties_claimed: i64,
    #[serde(rename = "Bounty_Hunting_Profit")]
    pub bounty_hunting_profit: i64,
    #[serde(rename = "Combat_Bonds")]
    pub combat_bonds: i64,
    #[serde(rename = "Combat_Bond_Profits")]
    pub combat_bond_profits: i64,
    pub assassinations: i64,
    #[serde(rename = "Assassination_Profits")]
    pub assassination_profits: i64,
    #[serde(rename = "Highest_Single_Reward")]
    pub highest_single_reward: i64,
    #[serde(rename = "Skimmers_Killed")]
    pub skimmers_killed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrimeStatistics {
    pub fines: i64,
    #[serde(rename = "Total_Fines")]
    pub total_fines: i64,
    #[serde(rename = "Bounties_Received")]
    pub bounties_received: i64,
    #[serde(rename = "Total_Bounties")]
    pub total_bounties: i64,
    #[serde(rename = "Highest_Bounty")]
    pub highest_bounty: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SmugglingStatistics {
    #[serde(rename = "Black_Markets_Traded_With")]
    pub black_markets_traded_with: i64,
    #[serde(rename = "Black_Markets_Profits")]
    pub black_markets_profits: i64,
    #[serde(rename = "Resources_Smuggled")]
    pub resources_smuggled: i64,
    #[serde(rename = "Average_Profit")]
    pub average_profit: i64,
    #[serde(rename = "Highest_Single_Transaction")]
    pub highest_single_transaction: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TradingStatistics {
    #[serde(rename = "Markets_Traded_With")]
    pub markets_traded_with: i64,
    #[serde(rename = "Market_Profits")]
    pub market_profits: i64,
    #[serde(rename = "Resources_Traded")]
    pub resources_traded: i64,
    #[serde(rename = "Average_Profit")]
    pub average_profit: i64,
    #[serde(rename = "Highest_Single_Transaction")]
    pub highest_single_transaction: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MiningStatistics {
    #[serde(rename = "Mining_Profits")]
    pub mining_profits: i64,
    #[serde(rename = "Quantity_Mined")]
    pub quantity_mined: i64,
    #[serde(rename = "Materials_Collected")]
    pub materials_collected: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExplorationStatistics {
    #[serde(rename = "Systems_Visited")]
    pub systems_visited: i64,
    #[serde(rename = "Fuel_Scooped")]
    pub fuel_scooped: i64,
    #[serde(rename = "Fuel_Purchased")]
    pub fuel_purchased: i64,
    #[serde(rename = "Exploration_Profits")]
    pub exploration_profits: i64,
    #[serde(rename = "Planets_Scanned_To_Level_2")]
    pub planets_scanned_to_level_2: i64,
    #[serde(rename = "Planets_Scanned_To_Level_3")]
    pub planets_scanned_to_level_3: i64,
    #[serde(rename = "Highest_Payout")]
    pub highest_payout: i64,
    #[serde(rename = "Total_Hyperspace_Distance")]
    pub total_hyperspace_distance: i64,
    #[serde(rename = "Total_Hyperspace_Jumps")]
    pub total_hyperspace_jumps: i64,
    #[serde(rename = "Greatest_Distance_From_Start")]
    pub greatest_distance_from_start: f64,
    #[serde(rename = "Time_Played")]
    pub time_played: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersStatistics {
    #[serde(rename = "Passengers_Missions_Bulk")]
    pub passengers_missions_bulk: i64,
    #[serde(rename = "Passengers_Missions_VIP")]
    pub passengers_missions_vip: i64,
    #[serde(rename = "Passengers_Missions_Delivered")]
    pub passengers_missions_delivered: i64,
    #[serde(rename = "Passengers_Missions_Ejected")]
    pub passengers_missions_ejected: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchAndRescueStatistics {
    #[serde(rename = "SearchRescue_Traded")]
    pub search_rescue_traded: i64,
    #[serde(rename = "SearchRescue_Profit")]
    pub search_rescue_profit: i64,
    #[serde(rename = "SearchRescue_Count")]
    pub search_rescue_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CraftingStatistics {
    #[serde(rename = "Spent_On_Crafting")]
    pub spent_on_crafting: i64,
    #[serde(rename = "Count_Of_Used_Engineers")]
    pub count_of_used_engineers: i64,
    #[serde(rename = "Recipes_Generated")]
    pub recipes_generated: i64,
    #[serde(rename = "Recipes_Generated_Rank_1")]
    pub recipes_generated_rank_1: i64,
    #[serde(rename = "Recipes_Generated_Rank_2")]
    pub recipes_generated_rank_2: i64,
    #[serde(rename = "Recipes_Generated_Rank_3")]
    pub recipes_generated_rank_3: i64,
    #[serde(rename = "Recipes_Generated_Rank_4")]
    pub recipes_generated_rank_4: i64,
    #[serde(rename = "Recipes_Generated_Rank_5")]
    pub recipes_generated_rank_5: i64,
    #[serde(rename = "Recipes_Applied")]
    pub recipes_applied: i64,
    #[serde(rename = "Recipes_Applied_Rank_1")]
    pub recipes_applied_rank_1: i64,
    #[serde(rename = "Recipes_Applied_Rank_2")]
    pub recipes_applied_rank_2: i64,
    #[serde(rename = "Recipes_Applied_Rank_3")]
    pub recipes_applied_rank_3: i64,
    #[serde(rename = "Recipes_Applied_Rank_4")]
    pub recipes_applied_rank_4: i64,
    #[serde(rename = "Recipes_Applied_Rank_5")]
    pub recipes_applied_rank_5: i64,
    #[serde(rename = "Recipes_Applied_On_Previously_Modified_Modules")]
    pub recipes_applied_on_previously_modified_modules: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewStatistics {
    #[serde(rename = "NpcCrew_TotalWages")]
    pub npc_crew_total_wages: i64,
    #[serde(rename = "NpcCrew_Hired")]
    pub npc_crew_hired: i64,
    #[serde(rename = "NpcCrew_Fired")]
    pub npc_crew_fired: i64,
    #[serde(rename = "NpcCrew_Died")]
    pub npc_crew_died: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MulticrewStatistics {
    #[serde(rename = "Multicrew_Time_Total")]
    pub multicrew_time_total: i64,
    #[serde(rename = "Multicrew_Gunner_Time_Total")]
    pub multicrew_gunner_time_total: i64,
    #[serde(rename = "Multicrew_Fighter_Time_Total")]
    pub multicrew_fighter_time_total: i64,
    #[serde(rename = "Multicrew_Credits_Total")]
    pub multicrew_credits_total: i64,
    #[serde(rename = "Multicrew_Fines_Total")]
    pub multicrew_fines_total: i64,
}
