use super::users::users_server::Users;
use tonic::{include_proto, Request, Response, Status};

include_proto!("users");

#[derive(Debug, Default)]
pub struct UsersService {}

#[tonic::async_trait]
impl Users for UsersService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        unimplemented!()
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        Ok(Response::new(GetUserResponse {
            id: request.into_inner().id,
            name: "name".to_string(),
            email: "email".to_string(),
        }))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        unimplemented!()
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        unimplemented!()
    }
}
