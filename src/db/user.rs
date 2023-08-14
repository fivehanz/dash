use crate::db::connection::CONNECTION as DB;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, instrument, warn};
use uuid::Uuid;

const TABLE: &str = "user";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: Role,
    pub profile_image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Role {
    Admin,
    Moderator,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    pub id: HashMap<String, Uuid>,
    // pub tb: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDetails {
    pub id: Id,
    pub username: String,
    pub email: String,
    pub name: String,
    pub role: Role,
    pub profile_image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    #[instrument]
    pub async fn new(
        email: String,
        password: String,
        username: String,
        name: String,
    ) -> Result<UserDetails, surrealdb::Error> {
        // Hash password
        let hashed_password = password;

        let new_user = Self {
            id: Uuid::new_v4().to_string(), // ! change it to Uuid::now_v7()
            username,
            email,
            password: hashed_password,
            name,
            role: Role::User,
            profile_image_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        match DB.create(TABLE).content(new_user).await {
            Ok(user) => {
                debug!("created new user");
                Ok(user)
            }
            Err(e) => {
                warn!("failed to created new user");
                Err(e)
            }
        }
    }

    #[instrument]
    pub async fn get_user(id: Uuid) -> Option<UserDetails> {
        match DB.select((TABLE, id.to_string())).await {
            Ok(user) => Some(user),
            Err(e) => {
                debug!("failed to get user: {:?}", e);
                None
            }
        }
    }

    // ! create UpdateStatus enum (generalized)
    // #[instrument(skip(self))]
    // async fn update(&self) {
    //     match DB.update((TABLE, self.id)).merge(self).await {
    //         Ok(user) => Ok(user),
    //         Err(e) => {
    //             debug!("failed to update user: {:?}", &e);
    //             Err(e)
    //         }
    //     }
    // }

    // async fn delete(&self, conn: &Surreal<Client>) -> SurrealResult<()> {
    //     conn.delete(("user", self.id.clone())).await
    // }

    // #[instrument(skip(self))]
    // pub fn update_name(&mut self, name: String) {
    //     self.name = name;
    //     self.updated_at = Utc::now();
    //     debug!("name updated");
    // }

    // #[instrument(skip(self))]
    // pub fn update_username(&mut self, username: String) {
    //     self.username = username;
    //     self.updated_at = Utc::now();
    //     debug!("username updated");
    // }

    // #[instrument(skip(self))]
    // pub fn update_email(&mut self, email: String) {
    //     self.email = email;
    //     self.updated_at = Utc::now();
    //     debug!("email updated");
    // }

    // #[instrument(skip(self))]
    // pub fn update_password(&mut self, new_password: String) {
    //     // Hash new password
    //     let hashed_password = new_password;

    //     self.password = hashed_password;
    //     self.updated_at = Utc::now();
    //     debug!("password updated");
    // }

    // #[instrument(skip(self))]
    // pub fn update_profile_image(&mut self, image_url: String) {
    //     self.profile_image_url = Some(image_url);
    //     self.updated_at = Utc::now();
    //     debug!("profile image updated");
    // }

    #[instrument(skip(self))]
    pub fn validate_email(&self, email: String) -> Result<(), String> {
        // Email validation logic
        if !email.contains("@") {
            return Err("Invalid email".to_string());
        }

        Ok(())
    }

    // pub fn get_role(&self) -> Role {
    //     self.role
    // }

    // pub fn is_admin(&self) -> bool {
    //     self.role == Role::Admin
    // }

    // pub fn is_moderator(&self) -> bool {
    //     self.role == Role::Moderator
    // }
}
