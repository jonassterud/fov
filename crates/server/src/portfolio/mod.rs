mod simple_account;

use anyhow::{anyhow, Result};
use coinbase_pro_api as cb;
use simple_account::SimpleAccount;
use sparebank1_api as spb;

#[derive(Debug)]
pub struct Portfolio {
    pub sparebank1_accounts: Vec<SimpleAccount>,
    pub nordnet_accounts: Vec<SimpleAccount>,
    pub coinbase_pro_accounts: Vec<SimpleAccount>,
}

impl Portfolio {
    pub fn new() -> Portfolio {
        Portfolio {
            sparebank1_accounts: vec![],
            nordnet_accounts: vec![],
            coinbase_pro_accounts: vec![],
        }
    }

    pub fn add_sparebank1_accounts(&mut self, accounts: spb::AccountsDTO) -> Result<()> {
        for account in accounts.accounts.ok_or(anyhow!("accounts field missing"))? {
            if account.balance > Some(0.0) {
                let simple_account = SimpleAccount::try_from(account)?;
                self.sparebank1_accounts.push(simple_account);
            }
        }

        Ok(())
    }

    pub async fn add_coinbase_pro_accounts(
        &mut self,
        accounts: Vec<cb::accounts::Account>,
    ) -> Result<()> {
        for account in accounts {
            if (account.balance.parse::<f64>()?) > 0.0 {
                let simple_account = SimpleAccount::try_from(account)?;
                self.coinbase_pro_accounts.push(simple_account);
            }
        }

        Ok(())
    }
}
