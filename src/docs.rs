use utoipa::OpenApi;
use worker::{Request, Response, Result, RouteContext};

use crate::modules::{
    post::application::{CreatePostDto, UpdatePostDto},
    post::domain::Post,
    user::application::{CreateUserDto, UpdateUserDto},
    user::domain::User,
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
        get_posts,
        get_post,
        create_post,
        update_post,
        delete_post,
        get_users,
        get_user,
        create_user,
        update_user,
        delete_user
    ),
    tags(
        (name = "Posts", description = "Post management"),
        (name = "Users", description = "User management")
    )
)]
pub struct ApiDoc;

// Các function dưới đây chỉ phục vụ sinh tài liệu.
// Handler thực tế vẫn nằm trong Controller.

#[utoipa::path(
    get,
    path = "/api/posts",
    tag = "Posts",
    responses(
        (status = 200, description = "Get all posts", body = [Post])
    )
)]
fn get_posts() {}

#[utoipa::path(
    get,
    path = "/api/posts/{id}",
    tag = "Posts",
    params(
        ("id" = u32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Post found", body = Post),
        (status = 404, description = "Post not found")
    )
)]
fn get_post() {}

#[utoipa::path(
    post,
    path = "/api/posts",
    tag = "Posts",
    request_body = CreatePostDto,
    responses(
        (status = 200, description = "Post created")
    )
)]
fn create_post() {}

#[utoipa::path(
    put,
    path = "/api/posts/{id}",
    tag = "Posts",
    params(
        ("id" = u32, Path)
    ),
    request_body = UpdatePostDto,
    responses(
        (status = 200, description = "Post updated"),
        (status = 404, description = "Post not found")
    )
)]
fn update_post() {}

#[utoipa::path(
    delete,
    path = "/api/posts/{id}",
    tag = "Posts",
    params(
        ("id" = u32, Path)
    ),
    responses(
        (status = 200, description = "Post deleted"),
        (status = 404, description = "Post not found")
    )
)]
fn delete_post() {}

// Users

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "Users",
    responses(
        (status = 200, description = "Get all users", body = [User])
    )
)]
fn get_users() {}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "Users",
    params(
        ("id" = u32, Path)
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    )
)]
fn get_user() {}

#[utoipa::path(
    post,
    path = "/api/users",
    tag = "Users",
    request_body = CreateUserDto,
    responses(
        (status = 200, description = "User created")
    )
)]
fn create_user() {}

#[utoipa::path(
    put,
    path = "/api/users/{id}",
    tag = "Users",
    params(
        ("id" = u32, Path)
    ),
    request_body = UpdateUserDto,
    responses(
        (status = 200, description = "User updated")
    )
)]
fn update_user() {}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = "Users",
    params(
        ("id" = u32, Path)
    ),
    responses(
        (status = 200, description = "User deleted")
    )
)]
fn delete_user() {}

pub async fn openapi(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::from_json(&ApiDoc::openapi())
}

pub async fn swagger(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::from_html(
        r#"<!DOCTYPE html>
            <html>
            <head>
                <title>Chiko API Docs</title>
                <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist/swagger-ui.css">
            </head>
            <body>
                <div id="swagger-ui"></div>

                <script src="https://unpkg.com/swagger-ui-dist/swagger-ui-bundle.js"></script>
                <script>
                    SwaggerUIBundle({
                        url: "/openapi.json",
                        dom_id: "\#swagger-ui"
                    });
                </script>
            </body>
            </html>"#,
    )
}
