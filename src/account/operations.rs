use dotenv::var;
use generic_get;
use generic_post;
use serde_json::{Error, Value};
use std::collections::HashMap;
pub fn details() -> Result<Value, &'static str> {
    generic_get(format!(
        "https://api.plivo.com/v1/Account/{}/",
        &var("PLIVO_AUTH_ID").unwrap()
    ))
}

pub fn modify_name(name: String) -> Result<Value, &'static str> {
    let chars_to_trim: &[char] = &['"'];
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("name", name.trim_matches(chars_to_trim));
    generic_post(
        format!(
            "https://api.plivo.com/v1/Account/{}/",
            &var("PLIVO_AUTH_ID").unwrap()
        ),
        &map,
    )
}

pub fn modify_address(address: String) -> Result<Value, &'static str> {
    let chars_to_trim: &[char] = &['"'];
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("address", address.trim_matches(chars_to_trim));
    generic_post(
        format!(
            "https://api.plivo.com/v1/Account/{}/",
            &var("PLIVO_AUTH_ID").unwrap()
        ),
        &map,
    )
}

pub fn modify_city(city: String) -> Result<Value, &'static str> {
    let chars_to_trim: &[char] = &['"'];
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("city", city.trim_matches(chars_to_trim));
    generic_post(
        format!(
            "https://api.plivo.com/v1/Account/{}/",
            &var("PLIVO_AUTH_ID").unwrap()
        ),
        &map,
    )
}

pub fn create_subaccount(name: String, enabled: String) -> Result<Value, &'static str> {
    let chars_to_trim: &[char] = &['"'];
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("name", name.trim_matches(chars_to_trim));
    map.insert("enabled", enabled.trim_matches(chars_to_trim));
    generic_post(
        format!(
            "https://api.plivo.com/v1/Account/{}/Subaccount/",
            &var("PLIVO_AUTH_ID").unwrap()
        ),
        &map,
    )
}

pub fn modify_subaccount(
    name: String,
    enabled: String,
    subauth_id: String,
) -> Result<Value, &'static str> {
    let chars_to_trim: &[char] = &['"'];
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("name", name.trim_matches(chars_to_trim));
    map.insert("enabled", enabled.trim_matches(chars_to_trim));
    generic_post(
        format!(
            "https://api.plivo.com/v1/Account/{AUTH}/Subaccount/{SUBAUTH}",
            AUTH = &var("PLIVO_AUTH_ID").unwrap(),
            SUBAUTH = subauth_id.trim_matches(chars_to_trim)
        ),
        &map,
    )
}
