//! utils.rs
use prodcast::config::app::{DatabaseSettings, Settings};
use sqlx::{migrate, postgres::PgConnectOptions, Connection, Executor, PgConnection};
use std::net::TcpListener;
use uuid::Uuid;

// create a test database using sqlx
async fn create_test_database(config: &DatabaseSettings) -> Result<(), sqlx::Error> {
    let connect_options = PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.user)
        .password(&config.password)
        // connect to the default postgres database
        .database("postgres");
    let mut default_conn = PgConnection::connect_with(&connect_options)
        .await
        .expect("Failed to connect to Postgres");
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
    let connect_options = PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.user)
        .password(&config.password)
        .database(&config.database);
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

pub async fn spawn_app() -> String {
    // We bind to port 0 to get an available port from the OS
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    // Create a random db name to run tests in isolation
    let database_name = Uuid::new_v4().to_string();
    // make a configuration object for test purposes
    let configuration = Settings {
        database: DatabaseSettings {
            host: "localhost".to_string(),
            port: 5432,
            user: "prodcast".to_string(),
            password: "password".to_string(),
            database: database_name,
        },
        port,
    };
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
