use std::net::SocketAddr;
use tonic::transport::{server::Router, Server};
use tracing::info;

pub fn create_router() -> Router {
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(super::users::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(super::users::users_server::UsersServer::new(
            super::users::UsersService::default(),
        ))
        .add_service(reflection)
}

pub async fn start_server(port: u16) -> Result<(), tokio::task::JoinError> {
    tokio::spawn(async move {
        // grpc ip address and port
        let grpc_addr = SocketAddr::from(([0, 0, 0, 0], port));
        info!("start gRPC server on {}", &grpc_addr);

        self::create_router().serve(grpc_addr).await.unwrap();
    })
    .await
}
