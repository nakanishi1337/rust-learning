use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    let n = form.n;
    let m = form.m;

    if n == 0 || m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD requires two nonzero integers.");
    }

    let response = format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>\n", n, m, gcd(n, m));
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m = m % n;
    }
    n
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:3000...");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
            <input type="text" name="n" />
            <input type="text" name="m" />
            <button type="submit">Calculate GCD</button>
            </form>
            "#,
        )
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test as actix_test};

    #[test]
    fn calculate_gcd() {
        assert_eq!(gcd(14, 15), 1);
        assert_eq!(gcd(2 * 3 * 5 * 11, 3 * 7 * 11), 3 * 11);
    }

    #[actix_web::test]
    async fn post_gcd_returns_result() {
        let app = actix_test::init_service(
            App::new()
                .route("/", web::get().to(get_index))
                .route("/gcd", web::post().to(post_gcd)),
        )
        .await;

        let request = actix_test::TestRequest::post()
            .uri("/gcd")
            .set_form([("n", "42"), ("m", "56")])
            .to_request();
        let response = actix_test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = actix_test::read_body(response).await;
        assert!(std::str::from_utf8(&body).unwrap().contains("<b>14</b>"));
    }

    #[actix_web::test]
    async fn post_gcd_rejects_zero() {
        let response = post_gcd(web::Form(GcdParameters { n: 0, m: 56 }))
            .await
            .respond_to(&actix_test::TestRequest::default().to_http_request());

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
