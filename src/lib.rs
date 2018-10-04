pub mod account;
extern crate dotenv;
extern crate reqwest;
extern crate url;
#[macro_use]
extern crate serde_derive;
use dotenv::var;
use serde_json::{Error, Value};

use reqwest::Client;

use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

fn generic_post(
    built_url: String,
    jason_hash: &HashMap<&str, &str>,
) -> Result<Value, &'static str> {
    let built_url = format!(
        "https://api.plivo.com/v1/Account/{}/",
        &var("PLIVO_AUTH_ID").unwrap()
    );
    let client = Client::new();
    let mut res = client
        .get(&built_url)
        .basic_auth(
            var("PLIVO_AUTH_ID").unwrap().to_string(),
            Some(var("PLIVO_AUTH_TOKEN").unwrap().to_string()),
        ).json(&jason_hash)
        .send()
        .unwrap();
    let mut v: Value = serde_json::from_str("{}".as_ref()).unwrap();
    let mut v: Value = serde_json::from_str("{}".as_ref()).unwrap();
    if res.status().is_success() {
        println!("a");
        v = serde_json::from_str(&res.text().unwrap()).unwrap();
        return Ok(v);
    } else {
        println!("test");
        println!("{:?}", res.status());
        return Err("eee");
    }
}
