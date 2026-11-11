pub mod application;
pub mod domain;
pub mod presentation;

use worker::Router;

use crate::modules::user::presentation::http::user_controller;

pub struct UserModule;

impl UserModule {
    pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
        router
            .get_async("/api/users", user_controller::all)
            .get_async("/api/users/:id", user_controller::one)
            .post_async("/api/users", user_controller::create)
            .put_async("/api/users/:id", user_controller::update)
            .delete_async("/api/users/:id", user_controller::delete)
    }
}
