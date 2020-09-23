use crate::middleware::ContextExt;
use crate::{Context, ErrorResponse};
use log::info;

use crate::auth::encode_token;
use crate::users::responses::UserResponse;
use domain::repositories::Repository;
use tide::prelude::*;
use tide::{Request, Response};

pub async fn get_current_user<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let repository = &cx.state().repository;
    info!("Get user {}", user_id);

    let user = repository.get_user_by_id(user_id)?;
    let token = encode_token(user.id);

    let response: UserResponse = (user, token).into();
    Ok(Response::builder(200).body(json!(&response)).into())
}
