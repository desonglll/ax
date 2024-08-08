pub mod response;
pub mod session;

pub mod routes {
    pub mod api;
    pub mod file;
    pub mod user;
}

pub mod handlers {
    pub mod file;
    pub mod user;
}
#[macro_use]
extern crate lazy_static;
