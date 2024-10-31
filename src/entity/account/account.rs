/*use uuid::Uuid; // Import Uuid
use crate::base::Base; // Import BaseEntity from the base_entity module

#[derive(Debug, Clone)]
pub struct Account {
    pub base: Base, // Composition with BaseEntity
    pub username: String,
    pub password: String
}

// Implement methods for AuroraEntity
impl Account {
    pub fn new(base: Base, username: &str, password: &str) -> Self {
        Account {
            base: Base::new(user, password),
            username: username,
            password: password,
        }
    }
}*/