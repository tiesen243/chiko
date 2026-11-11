pub mod application;
pub mod domain;
pub mod presentation;

use worker::Router;

use crate::modules::post::presentation::http::PostController;

pub struct PostModule;

impl PostModule {
    pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
        router
            .get_async("/api/posts", PostController::all)
            .get_async("/api/posts/:id", PostController::one)
            .post_async("/api/posts", PostController::create)
            .put_async("/api/posts/:id", PostController::update)
            .delete_async("/api/posts/:id", PostController::delete)
    }
}
