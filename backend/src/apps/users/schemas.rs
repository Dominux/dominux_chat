use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserSchema {
    pub name: String,
}
