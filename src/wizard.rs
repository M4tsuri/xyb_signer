use std::{fs::read_to_string, io::{stdout, Write, stdin}};

use json::JsonValue;

use crate::{error::SignerError, login::{check_login_status, self}, CLIENT};
use crate::api::*;

#[derive(Debug, Default)]
pub struct Context {
    pub username: String,
    pub address: Address,
    pub sessionid: String,
    pub train_id: String
}

struct ProjectInfo {
    name: String,
    plan_id: String
}

#[derive(Debug, Default)]
pub struct Address {
    pub text: String,
    pub lng: f32,
    pub lat: f32
}

pub async fn setup_wizard(config_path: &str) -> Result<Context, SignerError> {
    let mut config = json::parse(
        &read_to_string(config_path)?
    )?;

    let mut ctx = Context::default();
    ctx.username = config["username"].to_string();

    // login first
    setup_sessionid(&mut config, &mut ctx).await?;
    setup_train_id(&mut config, &mut ctx).await?;
    setup_signing_address(&mut config, &mut ctx).await?;

    std::fs::write(config_path, config.pretty(4))?;
    
    Ok(ctx)
}

async fn setup_signing_address(config: &mut JsonValue, ctx: &mut Context) -> Result<(), SignerError> {
    if config.has_key("address") {
        ctx.address = Address {
            text: config["address"]["text"].to_string(),
            lng: config["address"]["lng"].as_f32().unwrap(),
            lat: config["address"]["lat"].as_f32().unwrap()
        };
        Ok(())
    } else {
        Err(SignerError::ConfigError("missing address"))
    }
}

async fn setup_train_id(config: &mut JsonValue, ctx: &mut Context) -> Result<(), SignerError> {
    if config.has_key("train_id") {
        ctx.train_id = config["train_id"].to_string();
        return Ok(())
    }

    // list all projects
    let projects = get_project_list(&ctx.sessionid).await?;
    println!("[INFO] Project list retrived: ");
    projects.iter().enumerate().for_each(|(idx, p)| {
        println!("\t{}. {}", idx, p.name)
    });
    print!("[INPUT] Choose a project: ");
    stdout().flush()?;
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    let proj_idx = usize::from_str_radix(&buf[..buf.len() - 1], 10)
        .or(Err(SignerError::InvalidInput))?;
    let proj = &projects[proj_idx];
    
    print!("[INFO] Retriving traineeId for project...");
    let train_id = get_train_id(&ctx.sessionid, &proj.plan_id).await?;
    println!("done (id = {})", train_id);


    config.insert("train_id", train_id.clone())?;
    ctx.train_id = train_id;

    Ok(())
}

async fn setup_sessionid(config: &mut JsonValue, ctx: &mut Context) -> Result<(), SignerError> {
    if config.has_key("sessionid") {
        let sessionid = config["sessionid"].to_string();
        if check_login_status(&sessionid).await? {
            println!("[INFO] Session is still alive, reuse it.");

            ctx.sessionid = sessionid;
            return Ok(())
        }
    }

    let mobile = &ctx.username;

    let sessionid = if config.has_key("password") {
        let password = format!("{:x}", md5::compute(config["password"].to_string()));
        login::login_with_password(mobile, &password).await?
    } else {
        login::login_with_verify_code(mobile).await?
    };

    ctx.sessionid = sessionid.clone();
    config.insert("sessionid", sessionid)?;
    
    println!("[INFO] New sessionid saved.");
    Ok(())
}

async fn get_train_id(sessionid: &str, plan_id: &str) -> Result<String, SignerError> {
    let resp = CLIENT.post(EP_PLAN_INFO)
        .header("Cookie", format!("JSESSIONID={}", sessionid))
        .body(format!("planId={}", plan_id))
        .send().await?
        .text().await?;
    let resp = json::parse(&resp)?;

    let msg = resp["msg"].to_string();
    if msg != OPER_SUCCESS_HINT {
        return Err(SignerError::EndpointError(msg))
    }

    Ok(resp["data"]["clockVo"]["traineeId"].to_string())
}

async fn get_project_list(sessionid: &str) -> Result<Vec<ProjectInfo>, SignerError> {
    let resp = CLIENT.post(EP_PROJ_LIST)
        .header("Cookie", format!("JSESSIONID={}", sessionid))
        .send().await?
        .text().await?;
    let resp = json::parse(&resp)?;

    let msg = resp["msg"].to_string();
    if msg != PROJ_LIST_SUCCESS_HINT {
        return Err(SignerError::EndpointError(msg))
    }

    Ok(resp["data"].members().map(|e| {
        ProjectInfo { 
            name: e["projectList"][0]["projectName"].to_string(),
            plan_id: e["planId"].to_string()
        }
    }).collect())
}

