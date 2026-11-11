use worker::{Request, Response, Result, RouteContext};

use crate::modules::post::application::{CreatePostDto, PostService, UpdatePostDto};

pub struct PostController;

impl PostController {
    pub async fn all(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let db = ctx.d1("DB")?;
        let service = PostService::new(db);

        let posts = service.all().await?;

        Response::from_json(&posts)
    }

    pub async fn one(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let id = Self::parse_id(&ctx)?;

        let db = ctx.d1("DB")?;
        let service = PostService::new(db);

        match service.one(id).await? {
            Some(post) => Response::from_json(&post),
            None => Response::error("Post not found", 404),
        }
    }

    pub async fn create(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let data: CreatePostDto = req.json().await?;

        let db = ctx.d1("DB")?;
        let service = PostService::new(db);

        service.create(data).await?;

        Response::ok("Post created")
    }

    pub async fn update(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let id = Self::parse_id(&ctx)?;
        let data: UpdatePostDto = req.json().await?;

        let db = ctx.d1("DB")?;
        let service = PostService::new(db);

        service.update(id, data).await?;

        Response::ok("Post updated")
    }

    pub async fn delete(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let id = Self::parse_id(&ctx)?;

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
}
