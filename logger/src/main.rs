use actix_web::{put, web, App, HttpServer, HttpResponse};

mod error;

use error::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(put))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}

#[put("/")]
pub async fn put(
    p: web::Json<String>,
) -> Result<HttpResponse, Error> {
    println!("{}", p.0);
    Ok(HttpResponse::Ok().body("ok"))
}
