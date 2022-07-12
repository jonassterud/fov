use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDTO {
    pub name: String,
    pub description: String,
    pub balance: f64,
    pub currency_code: String,
}

#[derive(Deserialize)]
pub struct AccountsDTO {
    pub accounts: Vec<AccountDTO>,
}
