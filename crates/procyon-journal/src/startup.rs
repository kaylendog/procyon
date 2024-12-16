use serde::{Deserialize, Serialize};

/// An enumeration of the possible startup events found in the game journal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all_fields = "PascalCase")]
pub enum Startup {
    /// Represents cargo-related data during startup.
    ///
    /// Contains the vessel name and its inventory.
    Cargo {
        /// The name of the vessel.
        vessel: String,
        /// The inventory of the vessel.
        inventory: Option<Vec<CargoItem>>,
        /// The number of items in the inventory.
        count: u32,
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
        #[serde(rename = "FID")]
        fid: String,
        commander: String,
        horizons: bool,
        odyssey: bool,
        #[serde(flatten)]
        ship: Option<Ship>,
        fuel_level: Option<f64>,
        fuel_capacity: Option<f64>,
        game_mode: Option<String>,
        credits: u64,
        loan: u64,
        #[serde(rename = "language")]
        language: String,
        #[serde(rename = "gameversion")]
        gameversion: String,
        #[serde(rename = "build")]
        build: String,
    },
    /// Represents the ship's loadout during startup.
    ///
    /// Contains details about the ship and its modules.
    Loadout {
        /// The details of the ship.
        #[serde(flatten)]
        ship: Ship,
        hull_value: u64,
        modules_value: u64,
        hull_health: f64,
        unladen_mass: f64,
        cargo_capacity: u16,
        max_jump_range: f64,
        fuel_capacity: FuelCapacity,
        rebuy: i64,
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
        rank: u8,
        /// The merits earned by the commander.
        merits: u8,
        /// The votes contributed by the commander.
        votes: u8,
        /// The time pledged to the power (in seconds).
        time_pledged: u8,
    },
    /// Represents progress data during startup.
    ///
    /// Contains details about progress in various areas.
    Progress {
        /// The combat rank progress.
        combat: u8,
        /// The trade rank progress.
        trade: u8,
        /// The exploration rank progress.
        explore: u8,
        /// The Empire rank progress.
        empire: u8,
        /// The Federation rank progress.
        federation: u8,
        /// The CQC (Close Quarter Combat) rank progress.
        #[serde(rename = "CQC")]
        cqc: u8,
    },
    /// Represents rank data during startup.
    ///
    /// Contains details about ranks in various areas.
    Rank {
        /// The combat rank.
        combat: u8,
        /// The trade rank.
        trade: u8,
        /// The exploration rank.
        explore: u8,
        /// The Empire rank.
        empire: u8,
        /// The Federation rank.
        federation: u8,
        /// The CQC (Close Quarter Combat) rank.
        #[serde(rename = "CQC")]
        cqc: u8,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuelCapacity {
    pub main: f64,
    pub reserve: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Module {
    pub slot: String,
    pub item: String,
    pub on: bool,
    pub priority: i64,
    pub health: f64,
    pub value: Option<i64>,
    pub ammo_in_clip: Option<i64>,
    pub ammo_in_hopper: Option<i64>,
    pub engineering: Option<Engineering>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Engineering {
    pub engineer: String,
    #[serde(rename = "EngineerID")]
    pub engineer_id: i64,
    #[serde(rename = "BlueprintID")]
    pub blueprint_id: i64,
    pub blueprint_name: String,
    pub level: i64,
    pub quality: f64,
    pub modifiers: Vec<Modifier>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Modifier {
    pub label: String,
    pub value: f64,
    pub original_value: f64,
    pub less_is_good: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Material {
    pub name: String,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,
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
    pub bank_account: BankAccountStats,
    pub combat: CombatStats,
    pub crime: CrimeStats,
    pub smuggling: SmugglingStats,
    pub trading: TradingStats,
    pub mining: MiningStats,
    pub exploration: ExplorationStats,
    pub passengers: PassengersStats,
    #[serde(rename = "Search_And_Rescue")]
    pub search_and_rescue: SearchAndRescueStats,
    #[serde(rename = "TG_ENCOUNTERS")]
    pub thargoid_encounters: ThargoidEncounterStats,
    pub crafting: CraftingStats,
    pub crew: CrewStats,
    pub multicrew: MulticrewStats,
    #[serde(rename = "Material_Trader_Stats")]
    pub material_trader_stats: MaterialTraderStats,
    pub exobiology: ExobiologyStats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BankAccountStats {
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
    #[serde(rename = "Owned_Ship_Count")]
    pub owned_ship_count: i64,
    #[serde(rename = "Spent_On_Suits")]
    pub spent_on_suits: i64,
    #[serde(rename = "Spent_On_Weapons")]
    pub spent_on_weapons: i64,
    #[serde(rename = "Spent_On_Suit_Consumables")]
    pub spent_on_suit_consumables: i64,
    #[serde(rename = "Suits_Owned")]
    pub suits_owned: i64,
    #[serde(rename = "Weapons_Owned")]
    pub weapons_owned: i64,
    #[serde(rename = "Spent_On_Premium_Stock")]
    pub spent_on_premium_stock: i64,
    #[serde(rename = "Premium_Stock_Bought")]
    pub premium_stock_bought: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CombatStats {
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
    #[serde(rename = "OnFoot_Combat_Bonds")]
    pub on_foot_combat_bonds: i64,
    #[serde(rename = "OnFoot_Combat_Bonds_Profits")]
    pub on_foot_combat_bonds_profits: i64,
    #[serde(rename = "OnFoot_Vehicles_Destroyed")]
    pub on_foot_vehicles_destroyed: i64,
    #[serde(rename = "OnFoot_Ships_Destroyed")]
    pub on_foot_ships_destroyed: i64,
    #[serde(rename = "Dropships_Taken")]
    pub dropships_taken: i64,
    #[serde(rename = "Dropships_Booked")]
    pub dropships_booked: i64,
    #[serde(rename = "Dropships_Cancelled")]
    pub dropships_cancelled: i64,
    #[serde(rename = "ConflictZone_High")]
    pub conflict_zone_high: i64,
    #[serde(rename = "ConflictZone_Medium")]
    pub conflict_zone_medium: i64,
    #[serde(rename = "ConflictZone_Low")]
    pub conflict_zone_low: i64,
    #[serde(rename = "ConflictZone_Total")]
    pub conflict_zone_total: i64,
    #[serde(rename = "ConflictZone_High_Wins")]
    pub conflict_zone_high_wins: i64,
    #[serde(rename = "ConflictZone_Medium_Wins")]
    pub conflict_zone_medium_wins: i64,
    #[serde(rename = "ConflictZone_Low_Wins")]
    pub conflict_zone_low_wins: i64,
    #[serde(rename = "ConflictZone_Total_Wins")]
    pub conflict_zone_total_wins: i64,
    #[serde(rename = "Settlement_Defended")]
    pub settlement_defended: i64,
    #[serde(rename = "Settlement_Conquered")]
    pub settlement_conquered: i64,
    #[serde(rename = "OnFoot_Skimmers_Killed")]
    pub on_foot_skimmers_killed: i64,
    #[serde(rename = "OnFoot_Scavs_Killed")]
    pub on_foot_scavs_killed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrimeStats {
    pub notoriety: i64,
    pub fines: i64,
    #[serde(rename = "Total_Fines")]
    pub total_fines: i64,
    #[serde(rename = "Bounties_Received")]
    pub bounties_received: i64,
    #[serde(rename = "Total_Bounties")]
    pub total_bounties: i64,
    #[serde(rename = "Highest_Bounty")]
    pub highest_bounty: i64,
    #[serde(rename = "Malware_Uploaded")]
    pub malware_uploaded: i64,
    #[serde(rename = "Settlements_State_Shutdown")]
    pub settlements_state_shutdown: i64,
    #[serde(rename = "Production_Sabotage")]
    pub production_sabotage: i64,
    #[serde(rename = "Production_Theft")]
    pub production_theft: i64,
    #[serde(rename = "Total_Murders")]
    pub total_murders: i64,
    #[serde(rename = "Citizens_Murdered")]
    pub citizens_murdered: i64,
    #[serde(rename = "Omnipol_Murdered")]
    pub omnipol_murdered: i64,
    #[serde(rename = "Guards_Murdered")]
    pub guards_murdered: i64,
    #[serde(rename = "Data_Stolen")]
    pub data_stolen: i64,
    #[serde(rename = "Goods_Stolen")]
    pub goods_stolen: i64,
    #[serde(rename = "Sample_Stolen")]
    pub sample_stolen: i64,
    #[serde(rename = "Total_Stolen")]
    pub total_stolen: i64,
    #[serde(rename = "Turrets_Destroyed")]
    pub turrets_destroyed: i64,
    #[serde(rename = "Turrets_Overloaded")]
    pub turrets_overloaded: i64,
    #[serde(rename = "Turrets_Total")]
    pub turrets_total: i64,
    #[serde(rename = "Value_Stolen_StateChange")]
    pub value_stolen_state_change: i64,
    #[serde(rename = "Profiles_Cloned")]
    pub profiles_cloned: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SmugglingStats {
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
pub struct TradingStats {
    #[serde(rename = "Markets_Traded_With")]
    pub markets_traded_with: i64,
    #[serde(rename = "Market_Profits")]
    pub market_profits: i64,
    #[serde(rename = "Resources_Traded")]
    pub resources_traded: i64,
    #[serde(rename = "Average_Profit")]
    pub average_profit: f64,
    #[serde(rename = "Highest_Single_Transaction")]
    pub highest_single_transaction: i64,
    #[serde(rename = "Data_Sold")]
    pub data_sold: i64,
    #[serde(rename = "Goods_Sold")]
    pub goods_sold: i64,
    #[serde(rename = "Assets_Sold")]
    pub assets_sold: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MiningStats {
    #[serde(rename = "Mining_Profits")]
    pub mining_profits: i64,
    #[serde(rename = "Quantity_Mined")]
    pub quantity_mined: i64,
    #[serde(rename = "Materials_Collected")]
    pub materials_collected: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExplorationStats {
    #[serde(rename = "Systems_Visited")]
    pub systems_visited: i64,
    #[serde(rename = "Exploration_Profits")]
    pub exploration_profits: i64,
    #[serde(rename = "Planets_Scanned_To_Level_2")]
    pub planets_scanned_to_level_2: i64,
    #[serde(rename = "Planets_Scanned_To_Level_3")]
    pub planets_scanned_to_level_3: i64,
    #[serde(rename = "Efficient_Scans")]
    pub efficient_scans: i64,
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
    #[serde(rename = "OnFoot_Distance_Travelled")]
    pub on_foot_distance_travelled: i64,
    #[serde(rename = "Shuttle_Journeys")]
    pub shuttle_journeys: i64,
    #[serde(rename = "Shuttle_Distance_Travelled")]
    pub shuttle_distance_travelled: i64,
    #[serde(rename = "Spent_On_Shuttles")]
    pub spent_on_shuttles: i64,
    #[serde(rename = "First_Footfalls")]
    pub first_footfalls: i64,
    #[serde(rename = "Planet_Footfalls")]
    pub planet_footfalls: i64,
    #[serde(rename = "Settlements_Visited")]
    pub settlements_visited: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersStats {
    #[serde(rename = "Passengers_Missions_Accepted")]
    pub passengers_missions_accepted: i64,
    #[serde(rename = "Passengers_Missions_Disgruntled")]
    pub passengers_missions_disgruntled: i64,
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
pub struct SearchAndRescueStats {
    #[serde(rename = "SearchRescue_Traded")]
    pub search_rescue_traded: i64,
    #[serde(rename = "SearchRescue_Profit")]
    pub search_rescue_profit: i64,
    #[serde(rename = "SearchRescue_Count")]
    pub search_rescue_count: i64,
    #[serde(rename = "Salvage_Legal_POI")]
    pub salvage_legal_poi: i64,
    #[serde(rename = "Salvage_Legal_Settlements")]
    pub salvage_legal_settlements: i64,
    #[serde(rename = "Salvage_Illegal_POI")]
    pub salvage_illegal_poi: i64,
    #[serde(rename = "Salvage_Illegal_Settlements")]
    pub salvage_illegal_settlements: i64,
    #[serde(rename = "Maglocks_Opened")]
    pub maglocks_opened: i64,
    #[serde(rename = "Panels_Opened")]
    pub panels_opened: i64,
    #[serde(rename = "Settlements_State_FireOut")]
    pub settlements_state_fire_out: i64,
    #[serde(rename = "Settlements_State_Reboot")]
    pub settlements_state_reboot: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThargoidEncounterStats {
    #[serde(rename = "TG_ENCOUNTER_TOTAL")]
    pub tg_encounter_total: i64,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SYSTEM")]
    pub tg_encounter_total_last_system: String,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_TIMESTAMP")]
    pub tg_encounter_total_last_timestamp: String,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SHIP")]
    pub tg_encounter_total_last_ship: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CraftingStats {
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
    #[serde(rename = "Suit_Mods_Applied")]
    pub suit_mods_applied: i64,
    #[serde(rename = "Weapon_Mods_Applied")]
    pub weapon_mods_applied: i64,
    #[serde(rename = "Suits_Upgraded")]
    pub suits_upgraded: i64,
    #[serde(rename = "Weapons_Upgraded")]
    pub weapons_upgraded: i64,
    #[serde(rename = "Suits_Upgraded_Full")]
    pub suits_upgraded_full: i64,
    #[serde(rename = "Weapons_Upgraded_Full")]
    pub weapons_upgraded_full: i64,
    #[serde(rename = "Suit_Mods_Applied_Full")]
    pub suit_mods_applied_full: i64,
    #[serde(rename = "Weapon_Mods_Applied_Full")]
    pub weapon_mods_applied_full: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewStats {
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
pub struct MulticrewStats {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialTraderStats {
    #[serde(rename = "Trades_Completed")]
    pub trades_completed: i64,
    #[serde(rename = "Materials_Traded")]
    pub materials_traded: i64,
    #[serde(rename = "Encoded_Materials_Traded")]
    pub encoded_materials_traded: i64,
    #[serde(rename = "Raw_Materials_Traded")]
    pub raw_materials_traded: i64,
    #[serde(rename = "Grade_1_Materials_Traded")]
    pub grade_1_materials_traded: i64,
    #[serde(rename = "Grade_2_Materials_Traded")]
    pub grade_2_materials_traded: i64,
    #[serde(rename = "Grade_3_Materials_Traded")]
    pub grade_3_materials_traded: i64,
    #[serde(rename = "Grade_4_Materials_Traded")]
    pub grade_4_materials_traded: i64,
    #[serde(rename = "Grade_5_Materials_Traded")]
    pub grade_5_materials_traded: i64,
    #[serde(rename = "Assets_Traded_In")]
    pub assets_traded_in: i64,
    #[serde(rename = "Assets_Traded_Out")]
    pub assets_traded_out: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExobiologyStats {
    #[serde(rename = "Organic_Genus_Encountered")]
    pub organic_genus_encountered: i64,
    #[serde(rename = "Organic_Species_Encountered")]
    pub organic_species_encountered: i64,
    #[serde(rename = "Organic_Variant_Encountered")]
    pub organic_variant_encountered: i64,
    #[serde(rename = "Organic_Data_Profits")]
    pub organic_data_profits: i64,
    #[serde(rename = "Organic_Data")]
    pub organic_data: i64,
    #[serde(rename = "First_Logged_Profits")]
    pub first_logged_profits: i64,
    #[serde(rename = "First_Logged")]
    pub first_logged: i64,
    #[serde(rename = "Organic_Systems")]
    pub organic_systems: i64,
    #[serde(rename = "Organic_Planets")]
    pub organic_planets: i64,
    #[serde(rename = "Organic_Genus")]
    pub organic_genus: i64,
    #[serde(rename = "Organic_Species")]
    pub organic_species: i64,
}
