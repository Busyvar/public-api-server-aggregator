
use std::sync::{LazyLock, OnceLock};

use serde::Deserialize;

pub static HTTP_CLIENT:LazyLock<reqwest::Client> = LazyLock::new(||reqwest::Client::new());
pub static TOKEN_INFO:OnceLock<TokenInfo> = OnceLock::new();
pub static CLIENT_ID:OnceLock<String> = OnceLock::new();
pub static CLIENT_SECRET:OnceLock<String> = OnceLock::new();

#[derive(Deserialize, Debug)]
pub struct TokenInfo {
    pub access_token:String,
    token_type:String,
    expires_in:isize,
    scope:String
}

pub async fn get_token_info() {
// get oauth token from PISTE:
// curl -k -v -X POST -H 'Content-type: application/x-www-form-urlencoded' \
// -d "grant_type=client_credentials&
// client_id=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx&
// client_secret=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx&
// scope=openid" \
// https://sandbox-oauth.piste.gouv.fr/api/oauth/token
//
// response : {
// "access_token":"l6eAW3pGhIXQaX8V8QUlIWXB2afNHELKp23VTyPkEuPNFW5D8NiLBU",
// "token_type":"Bearer",
// "expires_in":3600,
// "scope":"openid resource.READ"
// }
    println!("retrieving oauth token from PISTE API");

let params = [
    ("grant_type", "client_credentials"), 
    ("client_id", CLIENT_ID.get().unwrap()),
    ("client_secret", CLIENT_SECRET.get().unwrap()),
    ("scope","openid")
];
let res = HTTP_CLIENT.post("https://sandbox-oauth.piste.gouv.fr/api/oauth/token")
    .form(&params)
    .send()
    .await.expect("Oauth request must be successfull")
    .json::<TokenInfo>()
    .await.expect("response must be parsed");
    println!("{res:#?}");
    TOKEN_INFO.set(res).unwrap();
}

