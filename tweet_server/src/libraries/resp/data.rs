use serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct Data<T> {
    pub data: Option<T>,
}

impl<T> Data<T> {
    pub fn new(data: Option<T>) -> Self {
        Self { data }
    }
}
