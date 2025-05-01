use actix_web::web;
use crate::controllers::user_controller;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(user_controller::create_user))
            .route("", web::get().to(user_controller::get_all_users))
            .route("/{id}", web::get().to(user_controller::get_user))
            .route("/{id}", web::put().to(user_controller::update_user))
            .route("/{id}", web::delete().to(user_controller::delete_user)),
    );
}
