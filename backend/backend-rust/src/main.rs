use dotenv::dotenv;
use std::env;
use tokio_postgres::{NoTls, Error};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde;
use actix_rt;
use actix_cors::Cors;
extern crate postgres;

#[derive(serde::Deserialize, serde::Serialize)]
struct ProductInfo {
    id: i32,
    name: String,
    price: i32,
}

async fn get_products() -> impl Responder {
    HttpResponse::Ok().json(_get_products())
}

fn _get_products() -> Vec<ProductInfo> {
    match get() {
        Ok(products) => return products,
        Err(_) => {
            return vec![]
        },        
    }
}

#[tokio::main]
async fn get() -> Result<Vec<ProductInfo>, Error> {
    let pg_yrl = "postgres://myuser:mypassword@localhost:5432/mydb";
    let (client, connection) 
        = tokio_postgres::connect(pg_yrl, NoTls).await?;

    // 接続タスクをスポーンして実行
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // テーブルから全レコードを取得
    let rows = client.query("SELECT * FROM products", &[]).await?;

    let mut products: Vec<ProductInfo> = vec![];
    for row in rows {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let price: i32 = row.get(2);
        
        products.push(ProductInfo {
            id: id,
            name: name,
            price: price,
        });
    }
    Ok(products)
}

async fn add_product(info: web::Json<ProductInfo>) -> impl Responder {
    let response = format!("id: {}, name: {}, price: {}", info.id, info.name, info.price);
    HttpResponse::Ok().body(response)
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
        .route("/products", web::get().to(get_products))
    })
    .bind("0.0.0.0:8765")?
    .run()
    .await
}