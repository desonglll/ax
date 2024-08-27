// use libraries::mkdir::make_directory;

use clap::{arg, command, Parser};
use libraries::mkdir::make_directory;

pub mod dbaccess;
pub mod errors;
pub mod handlers;
pub mod libraries;
pub mod models;
pub mod routes;
pub mod state;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = false)]
    rmfolder: bool,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
pub async fn preload() {
    let args = Args::parse();
    make_directory("uploads", args.rmfolder);
}
