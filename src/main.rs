use askama::Template;
use salvo::prelude::*;
use salvo::session::{CookieStore, SessionHandler};

static DOMAIN: &str = "localhost";
static COOKIE_SECRET: &[u8] = b"secretabsecretabsecretabsecretabsecretabsecretabsecretabsecretab";

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

    let session_handler = SessionHandler::builder(CookieStore::new(), COOKIE_SECRET)
        .cookie_name("session")
        .cookie_domain(DOMAIN)
        .same_site_policy(salvo::http::cookie::SameSite::Strict)
        .build()
        .unwrap();

    let router = Router::new()
        .hoop(session_handler)
        .get(index)
        .push(Router::with_path("rsvp").get(rsvp));

    let acceptor = TcpListener::new(("0.0.0.0", 3000)).bind().await;

    Server::new(acceptor).serve(router).await;
}
