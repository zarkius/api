use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(not(target_arch = "wasm32"))]
use actix_web::{web, App, HttpServer, Responder};

#[cfg(not(target_arch = "wasm32"))]
#[wasm_bindgen(start)]
pub async fn start() {
    HttpServer::new(|| {
        App::new()
            .route("/data", web::get().to(|| async { "Hello, Actix!" }))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}