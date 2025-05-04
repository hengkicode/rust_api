use actix_web::{web, HttpResponse, Responder};
use log::info; // Impor log info!
use sqlx::MySqlPool;
use crate::models::user::{User, CreateUserRequest, UpdateUserRequest};

pub async fn create_user(pool: web::Data<MySqlPool>, data: web::Json<CreateUserRequest>) -> impl Responder {
    info!("Creating user: {:?}", data);

    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        data.name,
        data.email
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            info!("User created successfully with name: {}", data.name);
            HttpResponse::Ok().body("User created")
        },
        Err(e) => {
            info!("Error creating user: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        },
    }
}

pub async fn get_all_users(pool: web::Data<MySqlPool>) -> impl Responder {
    info!("Fetching all users");

    let result = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(users) => {
            info!("Successfully fetched all users");
            HttpResponse::Ok().json(users)
        },
        Err(e) => {
            info!("Error fetching all users: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        },
    }
}

pub async fn get_user(pool: web::Data<MySqlPool>, id: web::Path<i64>) -> impl Responder {
    info!("Fetching user with ID: {}", id);

    let result = sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = ?", *id)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(user) => {
            info!("Successfully fetched user with ID: {}", id);
            HttpResponse::Ok().json(user)
        },
        Err(sqlx::Error::RowNotFound) => {
            info!("User with ID: {} not found", id);
            HttpResponse::NotFound().body("User not found")
        },
        Err(e) => {
            info!("Error fetching user with ID: {}: {}", id, e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        },
    }
}

pub async fn update_user(pool: web::Data<MySqlPool>, id: web::Path<i64>, data: web::Json<UpdateUserRequest>) -> impl Responder {
    info!("Updating user with ID: {}. Data: {:?}", id, data);

    let result = sqlx::query!(
        "UPDATE users SET name = COALESCE(?, name), email = COALESCE(?, email) WHERE id = ?",
        data.name,
        data.email,
        *id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() == 0 => {
            info!("User with ID: {} not found for update", id);
            HttpResponse::NotFound().body("User not found")
        },
        Ok(_) => {
            info!("User with ID: {} updated successfully", id);
            HttpResponse::Ok().body("User updated")
        },
        Err(e) => {
            info!("Error updating user with ID: {}: {}", id, e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        },
    }
}

pub async fn delete_user(pool: web::Data<MySqlPool>, id: web::Path<i64>) -> impl Responder {
    info!("Deleting user with ID: {}", id);

    let result = sqlx::query!("DELETE FROM users WHERE id = ?", *id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() == 0 => {
            info!("User with ID: {} not found for deletion", id);
            HttpResponse::NotFound().body("User not found")
        },
        Ok(_) => {
            info!("User with ID: {} deleted successfully", id);
            HttpResponse::Ok().body("User deleted")
        },
        Err(e) => {
            info!("Error deleting user with ID: {}: {}", id, e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        },
    }
}
