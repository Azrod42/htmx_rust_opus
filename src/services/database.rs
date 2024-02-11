use mongodb::{options::ClientOptions, Client};

use crate::structs::database::DatabaseConfig;

pub async fn init_database() -> mongodb::Client {
    let database_config = DatabaseConfig::new();
    let mut client_options = ClientOptions::parse(database_config.uri).await.unwrap();
    client_options.connect_timeout = database_config.connection_timeout;
    client_options.max_pool_size = database_config.max_pool_size;
    client_options.min_pool_size = database_config.min_pool_size;
    client_options.compressors = database_config.compressors;
    Client::with_options(client_options).unwrap()
}
