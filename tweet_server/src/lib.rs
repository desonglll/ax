// use infra::mkdir::make_directory;

use clap::Parser;

use infra::mkdir::make_directory;

pub mod dbaccess;
pub mod errors;
pub mod extractors;
pub mod handlers;
pub mod infra;
pub mod models;
pub mod routes;
pub mod services;
pub mod state;
pub mod utils;

/// Command line arguments for the server binary.
///
/// - `rmfolder`: If true, deletes and recreates the uploads folder at startup.
/// - `count`: Repeat count (currently unused).
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    rmfolder: bool,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

/// Preload service configurations.
///
/// This function parses command line arguments and establishes the storage directory environment.
pub async fn preload() {
    let args = Args::parse();
    make_directory("uploads", args.rmfolder);
}

/// Initialize the global tracing subscriber.
///
/// Set the maximum log level to DEBUG.
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}
