use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
pub enum Powerplay {
    #[serde(rename = "PowerplayCollect")]
    Collect,
    #[serde(rename = "PowerplayDefect")]
    Defect,
    #[serde(rename = "PowerplayDeliver")]
    Deliver,
    #[serde(rename = "PowerplayFastTrack")]
    FastTrack,
    #[serde(rename = "PowerplayJoin")]
    Join,
    #[serde(rename = "PowerplayLeave")]
    Leave,
    #[serde(rename = "PowerplaySalary")]
    Salary,
    #[serde(rename = "PowerplayVote")]
    Vote,
    #[serde(rename = "PowerplayVoucher")]
    Voucher,
}
