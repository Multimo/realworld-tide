use futures::future::BoxFuture;
use http::status::StatusCode;
use log::info;
use tide::{Error, Middleware, Next, Request, Response};

use crate::auth::{extract_claims, Claims};

#[derive(Clone, Default, Debug)]
pub struct JwtMiddleware {}

impl JwtMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait JwtContext {
    fn get_claims(&self) -> Result<&Claims, Error>;
}

impl<State> JwtContext for Request<State> {
    fn get_claims(&self) -> Result<&Claims, Error> {
        self.local::<Claims>()
            .ok_or_else(|| Error::from(StatusCode::UNAUTHORIZED))
    }
}

impl<State: Send + Sync + 'static> Middleware<State> for JwtMiddleware {
    fn handle<'a>(&'a self, cx: Request<State>, next: Next<'a, State>) -> BoxFuture<'a, Response> {
        Box::pin(async move {
            info!("Headers: {:?}", cx.headers());
            let claims = extract_claims(cx.headers());
            info!("Claims: {:?}", claims);
            return if let Some(c) = claims {
                next.run(cx.set_local(c)).await
            } else {
                next.run(cx).await
            };
        })
    }
}
