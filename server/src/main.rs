mod protos;
mod services;

use axum::{routing, Router};
use clap::{ArgMatches, Command};
use protos::coordinator_server::CoordinatorServer;
use semver::Version;
use services::{interface, Configuration, Service};
use sqlx::Connection;
use sqlx::PgConnection;
use std::sync::Arc;
use std::{env, fs};
use tokio::net::TcpListener;
use tonic::transport::Server;
use url::Url;

// List of commands.
// We do this to avoid using string literals in the code.
const START_COMMAND: &str = "start";
const MIGRATE_COMMAND: &str = "migrate";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cli = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Interface to setup and manage the server")
        .arg_required_else_help(true)
        .subcommand(start_command())
        .subcommand(migrate_command())
        .get_matches();

    match cli.subcommand() {
        Some((START_COMMAND, args)) => start_handler(args).await,
        Some((MIGRATE_COMMAND, args)) => migrate_handler(args).await,
        _ => unreachable!(),
    }
}

fn start_command() -> Command {
    Command::new(START_COMMAND)
        .alias("run")
        .about("Start the server")
}

async fn configuration() -> Configuration {
    let database_url = env::var("DL_DATABASE_URL")
        .expect("Please set the DL_DATABASE_URL environment variable")
        .parse::<Url>()
        .expect("Invalid database URL");

    let pool_size = match env::var("DL_POOL_SIZE").ok() {
        Some(pool_size) => pool_size.parse().expect("Invalid pool size"),
        None => 8,
    };

    Configuration {
        database_url,
        pool_size,
    }
}

async fn start_handler(_args: &ArgMatches) {
    let config = configuration().await;
    let service = Arc::new(Service::new(&config).await);

    // Check if the schema version matches the current version.
    let current_version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
    let schema_version = schema_version(&config.database_url).await;
    if schema_version != Some(current_version) {
        panic!("Please run the migrate command to update the schema.");
    }

    // Start the coordinator server in a separate task.
    let coordinator_service = service.clone();
    let coordinator_server = tokio::spawn(async move {
        start_coordinator_server(coordinator_service).await;
    });

    // Start the interface server in a separate task.
    let interface_service = service.clone();
    let interface_server = tokio::spawn(async move {
        start_interface_server(interface_service).await;
    });

    let _ = tokio::join!(coordinator_server, interface_server);
}

async fn start_coordinator_server(service: Arc<Service>) {
    let port = match env::var("DL_COORDINATOR_PORT").ok() {
        Some(port) => port.parse().expect("Invalid coordinator port"),
        None => 2500,
    };

    let addr = format!("[::]:{port}").parse().unwrap();
    tracing::info!("The coordinator server is ready on port {port}");

    Server::builder()
        .add_service(CoordinatorServer::new(service))
        .serve(addr)
        .await
        .expect("Failed to start the coordinator server");
}

async fn start_interface_server(service: Arc<Service>) {
    let port = match env::var("DL_INTERFACE_PORT").ok() {
        Some(port) => port.parse().expect("Invalid interface port"),
        None => 2505,
    };

    let listener = TcpListener::bind(format!("[::]:{port}"))
        .await
        .expect("Failed to bind a listener");

    let app = Router::new()
        .route("/", routing::get(interface::get_root))
        .with_state(service);

    tracing::info!("The interface server is ready on port {port}");
    axum::serve(listener, app)
        .await
        .expect("Failed to start the interface server");
}

fn migrate_command() -> Command {
    Command::new(MIGRATE_COMMAND)
        .about("Migrate the database schema to the latest version")
}

async fn migrate_handler(_args: &ArgMatches) {
    tracing::info!("Migrating the database schema...");

    let database_url = env::var("DL_DATABASE_URL")
        .expect("Please set the DL_DATABASE_URL environment variable")
        .parse::<Url>()
        .expect("Invalid database URL");

    let target_version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
    let schema_version = schema_version(&database_url)
        .await
        .unwrap_or("0.0.0".parse::<Version>().unwrap());

    if schema_version == target_version {
        tracing::info!("The database schema is up-to-date");
        return;
    }

    // List the migration scripts that need to be applied.
    let mut migrations = fs::read_dir("migrations")
        .expect("Failed to read the migrations directory")
        .filter_map(|entry| {
            let entry = entry.expect("Failed to read a directory entry");
            if entry.path().is_dir() {
                return None;
            }

            let _filename = entry.file_name();
            let filename = _filename.to_str().unwrap();
            if !filename.ends_with(".sql") {
                return None;
            }

            let _version = filename.split_at(filename.len() - 4).0;
            let version = Version::parse(_version).ok()?;
            match version <= schema_version || version > target_version {
                true => None,
                false => Some(version),
            }
        })
        .collect::<Vec<Version>>();

    migrations.sort_unstable();
    for migration in migrations.iter() {
        let path = format!("migrations/{}.sql", migration);
        let script = fs::read_to_string(&path)
            .expect("Failed to read the migration script");

        tracing::info!("Applying the migration script:\n\n{script}");

        let mut conn = PgConnection::connect(database_url.as_str())
            .await
            .expect("Failed to connect to the database");

        sqlx::raw_sql(&script)
            .execute(&mut conn)
            .await
            .expect("Failed to execute the migration script");

        sqlx::query("UPDATE version SET version = $1")
            .bind(migration.to_string())
            .execute(&mut conn)
            .await
            .expect("Failed to update the schema version");

        tracing::info!("Migrated the database schema to version {}", migration);
    }
}

async fn schema_version(url: &Url) -> Option<Version> {
    tracing::info!("Checking for the database schema version...");

    let mut conn = PgConnection::connect(url.as_str())
        .await
        .expect("Failed to connect to the database");

    let table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_name = 'version'
        )",
    )
    .fetch_one(&mut conn)
    .await
    .expect("Failed to check if the version table exists");

    if !table_exists {
        tracing::warn!("The version table does not exist");
        return None;
    }

    let version: Option<String> =
        sqlx::query_scalar("SELECT version FROM version")
            .fetch_optional(&mut conn)
            .await
            .expect("Failed to fetch the schema version");

    version.map(|version| {
        Version::parse(&version).expect("Unable to parse the schema version")
    })
}
