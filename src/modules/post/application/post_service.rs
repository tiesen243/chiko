use crate::modules::post::{
    application::{CreatePostDto, UpdatePostDto},
    domain::Post,
};

pub struct PostService {
    posts: Vec<Post>,
}

impl PostService {
    pub fn new() -> Self {
        PostService { posts: Vec::new() }
    }

    pub fn all(&self) -> &Vec<Post> {
        &self.posts
    }

    pub fn one(&self, id: u32) -> Option<&Post> {
        self.posts.iter().find(|post| post.get_id() == id)
    }

    pub fn create(&mut self, data: CreatePostDto) -> &Post {
        let post = Post::new(data.id, data.title, data.content);
        self.posts.push(post);
        self.posts.last().unwrap()
    }

    pub fn update(&mut self, id: u32, data: UpdatePostDto) -> Option<&Post> {
        if let Some(post) = self.posts.iter_mut().find(|post| post.get_id() == id) {
            if let Some(title) = data.title {
                post.set_title(title);
            }

            if let Some(content) = data.content {
                post.set_content(content);
            }

            return Some(post);
        }

        None
    }

    pub fn delete(&mut self, id: u32) -> bool {
        if let Some(pos) = self.posts.iter().position(|post| post.get_id() == id) {
            self.posts.remove(pos);
            return true;
        }

        false
    }
}
