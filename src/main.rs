use actix_web::{App, HttpServer};

mod soundbert;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(soundbert::init))
        .bind("localhost:8080")?
        .run()
        .await
}
