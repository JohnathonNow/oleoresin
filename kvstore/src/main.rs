use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer};

use actix_web::{get, put, HttpResponse};

mod error;

use error::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let x = web::Data::new(Arc::new(Mutex::new(HashMap::<String, String>::new())));
    HttpServer::new(move || App::new().service(get).service(put).app_data(x.clone()))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}

#[get("/{key}")]
pub async fn get(
    web::Path(key): web::Path<String>,
    db: web::Data<Arc<Mutex<HashMap<String, String>>>>,
) -> Result<HttpResponse, Error> {
    let l = db.lock().unwrap();
    Ok(HttpResponse::Ok().json(l.get(&key).ok_or_else(|| Error::new("key not found".into()))?))
}

#[put("/{key}")]
pub async fn put(
    web::Path(key): web::Path<String>,
    p: web::Json<String>,
    db: web::Data<Arc<Mutex<HashMap<String, String>>>>,
) -> Result<HttpResponse, Error> {
    let mut l = db.lock().unwrap();
    l.insert(key, p.0);
    Ok(HttpResponse::Ok().body("ok"))
}
