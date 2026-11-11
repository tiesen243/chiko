use worker::{Request, Response, Result, RouteContext};

use crate::modules::post::{
    application::{CreatePostDto, PostService, UpdatePostDto},
    domain::Post,
};

#[utoipa::path(
        get,
        path = "/api/posts",
        tag = "Posts",
        responses(
            (status = 200, description = "Get all posts", body = [Post])
        )
    )]
pub async fn all(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.d1("DB")?;
    let service = PostService::new(db);

    let posts = service.all().await?;

    Response::from_json(&posts)
}

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
pub async fn one(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = parse_id(&ctx)?;

    let db = ctx.d1("DB")?;
    let service = PostService::new(db);

    match service.one(id).await? {
        Some(post) => Response::from_json(&post),
        None => Response::error("Post not found", 404),
    }
}

#[utoipa::path(
        post,
        path = "/api/posts",
        tag = "Posts",
        request_body = CreatePostDto,
        responses(
            (status = 200, description = "Post created", body = String)
        )
    )]
pub async fn create(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let data: CreatePostDto = req.json().await?;

    let db = ctx.d1("DB")?;
    let service = PostService::new(db);

    service.create(data).await?;

    Response::ok("Post created")
}

#[utoipa::path(
        put,
        path = "/api/posts/{id}",
        tag = "Posts",
        params(
            ("id" = u32, Path, description = "Post ID")
        ),
        request_body = UpdatePostDto,
        responses(
            (status = 200, description = "Post updated", body = String)
        )
    )]
pub async fn update(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = parse_id(&ctx)?;
    let data: UpdatePostDto = req.json().await?;

    let db = ctx.d1("DB")?;
    let service = PostService::new(db);

    service.update(id, data).await?;

    Response::ok("Post updated")
}

#[utoipa::path(
        delete,
        path = "/api/posts/{id}",
        tag = "Posts",
        params(
            ("id" = u32, Path, description = "Post ID")
        ),
        responses(
            (status = 200, description = "Post deleted", body = String)
        )
    )]
pub async fn delete(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = parse_id(&ctx)?;

    let db = ctx.d1("DB")?;
    let service = PostService::new(db);

    service.delete(id).await?;

    Response::ok("Post deleted")
}

fn parse_id(ctx: &RouteContext<()>) -> Result<u32> {
    ctx.param("id")
        .unwrap()
        .parse::<u32>()
        .map_err(|_| worker::Error::RustError("Invalid post ID".into()))
}
