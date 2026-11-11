use worker::{Response, Router};

use crate::modules::{post::PostModule, user::UserModule};

pub fn router() -> Router<'static, ()> {
    let router = Router::new();

    let router = PostModule::register(router);
    let router = UserModule::register(router);

    router
        .get("/", |_req, _ctx| Response::ok("Hello, World!"))
        .get("/health", |_req, _ctx| Response::ok("OK"))
        .get_async("/openapi.json", crate::docs::openapi)
        .get_async("/docs", crate::docs::swagger)
}
