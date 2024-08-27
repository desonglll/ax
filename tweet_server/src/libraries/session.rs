use actix_session::Session;

use crate::errors::AxError;

pub fn is_admin(session: Session) -> Result<bool, AxError> {
    let is_admin = session.get::<bool>("is_admin").unwrap();
    Ok(is_admin.unwrap())
}
