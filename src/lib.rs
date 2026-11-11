pub mod docs;
pub mod modules;

use worker::*;

use crate::modules::{post::PostModule, user::UserModule};

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    let router = PostModule::register(router);
    let router = UserModule::register(router);

    router
        .get("/", |_req, _ctx| Response::ok("Hello, World!"))
        .get("/health", |_req, _ctx| Response::ok("OK"))
        .get_async("/openapi.json", crate::docs::openapi)
        .get_async("/docs", crate::docs::scalar)
        .run(req, env)
        .await
}
