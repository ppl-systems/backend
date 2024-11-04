use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    success: bool,
    message: String,
}

/*
#[instrument(name = "Verifying User Credentials")]
pub async fn verify_credentials(
    form: web::Form<LoginRequest>,
    pool: web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO login (id, username, password, subscribed)
    "#
    )
}
*/

#[instrument(name = "Logging in user")]
#[post("/api/login")]
pub async fn login(info: web::Json<LoginRequest>) -> impl Responder {
    if info.username == "user" && info.password == "password" {
        HttpResponse::Ok().json(LoginResponse {
            success: true,
            message: String::from("Login successful!"),
        })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: String::from("Invalid credentials"),
        })
    }
}
