use utoipa::OpenApi;
use worker::{Request, Response, Result, RouteContext};

use crate::modules::{
    post::{
        application::{CreatePostDto, UpdatePostDto},
        domain::Post,
        presentation::http::post_controller,
    },
    user::{
        application::{CreateUserDto, UpdateUserDto},
        domain::User,
        presentation::http::user_controller,
    },
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Chiko API",
        version = "1.0.0",
        description = "API documentation for Chiko"
    ),
    components(
        schemas(
            Post,
            CreatePostDto,
            UpdatePostDto,
            User,
            CreateUserDto,
            UpdateUserDto
        )
    ),
    paths(
        post_controller::all,
        post_controller::one,
        post_controller::create,
        post_controller::update,
        post_controller::delete,
        user_controller::all,
        user_controller::one,
        user_controller::create,
        user_controller::update,
        user_controller::delete
    ),
    tags(
        (name = "Posts", description = "Post management"),
        (name = "Users", description = "User management")
    )
)]
pub struct ApiDoc;

pub async fn openapi(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::from_json(&ApiDoc::openapi())
}

pub async fn scalar(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::from_html(
        r#"<!doctype html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Chiko API Reference</title>
            </head>
            <body>
                <script
                    id="api-reference"
                    data-url="/openapi.json"
                    data-configuration='{"theme":"kepler"}'
                ></script>

                <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
            </body>
            </html>"#,
    )
}
