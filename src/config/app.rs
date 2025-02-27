/// Settings for the database connection
// struct for capturing all postgres conneciton fields
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(serde::Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
}

// struct for capturing all the app settings, which includes database settings, and the app port
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: AppSettings,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    // Read the "base" configuration file
    let config_dir = base_path.join("config");
    settings.merge(config::File::from(config_dir.join("base")).required(true))?;

    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .into();
    settings.merge(config::File::from(config_dir.join(environment.as_str())).required(true))?;
    settings.try_into()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.user, self.password, self.host, self.port
        )
    }
}

/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    DockerLocal,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::DockerLocal => "docker",
            Environment::Production => "prod",
        }
    }
}

impl From<String> for Environment {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "local" => Environment::Local,
            "docker" => Environment::DockerLocal,
            "prod" => Environment::Production,
            _ => Environment::Local,
        }
    }
}
