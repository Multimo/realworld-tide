use crate::articles::responses::ArticleResponse;
use crate::middleware::ContextExt;
use crate::{Context, ErrorResponse};
use domain::repositories::Repository;
use serde::{Deserialize, Serialize};
use tide::prelude::*;
use tide::Response;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub article: NewArticleRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewArticleRequest {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}

impl From<NewArticleRequest> for domain::ArticleContent {
    fn from(a: NewArticleRequest) -> domain::ArticleContent {
        domain::ArticleContent {
            title: a.title,
            description: a.description,
            body: a.body,
            tag_list: a.tag_list.unwrap_or_else(Vec::new),
        }
    }
}

pub async fn insert_article<R: 'static + Repository + Sync + Send>(
    mut cx: tide::Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let request: Request = cx
        .body_json()
        .await
        .map_err(|e| Response::builder(400).body(e.to_string()))?;
    let author_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let repository = &cx.state().repository;

    let author = repository.get_user_by_id(author_id)?;
    let published_article = author.publish(request.article.into(), repository)?;

    Ok(Response::builder(200)
        .body(json!(&ArticleResponse::from(published_article)))
        .into())
}
