use dotenv::var;
use generic_get;
use generic_post;
use serde_json::{Error, Value};
use std::collections::HashMap;
/*
 * no method overloading :(
pub fn send_message(src: String, dst: String, text: String) -> Result<Value, &'static str> {
    let chars_to_trim: &[char] = &['"'];
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("src", src.trim_matches(chars_to_trim));
    map.insert("dst", dst.trim_matches(chars_to_trim));
    map.insert("text", text.trim_matches(chars_to_trim));

    generic_post(
        format!(
            "https://api.plivo.com/v1/Account/{}/Message/",
            &var("PLIVO_AUTH_ID").unwrap()
        ),
        &map,
    )
}
*/
pub fn send_message(
    src: String,
    dst: String,
    text: String,
    message_type: String,
    message_url: String,
    message_method: String,
    message_log: String,
) -> Result<Value, &'static str> {
    let chars_to_trim: &[char] = &['"'];
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("src", src.trim_matches(chars_to_trim));
    map.insert("dst", dst.trim_matches(chars_to_trim));
    map.insert("text", text.trim_matches(chars_to_trim));
    map.insert("type", message_type.trim_matches(chars_to_trim));
    map.insert("url", message_url.trim_matches(chars_to_trim));
    map.insert("method", message_method.trim_matches(chars_to_trim));
    map.insert("log", message_log.trim_matches(chars_to_trim));

    generic_post(
        format!(
            "https://api.plivo.com/v1/Account/{}/Message/",
            &var("PLIVO_AUTH_ID").unwrap()
        ),
        &map,
    )
}
