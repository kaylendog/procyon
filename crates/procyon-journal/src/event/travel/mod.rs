//! Contains data types for travel-related events, such as hyperspace jumps and station docks.

mod location;

use location::{Faction, Station, SystemFaction};
use serde::{Deserialize, Serialize};

/// An enumeration of travel related events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all_fields = "PascalCase")]
pub enum Travel {
    ApproachBody {},
    Docked {},
    DockingCancelled {},
    DockingDenied {},
    DockingGranted {},
    DockingRequested {},
    DockingTimeout {},
    FSDJump {
        taxi: bool,
        multicrew: bool,
        #[serde(flatten)]
        system: System,
        body: String,
        #[serde(rename = "BodyID")]
        body_id: i64,
        body_type: String,
        jump_dist: f64,
        fuel_used: f64,
        fuel_level: f64,
    },
    FSDTarget {
        system_address: u64,
        star_class: String,
        remaining_jumps_in_route: Option<u16>,
    },
    LeaveBody {},
    Liftoff {},
    Location {
        #[serde(rename = "DistFromStarLS")]
        dist_from_star_ls: Option<f64>,
        docked: bool,
        #[serde(flatten)]
        station: Option<Station>,
        taxi: bool,
        multicrew: bool,
        #[serde(flatten)]
        system: System,
        body: String,
        #[serde(rename = "BodyID")]
        body_id: i64,
        body_type: String,
        factions: Vec<Faction>,
        system_faction: SystemFaction,
    },
    StartJump {
        jump_type: String,
        taxi: bool,
    },
    SupercruiseEntry {},
    SupercruiseExit {},
    Touchdown {},
    Undocked {},
    NavRoute,
    NavRouteClear,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct System {
    #[serde(rename = "StarSystem")]
    name: String,
    #[serde(rename = "SystemAddress")]
    address: u64,
    #[serde(rename = "StarPos")]
    star_pos: (f64, f64, f64),
    #[serde(rename = "SystemAllegiance")]
    allegiance: String,
    #[serde(rename = "SystemEconomy")]
    economy: String,
    #[serde(rename = "SystemEconomy_Localised")]
    economy_localised: String,
    #[serde(rename = "SystemSecondEconomy")]
    second_economy: String,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    second_economy_localised: String,
    #[serde(rename = "SystemGovernment")]
    government: String,
    #[serde(rename = "SystemGovernment_Localised")]
    government_localised: String,
    #[serde(rename = "SystemSecurity")]
    security: String,
    #[serde(rename = "SystemSecurity_Localised")]
    security_localised: String,
    population: u64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_location() {
        let input = r#"{ "timestamp":"2024-12-09T14:37:10Z", "event":"Location", "Docked":false, "Taxi":false, "Multicrew":false, "StarSystem":"Okinura", "SystemAddress":7505556017866, "StarPos":[11.87500,0.37500,-30.25000], "SystemAllegiance":"Federation", "SystemEconomy":"$economy_Service;", "SystemEconomy_Localised":"Service", "SystemSecondEconomy":"$economy_Agri;", "SystemSecondEconomy_Localised":"Agriculture", "SystemGovernment":"$government_Corporate;", "SystemGovernment_Localised":"Corporate", "SystemSecurity":"$SYSTEM_SECURITY_high;", "SystemSecurity_Localised":"High Security", "Population":4729900750, "Body":"Okinura", "BodyID":0, "BodyType":"Star", "Factions":[ { "Name":"Okinura Democrats", "FactionState":"None", "Government":"Democracy", "Influence":0.030120, "Allegiance":"Federation", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }, { "Name":"Okinura Group", "FactionState":"None", "Government":"Corporate", "Influence":0.037149, "Allegiance":"Federation", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":5.280000 }, { "Name":"United Okinura Focus", "FactionState":"CivilUnrest", "Government":"Dictatorship", "Influence":0.015060, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000, "RecoveringStates":[ { "State":"InfrastructureFailure", "Trend":0 } ], "ActiveStates":[ { "State":"CivilUnrest" } ] }, { "Name":"Okinura Inc", "FactionState":"None", "Government":"Corporate", "Influence":0.038153, "Allegiance":"Federation", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }, { "Name":"Adnovi Security Holdings", "FactionState":"Expansion", "Government":"Corporate", "Influence":0.268072, "Allegiance":"Federation", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000, "ActiveStates":[ { "State":"Expansion" } ] }, { "Name":"Okinura Bureau", "FactionState":"None", "Government":"Dictatorship", "Influence":0.035141, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }, { "Name":"Earth Defense Fleet", "FactionState":"Expansion", "Government":"Corporate", "Influence":0.576305, "Allegiance":"Federation", "Happiness":"$Faction_HappinessBand1;", "Happiness_Localised":"Elated", "MyReputation":100.000000, "ActiveStates":[ { "State":"CivilLiberty" }, { "State":"Boom" }, { "State":"Expansion" } ] } ], "SystemFaction":{ "Name":"Earth Defense Fleet", "FactionState":"Expansion" } }"#;
        let location: super::Travel = serde_json::from_str(input).unwrap();
    }
}
