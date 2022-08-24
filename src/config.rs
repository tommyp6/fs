use std::env;

pub struct Config {
    pub session_key: String,
    pub host: String,
    pub port: u16,
    pub uploads_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            session_key: env::var("SESSION_KEY").unwrap_or("baddefaultkey".to_string()),
            host: env::var("HOST").unwrap_or("localhost".to_string()),
            port: env::var("PORT")
                .unwrap_or("8080".to_string())
                .parse()
                .unwrap_or(8080),
            uploads_dir: env::var("UPLOADS_DIR").unwrap_or("uploads".to_string()),
        }
    }
}
