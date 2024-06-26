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


/// Pagination : Details of paginated response
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Pagination {
    /// Resource listing may return a continuationKey if there's more results available. Request may be retried with the continuationKey, but otherwise same parameters, in order to get more results.
    #[serde(rename = "continuation_key", skip_serializing_if = "Option::is_none")]
    pub continuation_key: Option<String>,
}

#[allow(dead_code)]
impl Pagination {
    /// Details of paginated response
    pub fn new() -> Pagination {
        Pagination {
            continuation_key: None,
        }
    }
}
