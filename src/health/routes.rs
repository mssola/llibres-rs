use dropshot::{endpoint, ApiDescription, HttpError, HttpResponseOk, RequestContext};
use std::sync::Arc;

#[endpoint {
    method = GET,
    path = "/health",
}]
async fn health(_ctx: Arc<RequestContext<()>>) -> Result<HttpResponseOk<()>, HttpError> {
    Ok(HttpResponseOk(()))
}

/// Draw the different routes available for the books resource.
pub fn draw(api: &mut ApiDescription<()>) {
    api.register(health).unwrap();
}
