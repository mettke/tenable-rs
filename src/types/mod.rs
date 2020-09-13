//! Tenable Types returned or expected by server

mod acr_update;
mod asset_by_uuid;
mod assets;
mod assets_move;

pub use acr_update::*;
pub use asset_by_uuid::*;
pub use assets::*;
pub use assets_move::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// `Source` blueprint
pub struct Source {
    /// The name of the entity that reported the asset details. Sources can include sensors, connectors, and API imports. Source names can be customized by your organization (for example, you specify a name when you import asset records). If your organization does not customize source names, system-generated names include:\n - AWS—You obtained the asset data from an Amazon Web Services connector.\n - NESSUS_AGENT—You obtained the asset data obtained from a Nessus agent scan.\n - PVS—You obtained the asset data from a Nessus Network Monitor (NNM) scan.\n - NESSUS_SCAN—You obtained the asset data from a Nessus scan.\n - WAS—You obtained the asset data from a  Web Application Scanning scan.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ISO timestamp when the source first reported the asset.
    #[serde(rename = "first_seen", skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    /// The ISO timestamp when the source last reported the asset.
    #[serde(rename = "last_seen", skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// `AcrDriver` blueprint
pub struct AcrDriver {
    /// The type of characteristic.
    #[serde(rename = "driver_name", skip_serializing_if = "Option::is_none")]
    pub driver_name: Option<String>,
    /// The characteristic value.
    #[serde(rename = "driver_value", skip_serializing_if = "Option::is_none")]
    pub driver_value: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Copy)]
/// `ScanFrequency` blueprint
pub struct ScanFrequency {
    /// The number of days over which Tenable searches for scans involving the asset.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    /// The number of times that a scan ran against the asset during the specified interval.
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
    /// Indicates whether the asset was licensed at the time of the identified scans.
    #[serde(rename = "licensed", skip_serializing_if = "Option::is_none")]
    pub licensed: Option<bool>,
}
