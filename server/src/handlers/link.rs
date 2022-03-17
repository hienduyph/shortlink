use actix_web::{get, http::header, post, web, HttpResponse, Responder};
use serde::Serialize;

use crate::services::link::{LinkService, LinkCreation};

#[get("/x/{key}")]
pub(crate) async fn expand_link_handler(
    path: web::Path<(String,)>,
    link_svc: web::Data<LinkService>,
) -> super::Result<impl Responder> {
    let (key,) = path.into_inner();
    let detail = link_svc.expand(&key).await?;
    // now redirect 302
    // find the details and redirect
    let resp = HttpResponse::Found()
        .insert_header((header::LOCATION, detail.url))
        .finish();
    Ok(resp)
}


#[derive(Serialize, Debug, Clone)]
pub struct LinkCreationResp {
    shorten: String,
    url: String,
}


#[post("/x")]
pub(crate) async fn create_link_handler(
    input: web::Json<LinkCreation>,
    link_svc: web::Data<LinkService>,
) -> super::Result<impl Responder> {
    link_svc.create(&input).await.map(|v| {
        web::Json(LinkCreationResp{
            shorten: v.shorten,
            url: v.url,
        })
    })
}
