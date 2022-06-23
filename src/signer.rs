use json::object;
use reqwest::header::HeaderMap;

use crate::{error::SignerError, utils::json_to_urlencoded, secret::TokenData, wizard::Context};
use crate::api::*;


pub async fn signer(ctx: Context) -> Result<(), SignerError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type", 
        "application/x-www-form-urlencoded".try_into().unwrap()
    );
    headers.insert(
        "Cookie", 
        format!("JSESSIONID={}", ctx.sessionid).try_into().unwrap()
    );

    let client = reqwest::Client::new();
    let sign_req = object! {
        traineeId: ctx.train_id,
        adcode: 0,
        lat: ctx.address.lat,
        lng: ctx.address.lng,
        address: ctx.address.text,
        model: "",
        brand: "",
        platform: "",
        system: "",
        openId: "",
        unionId: "",
        deviceName: "",
        punchInStatus: 0,
        clockStatus: 2
    };

    let body = json_to_urlencoded(&sign_req);
    TokenData::new(sign_req).add_to_headers(&mut headers);
    
    let resp = client.post(EP_SIGN)
        .headers(headers)
        .body(body)
        .send().await?
        .text().await?;
    let resp = json::parse(&resp)?;

    let msg = resp["msg"].to_string();

    match msg.as_str() {
        SIGN_SUCCESS_HINT => println!("[SUCCESS] Successfully signed."),
        ALREADY_SIGNED_HINT => println!("[INFO] Already signed."),
        _ => return Err(SignerError::EndpointError(msg + " when signing"))
    }
    
    Ok(())    
}
