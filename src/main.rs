use tokio;
use warp;
use warp::Filter;

#[tokio::main]
async fn main() {
    let cookie_filter = warp::filters::cookie::optional("cookie_consented");
    let check = warp::path!("gdpr" / "check")
        .and(cookie_filter)
        .map(|cookie: Option<_>| {
            // if the gdprConsent cookie is set, we return 200 to nginx so it will
            // serve the page that was originally requested
            warp::reply::with_status(
                "",
                if cookie.is_some() && cookie.unwrap() == "true" {
                    warp::http::status::StatusCode::from_u16(200).unwrap()
                } else {
                    warp::http::status::StatusCode::from_u16(401).unwrap()
                },
            )
        });

    let consent_wall = warp::path!("gdpr" / "consent").map(|| {
        // serve index.html
        warp::reply::with_header(include_str!("index.html"), "Content-Type", "text/html")
    });
    let css = warp::path!("gdpr" / "bulma.min.css").map(|| {
        // serve stylesheet
        warp::reply::with_header(include_str!("bulma.min.css"), "Content-Type", "text/css")
    });
    let js = warp::path!("gdpr" / "setcookie.js").map(|| {
        // serve js that sets the cookie
        warp::reply::with_header(include_str!("setcookie.js"), "Content-Type", "application/javascript")
    });

    let router = check.or(consent_wall).or(css).or(js);

    warp::serve(router).run(([127, 0, 0, 1], 7070)).await;
}
