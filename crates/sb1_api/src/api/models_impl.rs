use crate::api::models::*;

impl Into<shared::Asset> for AccountDTO {
    fn into(self) -> shared::Asset {
        shared::Asset {
            name: self.name.expect("AccountDTO missing name"),
            description: self.description,
            balance: self.balance.expect("AccountDTO missing balance"),
            currency: self.currency_code.expect("AccountDTO missing currency"),
        }
    }
}
