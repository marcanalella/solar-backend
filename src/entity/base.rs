/*use crate::utils::date;
use crate::utils::date::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Import Uuid from the uuid crate

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Base {
    pub id: i32,
    pub created_at: Date,
    pub updated_at: Date,
}

// Implement methods for BaseEntity
impl Base {
    pub fn new(id: Uuid, password: &str) -> Self {
        let now = date::now();
        Self {
            id: id,
            updated_at: now,
            created_at: now,
        }
    }
}*/
