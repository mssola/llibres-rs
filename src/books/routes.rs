use crate::books::{lang_from_context, t};
use crate::books::{Book, BookRequest};
use dropshot::{
    endpoint, ApiDescription, HttpError, HttpResponseCreated, HttpResponseDeleted, HttpResponseOk,
    HttpResponseUpdatedNoContent, Path, RequestContext, TypedBody,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

/// Parameters as expected from the path of these endpoints.
#[derive(Deserialize, JsonSchema)]
struct PathParams {
    id: Uuid,
}

#[endpoint {
    method = GET,
    path = "/books",
}]
async fn index(_ctx: Arc<RequestContext<()>>) -> Result<HttpResponseOk<Vec<Book>>, HttpError> {
    let books = Book::all()?;
    Ok(HttpResponseOk(books))
}

#[endpoint {
    method = POST,
    path = "/books",
}]
async fn create(
    ctx: Arc<RequestContext<()>>,
    body: TypedBody<BookRequest>,
) -> Result<dropshot::HttpResponseCreated<Uuid>, HttpError> {
    let lang = lang_from_context(ctx).await;
    let body = body.into_inner();
    validate_request(&body, lang.as_str())?;

    let book = Book::create(body);
    match book {
        Ok(v) => Ok(HttpResponseCreated(v)),
        Err(_) => Err(HttpError::for_bad_request(
            None,
            t(lang.as_str(), "unknown"),
        )),
    }
}

#[endpoint {
    method = PUT,
    path = "/books/{id}",
}]
async fn update(
    ctx: Arc<RequestContext<()>>,
    path: Path<PathParams>,
    body: TypedBody<BookRequest>,
) -> Result<dropshot::HttpResponseUpdatedNoContent, HttpError> {
    let lang = lang_from_context(ctx).await;
    let body = body.into_inner();
    validate_request(&body, lang.as_str())?;

    match Book::update(path.into_inner().id, body) {
        Ok(_) => Ok(HttpResponseUpdatedNoContent()),
        Err(_) => Err(HttpError::for_bad_request(
            None,
            t(lang.as_str(), "unknown"),
        )),
    }
}

#[endpoint {
    method = DELETE,
    path = "/books/{id}",
}]
async fn delete(
    ctx: Arc<RequestContext<()>>,
    path: Path<PathParams>,
) -> Result<dropshot::HttpResponseDeleted, HttpError> {
    let lang = lang_from_context(ctx).await;
    let res = Book::delete(path.into_inner().id);

    match res {
        Ok(0) => Err(HttpError::for_not_found(None, t(lang.as_str(), "notfound"))),
        Ok(1) => Ok(HttpResponseDeleted()),
        Ok(_) => Err(HttpError::for_not_found(None, t(lang.as_str(), "notfound"))),
        Err(_) => Err(HttpError::for_bad_request(None, t(lang.as_str(), "oops"))),
    }
}

/// Draw the different routes available for the books resource.
pub fn draw(api: &mut ApiDescription<()>) {
    api.register(index).unwrap();
    api.register(create).unwrap();
    api.register(update).unwrap();
    api.register(delete).unwrap();
}

/// Validate a given book request. This should work either for `create` or
/// `update`.
fn validate_request(br: &BookRequest, lang: &str) -> Result<(), HttpError> {
    if br.title.trim().is_empty() {
        return Err(HttpError::for_bad_request(None, t(lang, "emptytitle")));
    }
    if br.publisher.trim().is_empty() {
        return Err(HttpError::for_bad_request(None, t(lang, "emptypublisher")));
    }
    if br.language.trim().is_empty() {
        return Err(HttpError::for_bad_request(None, t(lang, "emptylanguage")));
    }
    if br.rate < 0 || br.rate > 10 {
        return Err(HttpError::for_bad_request(None, t(lang, "badrating")));
    }
    if br.status < 0 || br.status > 5 {
        return Err(HttpError::for_bad_request(None, t(lang, "badstatus")));
    }
    if let Some(v) = br.kind {
        if !(0..=4).contains(&v) {
            return Err(HttpError::for_bad_request(None, t(lang, "badkind")));
        }
    }

    Ok(())
}
