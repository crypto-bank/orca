//! Clients utilities.

use std::collections::HashMap;

pub fn urlencode(hashmap: &HashMap<&str, &str>) -> String {
    hashmap.iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}
