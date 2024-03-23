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

#[derive(serde::Deserialize, serde::Serialize)]
struct ProductInfoReq {
    name: String,
    price: i32,
}

async fn get_products() -> impl Responder {
    HttpResponse::Ok().json(_get_products().await)
}

async fn _get_products() -> Vec<ProductInfo> {
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

async fn add_product(info: web::Json<ProductInfoReq>) -> impl Responder {
    println!("add_product");
    let response = format!("id: {}, name: {}, price: {}", -1, info.name, info.price);
    match post(info) {
        Ok(_) => {println!("Data inserted successfully!"); HttpResponse::Ok().body(response)},
        Err(e) => {println!("Error inserting data: {}", e); HttpResponse::ExpectationFailed().body("Error inserting data".to_string())},
    }
}

#[tokio::main]
async fn post(info: web::Json<ProductInfoReq>) -> Result<(), Error> {
    println!("post");
    let pg_yrl = "postgres://myuser:mypassword@localhost:5432/mydb";
    let (client, connection) 
        = tokio_postgres::connect(pg_yrl, NoTls).await?;

    // 接続タスクをスポーンして実行
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // データ挿入
    match client.execute(
        "INSERT INTO products (product_name, price) VALUES ($1, $2)",
        &[&(info.name), &(info.price)],
    ).await {
        Ok(_) => {println!("Data inserted successfully!"); Ok(())},
        Err(e) => {Err(e)},
    }
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