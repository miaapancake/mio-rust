use std::env;

use deadpool_postgres::{Pool};

pub struct PgConfig {
    host: String,
    port: u16,
    database: String,
    user: String,
    password: String,
    pool_size: usize,
}

impl PgConfig {
    pub fn new() -> Self {
        let host = env::var("POSTGRES_HOST")
            .expect("POSTGRES_HOST needs to be set to create a database connection");
        let port = env::var("POSTGRES_PORT")
            .expect("POSTGRES_PORT needs to be set to create a database connection")
            .parse::<u16>()
            .expect("POSTGRES_PORT should be an integer between 1 and 65535");

        let database = env::var("POSTGRES_DATABASE")
            .expect("POSTGRES_DATABASE needs to be set to create a database connection");

        let pg_user = env::var("POSTGRES_USER")
            .expect("POSTGRES_USER needs to be set to create a database connection");

        let pg_password = env::var("POSTGRES_PASSWORD")
            .expect("POSTGRES_PASSWORD needs to be set to create a database connection");

        let pool_size = env::var("POSTGRES_POOL_SIZE")
            .unwrap_or("5".to_string())
            .parse::<usize>()
            .expect("POSTGRES_POOL_SIZE should be a number");

        Self {
            host: host,
            port: port,
            database: database,
            user: pg_user,
            password: pg_password,
            pool_size: pool_size,
        }
    }

    fn make_deadpool_cfg(&self) -> deadpool_postgres::Config {
        // change and mutate configuration
        let mut cfg = deadpool_postgres::Config::default();
        cfg.host = Some(self.host.to_string());
        cfg.port = Some(self.port);
        cfg.user = Some(self.user.to_string());
        cfg.dbname = Some(self.database.to_string());
        cfg.password = Some(self.password.to_string());

        // change the pool config
        let mut pool_config = cfg.get_pool_config();
        pool_config.max_size = self.pool_size;
        cfg.pool = Some(pool_config);

        cfg
    }

    pub async fn make_db_pool(&self) -> Pool {
        let config = self.make_deadpool_cfg();
        let Ok(pool) = config.create_pool(None, tokio_postgres::NoTls) else {
            panic!("Could not create a postgres connection pool")
        };
        pool
    }
}