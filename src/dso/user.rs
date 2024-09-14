use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::Type, PartialEq, Eq, Clone, Copy)]
#[sqlx(transparent)]
pub struct UserId(i32);
