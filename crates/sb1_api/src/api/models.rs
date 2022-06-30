// Models for Accounts API
// https://developer.sparebank1.no/#/api/2682DF86994D4B348363BE9AC4644EFC

use std::collections::HashMap;

pub type number = f64;

// TODO: Use Serde to rename to camelCase and fix special cases

pub struct AccountDTO {
    pub key: Option<String>,
    pub account_number: Option<String>,
    pub iban: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub balance: Option<number>,
    pub available_balance: Option<number>,
    pub credit_card_credit_limit: Option<number>,
    pub credit_card_account_id: Option<String>,
    pub currency_code: Option<String>,
    pub owner: Option<CustomerDTO>,
    pub product_type: Option<String>,
    pub _type: Option<String>,
    pub product_id: Option<String>,
    pub description_code: Option<String>,
    pub obs_codes: Option<Vec<String>>,
    pub disposal_role: Option<bool>,
    pub account_properties: Option<AccountPropertiesDTO>,
    pub links_dto: Option<LinksDTO>,
    pub e_invoice_customer_reference: Option<String>,
}

pub struct AccountPropertiesDTO {
    pub has_access: Option<bool>,
    pub user_has_right_of_disposal: Option<bool>,
    pub user_has_right_of_access: Option<bool>,
    pub is_transfer_from_enabled: Option<bool>,
    pub is_transfer_to_enabled: Option<bool>,
    pub is_payment_from_enabled: Option<bool>,
    pub is_balance_preferred: Option<bool>,
    pub is_flexi_loan: Option<bool>,
    pub is_codebitor_loan: Option<bool>,
    pub is_security_balance: Option<bool>,
    pub is_aksjesparekonto: Option<bool>,
    pub is_savings_account: Option<bool>,
    pub is_bonus_account: Option<bool>,
    pub is_owned: Option<bool>,
    pub is_withdrawals_allowed: Option<bool>,
    pub is_blocked: Option<bool>,
    pub is_hidden: Option<bool>,
    pub is_balance_updated_immediately_on_transfer_to: Option<bool>,
    pub is_default_payment_account: Option<bool>,
}

pub struct AccountsDTO {
    pub accounts: Option<Vec<AccountDTO>>,
    links_dto. Option<LinksDTO>,
}

pub struct CustomerDTO {
    pub name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub age: Option<number>,
    pub customer_key: Option<String>,
    pub ssn_key: Option<String>,
    pub organisation_number: Option<String>,
}

pub struct LinksDTO {

}

/*
pub struct Number {

}
*/

pub struct AccountBalanceDTO {
    pub account_balance: Option<number>,
    pub name: Option<String>,
}

pub struct ErrorDTO {
    pub code: Option<String>,
    pub message: Option<String>,
    pub http_code: Option<i32>,
    pub resource: Option<String>,
    pub localized_message: Option<LocalizedMessage>,
    pub trace_id: Option<String>,
}

pub struct ErrorsDTO {
    pub errors: Option<Vec<ErrorDTO>>,
}

pub struct LocalizedMessage {
    pub template: Option<String>,
    pub values: Option<Vec<String>>,
}

pub struct AccountBalanceRequestDTO {
    pub account_number: Option<String>,
}

pub AccountRoleDTO {
    pub role_type: Option<String>,
    pub valid_from_date: Option<String>,
    pub valid_to_date: Option<String>,
}

pub AccountRolesDTO {
    pub roles: Option<Vec<AccountRoleDTO>>,
    pub owner_has_right_of_disposal: Option<bool>,
}

pub AccountKeysDTO {
    pub account_keys_map: Option<HashMap<String, String>>,
}

pub struct AccountDetailsDTO {
    pub free_withdrawels_left: Option<i32>,
    pub last_withdrawal_date: Option<String>,
    pub interest_rate: Option<number>,
    pub blocked_amount: Option<number>,
    pub blocked_bsu_amount: Option<number>,
    pub credit_line: Option<number>,
    pub total_confirmed_amount: Option<number>,
    pub net_calculated_credit_interest: Option<number>,
    pub net_calculated_credit_line_interest: Option<number>,
    pub total_credit_interest_last_year: Option<number>,
    pub bank_identifier_code: Option<String>,
}