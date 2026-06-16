use serde::{Deserialize, Serialize};

/// Wrapper structure for error messages.
///
/// This structure encapsulates an error message string to facilitate serialization
/// into JSON response bodies.
#[derive(Serialize, Deserialize)]
pub struct ErrorMsg(pub String);
