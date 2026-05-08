use serde::{Deserialize, Serialize};

/// 错误消息包装结构
///
/// 用于将错误消息字符串包装为可序列化的 JSON 响应体。
#[derive(Serialize, Deserialize)]
pub struct ErrorMsg(pub String);
