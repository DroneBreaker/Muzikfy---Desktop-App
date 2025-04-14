pub mod api;
pub mod services;


use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::thread;

#[allow(dead_code)]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix and DroneBreaker in Tauri!")
}

pub fn start_server() {
    thread::spawn(|| {
        let sys = actix_web::rt::System::new();

        sys.block_on(async {
            HttpServer::new(|| {
                App::new()
                    .route("/api/hello", web::get().to(hello))
            })
            .bind(("127.0.0.1", 8080))
            .expect("Failed to bind server")
            .run()
            .await
            .expect("Failed to run server");
        });
    });
}
