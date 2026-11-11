pub mod modules;
pub mod router;

use worker::*;

use crate::router::router;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    router().run(req, env).await
}
