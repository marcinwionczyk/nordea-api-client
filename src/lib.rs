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
