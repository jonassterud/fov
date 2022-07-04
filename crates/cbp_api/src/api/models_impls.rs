use crate::api::models::*;

impl Into<shared::Asset> for Account {
    fn into(self) -> shared::Asset {
        shared::Asset {
            name: self.id.expect("Account missing name"),
            description: None,
            balance: self
                .balance
                .expect("Account missing balance")
                .parse()
                .expect("Account balance is not f64"),
            currency: self.currency.expect("Account missing currency"),
            value: 0.0,
        }
    }
}
