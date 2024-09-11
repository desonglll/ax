use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
    pub count: Option<i64>,
}

pub struct PaginationBuilder {
    pub limit: i64,
    pub offset: i64,
    pub count: Option<i64>,
}

impl PaginationBuilder {
    pub fn new(limit: i64, offset: i64) -> PaginationBuilder {
        PaginationBuilder {
            limit,
            offset,
            count: None,
        }
    }
    pub fn set_limit(mut self, limit: i64) -> Self {
        self.limit = limit;
        self
    }
    pub fn set_offset(mut self, offset: i64) -> Self {
        self.offset = offset;
        self
    }
    pub fn set_count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn build(self) -> Pagination {
        Pagination {
            limit: self.limit,
            offset: self.offset,
            count: self.count,
        }
    }
}
