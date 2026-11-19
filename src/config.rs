use std::env;
use std::sync::OnceLock;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    fn conf_env() -> Self {
        dotenvy::dotenv().ok();
        let host = env::var("HOST").unwrap_or(String::from("0.0.0.0"));
        let port = env::var("PORT").unwrap_or(String::from("3000"));
        let port = port
            .parse::<u16>()
            .unwrap_or_else(|e| panic!("port {port:?} is not a port number: {e}"));
        let database_url = env::var("DATABASE_URL").unwrap_or(String::from("sqlite://blog.db"));
        Self {
            host,
            port,
            database_url,
        }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    CONFIG.get_or_init(|| {
        #[cfg(feature = "dev")]
        dotenvy::from_filename(".env.dev").expect("Failed to load .env.dev");

        #[cfg(not(feature = "dev"))]
        dotenvy::dotenv().expect("Failed to load .env");

        Config::conf_env()
    })
}

// pub fn config() -> &'static Config {
//     CONFIG.get_or_init(Config::conf_env)
// }
