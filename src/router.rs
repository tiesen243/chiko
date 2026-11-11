use worker::{Response, Router};

pub fn router() -> Router<'static, ()> {
    let router = Router::new();

    let router = crate::modules::user::UserModule::register(router);
    let router = crate::modules::post::PostModule::register(router);

    router
        .get("/", |_req, _ctx| Response::ok("Hello, World!"))
        .get("/health", |_req, _ctx| Response::ok("OK"))
}
