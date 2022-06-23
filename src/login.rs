use json::object;

use crate::error::SignerError;
use crate::utils::{checked_post, get_input};
use crate::api::*;

pub async fn check_login_status(sessionid: &str) -> Result<bool, SignerError> {
    let resp = checked_post(
        EP_LOGIN_STATUS, &object! {}, Some(sessionid)
    ).await?;

    Ok(resp["msg"] == OPER_SUCCESS_HINT)
}

pub async fn login_with_password(mobile: &str, password: &str) -> Result<String, SignerError> {
    let resp = checked_post(
        EP_LOGIN_BY_PASSWD, 
        &object! {
            username: mobile,
            password: password,
            openId: "",
            unionId: "",
            wxname: "",
            wxCity: "",
            avatarTempPath: ""
        }, 
        None
    ).await?;

    let sessionid = resp["data"]["sessionId"].to_string();
    println!("[INFO] Login success.");
    Ok(sessionid)
}

pub async fn reset_password(mobile: &str) -> Result<String, SignerError> {
    print!("[INPUT] Do you want to set a password? [Y/n]: ");
    let set: String = get_input()?;

    if set.to_ascii_lowercase() != "y" {
        return Err(SignerError::ConfigError("password not provided."))
    }

    checked_post(
        EP_SEND_RESET_CODE, 
        &object! {
            authStr: mobile,
            type: 1
        }, 
        None
    ).await?;

    print!("[INPUT] Please input the verify code you received: ");
    let verify_code: String = get_input()?;
    print!("[INPUT] Please input your new password: ");
    let password: String = get_input()?;
    let password_md5 = format!("{:x}", md5::compute(&password));

    checked_post(
        EP_RESET_PASSWD, 
        &object! {
            authStr: mobile,
            msgCode: verify_code,
            newPsw: password_md5
        }, 
        None
    ).await?;

    println!("[INFO] Password successfully reset.");
    Ok(password)
}
