use std::{io::{stdout, stdin, Write}, str::FromStr};

use json::JsonValue;
use reqwest::header::HeaderMap;

use crate::{error::SignerError, secret::TokenData, CLIENT};

pub fn json_to_urlencoded(src: &JsonValue) -> String {
    src.entries().map(|(k, v)| 
        format!("{}={}", k, urlencoding::encode(&v.to_string()))
    ).fold(String::new(), |acc, val| acc + &val + "&")
}

pub fn get_input<T: FromStr>() -> Result<T, SignerError> {
    let mut buf = String::new();
    stdout().flush()?;
    stdin().read_line(&mut buf)?;

    let buf = buf.trim_end().to_string();
    Ok(buf.parse().or(Err(SignerError::InvalidInput))?)
}

pub async fn checked_post(
    ep: &str, body: &JsonValue, sessionid: Option<&str>
) -> Result<JsonValue, SignerError> {
    let mut headers = HeaderMap::new();
    TokenData::new(body).add_to_headers(&mut headers);

    let mut req = CLIENT.post(ep)
        .header("content-type", "application/x-www-form-urlencoded")
        .header("User-Agent", "Mozilla/5.0 AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/114514 MicroMessenger/8.0.23(0x1919810) NetType/WIFI Language/en");
    
    if let Some(sessionid) = sessionid {
        req = req.header("Cookie", format!("JSESSIONID={}", sessionid))
    }
        
    let resp = req.headers(headers)
        .body(json_to_urlencoded(body))
        .send().await?
        .text().await?;
    let resp = json::parse(&resp)?;

    let code = resp["code"].to_string();
    if code != "200" {
        Err(SignerError::EndpointError(resp["msg"].to_string()))
    } else {
        Ok(resp)
    }
}
