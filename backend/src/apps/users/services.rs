use entity::users::Model as User;
use sea_orm::DbConn;

use super::{repositories::UsersRepository, schemas::CreateUserSchema};
use crate::common::errors::AppResult;

pub struct UserService<'a> {
    repo: UsersRepository<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        let repo = UsersRepository::new(db);
        Self { repo }
    }

    pub async fn create(&self, user: &CreateUserSchema) -> AppResult<User> {
        self.repo.create(user).await
    }

    pub async fn list(&self) -> AppResult<Vec<User>> {
        self.repo.list().await
    }

    pub async fn get(&self, user_id: uuid::Uuid) -> AppResult<User> {
        self.repo.get(user_id).await
    }
}
