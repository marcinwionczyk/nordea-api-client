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
pub struct CardPayload {
    #[serde(rename = "card_id", skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
}

#[allow(dead_code)]
impl CardPayload {
    pub fn new() -> CardPayload {
        CardPayload { card_id: None }
    }
}
