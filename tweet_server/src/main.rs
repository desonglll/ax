use chrono::prelude::*;

fn main() {
    // 获取当前的 UTC 时间
    let utc_time = Utc::now();

    // 转换为本地时间
    let local_time = utc_time.with_timezone(&Local);

    println!("UTC Time: {}", utc_time);
    println!("Local Time: {}", local_time);
}
