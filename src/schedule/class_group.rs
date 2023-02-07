#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

/// A class group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassGroup {
    /// The id of the class group.
    pub id: u32,

    /// The name of the class group.
    pub name: String,
}

impl ClassGroup {
    /// Create a new class group.
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }
}