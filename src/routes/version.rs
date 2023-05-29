use std::env;

use rocket::serde::Serialize;

use crate::types::{OkResponse, ResponseResult};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct VersionResponse {
    version: String,
    r#type: String,
}

#[get("/version")]
pub fn version() -> ResponseResult<VersionResponse> {
    let version = env!("CARGO_PKG_VERSION").to_string();
    let version_type = env::var("CARGO_VERSION_TYPE").unwrap_or("git".to_string());

    let response = VersionResponse {
        version,
        r#type: version_type,
    };

    Ok(OkResponse::new(response))
}
