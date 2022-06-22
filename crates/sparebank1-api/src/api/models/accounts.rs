#![allow(non_snake_case)]

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct AccountDTO {
    pub key: Option<String>,
    pub accountNumber: Option<String>,
    pub iban: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub balance: Option<f64>,
    pub availableBalance: Option<f64>,
    pub creditCardCreditLimit: Option<f64>,
    pub creditCardAccountID: Option<String>,
    pub currencyCode: Option<String>,
    pub owner: Option<CustomerDTO>,
    pub productType: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub productId: Option<String>,
    pub descriptionCode: Option<String>,
    pub obsCodes: Option<Vec<String>>,
    pub disposalRole: Option<bool>,
    pub accountProperties: Option<AccountPropertiesDTO>,
    pub LinksDTO: Option<LinksDTO>,
    pub eInvoiceCustomerReference: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AccountPropertiesDTO {
    pub hasAccess: Option<bool>,
    pub userHasRightOfDisposal: Option<bool>,
    pub userHasRightOfAccess: Option<bool>,
    pub isTransferFromEnabled: Option<bool>,
    pub isTransferToEnabled: Option<bool>,
    pub isPaymentFromEnabled: Option<bool>,
    pub isBalancePreferred: Option<bool>,
    pub isFlexiLoan: Option<bool>,
    pub isCodebitorLoan: Option<bool>,
    pub isSecurityBalance: Option<bool>,
    pub isAksjesparekonto: Option<bool>,
    pub isSavingsAccount: Option<bool>,
    pub isBonusAccount: Option<bool>,
    pub isOwned: Option<bool>,
    pub isWithdrawalsAllowed: Option<bool>,
    pub isBlocked: Option<bool>,
    pub isHidden: Option<bool>,
    pub isBalanceUpdatedImmediatelyOnTransferTo: Option<bool>,
    pub isDefaultPaymentAccount: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AccountsDTO {
    pub accounts: Option<Vec<AccountDTO>>,
    pub LinksDTO: Option<LinksDTO>,
}

#[derive(Debug, Deserialize)]
pub struct CustomerDTO {
    pub name: Option<String>,
    pub firstName: Option<String>,
    pub lastName: Option<String>,
    pub age: Option<i32>,
    pub customerKey: Option<String>,
    pub ssn: Option<String>,
    pub organisationNumber: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LinksDTO {}

/*
#[derive(Debug, Deserialize)]
pub struct Number {}
*/

#[derive(Debug, Deserialize)]
pub struct ErrorDTO {
    pub code: Option<String>,
    pub message: Option<String>,
    pub httpCode: Option<i32>,
    pub resource: Option<String>,
    pub localizedMessage: Option<LocalizedMessage>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorsDTO {
    pub errors: Option<Vec<ErrorDTO>>,
}

#[derive(Debug, Deserialize)]
pub struct LocalizedMessage {
    pub template: Option<String>,
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct AccountDetailsDTO {
    pub freeWithdrawalsLeft: Option<i32>,
    pub lastWithdrawalDate: Option<String>,
    pub interestRate: Option<f64>,
    pub blockedAmount: Option<f64>,
    pub blockedBsuAmount: Option<f64>,
    pub creditLine: Option<f64>,
    pub totalConfirmedAmount: Option<f64>,
    pub netCalculatedCreditInterest: Option<f64>,
    pub netCalculatedCreditLineInterest: Option<f64>,
    pub totalCreditInterestLastYear: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct AccountBalanceDTO {
    pub accountBalance: Option<f64>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AccountBalanceRequestDTO {
    pub accountNumber: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AccountKeysDTO {
    pub accountKeysMap: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct AccountRoleDTO {
    pub roleType: Option<String>,
    pub validFromDate: Option<String>,
    pub validToDate: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AccountRolesDTO {
    pub roles: Option<Vec<AccountRoleDTO>>,
    pub ownerHasRightOfDisposal: Option<bool>,
}
