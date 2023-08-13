use crate::configs;
use surrealdb::{
    engine::any::Any,
    opt::auth::{Database, Jwt},
    Surreal,
};
use tracing::error;

pub static DB: Surreal<Any> = Surreal::init();

// connect and signin to the database
async fn new(configs: configs::Configs) -> surrealdb::Result<Jwt> {
    // Connect to the database
    DB.connect(configs.db_host).await?;

    // sign in to the database
    DB.signin(Database {
        namespace: &configs.db_namespace,
        database: &configs.db_name,
        username: &configs.db_user,
        password: &configs.db_password,
    })
    .await
}

// initialize db connection with error handling
pub async fn init(configs: configs::Configs) -> surrealdb::Result<Jwt> {
    match new(configs).await {
        Ok(jwt) => Ok(jwt),
        Err(err) => {
            error!("{:?}", err);
            std::process::exit(1);
        }
    }
}
