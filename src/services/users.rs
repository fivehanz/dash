use super::proto::{
    users_server::Users, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, GetUserRequest, GetUserResponse, UpdatePasswordRequest,
    UpdatePasswordResponse, UpdateUserRequest, UpdateUserResponse, User,
};
use crate::db::user::User as UserDb;
use serde_derive::{Deserialize, Serialize};
use tonic::{Request, Response, Status};
use tracing::{debug, instrument, warn};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UsersService {}

#[tonic::async_trait]
impl Users for UsersService {
    /// Create a new user.
    ///
    /// This function handles the creation of a new user based on the provided request.
    /// It returns a response indicating whether the user creation was successful or not.
    ///
    /// # Arguments
    ///
    /// * `self` - The reference to the struct implementing this function.
    /// * `request` - The request object containing the user details.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response object indicating the success or failure of the user creation.
    ///
    /// # Errors
    ///
    /// An error is returned if there is a failure in processing the user creation request.
    #[instrument]
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        // Log the incoming request
        debug!("REQUEST create_user");

        let request = request.into_inner();

        // Create a new user
        match UserDb::new(
            request.email,
            request.password,
            request.username,
            request.name,
        )
        .await
        {
            Ok(user) => {
                // construct the response
                let response = Response::new(CreateUserResponse {
                    success: true,
                    id: user.id.id.get("String").unwrap().to_string(),
                });
                // Log the response
                debug!("RESPONSE create_user request {:#?}", &response);
                // return the response
                Ok(response)
            }
            Err(e) => {
                // ! handle error with specific message
                debug!("RESPONSE failed to process create_user, {:#?}", &e);
                Err(Status::unknown(e.to_string()))
            }
        }
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
    #[instrument]
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        // Log the incoming request
        debug!("REQUEST get_user");

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
                None => {
                    let error = Status::not_found("user not found".to_string());
                    warn!("RESPONSE failed to process get_user, {:#?}", &error);
                    return Err(error);
                }
            },
            Err(_) => {
                let error = Status::invalid_argument("invalid uuid".to_string());
                warn!("RESPONSE failed to process get_user, {:#?}", &error);
                return Err(error);
            }
        };

        // Log the outgoing response
        debug!("RESPONSE get_user: {:#?}", &response);
        Ok(response)
    }

    /// Updates a user based on the provided request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request containing the user ID and updated details.
    ///
    /// # Returns
    ///
    /// * `Result<Response<UpdateUserResponse>, Status>` - The response indicating the success or failure of the user update.
    /// Updates a user based on the given request.
    #[instrument]
    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        unimplemented!();
        // // Log the incoming request
        // debug!("REQUEST update_user");

        // let request: UpdateUserRequest = request.into_inner();

        // // Convert the user ID to the appropriate type
        // let user_id: uuid::Uuid = match request.id.as_str().try_into() {
        //     Ok(id) => id,
        //     Err(_) => {
        //         let error = Status::invalid_argument("invalid uuid".to_string());
        //         debug!("RESPONSE failed to process update_user, {:#?}", &error);
        //         return Err(error);
        //     }
        // };

        // // Update the user
        // match UserDb::update_user(user_id, request).await {
        //     Some(user) => {
        //         // Construct the response
        //         let response = Response::new(UpdateUserResponse {
        //             success: true,
        //             id: user.id.id.get("String").unwrap().to_string(),
        //             message: "user updated".to_string(),
        //         });
        //         // Log the response
        //         debug!("RESPONSE update_user: {:#?}", &response);
        //         // Return the response
        //         Ok(response)
        //     }
        //     None => {
        //         let error = Status::not_found("user not found".to_string());
        //         debug!("RESPONSE failed to process update_user, {:#?}", &error);
        //         Err(error)
        //     }
        // }
    }

    #[instrument]
    async fn update_password(
        &self,
        request: Request<UpdatePasswordRequest>,
    ) -> Result<Response<UpdatePasswordResponse>, Status> {
        debug!("REQUEST update_password: {:#?}", &request);
        unimplemented!()
    }

    // delete user by uuid
    /// Deletes a user based on the provided request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request containing the user ID.
    ///
    /// # Returns
    ///
    /// * `Result<Response<DeleteUserResponse>, Status>` - The response indicating the success or failure of the user deletion.
    /// Deletes a user based on the given request.
    #[instrument]
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        // Log the incoming request
        debug!("REQUEST delete_user");

        let request: DeleteUserRequest = request.into_inner();

        // Convert the user ID to the appropriate type
        let user_id: uuid::Uuid = match request.id.as_str().try_into() {
            Ok(id) => id,
            Err(_) => {
                let error = Status::invalid_argument("invalid uuid".to_string());
                debug!("RESPONSE failed to process delete_user, {:#?}", &error);
                return Err(error);
            }
        };

        // Delete the user
        match UserDb::delete_user(user_id).await {
            Some(user) => {
                // Construct the response
                let response = Response::new(DeleteUserResponse {
                    success: true,
                    id: user.id.id.get("String").unwrap().to_string(),
                    message: "user deleted".to_string(),
                });
                // Log the response
                debug!("RESPONSE delete_user: {:#?}", &response);
                // Return the response
                Ok(response)
            }
            None => {
                let error = Status::not_found("user not found".to_string());
                debug!("RESPONSE failed to process delete_user, {:#?}", &error);
                Err(error)
            }
        }
    }
}
