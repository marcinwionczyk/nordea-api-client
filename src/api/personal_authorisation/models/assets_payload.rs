/*
 * Authorization API
 *
 * Nordea Authorization API v5
 *
 * The version of the OpenAPI document: 5.0
 * Contact: support@nordeaopenbanking.com
 * Generated by: https://openapi-generator.tech
 */

use serde_derive::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetsPayload {
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<crate::api::personal_authorisation::models::AccountPayload>>,
    #[serde(rename = "cards", skip_serializing_if = "Option::is_none")]
    pub cards: Option<Vec<crate::api::personal_authorisation::models::CardPayload>>,
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<crate::api::personal_authorisation::models::Scope>>,
}


#[allow(dead_code)]
impl AssetsPayload {
    pub fn new() -> AssetsPayload {
        AssetsPayload {
            accounts: None,
            cards: None,
            scopes: None,
        }
    }
}