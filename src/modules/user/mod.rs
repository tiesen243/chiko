pub mod application;
pub mod domain;
pub mod presentation;

use worker::Router;

use crate::modules::user::{application::UserService, presentation::http::UserController};

pub struct UserModule;

impl UserModule {
    pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
        let service = UserService::new();
        let _controller = UserController::new(service);

        router
    }
}
