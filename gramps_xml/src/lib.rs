//! Rust library to work with Gramps XML files. Implements `GRAMPS XML format 1.7.2`.
//!
//! > ⚠️ Note that this library is very experimental ⚠️
//! > 
//! > Also the Rust types are pretty weird, because they reflect the XML data structure.
//!
//! ```
#![doc = include_str!("../examples/manipulate.rs")]
//! ```

pub mod types;

use crate::types::Database;

pub fn from_str(s: &str) -> Result<Database, String> {
    serde_xml_rs::from_str(s).map_err(|e| e.to_string())
}

pub fn to_string(db: Database) -> Result<String, String> {
    serde_xml_rs::to_string(&db).map_err(|e| e.to_string())
}
