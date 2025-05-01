use actix_web::{web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::models::user::{User, CreateUserRequest, UpdateUserRequest};

pub async fn create_user(pool: web::Data<MySqlPool>, data: web::Json<CreateUserRequest>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        data.name,
        data.email
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User created"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn get_all_users(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn get_user(pool: web::Data<MySqlPool>, id: web::Path<i64>) -> impl Responder {
    let result = sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = ?", *id)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn update_user(pool: web::Data<MySqlPool>, id: web::Path<i64>, data: web::Json<UpdateUserRequest>) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE users SET name = COALESCE(?, name), email = COALESCE(?, email) WHERE id = ?",
        data.name,
        data.email,
        *id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() == 0 => HttpResponse::NotFound().body("User not found"),
        Ok(_) => HttpResponse::Ok().body("User updated"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn delete_user(pool: web::Data<MySqlPool>, id: web::Path<i64>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", *id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() == 0 => HttpResponse::NotFound().body("User not found"),
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
