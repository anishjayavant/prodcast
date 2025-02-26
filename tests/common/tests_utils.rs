//! utils.rs
use prodcast::config::app::{DatabaseSettings, Settings};
use sqlx::{migrate, postgres::PgConnectOptions, Connection, Executor, PgConnection};
use std::net::TcpListener;
use uuid::Uuid;

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

/// Initialize the Postgres connection options for tests
pub fn initialize_pg_connect_options(
    config: &DatabaseSettings,
    database_name: String,
) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.user)
        .password(&config.password)
        // connect to the default postgres database
        .database(&database_name)
}

/// Create a test configuration object
pub fn get_test_config(port: u16, database_name: String) -> Settings {
    Settings {
        database: DatabaseSettings {
            host: "localhost".to_string(),
            port: 5432,
            user: "prodcast".to_string(),
            password: "password".to_string(),
            database: database_name,
        },
        port,
    }
}

pub async fn spawn_app() -> String {
    // We bind to port 0 to get an available port from the OS
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    // Create a random db name to run tests in isolation
    let database_name = Uuid::new_v4().to_string();
    // make a configuration object for test purposes
    let configuration = get_test_config(port, database_name);
    // create the test database
    create_test_database(&configuration.database)
        .await
        .expect("Failed to create test database");
    let server = prodcast::run(listener, configuration)
        .await
        .expect("Failed to bind address");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
