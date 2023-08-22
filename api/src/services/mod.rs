pub mod grpc;
pub mod router;
pub mod users;
#[allow(clippy::match_single_binding)]
pub mod proto {
    tonic::include_proto!("api");
}
