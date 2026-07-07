use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub act: String,
    pub iat: u64,
    pub exp: u64,
    pub rexp: u64,
}
