use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[get("/users")]
pub async fn all_users() -> impl Responder {
    HttpResponse::Ok().body("Users")
}

#[post("/register")]
async fn register(payload: web::Json<Register>) -> impl Responder {
    // Hash the password and save the user to the DB so DO IT LATER
    println!("Registering: {:?}", payload);

    HttpResponse::Ok().json(UserResponse {
        id: 1,
        username: payload.username.clone(),
        email: payload.email.clone(),
        token: "drone-jwt-token-b9334".to_string(),
    })
}

#[post("/login")]
async fn login(payload: web::Json<Login>) -> impl Responder {
    // Verify credentials here so DO LATER
    println!("Logging in: {:?}", payload);

    HttpResponse::Ok().json(UserResponse {
        id: 1,
        username: payload.username.clone(),
        email: payload.email.clone(),
        token: "drone-jwt-token-b9334".to_string(),
    })
}

#[get("/me")]
async fn current_user() -> impl Responder {
    HttpResponse::Ok().json(UserResponse {
        id: 1,
        username: "dronebreaker".to_string(),
        email: "user@example.com".to_string(),
        token: "drone-jwt-token-b9334".to_string(),
    })
}

#[allow(dead_code)]
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
        .service(login)
        .service(current_user);
}