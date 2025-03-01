//! utils.rs
use once_cell::sync::Lazy;
use prodcast::config::app::{AppSettings, DatabaseSettings, Settings};
use prodcast::telemetry::tracing::{get_subscriber, init_subscriber};
use secrecy::{ExposeSecret, Secret};
use sqlx::{migrate, postgres::PgConnectOptions, Connection, Executor, PgConnection};
use std::net::TcpListener;
use uuid::Uuid;

// Create a struct to hold the app port and test database name
pub struct TestApp {
    pub address: String,
    pub database_name: String,
}

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    // We cannot assign the output of `get_subscriber` to a variable based on the value of `TEST_LOG`
    // because the sink is part of the type returned by `get_subscriber`, therefore they are not the
    // same type. We could work around it, but this is the most straight-forward way of moving forward.
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

/// Creates a test database
pub async fn create_test_database(config: &DatabaseSettings) -> Result<(), sqlx::Error> {
    let mut default_conn = create_default_connection(config).await;
    default_conn
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database))
        .await
        .expect("Failed to create database");

    // close the connection
    default_conn
        .close()
        .await
        .expect("Failed to close connection");
    // create a new connection to the test database
    let connect_options = initialize_pg_connect_options(config, config.database.clone());
    let mut test_db_conn = PgConnection::connect_with(&connect_options)
        .await
        .expect("Failed to connect to Postgres");

    // run the migrations using the same connection
    migrate!("./migrations")
        .run(&mut test_db_conn)
        .await
        .expect("Failed to migrate the database");
    // close the connection
    test_db_conn
        .close()
        .await
        .expect("Failed to close connection");
    Ok(())
    // return the connection as a result or an error
}

/// Create a default connection to the system 'postgres' database
pub async fn create_default_connection(config: &DatabaseSettings) -> PgConnection {
    let connect_options = initialize_pg_connect_options(config, "postgres".to_string());
    PgConnection::connect_with(&connect_options)
        .await
        .expect("Failed to connect to Postgres")
}

/// Create a connection pool to the test database
pub async fn create_test_connection_pool(config: &DatabaseSettings) -> sqlx::Result<sqlx::PgPool> {
    let connect_options = initialize_pg_connect_options(config, config.database.clone());
    sqlx::PgPool::connect_with(connect_options).await
}

/// Initialize the Postgres connection options for tests
pub fn initialize_pg_connect_options(
    config: &DatabaseSettings,
    database_name: String,
) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.user)
        .password(config.password.expose_secret())
        // connect to the default postgres database
        .database(&database_name)
}

/// Create a test configuration object
pub fn get_test_config(port: u16, database_name: &str) -> Settings {
    Settings {
        database: DatabaseSettings {
            host: "localhost".to_string(),
            port: 5432,
            user: "prodcast".to_string(),
            password: Secret::new("password".to_string()),
            database: String::from(database_name),
            connect_timeout_secs: 2,
        },
        application: AppSettings {
            port,
            host: "127.0.0.1".to_string(),
        },
    }
}

pub async fn spawn_app() -> TestApp {
    // Ensure that the tracing stack is only initialized once
    Lazy::force(&TRACING);
    // We bind to port 0 to get an available port from the OS
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    // Create a random db name to run tests in isolation
    let database_name = Uuid::new_v4().to_string();
    // make a configuration object for test purposes
    let configuration = get_test_config(port, &database_name);
    // create the test database
    create_test_database(&configuration.database)
        .await
        .expect("Failed to create test database");
    let server = prodcast::run(listener, configuration)
        .await
        .expect("Failed to bind address");
    tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        database_name: database_name.clone(),
    }
}
