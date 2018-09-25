use dotenv::var;
use reqwest::Client;
use reqwest::StatusCode;
use serde_json;
use serde_json::{Error, Value};
use std::collections::HashMap;

pub fn details() -> Value {
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
        ).send()
        .unwrap();
    let mut v: Value = serde_json::from_str("{}".as_ref()).unwrap();
    if res.status().is_success() {
        v = serde_json::from_str(&res.text().unwrap()).unwrap();
        return v;
    } else {
        match res.status() {
            StatusCode::Ok => println!("success!"),
            StatusCode::PayloadTooLarge => {
                println!("Request payload is too large!");
            }
            s => println!("Received response status: {:?}", s),
            _ => println!("{:?}", res.status()),
        };
    }

    v
}

pub fn modify_name(name: String) -> Result<Value, &'static str> {
    let chars_to_trim: &[char] = &['"'];
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("name", name.trim_matches(chars_to_trim));
    modify(&map)
}

pub fn modify_address(address: String) -> Result<Value, &'static str> {
    let chars_to_trim: &[char] = &['"'];
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("address", address.trim_matches(chars_to_trim));
    modify(&map)
}

pub fn modify_city(city: String) -> Result<Value, &'static str> {
    let chars_to_trim: &[char] = &['"'];
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("city", city.trim_matches(chars_to_trim));
    modify(&map)
}

fn modify(jason_hash: &HashMap<&str, &str>) -> Result<Value, &'static str> {
    let built_url = format!(
        "https://api.plivo.com/v1/Account/{}/",
        &var("PLIVO_AUTH_ID").unwrap()
    );
    let client = Client::new();
    let mut res = client
        .post(&built_url)
        .basic_auth(
            var("PLIVO_AUTH_ID").unwrap().to_string(),
            Some(var("PLIVO_AUTH_TOKEN").unwrap().to_string()),
        ).json(&jason_hash)
        .send()
        .unwrap();
    let mut v: Value = serde_json::from_str("{}".as_ref()).unwrap();
    println!("{:?}", res.status());
    if res.status().is_success() {
        v = serde_json::from_str(&res.text().unwrap()).unwrap();
        return Ok(v);
    } else {
        return Err("eee");
    }
}

fn create_subaccount(name: String, enabled: bool) -> Result<Value, &'static str> {
    let built_url = format!(
        "https://api.plivo.com/v1/Account/{}/Subaccount/",
        &var("PLIVO_AUTH_ID").unwrap()
    );
}
