use serde::Serialize;

/// Struct to hold information about an asset, used by the server API
#[derive(Debug, Serialize, Clone)]
pub struct Asset {
    /// Name of the asset
    pub name: String,
    /// Description of the asset
    pub description: Option<String>,
    /// Balance of the asset
    pub balance: f64,
    /// Currency of the asset
    pub currency: String,
    /// Value of asset in NOK
    pub value: f64,
}
