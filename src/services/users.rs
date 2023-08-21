use super::proto::{
    users_server::Users, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, GetUserRequest, GetUserResponse, UpdatePasswordRequest,
    UpdatePasswordResponse, UpdateUserRequest, UpdateUserResponse, User,
};
use crate::db::user::User as UserDb;
use serde_derive::{Deserialize, Serialize};
use tonic::{Request, Response, Status};
use tracing::debug;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UsersService {}

#[tonic::async_trait]
impl Users for UsersService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        debug!("REQUEST create_user: {:#?}", &request);
        unimplemented!()
    }

    // get user by uuid
    /// Retrieves a user based on the provided request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request containing the user ID.
    ///
    /// # Returns
    ///
    /// * `Result<Response<GetUserResponse>, Status>` - The response containing the user information or an error status.
    /// Retrieves a user based on the given request.
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        // Log the incoming request
        debug!("REQUEST get_user: {:#?}", &request);

        let response: Response<GetUserResponse> = match request.into_inner().id.as_str().try_into()
        {
            Ok(id) => match UserDb::get_user(id).await {
                Some(user) => Response::new(GetUserResponse {
                    success: true,
                    user: Some(User {
                        id: user.id.id.get("String").unwrap().to_string(),
                        email: user.email,
                        username: user.username,
                        name: user.name,
                        role: user.role as u32,
                        profile_image_url: user.profile_image_url.unwrap_or_default(),
                        created_at: user.created_at.to_string(),
                        updated_at: user.updated_at.to_string(),
                    }),
                    message: None,
                }),
                None => Response::new(GetUserResponse {
                    success: false,
                    user: None,
                    message: Some("user not found".to_string()),
                }),
            },
            Err(_) => Response::new(GetUserResponse {
                success: false,
                user: None,
                message: Some("invalid uuid".to_string()),
            }),
        };

        // Log the outgoing response
        debug!("RESPONSE get_user: {:#?}", &response);
        Ok(response)
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        debug!("REQUEST update_user: {:#?}", &request);
        unimplemented!()
    }

    async fn update_password(
        &self,
        request: Request<UpdatePasswordRequest>,
    ) -> Result<Response<UpdatePasswordResponse>, Status> {
        debug!("REQUEST update_password: {:#?}", &request);
        unimplemented!()
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        debug!("REQUEST delete_user: {:#?}", &request);
        unimplemented!()
    }
}
