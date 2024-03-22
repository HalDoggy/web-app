use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde;
use actix_rt;
use actix_cors::Cors;

async fn add_product(info: web::Json<ProductInfo>) -> impl Responder {
    let response = format!("name: {}, price: {}", info.name, info.price);
    HttpResponse::Ok().body(response)
}

#[derive(serde::Deserialize)]
struct ProductInfo {
    name: String,
    price: u32,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(
            Cors::permissive()
            .allowed_methods(vec!["POST", "GET"])
            .supports_credentials()
            .max_age(3600)
        )
        .route("/add", web::post().to(add_product))
    })
    .bind("0.0.0.0:8765")?
    .run()
    .await
}