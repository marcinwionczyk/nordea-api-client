use std::collections::HashMap;

use cucumber::{given, then, when, World};
use randomizer::Randomizer;
use url::Url;

use automaton_test_framework::api::configuration::Configuration;
use automaton_test_framework::api::personal_authorisation::endpoints::decoupled_authentication_and_authorization::*;
use automaton_test_framework::api::personal_authorisation::endpoints::get_assets::get_assets;
use automaton_test_framework::api::personal_authorisation::models::{AssetResponse, AuthRequest, BearerToken};
use automaton_test_framework::api::personal_authorisation::models::auth_request::*;
use automaton_test_framework::api::personal_authorisation::models::Scope::*;

#[derive(World, Debug, Default)]
struct AuthorisationWorld {
    bearer_token: BearerToken,
    get_assets_response: AssetResponse,
}

#[given(regex = r"I have a (valid|invalid) access token")]
async fn get_valid_access_token(w: &mut AuthorisationWorld, state: String) {
    if state.as_str() == "invalid" {
        let invalid_bearer_token = BearerToken {
            access_token: Some(format!("INVALIDTOKEN{}",
                                       Randomizer::ALPHANUMERIC(1536).string().unwrap())),
            expires_in: Some(3599),
            token_type: Some(String::from("Bearer")),
            refresh_token: Some(format!("{}INVALIDTOKEN",
                                        Randomizer::ALPHANUMERIC(1536).string().unwrap())),
        };
        w.bearer_token = invalid_bearer_token;
        return;
    }
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
    // execute POST /decoupled/v5/authorize endpoint and get "Location" header
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
    let code = query_hash.get("code");
    assert!(code.is_some());
    // use this "code" to get access token
    let get_token_result =
        access_token_using_post(&configuration, "authorization_code",
                                None, None, code.map(|x| x.as_str()), None, None).await;
    assert!(get_token_result.is_ok());
    // assign returned response to w.bearer_token
    w.bearer_token = get_token_result.unwrap();
}

#[when("I send a GET request to \"/v5/assets\"")]
async fn send_get_request(w: &mut AuthorisationWorld) {
    assert!(&w.bearer_token.access_token.is_some());
    let get_assets_result = get_assets(
        &Configuration::new(), w.bearer_token.access_token.clone().unwrap().as_str(),
        None, None).await;
    assert!(get_assets_result.is_ok());
    w.get_assets_response = get_assets_result.unwrap();
}


#[tokio::main]
async fn main() {
    AuthorisationWorld::run("tests/features/authorization.feature").await;
}