/*
 * Authorization API
 *
 * Nordea Authorization API v5
 *
 * The version of the OpenAPI document: 5.0
 * Contact: support@nordeaopenbanking.com
 * Generated by: https://openapi-generator.tech
 */

use std::collections::HashMap;
use super::Error;
use crate::api::configuration;
use crate::api::personal_authorisation::endpoints::ResponseContent;
use reqwest;
use reqwest::header::{HeaderMap};
use serde_derive::{Deserialize, Serialize};
use url::Url;
use crate::{calculate_digest, encrypt_signature, get_signature_base, nordea_utc_now};


/// struct for typed errors of method [`access_token_using_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccessTokenUsingPostError {
    Status400(crate::api::personal_authorisation::models::ErrorResponse),
    Status401(crate::api::personal_authorisation::models::ErrorResponse),
    Status403(crate::api::personal_authorisation::models::ErrorResponse),
    Status404(crate::api::personal_authorisation::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`authorization_v5_se_fi_dk_no`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthorizationV5SeFiDkNoError {
    Status400(crate::api::personal_authorisation::models::ErrorResponse),
    Status401(crate::api::personal_authorisation::models::ErrorResponse),
    Status403(crate::api::personal_authorisation::models::ErrorResponse),
    Status404(crate::api::personal_authorisation::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

fn resolve_request_url_encoded(request_data: &HashMap<String, String>) -> String {
    if request_data.is_empty(){
        return "".to_string()
    }
    let mut data: Vec<String> = request_data.iter().map(|(key, value)|
        format!("{}={}", key, value)).collect();
    data.sort();
    data.join("&")
}

#[allow(dead_code)]

pub async fn access_token_using_post(
    configuration: &configuration::Configuration,
    grant_type: &str,
    x_nordea_originating_user_agent: Option<&str>,
    x_nordea_originating_user_ip: Option<&str>,
    code: Option<String>,
    redirect_uri: Option<&str>,
    refresh_token: Option<&str>,
) -> Result<crate::api::personal_authorisation::models::BearerToken, Error<AccessTokenUsingPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;
    let date = nordea_utc_now();
    let local_var_uri_str = format!("{}/v5/authorize/token", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("X-IBM-Client-Id", configuration.x_ibm_client_id.to_string());
    local_var_req_builder =
        local_var_req_builder.header("X-IBM-Client-Secret", configuration.x_ibm_client_secret.to_string());
    let mut hash_map: HashMap<String, String> = HashMap::new();
    let form_encoded = match grant_type {
        "authorization_code" => {
            match redirect_uri {
                None => {
                    hash_map.insert("code".to_string(), code.clone().unwrap());
                    hash_map.insert("grant_type".to_string(), "authorization_code".to_string());
                    resolve_request_url_encoded(&hash_map)
                },
                Some(r) => {
                    hash_map.insert("code".to_string(), code.clone().unwrap());
                    hash_map.insert("grant_type".to_string(), "authorization_code".to_string());
                    hash_map.insert("redirect_uri".to_string(), r.to_string());
                    resolve_request_url_encoded(&hash_map)
                },
            }
        },
        "refresh_token" => {
            hash_map.insert("grant_type".to_string(), "refresh_token".to_string());
            hash_map.insert("refresh_token".to_string(), refresh_token.unwrap().to_string());
            resolve_request_url_encoded(&hash_map)
        },
        _ => "".to_string()
    };
    let digest = calculate_digest(form_encoded.clone());
    local_var_req_builder = local_var_req_builder.header("Digest", digest.clone());
    let signature = get_signature_base(
        Url::parse(local_var_uri_str.as_str()).unwrap(),
        reqwest::Method::POST,
        &date,
        Some("application/x-www-form-urlencoded"),
        Some(digest.as_str()));
    let signature_header = encrypt_signature(signature, configuration);
    local_var_req_builder = local_var_req_builder.header("Signature", signature_header);
    local_var_req_builder = local_var_req_builder.header(
        "X-Nordea-Originating-Date",
        date.clone(),
    );
    local_var_req_builder = local_var_req_builder.header(
        "X-Nordea-Originating-Host",
        &configuration.originating_host,
    );
    if let Some(local_var_param_value) = x_nordea_originating_user_agent {
        local_var_req_builder = local_var_req_builder.header(
            "X-Nordea-Originating-User-Agent",
            local_var_param_value.to_string(),
        );
    }
    if let Some(local_var_param_value) = x_nordea_originating_user_ip {
        local_var_req_builder = local_var_req_builder.header(
            "X-Nordea-Originating-User-Ip",
            local_var_param_value.to_string(),
        );
    }
    let mut local_var_form_params = HashMap::new();
    if let Some(local_var_param_value) = code {
        local_var_form_params.insert("code", local_var_param_value);
    }
    local_var_form_params.insert("grant_type", grant_type.to_string());
    if let Some(local_var_param_value) = redirect_uri {
        local_var_form_params.insert("redirect_uri", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = refresh_token {
        local_var_form_params.insert("refresh_token", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AccessTokenUsingPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
#[allow(dead_code)]
pub async fn authorization_v5_se_fi_dk_no(
    configuration: &configuration::Configuration,
    request: crate::api::personal_authorisation::models::AuthRequest,
    x_nordea_originating_user_agent: Option<&str>,
    x_nordea_originating_user_ip: Option<&str>,
) -> Result<HeaderMap, Error<AuthorizationV5SeFiDkNoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;
    let local_var_uri_str = format!("{}/v5/authorize", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    let date = nordea_utc_now();
    if let Ok(value) = serde_json::to_string(&request){
        let mut header_map = HeaderMap::new();
        header_map.insert("Content-Type", "application/json".parse().unwrap());
        let digest = calculate_digest(value);
        local_var_req_builder = local_var_req_builder.header("Digest", digest.clone());
        let signature = get_signature_base(
            Url::parse(local_var_uri_str.as_str()).unwrap(),
            reqwest::Method::POST,
            &date,
            Some("application/json"),
            Some(digest.as_str()));
        let signature_header = encrypt_signature(signature, configuration);
        local_var_req_builder = local_var_req_builder.header("Signature", signature_header);
    }
    local_var_req_builder =
        local_var_req_builder.header("X-IBM-Client-Id", &configuration.x_ibm_client_id);
    local_var_req_builder =
        local_var_req_builder.header("X-IBM-Client-Secret", &configuration.x_ibm_client_secret);
    local_var_req_builder = local_var_req_builder.header(
        "X-Nordea-Originating-Date",
        date,
    );
    local_var_req_builder = local_var_req_builder.header(
        "X-Nordea-Originating-Host",
        &configuration.originating_host,
    );
    if let Some(local_var_param_value) = x_nordea_originating_user_agent {
        local_var_req_builder = local_var_req_builder.header(
            "X-Nordea-Originating-User-Agent",
            local_var_param_value.to_string(),
        );
    }
    if let Some(local_var_param_value) = x_nordea_originating_user_ip {
        local_var_req_builder = local_var_req_builder.header(
            "X-Nordea-Originating-User-Ip",
            local_var_param_value.to_string(),
        );
    }
    local_var_req_builder = local_var_req_builder.json(&request);
    let local_var_resp = local_var_req_builder.send().await?;
    let local_var_status = local_var_resp.status();
    let headers = local_var_resp.headers().clone();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(headers)
    } else {
        let local_var_entity: Option<AuthorizationV5SeFiDkNoError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}