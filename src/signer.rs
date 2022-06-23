use json::object;

use crate::utils::checked_post;
use crate::{error::SignerError, wizard::Context};
use crate::api::*;


pub async fn signer(ctx: Context) -> Result<(), SignerError> {
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
    
    checked_post(
        EP_SIGN, &sign_req, Some(&ctx.sessionid)
    ).await?;
    println!("[SUCCESS] successfully signed.");
    
    Ok(())    
}
