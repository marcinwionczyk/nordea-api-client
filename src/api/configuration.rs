/*
 * Authorization API
 *
 * Nordea Authorization API v5
 *
 * The version of the OpenAPI document: 5.0
 * Contact: support@nordeaopenbanking.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest::redirect::Policy;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub x_ibm_client_id: String,
    pub x_ibm_client_secret: String,
    pub originating_host: String,
    pub redirect_url: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}


pub type BasicAuth = (String, Option<String>);


#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

#[allow(dead_code)]

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            base_path: "https://api.nordeaopenbanking.com/personal".to_owned(),
            user_agent: Some("OpenAPI-Generator/5.0/rust".to_owned()),
            redirect_url: "https://example.com".to_string(),
            x_ibm_client_secret: "********************************".to_string(),
            x_ibm_client_id: "********************************".to_string(),
            originating_host: "api.nordeaopenbanking.com".to_string(),
            client: reqwest::Client::builder()
                .redirect(Policy::none())
                .build()
                .expect("Failed to build a reqwest client"),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}
