use askama::Template;
use salvo::prelude::*;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "rsvp.html")]
struct RsvpTemplate {}

#[handler]
async fn index(res: &mut Response) {
    let index_template = IndexTemplate {};
    res.render(Text::Html(index_template.render().unwrap()));
}

#[handler]
async fn rsvp(res: &mut Response) {
    let rsvp_template = RsvpTemplate {};
    res.render(Text::Html(rsvp_template.render().unwrap()));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .get(index)
        .push(Router::with_path("rsvp").get(rsvp));
    let acceptor = TcpListener::new(("0.0.0.0", 3000)).bind().await;

    Server::new(acceptor).serve(router).await;
}
