fn main() {
    let s = include_str!("../example.gramps");
    let db = gramps_xml::from_str(s).unwrap();
    dbg!(db);
}
