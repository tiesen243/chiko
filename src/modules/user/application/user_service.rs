use crate::modules::user::{
    application::{CreateUserDto, UpdateUserDto},
    domain::User,
};

pub struct UserService {
    users: Vec<User>,
}

impl UserService {
    pub fn new() -> Self {
        UserService { users: Vec::new() }
    }

    pub fn all(&self) -> &Vec<User> {
        &self.users
    }

    pub fn one(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|user| user.get_id() == id)
    }

    pub fn create(&mut self, data: CreateUserDto) -> &User {
        let user = User::new(data.id, data.username, data.email);
        self.users.push(user);
        self.users.last().unwrap()
    }

    pub fn update(&mut self, id: u32, data: UpdateUserDto) -> Option<&User> {
        if let Some(user) = self.users.iter_mut().find(|user| user.get_id() == id) {
            if let Some(username) = data.username {
                user.set_username(username);
            }

            if let Some(email) = data.email {
                user.set_email(email);
            }

            return Some(user);
        }

        None
    }

    pub fn delete(&mut self, id: u32) -> bool {
        if let Some(pos) = self.users.iter().position(|user| user.get_id() == id) {
            self.users.remove(pos);
            return true;
        }

        false
    }
}
