use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, EntityTrait};
use uuid;

use entity::users;

use crate::{
    apps::users::schemas::CreateUserSchema,
    common::errors::{AppError, AppResult},
};

pub struct UsersRepository<'a> {
    db: &'a DbConn,
}

impl<'a> UsersRepository<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }

    pub async fn create(&self, user: &CreateUserSchema) -> AppResult<users::Model> {
        let user = users::ActiveModel {
            id: ActiveValue::Set(uuid::Uuid::new_v4()),
            name: ActiveValue::Set(user.name.clone()),
        };

        let user_from_db = user.insert(self.db).await?;

        Ok(user_from_db)
    }

    pub async fn list(&self) -> AppResult<Vec<users::Model>> {
        let db_users = users::Entity::find().all(self.db).await?;
        Ok(db_users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn get(&self, user_id: uuid::Uuid) -> AppResult<users::Model> {
        users::Entity::find_by_id(user_id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound(format!("user with id {user_id}")))
    }
}
