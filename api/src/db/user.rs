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
    User,
    Moderator,
    Admin,
}

// implement Role: From<u32>
impl From<u32> for Role {
    fn from(role: u32) -> Self {
        match role {
            0 => Role::User,
            1 => Role::Moderator,
            2 => Role::Admin,
            _ => Role::User,
        }
    }
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

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct UserUpdateDetails {
    #[validate(length(min = 4, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub name: String,
    pub role: Role,
    #[validate(url)]
    pub profile_image_url: Option<String>,
    pub updated_at: DateTime<Utc>,
}

// implement From<UserDetails> to UserUpdateDetails
impl From<UserDetails> for UserUpdateDetails {
    fn from(user: UserDetails) -> Self {
        UserUpdateDetails {
            username: user.username,
            email: user.email,
            name: user.name,
            role: user.role,
            profile_image_url: user.profile_image_url,
            updated_at: Utc::now(),
        }
    }
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

    #[instrument]
    pub async fn update_user(
        id: Uuid,
        user: UserUpdateDetails,
    ) -> Result<UserDetails, surrealdb::Error> {
        // update in database
        match DB.update((TABLE, id.to_string())).merge(user).await {
            Ok(user) => Ok(user),
            Err(e) => {
                debug!("failed to update user: {:?}", &e);
                Err(e)
            }
        }
    }

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
    // pub fn update_password(&mut self, new_password: String) {
    //     // Hash new password
    //     let hashed_password = new_password;

    //     self.password = hashed_password;
    //     self.updated_at = Utc::now();
    //     debug!("password updated");
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
