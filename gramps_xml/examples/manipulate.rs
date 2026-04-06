use gramps_xml::types::{
    Database,
    people::{Name, People, Person, Surname},
};

fn main() {
    // Read a Gramp XML file
    let s = include_str!("../example.gramps");
    let mut db: Database = gramps_xml::from_str(s).unwrap();

    // Manipulate it
    let mut people: People = db.people.unwrap_or_default();
    let mut persons: Vec<Person> = people.person.unwrap_or_default();

    let new_person = Person {
        name: Some(vec![Name {
            first: Some("Greta".into()),
            surname: Some(vec![Surname {
                text: Some("Thunberg".into()),
                ..Default::default()
            }]),
            ..Default::default()
        }]),
        ..Default::default()
    };
    persons.push(new_person);

    people.person = Some(persons);
    db.people = Some(people);

    // Write as Gramp XML
    let s = gramps_xml::to_string(db).unwrap();
    println!("{s}");
}
