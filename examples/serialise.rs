use gramps_rs::types::{
    Database,
    people::{People, Person},
};

fn main() {
    let mut db = Database::default();

    db.people = Some(People {
        person: Some(vec![Person {
            gender: "U".to_string(),
            ..Default::default()
        }]),
        ..Default::default()
    });

    let s = gramps_rs::to_string(db).unwrap();
    println!("{s}");
}
