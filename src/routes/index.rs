use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

pub async fn route_handler() -> IndexTemplate {
    IndexTemplate {}
}
