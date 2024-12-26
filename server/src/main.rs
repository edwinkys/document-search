mod protos;
mod services;

use clap::{arg, ArgMatches, Command};
use protos::coordinator_server::CoordinatorServer;
use services::{Configuration, Service};
use std::env;
use std::sync::Arc;
use tonic::transport::Server;

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

async fn start_handler(_args: &ArgMatches) {
    let config = Configuration {};
    let service = Arc::new(Service::new(&config));

    // Start the coordinator server in a separate task.
    let coordinator_service = service.clone();
    let coordinator_server = tokio::spawn(async move {
        start_coordinator_server(coordinator_service).await;
    });

    let _ = tokio::join!(coordinator_server);
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

fn migrate_command() -> Command {
    let arg_version = arg!(--version <version> "Schema version to migrate to")
        .default_value(env!("CARGO_PKG_VERSION"));

    Command::new(MIGRATE_COMMAND)
        .about("Migrate the database schema")
        .arg(arg_version)
}

async fn migrate_handler(_args: &ArgMatches) {}
