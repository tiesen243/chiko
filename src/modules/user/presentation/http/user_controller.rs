use worker::{Request, Response, Result, RouteContext};

use crate::modules::user::application::{CreateUserDto, UpdateUserDto, UserService};

pub struct UserController;

impl UserController {
    pub async fn all(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let db = ctx.d1("DB")?;
        let service = UserService::new(db);

        let users = service.all().await?;

        Response::from_json(&users)
    }

    pub async fn one(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let id = Self::parse_id(&ctx)?;

        let db = ctx.d1("DB")?;
        let service = UserService::new(db);

        match service.one(id).await? {
            Some(user) => Response::from_json(&user),
            None => Response::error("User not found", 404),
        }
    }

    pub async fn create(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let data: CreateUserDto = req.json().await?;

        let db = ctx.d1("DB")?;
        let service = UserService::new(db);

        service.create(data).await?;

        Response::ok("User created")
    }

    pub async fn update(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let id = Self::parse_id(&ctx)?;
        let data: UpdateUserDto = req.json().await?;

        let db = ctx.d1("DB")?;
        let service = UserService::new(db);

        service.update(id, data).await?;

        Response::ok("User updated")
    }

    pub async fn delete(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let id = Self::parse_id(&ctx)?;

        let db = ctx.d1("DB")?;
        let service = UserService::new(db);

        service.delete(id).await?;

        Response::ok("User deleted")
    }

    fn parse_id(ctx: &RouteContext<()>) -> Result<u32> {
        ctx.param("id")
            .unwrap()
            .parse::<u32>()
            .map_err(|_| worker::Error::RustError("Invalid user ID".into()))
    }
}
