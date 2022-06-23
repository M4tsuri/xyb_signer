use json::JsonValue;

pub fn json_to_urlencoded(src: &JsonValue) -> String {
    src.entries().map(|(k, v)| 
        format!("{}={}", k, urlencoding::encode(&v.to_string()))
    ).fold(String::new(), |acc, val| acc + &val + "&")
}
