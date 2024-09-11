use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ErrorMsg(pub String);
