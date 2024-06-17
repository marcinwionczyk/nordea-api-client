pub mod api;

use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use ring::{digest, signature};
use ring::rand::SystemRandom;
use url::Url;
use crate::api::configuration::Configuration;
use static_include_bytes::static_include_bytes;

static_include_bytes!(#[no_mangle] EIDAS_PRIVATE_KEY = concat!(env!("EIDAS_PRIVATE_KEY_DIR"), "/private-key.pk8"));
pub fn nordea_utc_now() -> String {
    Utc::now().format("%a, %d %b %Y %T GMT").to_string()
}

pub struct SignatureBase {
    host: String,
    path: String,
    headers: String,
    normalized_string: String,
}

impl Default for SignatureBase {
    fn default() -> Self {
        SignatureBase {
            host: "".to_string(),
            path: "".to_string(),
            headers: "".to_string(),
            normalized_string: "".to_string(),
        }
    }
}

pub fn get_signature_base(
    url: Url,
    method: reqwest::Method,
    date: &String,
    content_type: Option<&str>,
    digest: Option<&str>,
) -> SignatureBase {
    let mut signature_base = SignatureBase::default();
    signature_base.host = match url.host() {
        None => String::from(""),
        Some(h) => h.to_string().to_ascii_lowercase(),
    };
    signature_base.path = url.path().to_ascii_lowercase();
    let method_as_string = method.to_string().to_ascii_lowercase();
    signature_base.normalized_string = format!(
        "(request-target): {} {}\nx-nordea-originating-host: {}\nx-nordea-originating-date: {}",
        method_as_string, signature_base.path, signature_base.host, date
    );
    signature_base.headers =
        "(request-target) x-nordea-originating-host x-nordea-originating-date".to_string();
    if vec![
        reqwest::Method::POST,
        reqwest::Method::PUT,
        reqwest::Method::PATCH,
    ]
        .contains(&method)
    {
        signature_base.headers = "(request-target) x-nordea-originating-host x-nordea-originating-date content-type digest".to_string();
        match (content_type, digest) {
            (Some(c), Some(d)) => {
                signature_base.normalized_string +=
                    format!("\ncontent-type: {}\ndigest: {}", c, d).as_str();
                signature_base.headers = String::from("(request-target) x-nordea-originating-host x-nordea-originating-date content-type digest");
            }
            (Some(c), None) => {
                signature_base.normalized_string += format!("\ncontent-type: {}", c).as_str();
                signature_base.headers = String::from("(request-target) x-nordea-originating-host x-nordea-originating-date content-type");
            }
            (None, Some(d)) => {
                signature_base.normalized_string += format!("\ndigest: {}", d).as_str();
                signature_base.headers = String::from(
                    "(request-target) x-nordea-originating-host x-nordea-originating-date digest",
                );
            }
            (None, None) => {
                signature_base.headers = String::from(
                    "(request-target) x-nordea-originating-host x-nordea-originating-date",
                );
            }
        }
    }
    signature_base
}


pub fn calculate_digest(input_data: String) -> String {
    let ring_digest = digest::digest(&digest::SHA256, input_data.as_bytes());
    format! {"sha-256={}", general_purpose::STANDARD.encode(ring_digest.as_ref())}
}

pub fn calculate_digest_from_url_form_data(url: Url) -> String {
    let mut data = String::new();
    if let Some(query) = url.query() {
        data = query.to_string();
    }
    let ring_digest = digest::digest(&digest::SHA256, data.as_bytes());
    format! {"sha-256={}", general_purpose::STANDARD.encode(ring_digest.as_ref())}
}

pub fn encrypt_signature(signature_base: SignatureBase, configuration: &Configuration) -> String {
    let key_pair = signature::RsaKeyPair::from_pkcs8(&EIDAS_PRIVATE_KEY);
    assert!(key_pair.is_ok(), "unable to read key pair");
    let key_pair = key_pair.unwrap();
    // Prepare signature
    let random = SystemRandom::new();
    let mut signature = vec![0; key_pair.public().modulus_len()];
    // Sign signature.base_normalized string and store it to signature
    key_pair.sign(&signature::RSA_PKCS1_SHA256,&random,
                  signature_base.normalized_string.as_bytes(),
                  &mut signature).unwrap_or_else(|_| assert!(false, "Signing failed"));
    // // prepare signature header
    let signature_header = format!("keyId=\"{}\",algorithm=\"rsa-sha256\",headers=\"{}\",signature=\"{}\"",
                                   configuration.x_ibm_client_id, signature_base.headers, general_purpose::STANDARD.encode(signature));
    signature_header
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::api::personal_authorisation::endpoints::decoupled_authentication_and_authorization::{access_token_using_post, authorization_v5_se_fi_dk_no};
    use crate::api::personal_authorisation::endpoints::get_assets::get_assets;
    use crate::api::personal_authorisation::models::AuthRequest;
    use crate::api::personal_authorisation::models::auth_request::{AuthenticationMethod, Country, Language};
    use crate::api::personal_authorisation::models::Scope::{AccountsBalances, AccountsBasic, AccountsTransactions, PaymentsMultiple};
    use super::*;
    #[tokio::test]
    /// Given I have a valid access token
    /// When I send a GET request to "/v5/assets"
    /// Then The response should be 200 code
    /// Then I should receive user assets information
    async fn get_valid_access_token_then_response_from_v5_assets_should_be_200_ok_status() {
        let configuration = Configuration::new();
        // prepare params for /decoupled/v5/authorize endpoint
        let mut auth_request = AuthRequest::new(
            Country::Dk, 3600, "https://www.example.com".to_string(),
            vec![AccountsBasic, AccountsBalances, AccountsTransactions, PaymentsMultiple],
            "anyString".to_string(),
        );
        auth_request.account_list = Some(vec!["ALL".to_string()]);
        auth_request.authentication_method = Some(AuthenticationMethod::MtaDk);
        auth_request.language = Some(Language::En);
        auth_request.max_tx_history = Some(10);
        auth_request.skip_account_selection = Some(true);
        // execute POST personal/v5/decoupled/authentications endpoint and get "Location" header
        let headers_result = authorization_v5_se_fi_dk_no(&configuration, auth_request, None, None).await;
        assert!(headers_result.is_ok());
        let headers = headers_result.unwrap();
        let location_header = headers.get("location");
        assert!(location_header.is_some());
        let location = String::from_utf8_lossy(location_header.unwrap().as_bytes()).into_owned();
        // extract "code" from Location header
        let location_as_url = Url::parse(location.as_str());
        assert!(location_as_url.is_ok());
        let query_hash: HashMap<String, String> = location_as_url.unwrap().query_pairs().into_owned().collect();
        let code = query_hash.get("code").map(|s| s.to_string());
        assert!(code.is_some());
        // use that "code" to get access token
        let get_token_result =
            access_token_using_post(&configuration, "authorization_code",
                                    None, None, code, Some("https://www.example.com"), None).await;
        assert!(get_token_result.is_ok());
        // with access token we can get assets
        if let Some(access_token) = get_token_result.unwrap().access_token {
            let assets_result = get_assets(&configuration, &access_token, None, None).await;
            assert!(assets_result.is_ok());
            let group_header = assets_result.as_ref()
                .expect("/v5/assets response should have JSON body")
                .group_header.clone()
                .expect("/v5/assets response should have JSON with group_header");
            let response = assets_result
                .expect("/v5/assets response should have JSON body")
                .response;
            assert_eq!(group_header.http_code, Some(200));
            assert!(response.is_some());
            println!("/v5/assets response:\n{:?}", response.unwrap());
        }
    }
}