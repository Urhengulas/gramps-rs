use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    pub bookmarks: Option<Bookmarks>,
    pub citations: Option<Citations>,
    pub events: Option<Events>,
    pub families: Option<Families>,
    pub header: Header,
    pub namemaps: Option<NameMaps>,
    #[serde(rename = "name-formats")]
    pub name_formats: Option<NameFormats>,
    pub notes: Option<Notes>,
    pub objects: Option<Objects>,
    pub people: Option<People>,
    pub places: Option<Places>,
    pub repositories: Option<Repositories>,
    pub sources: Option<Sources>,
    pub tags: Option<Tags>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmarks {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Citations {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Events {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Families {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Header {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NameFormats {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NameMaps {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Notes {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Objects {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct People {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Places {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repositories {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sources {
    // todo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tags {
    // todo
}
