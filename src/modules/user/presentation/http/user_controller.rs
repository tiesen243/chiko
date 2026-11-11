use std::sync::{Arc, Mutex};

use worker::{Request, Response, Result, RouteContext};

use crate::modules::user::application::{CreateUserDto, UpdateUserDto, UserService};

pub struct UserController {
    user_service: Arc<Mutex<UserService>>,
}

impl UserController {
    pub fn new(user_service: UserService) -> Self {
        Self {
            user_service: Arc::new(Mutex::new(user_service)),
        }
    }

    pub async fn all(&self, _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
        let service = self.user_service.lock().unwrap();

        Response::from_json(service.all())
    }

    pub async fn one(&self, _req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let service = self.user_service.lock().unwrap();

        let id = ctx.param("id").unwrap().parse::<u32>().unwrap();

        match service.one(id) {
            Some(user) => Response::from_json(user),
            None => Response::error("User not found", 404),
        }
    }

    pub async fn create(&self, mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
        let mut service = self.user_service.lock().unwrap();

        let data: CreateUserDto = req.json().await?;
        let user = service.create(data);

        Response::from_json(user)
    }

    pub async fn update(&self, mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let mut service = self.user_service.lock().unwrap();

        let id = ctx.param("id").unwrap().parse::<u32>().unwrap();
        let data: UpdateUserDto = req.json().await?;

        match service.update(id, data) {
            Some(user) => Response::from_json(user),
            None => Response::error("User not found", 404),
        }
    }

    pub async fn delete(&self, _req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let mut service = self.user_service.lock().unwrap();

        let id = ctx.param("id").unwrap().parse::<u32>().unwrap();

        if service.delete(id) {
            Response::ok("User deleted")
        } else {
            Response::error("User not found", 404)
        }
    }
}
