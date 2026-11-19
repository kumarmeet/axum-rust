use std::env;
use std::sync::OnceLock;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    fn conf_env() -> Self {
        dotenvy::dotenv().ok();
        let host = env::var("HOST").unwrap_or(String::from("0.0.0.0"));
        let port = env::var("PORT").unwrap_or(String::from("3000"));
        let port = port
            .parse::<u16>()
            .unwrap_or_else(|e| panic!("port {port:?} is not a port number: {e}"));
        Self { host, port }
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
