use std::env;

pub struct PgConfig {
    host: String,
    port: u16,
    database: String,
    user: String,
    password: String,
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

        
        Self {
            host: host,
            port: port,
            database: database,
            user: pg_user,
            password: pg_password
        }
    }

    /// Create a Postgres Connection URI
    pub fn make_uri(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            urlencoding::encode(&self.user), urlencoding::encode(&self.password), urlencoding::encode(&self.host), &self.port, urlencoding::encode(&self.database)
        )
    }
}