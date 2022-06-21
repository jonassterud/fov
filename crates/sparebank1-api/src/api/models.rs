#![allow(non_snake_case)]

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct AccountDTO {
    pub key: String,
    pub accountNumber: String,
    pub iban: String,
    pub name: String,
    pub description: String,
    pub balance: f64,
    pub availableBalance: f64,
    pub creditCardCreditLimit: f64,
    pub creditCardAccountID: String,
    pub currencyCode: String,
    pub owner: CustomerDTO,
    pub productType: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub productId: String,
    pub descriptionCode: String,
    pub obsCodes: Vec<String>,
    pub disposalRole: bool,
    pub accountProperties: AccountPropertiesDTO,
    pub linksDTO: LinksDTO,
    pub eInvoiceCustomerReference: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountPropertiesDTO {
    pub hasAccess: bool,
    pub userHasRightOfDisposal: bool,
    pub userHasRightOfAccess: bool,
    pub isTransferFromEnabled: bool,
    pub isTransferToEnabled: bool,
    pub isPaymentFromEnabled: bool,
    pub isBalancePreferred: bool,
    pub isFlexiLoan: bool,
    pub isCodebitorLoan: bool,
    pub isSecurityBalance: bool,
    pub isAksjesparekonto: bool,
    pub isSavingsAccount: bool,
    pub isBonusAccount: bool,
    pub isOwned: bool,
    pub isWithdrawalsAllowed: bool,
    pub isBlocked: bool,
    pub isHidden: bool,
    pub isBalanceUpdatedImmediatelyOnTransferTo: bool,
    pub isDefaultPaymentAccount: bool,
}

#[derive(Debug, Deserialize)]
pub struct AccountsDTO {
    pub accounts: Vec<AccountDTO>,
    pub linksDTO: LinksDTO,
}

#[derive(Debug, Deserialize)]
pub struct CustomerDTO {
    pub name: String,
    pub firstName: String,
    pub lastName: String,
    pub age: Number,
    pub customerKey: String,
    pub ssn: String,
    pub organisationNumber: String,
}

#[derive(Debug, Deserialize)]
pub struct LinksDTO {}

#[derive(Debug, Deserialize)]
pub struct Number {}

#[derive(Debug, Deserialize)]
pub struct ErrorDTO {
    pub code: String,
    pub message: String,
    pub httpCode: i32,
    pub resource: String,
    pub localizedMessage: LocalizedMessage,
}

#[derive(Debug, Deserialize)]
pub struct ErrorsDTO {
    pub errors: Vec<ErrorDTO>,
}

#[derive(Debug, Deserialize)]
pub struct LocalizedMessage {
    pub template: String,
    pub values: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AccountDetailsDTO {
    pub freeWithdrawalsLeft: i32,
    pub lastWithdrawalDate: String,
    pub interestRate: f64,
    pub blockedAmount: f64,
    pub blockedBsuAmount: f64,
    pub creditLine: f64,
    pub totalConfirmedAmount: f64,
    pub netCalculatedCreditInterest: f64,
    pub netCalculatedCreditLineInterest: f64,
    pub totalCreditInterestLastYear: f64,
}

#[derive(Debug, Deserialize)]
pub struct AccountBalanceDTO {
    pub accountBalance: f64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountBalanceRequestDTO {
    pub accountNumber: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountKeysDTO {
    pub accountKeysMap: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct AccountRoleDTO {
    pub roleType: String,
    pub validFromDate: String,
    pub validToDate: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountRolesDTO {
    pub roles: Vec<AccountRoleDTO>,
    pub ownerHasRightOfDisposal: bool,
}
