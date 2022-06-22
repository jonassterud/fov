use anyhow::{anyhow, Result};
use coinbase_pro_api as cb;
use sparebank1_api as spb;

#[derive(Debug)]
pub struct SimpleAccount {
    pub name: String,
    pub description: String,
    pub balance: f64,
    pub currency_code: String,
}

impl TryFrom<spb::AccountDTO> for SimpleAccount {
    type Error = anyhow::Error;

    fn try_from(item: spb::AccountDTO) -> Result<Self> {
        let name = item
            .name
            .ok_or(anyhow!("missing field name on AccountDTO"))?;
        let description = item
            .description
            .ok_or(anyhow!("missing field description on AccountDTO"))?;
        let balance = item
            .balance
            .ok_or(anyhow!("missing field balance on AccountDTO"))?;
        let currency_code = item
            .currencyCode
            .ok_or(anyhow!("missing field currencyCode on AccountDTO"))?;

        Ok(SimpleAccount {
            name: name,
            description: description,
            balance: balance,
            currency_code: currency_code,
        })
    }
}

impl TryFrom<cb::Account> for SimpleAccount {
    type Error = anyhow::Error;

    fn try_from(item: cb::Account) -> Result<Self> {
        Ok(SimpleAccount {
            name: item.currency,
            description: "".to_string(),
            balance: item.balance.parse::<f64>()?,
            currency_code: "NOK".to_string(),
        })
    }
}
