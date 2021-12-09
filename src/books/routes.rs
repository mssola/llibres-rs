use crate::books::{Book, BookRequest};
use uuid::Uuid;
use dropshot::{
    ApiDescription, RequestContext, endpoint, HttpError, HttpResponseOk,
    HttpResponseCreated, HttpResponseUpdatedNoContent, HttpResponseDeleted,
    TypedBody, Path
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

/// Parameters as expected from the path of these endpoints.
#[derive(Deserialize, JsonSchema)]
struct PathParams {
    id: Uuid,
}

#[endpoint {
    method = GET,
    path = "/books",
}]
async fn index(
    _ctx: Arc<RequestContext<()>>
) -> Result<HttpResponseOk<Vec<Book>>, HttpError> {
    let books = Book::all()?;
    Ok(HttpResponseOk(books))
}

#[endpoint {
    method = POST,
    path = "/books",
}]
async fn create(
    _ctx: Arc<RequestContext<()>>,
    body: TypedBody<BookRequest>
) -> Result<dropshot::HttpResponseCreated<Uuid>, HttpError> {
    let body = body.into_inner();
    validate_request(&body)?;

    let book = Book::create(body)?;
    Ok(HttpResponseCreated(book))
}

#[endpoint {
    method = PUT,
    path = "/books/{id}",
}]
async fn update(
    _ctx: Arc<RequestContext<()>>,
    path: Path<PathParams>,
    body: TypedBody<BookRequest>
) -> Result<dropshot::HttpResponseUpdatedNoContent, HttpError> {
    let body = body.into_inner();
    validate_request(&body)?;

    Book::update(path.into_inner().id, body)?;
    Ok(HttpResponseUpdatedNoContent())
}

#[endpoint {
    method = DELETE,
    path = "/books/{id}",
}]
async fn delete(
    _ctx: Arc<RequestContext<()>>,
    path: Path<PathParams>
) -> Result<dropshot::HttpResponseDeleted, HttpError> {
    let res = Book::delete(path.into_inner().id);

    match res {
        Ok(0) => Err(HttpError::for_not_found(None, String::from("not found"))),
        Ok(1) => Ok(HttpResponseDeleted()),
        Ok(_) => Err(HttpError::for_not_found(None, String::from("not found"))),
        Err(_) => Err(HttpError::for_bad_request(None, String::from("oops"))),
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
fn validate_request(br: &BookRequest) -> Result<(), HttpError> {
    if br.title.trim().is_empty() {
        return Err(HttpError::for_bad_request(None, String::from("empty title")));
    }
    if br.publisher.trim().is_empty() {
        return Err(HttpError::for_bad_request(None, String::from("empty publisher")));
    }
    if br.language.trim().is_empty() {
        return Err(HttpError::for_bad_request(None, String::from("empty language")));
    }
    if br.rate < 0 || br.rate > 10 {
        return Err(HttpError::for_bad_request(None, String::from("bad rating")));
    }
    if br.status < 0 || br.status > 5 {
        return Err(HttpError::for_bad_request(None, String::from("bad status")));
    }
    match br.kind {
        Some(v) => {
            if v < 0 || v > 4 {
                return Err(HttpError::for_bad_request(None, String::from("bad kind")));
            }
        },
        None => ()
    }

    Ok(())
}
