mod types;

use crate::types::Database;

fn main() {
    let s = include_str!("../example.gramps");

    let db = serde_xml_rs::from_str::<Database>(s).unwrap();

    dbg!(db);
}
