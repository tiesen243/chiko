pub mod application;
pub mod domain;
pub mod presentation;

use worker::Router;

use crate::modules::post::presentation::http::post_controller;

pub struct PostModule;

impl PostModule {
    pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
        router
            .get_async("/api/posts", post_controller::all)
            .get_async("/api/posts/:id", post_controller::one)
            .post_async("/api/posts", post_controller::create)
            .put_async("/api/posts/:id", post_controller::update)
            .delete_async("/api/posts/:id", post_controller::delete)
    }
}
