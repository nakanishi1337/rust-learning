use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    let n = form.n;
    let m = form.m;

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