use std::sync::{Arc, Mutex};

use worker::{Request, Response, Result, RouteContext};

use crate::modules::post::application::{CreatePostDto, PostService, UpdatePostDto};

pub struct PostController {
    post_service: Arc<Mutex<PostService>>,
}

impl PostController {
    pub fn new(post_service: PostService) -> Self {
        Self {
            post_service: Arc::new(Mutex::new(post_service)),
        }
    }

    pub async fn all(&self, _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
        let service = self.post_service.lock().unwrap();

        Response::from_json(service.all())
    }

    pub async fn one(&self, _req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let service = self.post_service.lock().unwrap();

        let id = ctx.param("id").unwrap().parse::<u32>().unwrap();

        match service.one(id) {
            Some(post) => Response::from_json(post),
            None => Response::error("Post not found", 404),
        }
    }

    pub async fn create(&self, mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
        let mut service = self.post_service.lock().unwrap();

        let data: CreatePostDto = req.json().await?;
        let post = service.create(data);

        Response::from_json(post)
    }

    pub async fn update(&self, mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let mut service = self.post_service.lock().unwrap();

        let id = ctx.param("id").unwrap().parse::<u32>().unwrap();
        let data: UpdatePostDto = req.json().await?;

        match service.update(id, data) {
            Some(post) => Response::from_json(post),
            None => Response::error("Post not found", 404),
        }
    }

    pub async fn delete(&self, _req: Request, ctx: RouteContext<()>) -> Result<Response> {
        let mut service = self.post_service.lock().unwrap();

        let id = ctx.param("id").unwrap().parse::<u32>().unwrap();

        if service.delete(id) {
            Response::ok("Post deleted")
        } else {
            Response::error("Post not found", 404)
        }
    }
}
