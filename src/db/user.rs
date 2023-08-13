use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Result as SurrealResult, Surreal};
use tracing::{debug, instrument, warn};
use uuid::Uuid;

use crate::db::connection::DB;

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

        match new_user.create(&DB).await {
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

    pub async fn get_user(id: Uuid) -> Option<UserDetails> {
        Self::read(&DB, id.to_string()).await.unwrap()
    }

    // ! generalize CRUD into Database struct on connection file
    async fn create(&self, conn: &Surreal<Client>) -> SurrealResult<UserDetails> {
        conn.create("user").content(self).await
    }

    async fn read(conn: &Surreal<Client>, id: String) -> SurrealResult<Option<UserDetails>> {
        conn.select(("user", id)).await
    }

    // async fn update(&self, conn: &Surreal<Client>) -> SurrealResult<User> {
    //     conn.update(("user", self.id.clone())).merge(self).await
    // }

    // async fn delete(&self, conn: &Surreal<Client>) -> SurrealResult<()> {
    //     conn.delete(("user", self.id.clone())).await
    // }

    // pub fn get_user(id: Uuid) -> Result<UserDetails, String> {
    //     // Query database for user

    //     let user = user.find_by_id(id);

    //     match user {
    //         Some(user) => Ok(UserDetails {
    //             id: user.id,
    //             username: user.username,
    //             email: user.email,
    //             name: user.name,
    //             profile_image_url: user.profile_image_url,
    //             created_at: user.created_at,
    //             updated_at: user.updated_at,
    //         }),
    //         None => Err("User not found".to_string()),
    //     }
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
