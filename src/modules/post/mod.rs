pub mod application;
pub mod domain;
pub mod presentation;

use crate::modules::post::{application::PostService, presentation::http::PostController};
use worker::Router;

pub struct PostModule;

impl PostModule {
    pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
        let service = PostService::new();
        let _controller = PostController::new(service);

        router
    }
}
