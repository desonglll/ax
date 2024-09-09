use serde::{Deserialize, Serialize};

use super::{data::Data, pagination::ResponsePagination};

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ListResponse<T> {
    /// 封装的响应数据
    pub data: Data<T>,

    /// 可选的分页信息
    pub pagination: Option<ResponsePagination>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMsg(pub String);
