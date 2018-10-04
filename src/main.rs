extern crate plivo_sdk_rust;
use std::{thread, time};
fn main() {
    //    println!(
    //        "Hello in English: {}",
    //        plivo_sdk_rust::account::operations::details()
    //    );
    let ten_millis = time::Duration::from_millis(1000);
    let now = time::Instant::now();
    println!(
        "iasdas {:?}",
        plivo_sdk_rust::account::operations::modify_name("mike2".to_string())
    );
    thread::sleep(ten_millis);
    println!(
        "iasdas {:?}",
        plivo_sdk_rust::account::operations::modify_address("mike22".to_string())
    );
    thread::sleep(ten_millis);

    println!(
        "iasdas {:?}",
        plivo_sdk_rust::account::operations::modify_city("mike222".to_string())
    );
    thread::sleep(ten_millis);
    println!(
        "Hello in English: {}",
        plivo_sdk_rust::account::operations::details().to_string()
    );
    /*    println!(
        "iasdas {:?}",
        plivo_sdk_rust::account::operations::create_subaccount(
            "test".to_string(),
            "true".to_string()
        )
    )*/
}
