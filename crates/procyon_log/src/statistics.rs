use serde::{Deserialize, Serialize};

/// Event containing player statistics information
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Statistics {
    #[serde(rename = "Bank_Account")]
    pub bank_account: BankAccount,
    pub combat: Combat,
    pub crime: Crime,
    pub smuggling: Smuggling,
    pub trading: Trading,
    pub mining: Mining,
    pub exploration: Exploration,
    pub passengers: Passengers,
    #[serde(rename = "Search_And_Rescue")]
    pub search_and_rescue: SearchAndRescue,
    #[serde(rename = "TG_ENCOUNTERS")]
    pub tg_encounters: TgEncounters,
    pub crafting: Crafting,
    pub crew: Crew,
    pub multicrew: Multicrew,
    #[serde(rename = "Material_Trader_Stats")]
    pub material_trader_stats: MaterialTraderStats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BankAccount {
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Combat {
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
pub struct Crime {
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Smuggling {
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
pub struct Trading {
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mining {
    #[serde(rename = "Mining_Profits")]
    pub mining_profits: i64,
    #[serde(rename = "Quantity_Mined")]
    pub quantity_mined: i64,
    #[serde(rename = "Materials_Collected")]
    pub materials_collected: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Exploration {
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Passengers {
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
pub struct SearchAndRescue {
    #[serde(rename = "SearchRescue_Traded")]
    pub search_rescue_traded: i64,
    #[serde(rename = "SearchRescue_Profit")]
    pub search_rescue_profit: i64,
    #[serde(rename = "SearchRescue_Count")]
    pub search_rescue_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TgEncounters {
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
pub struct Crafting {
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Crew {
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
pub struct Multicrew {
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
}
