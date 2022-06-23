use std::io::{stdin, stdout, Write};

use json::object;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::error::SignerError;
use crate::secret::TokenData;
use crate::utils::json_to_urlencoded;
use crate::api::*;

pub async fn check_login_status(sessionid: &str) -> Result<bool, SignerError> {
    let client = reqwest::Client::new();

    let resp = client.post(EP_LOGIN_STATUS)
        .header("content-type", "application/x-www-form-urlencoded")
        .header("Cookie", format!("JSESSIONID={}", sessionid))
        .send().await?
        .text().await?;
    let resp = json::parse(&resp)?;

    Ok(resp["msg"] == OPER_SUCCESS_HINT)
}

pub async fn login_with_password(mobile: &str, password: &str) -> Result<String, SignerError> {
    let login_req = object! {
        username: mobile,
        password: password,
        openId: "",
        unionId: "",
        wxname: "",
        wxCity: "",
        avatarTempPath: ""
    };

    let body = json_to_urlencoded(&login_req);
    let mut headers = HeaderMap::new();
    TokenData::new(login_req).add_to_headers(&mut headers);

    let client = reqwest::Client::new();
    let resp = client.post(EP_LOGIN_BY_PASSWD)
        .header("content-type", "application/x-www-form-urlencoded")
        .headers(headers)
        .body(body)
        .send().await?
        .text().await?;

    let resp = json::parse(&resp)?;

    let msg = resp["msg"].to_string();
    if msg != LOGIN_SUCCESS_HINT {
        return Err(SignerError::LoginFailed(msg))
    }

    let sessionid = resp["data"]["sessionId"].to_string();
    println!("[INFO] Login success.");
    Ok(sessionid)
}

pub async fn login_with_verify_code(mobile: &str) -> Result<String, SignerError> {
    let resp = reqwest::get(EP_GET_MOBILE_TOKEN).await?;
    let resp = json::parse(&resp.text().await?)?;

    let sessionid = resp["data"]["sessionId"].to_string();
    let token = resp["data"]["token"].to_string();

    let send_code_req = format!(
        "mobile={}&mobileToken={}&type=9",
        mobile,
        token
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type", 
        HeaderValue::from_str("application/x-www-form-urlencoded").unwrap()
    );
    headers.insert(
        "Cookie", 
        HeaderValue::from_str(&format!("JSESSIONID={}", sessionid)).unwrap()
    );


    let client = reqwest::Client::new();
    let resp = client.post(EP_SEND_CODE)
        .headers(headers.clone())
        .body(send_code_req)
        .send().await?
        .text().await?;
    let resp = json::parse(&resp)?;

    let msg = resp["msg"].to_string();

    if msg != OPER_SUCCESS_HINT {
        return Err(SignerError::VerifyCodeError(msg))
    }

    let mut verify_code = String::new();
    print!("[INPUT] Please input the verify code you received: ");
    stdout().flush()?;
    stdin().read_line(&mut verify_code)?;

    let login_req = format!(
        "phone={}&username={}&verifyCode={}&openId=&unionId=&wxname=&wxCity=&avatarTempPath=",
        mobile, mobile, &verify_code[..6]
    );

    let resp = client.post(EP_LOGIN_BY_MOBILE)
        .headers(headers)
        .body(login_req)
        .send().await?
        .text().await?;
    let resp = json::parse(&resp)?;

    let msg = resp["msg"].to_string();
    if msg != LOGIN_SUCCESS_HINT_C {
        return Err(SignerError::LoginFailed(msg))
    }

    let sessionid = resp["data"]["sessionId"].to_string();
    println!("[INFO] Login success.");
    Ok(sessionid)
}
