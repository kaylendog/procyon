use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
pub enum Carrier {
    #[serde(rename = "CarrierJump")]
    Jump,
    #[serde(rename = "CarrierBuy")]
    Buy,
    #[serde(rename = "CarrierStats")]
    Stats,
    #[serde(rename = "CarrierJumpRequest")]
    JumpRequest,
    #[serde(rename = "CarrierDecommission")]
    Decommission,
    #[serde(rename = "CarrierCancelDecommission")]
    CancelDecommission,
    #[serde(rename = "CarrierBankTransfer")]
    BankTransfer,
    #[serde(rename = "CarrierDepositFuel")]
    DepositFuel,
    #[serde(rename = "CarrierCrewServices")]
    CrewServices,
    #[serde(rename = "CarrierFinance")]
    Finance,
    #[serde(rename = "CarrierShipPack")]
    ShipPack,
    #[serde(rename = "CarrierModulePack")]
    ModulePack,
    #[serde(rename = "CarrierTradeOrder")]
    TradeOrder,
    #[serde(rename = "CarrierDockingPermission")]
    DockingPermission,
    #[serde(rename = "CarrierNameChanged")]
    NameChanged,
    #[serde(rename = "CarrierJumpCancelled")]
    JumpCancelled,
}
