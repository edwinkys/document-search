use clap::{ArgMatches, Command};

const START_COMMAND: &str = "start";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cli = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Interface to setup and manage the server")
        .arg_required_else_help(true)
        .subcommand(start_command())
        .get_matches();

    match cli.subcommand() {
        Some((START_COMMAND, args)) => start_handler(args).await,
        _ => unreachable!(),
    }
}

fn start_command() -> Command {
    Command::new(START_COMMAND)
        .alias("run")
        .about("Start the server")
}

async fn start_handler(_args: &ArgMatches) {}
