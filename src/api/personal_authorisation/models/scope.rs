use serde_derive::{Deserialize, Serialize};


/// Scope of the requested access with the possible values listed below: AIS scopes: ACCOUNTS_BASIC (note: Must always be present at a minimum, when requesting ACCOUNTS_* scopes) ACCOUNTS_BALANCES ACCOUNTS_DETAILS ACCOUNTS_TRANSACTIONS PIS scope: PAYMENTS_MULTIPLE  Cards scopes: CARDS_INFORMATION CARDS_TRANSACTIONS
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "ACCOUNTS_BASIC")]
    AccountsBasic,
    #[serde(rename = "ACCOUNTS_BALANCES")]
    AccountsBalances,
    #[serde(rename = "ACCOUNTS_DETAILS")]
    AccountsDetails,
    #[serde(rename = "ACCOUNTS_TRANSACTIONS")]
    AccountsTransactions,
    #[serde(rename = "PAYMENTS_MULTIPLE")]
    PaymentsMultiple,
    #[serde(rename = "CARDS_INFORMATION")]
    CardsInformation,
    #[serde(rename = "CARDS_TRANSACTIONS")]
    CardsTransactions,
}


impl Default for Scope {
    fn default() -> Scope {
        Self::AccountsBasic
    }
}
