use actix_web::{get, Responder, web};

#[get("/x/{key}")]
pub(crate) async fn expand_link_handler(path: web::Path<(String,)>) -> super::Result<impl Responder> {
    let key = path.into_inner();
    // find the details and redirect
    panic!("impl")
}
