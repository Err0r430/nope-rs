use dotenv;
use std::env;

pub struct EnvConfig {
    pub port: i32,
    pub keys: Keys,
    pub db_url: String,
}

pub struct Keys {
    pub ring0: String,
    pub ring1: String,
    pub ring2: String,
}

/// Permission levels for API access; lower rings have more privileges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PermissionLevel {
    /// Ring 0 is root. Dev access to absolutely EVERYTHING. This is ⚠️VERY DANGEROUS⚠️
    Ring0,
    /// Ring 1 is a permitted user. This has full CRUD access to the API. Still risky but manually assigned.
    Ring1,
    /// Ring 2 is a read only user. This has read-only access to the API on certain routes. Read risky.
    Ring2,
    /// Ring 3 (public) is just public for anybody. Things have to be assigned this before it can be viewed.
    Public,
}


impl EnvConfig {
    // Get from env
    fn get_env(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| panic!("Environment variable {} not set", key))
    }

    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let port: i32 = Self::get_env("PORT").parse().unwrap_or(8081);

        // Key things
        let ring0_key = Self::get_env("API_KEY_0");

        let ring1_key = Self::get_env("API_KEY_1");

        let ring2_key = Self::get_env("API_KEY_2");

        let db_url = Self::get_env("DATABASE_URL");
        
        let api_key = Keys {
            ring0: ring0_key,
            ring1: ring1_key,
            ring2: ring2_key,
        };
        // End key things

        EnvConfig {
            port,
            keys: api_key,
            db_url,
        }
    }
}