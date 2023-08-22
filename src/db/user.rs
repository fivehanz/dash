use crate::db::connection::CONNECTION as DB;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, instrument, warn};
use uuid::Uuid;
use validator::Validate;

const TABLE: &str = "user";

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    #[validate(length(min = 4, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    #[validate(length(min = 3))]
    pub name: String,
    pub role: Role,
    #[validate(url)]
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

#[derive(Debug, Deserialize)]
pub struct Id {
    pub id: HashMap<String, Uuid>,
    // pub tb: String,
}

#[derive(Debug, Deserialize)]
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

// ! create UserStatus enum (generalized)
#[allow(dead_code)]
impl User {
    #[instrument(skip(new_user))]
    pub async fn new(new_user: Self) -> Result<UserDetails, surrealdb::Error> {
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
                warn!("failed to get user: {:?}", e);
                None
            }
        }
    }

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

    #[instrument]
    pub async fn delete_user(id: Uuid) -> Option<UserDetails> {
        match DB.delete((TABLE, id.to_string())).await {
            Ok(user) => {
                debug!("deleted user");
                Some(user)
            }
            Err(e) => {
                warn!("failed to delete user: {:?}", &e);
                None
            }
        }
    }

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

    pub fn get_role(&self) -> Role {
        self.role
    }

    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }

    pub fn is_moderator(&self) -> bool {
        self.role == Role::Moderator
    }
}
