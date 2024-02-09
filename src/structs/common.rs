use mongodb::options::Compressor;
use std::time::Duration;

pub struct DatabaseConfig {
    pub uri: String,
    pub connection_timeout: Option<Duration>,
    pub min_pool_size: Option<u32>,
    pub max_pool_size: Option<u32>,
    pub compressors: Option<Vec<Compressor>>,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        let mongo_uri: String = "mongodb+srv://tomsorabella:47BpnKFcypXxbltS@opusdb.zctglq9.mongodb.net/?retryWrites=true&w=majority".to_string();

        let mongo_connection_timeout: u64 = 10000;

        let mongo_min_pool_size: u32 = 2;

        let mongo_max_pool_size: u32 = 10;

        Self {
            uri: mongo_uri,
            connection_timeout: Some(Duration::from_secs(mongo_connection_timeout)),
            min_pool_size: Some(mongo_min_pool_size),
            max_pool_size: Some(mongo_max_pool_size),
            compressors: Some(vec![
                Compressor::Snappy,
                Compressor::Zlib {
                    level: Default::default(),
                },
                Compressor::Zstd {
                    level: Default::default(),
                },
            ]),
        }
    }
}
