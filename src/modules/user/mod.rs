pub mod application;
pub mod domain;
pub mod presentation;

use worker::Router;

use crate::modules::user::presentation::http::UserController;

pub struct UserModule;

impl UserModule {
    pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
        router
            .get_async("/api/users", UserController::all)
            .get_async("/api/users/:id", UserController::one)
            .post_async("/api/users", UserController::create)
            .put_async("/api/users/:id", UserController::update)
            .delete_async("/api/users/:id", UserController::delete)
    }
}
