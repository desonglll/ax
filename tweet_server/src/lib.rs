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

/// 命令行参数
///
/// - `-r` / `--rmfolder`: 启动时是否删除并重新创建 uploads 文件夹
/// - `-c` / `--count`: 重复次数（暂未使用）
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    rmfolder: bool,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

/// 预加载函数
///
/// 解析命令行参数，根据参数创建或重置 uploads 目录。
pub async fn preload() {
    let args = Args::parse();
    make_directory("uploads", args.rmfolder);
}

/// 初始化 tracing 日志
///
/// 设置最大日志级别为 DEBUG。
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}
