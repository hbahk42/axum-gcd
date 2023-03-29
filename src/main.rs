use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_index))
        .route("/gcd", post(post_gcd));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_index() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n" />
                    <input type="text" name="m" />
                    <button type="submit">Compute GCD</button>
                </form>
            </body>
        </html>
    "#,
    )
}

async fn post_gcd(Form(form): Form<GcdParameters>) -> StatusCode {
    if form.n == 0 || form.m == 0 {
        println!("Computing the GCD with zero is boring.");
        return StatusCode::BAD_REQUEST;
    }

    println!(
        "The greatest common divisor of the numbers {} and {} is {}",
        &form.n,
        &form.m,
        gcd(form.n, form.m)
    );
    StatusCode::OK
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
