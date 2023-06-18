use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}