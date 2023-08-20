use tonic::{include_proto, Request, Response, Status};

use users::users_server::{Users, UsersServer};
use users::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, GetUserRequest,
    GetUserResponse, UpdateUserRequest, UpdateUserResponse,
};

#[derive(Debug, Default)]
pub struct UsersService {}

include_proto!("users");

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
        unimplemented!()
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
