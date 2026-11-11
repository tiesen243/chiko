use worker::{Request, Response, Result, RouteContext};

use crate::modules::user::{
    application::{CreateUserDto, UpdateUserDto, UserService},
    domain::User,
};

#[utoipa::path(
        get,
        path = "/api/users",
        tag = "Users",
        responses(
            (status = 200, description = "Get all users", body = [User])
        )
    )]
pub async fn all(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.d1("DB")?;
    let service = UserService::new(db);

    let users = service.all().await?;

    Response::from_json(&users)
}

#[utoipa::path(
        get,
        path = "/api/users/{id}",
        tag = "Users",
        params(
            ("id" = u32, Path, description = "User ID")
        ),
        responses(
            (status = 200, description = "User found", body = User),
            (status = 404, description = "User not found")
        )
    )]
pub async fn one(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = parse_id(&ctx)?;

    let db = ctx.d1("DB")?;
    let service = UserService::new(db);

    match service.one(id).await? {
        Some(user) => Response::from_json(&user),
        None => Response::error("User not found", 404),
    }
}

#[utoipa::path(
        post,
        path = "/api/users",
        tag = "Users",
        request_body = CreateUserDto,
        responses(
            (status = 200, description = "User created", body = String)
        )
    )]
pub async fn create(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let data: CreateUserDto = req.json().await?;

    let db = ctx.d1("DB")?;
    let service = UserService::new(db);

    service.create(data).await?;

    Response::ok("User created")
}

#[utoipa::path(
        put,
        path = "/api/users/{id}",
        tag = "Users",
        params(
            ("id" = u32, Path, description = "User ID")
        ),
        request_body = UpdateUserDto,
        responses(
            (status = 200, description = "User updated", body = String)
        )
    )]
pub async fn update(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = parse_id(&ctx)?;
    let data: UpdateUserDto = req.json().await?;

    let db = ctx.d1("DB")?;
    let service = UserService::new(db);

    service.update(id, data).await?;

    Response::ok("User updated")
}

#[utoipa::path(
        delete,
        path = "/api/users/{id}",
        tag = "Users",
        params(
            ("id" = u32, Path, description = "User ID")
        ),
        responses(
            (status = 200, description = "User deleted", body = String)
        )
    )]
pub async fn delete(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = parse_id(&ctx)?;

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
