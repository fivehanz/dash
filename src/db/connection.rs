use crate::configs;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Database, Jwt},
    Surreal,
};

use tracing::{debug, error};

pub static CONNECTION: Surreal<Client> = Surreal::init();

#[derive(Debug)]
pub struct Connection {
    jwt: Option<Jwt>,
}

#[derive(Debug)]
pub enum DatabaseConnectionStatus {
    Successful,
    Failed,
}

impl Connection {
    pub fn new() -> Self {
        debug!("created new db connection instance");
        Self { jwt: None }
    }

    // initialize db connection with error handling
    pub async fn init(&mut self, configs: &configs::Configs) -> DatabaseConnectionStatus {
        match Self::connect(&self, configs).await {
            Ok(jwt) => {
                self.jwt = Some(jwt);
                debug!("connected to database {:?}", &self);
                DatabaseConnectionStatus::Successful
            }
            Err(err) => {
                error!("{:?}", err);
                DatabaseConnectionStatus::Failed
            }
        }
    }

    // connect and signin to the database
    async fn connect(&self, configs: &configs::Configs) -> surrealdb::Result<Jwt> {
        // Connect to the database
        CONNECTION.connect::<Ws>(configs.db_host.clone()).await?;

        // sign in to the database
        CONNECTION
            .signin(Database {
                namespace: &configs.db_namespace,
                database: &configs.db_name,
                username: &configs.db_user,
                password: &configs.db_password,
            })
            .await
    }
}
