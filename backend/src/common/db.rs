use sea_orm::{Database, DbConn};

use crate::common::errors::AppResult;

pub async fn get_db(db_uri: &str) -> AppResult<DbConn> {
    Ok(Database::connect(db_uri).await?)
}
