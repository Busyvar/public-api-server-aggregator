use std::collections::HashMap;
use dotenv::dotenv;

use oauth::{CLIENT_ID, CLIENT_SECRET, HTTP_CLIENT, TOKEN_INFO};
use serde_json::json;

mod oauth;

#[macro_use] extern crate rocket;

///Example request/response handling with Rocket as http server
///
/// to test: curl 127.0.0.1:8000/v1/hello
#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!\n"
}

///Example request/response handling with parameter
#[get("/hello/<name>")]
fn hello_named(name:&str) -> String {
    format!("Hello, {name}!\n")
}

///Example request/response handling with Reqwest as http client
#[get("/server_ip")]
async fn get_server_ip() {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await.expect("request must be reached")
        .json::<HashMap<String, String>>()
        .await.expect("response must be parsed");
    println!("{resp:#?}");
}

/// Example GET request to PISTE API
/// curl -is -H 'Authorization: Bearer uhp0K9ZocT95NY6zocjtmEs1MKH660hwllBMvVIQYXliQwlOTRhwZs' -X GET https://sandbox-api.piste.gouv.fr/dila/legifrance/lf-engine-app/consult/ping
#[get("/ping_public")]
pub async fn ping_public_api()->String {
    println!("send ping to LégiFrance with token:{}",TOKEN_INFO.get().unwrap().access_token);
let res = HTTP_CLIENT.get("https://sandbox-api.piste.gouv.fr/dila/legifrance/lf-engine-app/consult/ping")
    .bearer_auth(&TOKEN_INFO.get().unwrap().access_token)
    .send()
    .await.expect("request must be successfull")
    .text()
    .await.expect("response must be parsed");
    res
}

/// Example POST request to PISTE API
/// curl -is -H 'Authorization: Bearer uhp0K9ZocT95NY6zocjtmEs1MKH660hwllBMvVIQYXliQwlOTRhwZs'      
/// -H 'accept: application/json'   
/// -H 'Content-Type: application/json'     
/// -d '{ "years": [ 2016, 2017 ] }'  -X POST https://sandbox-api.piste.gouv.fr/dila/legifrance/lf-engine-app/list/docsAdmins
#[get("/doc_public")]
pub async fn get_doc_from_public_api()->String {
    println!("get doc references from LégiFrance");
    let res = HTTP_CLIENT.post("https://sandbox-api.piste.gouv.fr/dila/legifrance/lf-engine-app/list/docsAdmins")
        .bearer_auth(&TOKEN_INFO.get().unwrap().access_token)
        .json(&json!({ "years": [ 2016, 2017 ] }))
        .send()
        .await.expect("request must be successfull")
        .text()
        .await.expect("response must be parsed");
    // println!("{res:#?}");
    res
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    //init environment variables (check .env)
    dotenv().ok();
    CLIENT_ID.set( std::env::var("PISTE_CLIENT_ID").expect("PISTE_CLIENT_ID must be set in environement.")).unwrap();
    CLIENT_SECRET.set( std::env::var("PISTE_CLIENT_SECRET").expect("PISTE_CLIENT_SECRET must be set in environment.")).unwrap();

    //retrieve a token to request PISTE API
    oauth::get_token_info().await;

    //Listent routes to interact with clients
    let _rocket = rocket::build()
        .mount("/v1", routes![
            hello, hello_named, get_server_ip, 
            ping_public_api, get_doc_from_public_api
        ])
        .launch()
        .await?;
    Ok(())
}

