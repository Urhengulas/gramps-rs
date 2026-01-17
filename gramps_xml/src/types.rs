use serde::{Deserialize, Serialize};

// <!ELEMENT database (header, name-formats?, tags?, events?, people?, families?,
//                     citations?, sources?, places?, objects?, repositories?,
//                     notes?, bookmarks?, namemaps?)>
// <!ATTLIST database xmlns CDATA #FIXED "http://gramps-project.org/xml/1.7.2/">
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Database {
    // attributes
    //
    // pub xmlns: String, // Does not work even though present in XML

    // child tags
    //
    pub bookmarks: Option<bookmarks::Bookmarks>,
    pub citations: Option<citations::Citations>,
    pub events: Option<events::Events>,
    pub families: Option<families::Families>,
    pub header: header::Header,
    pub namemaps: Option<namemaps::NameMaps>,
    #[serde(rename = "name-formats")]
    pub name_formats: Option<name_formats::NameFormats>,
    pub notes: Option<notes::Notes>,
    pub objects: Option<objects::Objects>,
    pub people: Option<people::People>,
    pub places: Option<places::Places>,
    pub repositories: Option<repositories::Repositories>,
    pub sources: Option<sources::Sources>,
    pub tags: Option<tags::Tags>,
}

pub mod bookmarks {
    use super::*;

    // <!ELEMENT bookmarks (bookmark)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Bookmarks {
        pub bookmark: Option<Vec<Citation>>,
    }

    // <!ELEMENT bookmark EMPTY>
    // <!ATTLIST bookmark
    //         target (person|family|event|source|citation|place|media|repository|
    //                 note) #REQUIRED
    //         hlink  IDREF #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Citation {
        // attributes
        //
        #[serde(rename = "@target")]
        pub target: String,
        #[serde(rename = "@hlink")]
        pub hlink: String,
    }
}

pub mod citations {
    use super::*;

    // <!ELEMENT citations (citation)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Citations {
        pub citation: Option<Vec<Citation>>,
    }

    // <!ELEMENT citation ((daterange|datespan|dateval|datestr)?, page?, confidence,
    //                     noteref*, objref*, srcattribute*, sourceref, tagref*)>
    // <!ATTLIST citation
    //         id        CDATA #IMPLIED
    //         handle    ID    #REQUIRED
    //         priv      (0|1) #IMPLIED
    //         change    CDATA #REQUIRED
    // >
    //
    // <!ELEMENT page (#PCDATA)>
    // <!ELEMENT confidence (#PCDATA)>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Citation {
        // attributes
        //
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "@handle")]
        pub handle: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@change")]
        pub change: String,

        // child tags
        //
        pub page: Option<String>,
        pub confidence: String,
        pub noteref: Option<Vec<shared::HLink>>,
        pub objref: Option<Vec<shared::ObjRef>>,
        pub srcattribute: Option<Vec<shared::Attribute>>,
        pub sourceref: shared::HLink,
        pub tagref: Option<Vec<shared::HLink>>,

        // FIXME: this should be an enum
        pub daterange: Option<date::DateRange>,
        pub datespan: Option<date::DateSpan>,
        pub dateval: Option<date::DateVal>,
        pub datestr: Option<date::DateStr>,
    }
}

pub mod events {
    use super::*;

    // <!ELEMENT events (event)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Events {
        pub event: Option<Vec<Event>>,
    }

    // <!ELEMENT event (type?, (daterange|datespan|dateval|datestr)?, place?, cause?,
    //                  description?, attribute*, noteref*, citationref*, objref*,
    //                  tagref*)>
    // <!ATTLIST event
    //         id        CDATA #IMPLIED
    //         handle    ID    #REQUIRED
    //         priv      (0|1) #IMPLIED
    //         change    CDATA #REQUIRED
    // >
    //
    // <!ELEMENT type (#PCDATA)>
    // <!ELEMENT cause (#PCDATA)>
    // <!ELEMENT description (#PCDATA)>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Event {
        // attributes
        //
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "@handle")]
        pub handle: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@change")]
        pub change: String,

        // child tags
        //
        #[serde(rename = "type")]
        pub type_: Option<String>,
        pub place: Option<shared::HLink>,
        pub cause: Option<String>,
        pub description: Option<String>,
        pub attribute: Option<Vec<shared::Attribute>>,
        pub noteref: Option<Vec<shared::HLink>>,
        pub citationref: Option<Vec<shared::HLink>>,
        pub objref: Option<Vec<shared::ObjRef>>,
        pub tagref: Option<Vec<shared::HLink>>,

        // FIXME: this should be an enum
        pub daterange: Option<date::DateRange>,
        pub datespan: Option<date::DateSpan>,
        pub dateval: Option<date::DateVal>,
        pub datestr: Option<date::DateStr>,
    }
}

pub mod families {
    use super::*;

    // <!ELEMENT families (family)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Families {
        pub family: Option<Vec<Family>>,
    }

    // <!ELEMENT family (rel?, father?, mother?, eventref*, lds_ord*, objref*,
    //                   childref*, attribute*, noteref*, citationref*, tagref*)>
    // <!ATTLIST family
    //         id        CDATA #IMPLIED
    //         handle    ID    #REQUIRED
    //         priv      (0|1) #IMPLIED
    //         change    CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Family {
        // attributes
        //
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "@handle")]
        pub handle: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@change")]
        pub change: String,

        // child tags
        //
        pub rel: Option<Rel>,
        pub father: Option<shared::HLink>,
        pub mother: Option<shared::HLink>,
        pub eventref: Option<Vec<shared::EventRef>>,
        pub lds_ord: Option<Vec<lds::LdsOrd>>,
        pub objref: Option<Vec<shared::HLink>>,
        pub childref: Option<Vec<Childref>>,
        pub attribute: Option<Vec<shared::Attribute>>,
        pub noteref: Option<Vec<shared::HLink>>,
        pub citationref: Option<Vec<shared::HLink>>,
        pub tagref: Option<Vec<shared::HLink>>,
    }

    // <!ELEMENT rel EMPTY>
    // <!ATTLIST rel type CDATA #REQUIRED>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Rel {
        #[serde(rename = "@type")]
        pub type_: String,
    }

    // <!ELEMENT childref (citationref*,noteref*)>
    // <!ATTLIST childref
    //         hlink IDREF #REQUIRED
    //         priv  (0|1) #IMPLIED
    //         mrel  CDATA #IMPLIED
    //         frel  CDATA #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Childref {
        // attributes
        //
        #[serde(rename = "@hlink")]
        hlink: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@mrel")]
        pub mrel: Option<String>,
        #[serde(rename = "@frel")]
        pub frel: Option<String>,

        // child tags
        //
        pub citationref: Option<Vec<shared::HLink>>,
        pub noteref: Option<Vec<shared::HLink>>,
    }
}

pub mod header {
    use super::*;

    // <!ELEMENT header (created, researcher?, mediapath?)>
    // <!ELEMENT mediapath  (#PCDATA)>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Header {
        pub created: Created,
        pub researcher: Option<Researcher>,
        pub mediapath: Option<String>,
    }

    // <!ELEMENT created EMPTY>
    // <!ATTLIST created
    //         date     CDATA #REQUIRED
    //         version  CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Created {
        #[serde(rename = "@date")]
        pub date: String,
        #[serde(rename = "@version")]
        pub version: String,
    }

    // <!ELEMENT researcher (resname?, resaddr?, reslocality?, rescity?, resstate?,
    //                       rescountry?, respostal?, resphone?, resemail?)>
    // <!ELEMENT resname        (#PCDATA)>
    // <!ELEMENT resaddr        (#PCDATA)>
    // <!ELEMENT reslocality    (#PCDATA)>
    // <!ELEMENT rescity        (#PCDATA)>
    // <!ELEMENT resstate       (#PCDATA)>
    // <!ELEMENT rescountry     (#PCDATA)>
    // <!ELEMENT respostal      (#PCDATA)>
    // <!ELEMENT resphone       (#PCDATA)>
    // <!ELEMENT resemail       (#PCDATA)>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Researcher {
        pub resname: Option<String>,
        pub resaddr: Option<String>,
        pub reslocality: Option<String>,
        pub rescity: Option<String>,
        pub resstate: Option<String>,
        pub rescountry: Option<String>,
        pub respostal: Option<String>,
        pub resphone: Option<String>,
        pub resemail: Option<String>,
    }
}

pub mod name_formats {
    use super::*;

    // <!ELEMENT name-formats (format)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NameFormats {
        pub format: Option<Vec<Format>>,
    }

    // <!ELEMENT format EMPTY>
    // <!ATTLIST format
    //         number  CDATA #REQUIRED
    //         name    CDATA #REQUIRED
    //         fmt_str CDATA #REQUIRED
    //         active  (0|1) #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Format {
        #[serde(rename = "@number")]
        pub number: String,
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "@fmt_str")]
        pub fmt_str: String,
        #[serde(rename = "@active")]
        pub active: Option<bool>,
    }
}

pub mod namemaps {
    use super::*;

    // <!ELEMENT namemaps (map)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NameMaps {
        pub map: Option<Vec<Map>>,
    }

    // <!ELEMENT map EMPTY>
    // <!ATTLIST map
    //         type  CDATA #REQUIRED
    //         key   CDATA #REQUIRED
    //         value CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Map {
        #[serde(rename = "@type")]
        pub type_: String,
        #[serde(rename = "@key")]
        pub key: String,
        #[serde(rename = "@value")]
        pub value: String,
    }
}

pub mod notes {
    use super::*;

    // <!ELEMENT notes (note)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Notes {
        pub note: Option<Vec<Note>>,
    }

    // <!ELEMENT note (text, style*, tagref*)>
    // <!ATTLIST note
    //         id        CDATA #IMPLIED
    //         handle    ID    #REQUIRED
    //         priv      (0|1) #IMPLIED
    //         change    CDATA #REQUIRED
    //         format    (0|1) #IMPLIED
    //         type      CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Note {
        // attributes
        //
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "@handle")]
        pub handle: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@change")]
        pub change: String,
        #[serde(rename = "@format")]
        pub format: Option<bool>,
        #[serde(rename = "@type")]
        pub type_: String,

        // child tags
        //
        pub text: String,
        pub style: Option<Vec<Style>>,
        pub tagref: Option<Vec<shared::HLink>>,
    }

    // <!ELEMENT style (range+)>
    // <!ATTLIST style
    //         name    (bold|italic|underline|fontface|fontsize|
    //                 fontcolor|highlight|superscript|link) #REQUIRED
    //         value   CDATA #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Style {
        // attributes
        //
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "@value")]
        pub value: Option<String>,

        // child tags
        //
        range: Vec<Range>,
    }

    // <!ELEMENT range EMPTY>
    // <!ATTLIST range
    //         start   CDATA #REQUIRED
    //         end     CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Range {
        #[serde(rename = "@start")]
        pub start: String,
        #[serde(rename = "@end")]
        pub end: String,
    }
}

pub mod objects {
    use super::*;

    // <!ELEMENT objects (object)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Objects {
        object: Option<Vec<Object>>,
    }

    // <!ELEMENT object (file, attribute*, noteref*,
    //                  (daterange|datespan|dateval|datestr)?, citationref*, tagref*)>
    // <!ATTLIST object
    //         id        CDATA #IMPLIED
    //         handle    ID    #REQUIRED
    //         priv      (0|1) #IMPLIED
    //         change    CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Object {
        // attributes
        //
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "@handle")]
        pub handle: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@change")]
        pub change: String,

        // child tags
        //
        pub file: File,
        pub attribute: Option<Vec<shared::Attribute>>,
        pub noteref: Option<Vec<shared::HLink>>,
        pub citationref: Option<Vec<shared::HLink>>,
        pub tagref: Option<Vec<shared::HLink>>,

        // FIXME: this should be an enum
        pub daterange: Option<date::DateRange>,
        pub datespan: Option<date::DateSpan>,
        pub dateval: Option<date::DateVal>,
        pub datestr: Option<date::DateStr>,
    }

    // <!ELEMENT file EMPTY>
    // <!ATTLIST file
    //         src         CDATA #REQUIRED
    //         mime        CDATA #REQUIRED
    //         checksum    CDATA #IMPLIED
    //         description CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct File {
        #[serde(rename = "@src")]
        pub src: String,
        #[serde(rename = "@mime")]
        pub mime: String,
        #[serde(rename = "@checksum")]
        pub checksum: Option<String>,
        #[serde(rename = "@description")]
        pub description: String,
    }
}

pub mod people {
    use super::*;

    // <!ELEMENT people (person)*>
    // <!ATTLIST people
    //         default CDATA #IMPLIED
    //         home    IDREF #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct People {
        // attributes
        //
        #[serde(rename = "@default")]
        pub default: Option<String>,
        #[serde(rename = "@home")]
        pub home: Option<String>,

        // child tags
        //
        pub person: Option<Vec<Person>>,
    }

    // <!ELEMENT person (gender, name*, eventref*, lds_ord*,
    //                   objref*, address*, attribute*, url*, childof*,
    //                   parentin*, personref*, noteref*, citationref*, tagref*)>
    // <!ATTLIST person
    //         id        CDATA #IMPLIED
    //         handle    ID    #REQUIRED
    //         priv      (0|1) #IMPLIED
    //         change    CDATA #REQUIRED
    // >
    //
    // <!ELEMENT gender  (#PCDATA)>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Person {
        // attributes
        //
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "@handle")]
        pub handle: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@change")]
        pub change: String,

        // child tags
        //
        pub gender: String,
        pub name: Option<Vec<Name>>,
        pub eventref: Option<Vec<shared::EventRef>>,
        pub lds_ord: Option<Vec<lds::LdsOrd>>,
        pub objref: Option<Vec<shared::ObjRef>>,
        pub address: Option<Vec<Address>>,
        pub attribute: Option<Vec<shared::Attribute>>,
        pub url: Option<Vec<shared::Url>>,
        pub childof: Option<Vec<shared::HLink>>,
        pub parentin: Option<Vec<shared::HLink>>,
        pub personref: Option<Vec<PersonRef>>,
        pub noteref: Option<Vec<shared::HLink>>,
        pub citationref: Option<Vec<shared::HLink>>,
        pub tagref: Option<Vec<shared::HLink>>,
    }

    // <!ELEMENT name    (first?, call?, surname*, suffix?, title?, nick?, familynick?, group?,
    //                   (daterange|datespan|dateval|datestr)?, noteref*, citationref*)>
    // <!ATTLIST name
    //         alt       (0|1) #IMPLIED
    //         type      CDATA #IMPLIED
    //         priv      (0|1) #IMPLIED
    //         sort      CDATA #IMPLIED
    //         display   CDATA #IMPLIED
    // >
    //
    // <!ELEMENT first      (#PCDATA)>
    // <!ELEMENT call       (#PCDATA)>
    // <!ELEMENT suffix     (#PCDATA)>
    // <!ELEMENT title      (#PCDATA)>
    // <!ELEMENT nick       (#PCDATA)>
    // <!ELEMENT familynick (#PCDATA)>
    // <!ELEMENT group      (#PCDATA)>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Name {
        // attributes
        //
        #[serde(rename = "@alt")]
        pub alt: Option<bool>,
        #[serde(rename = "@type")]
        pub type_: Option<String>,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@sort")]
        pub sort: Option<String>,
        #[serde(rename = "@display")]
        pub display: Option<String>,

        // child tags
        //
        pub first: Option<String>,
        pub call: Option<String>,
        pub surname: Option<Vec<Surname>>,
        pub suffix: Option<String>,
        pub title: Option<String>,
        pub nick: Option<String>,
        pub familynick: Option<String>,
        pub group: Option<String>,
        pub noteref: Option<Vec<shared::HLink>>,
        pub citationref: Option<Vec<shared::HLink>>,

        // FIXME: this should be an enum
        pub daterange: Option<date::DateRange>,
        pub datespan: Option<date::DateSpan>,
        pub dateval: Option<date::DateVal>,
        pub datestr: Option<date::DateStr>,
    }

    // <!ELEMENT surname    (#PCDATA)>
    // <!ATTLIST surname
    //         prefix      CDATA #IMPLIED
    //         prim        (1|0) #IMPLIED
    //         derivation  CDATA #IMPLIED
    //         connector   CDATA #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Surname {
        // attributes
        //
        #[serde(rename = "@prefix")]
        pub prefix: Option<String>,
        #[serde(rename = "@prim")]
        pub prim: Option<bool>,
        #[serde(rename = "@derivation")]
        pub derivation: Option<String>,
        #[serde(rename = "@connector")]
        pub connector: Option<String>,

        // text content
        //
        #[serde(rename = "#text")]
        pub text: Option<String>,
    }

    // <!ELEMENT address ((daterange|datespan|dateval|datestr)?, street?,
    //                    locality?, city?, county?, state?, country?, postal?,
    //                    phone?, noteref*, citationref*)>
    // <!ATTLIST address priv (0|1) #IMPLIED>

    // <!ELEMENT street   (#PCDATA)>
    // <!ELEMENT locality (#PCDATA)>
    // <!ELEMENT city     (#PCDATA)>
    // <!ELEMENT county   (#PCDATA)>
    // <!ELEMENT state    (#PCDATA)>
    // <!ELEMENT country  (#PCDATA)>
    // <!ELEMENT postal   (#PCDATA)>
    // <!ELEMENT phone    (#PCDATA)>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Address {
        // attributes
        //
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,

        // child tags
        //
        pub street: Option<String>,
        pub locality: Option<String>,
        pub city: Option<String>,
        pub county: Option<String>,
        pub state: Option<String>,
        pub country: Option<String>,
        pub postal: Option<String>,
        pub phone: Option<String>,
        pub noteref: Option<Vec<shared::HLink>>,
        pub citationref: Option<Vec<shared::HLink>>,

        // FIXME: this should be an enum
        pub daterange: Option<date::DateRange>,
        pub datespan: Option<date::DateSpan>,
        pub dateval: Option<date::DateVal>,
        pub datestr: Option<date::DateStr>,
    }

    // <!ELEMENT personref (citationref*, noteref*)>
    // <!ATTLIST personref
    //         hlink IDREF #REQUIRED
    //         priv  (0|1) #IMPLIED
    //         rel   CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PersonRef {
        // attributes
        //
        #[serde(rename = "@hlink")]
        pub hlink: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@rel")]
        pub rel: String,

        // child tags
        //
        pub citationref: Option<Vec<shared::HLink>>,
        pub noteref: Option<Vec<shared::HLink>>,
    }
}

pub mod places {
    use super::*;

    // <!ELEMENT places (placeobj)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Places {
        pub placeobj: Option<Vec<PlaceObj>>,
    }

    // <!ELEMENT placeobj (ptitle?, code?, pname+, coord?, placeref*, location*,
    //                     objref*, url*, noteref*, citationref*, tagref*)>
    // <!ATTLIST placeobj
    //         id        CDATA #IMPLIED
    //         handle    ID    #REQUIRED
    //         priv      (0|1) #IMPLIED
    //         change    CDATA #REQUIRED
    //         type      CDATA #REQUIRED
    // >
    //
    // <!ELEMENT ptitle (#PCDATA)>
    // <!ELEMENT code (#PCDATA)>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PlaceObj {
        // attributes
        //
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "@handle")]
        pub handle: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@change")]
        pub change: String,
        #[serde(rename = "@type")]
        pub type_: String,

        // child tags
        //
        pub ptitle: Option<String>,
        pub code: Option<String>,
        pub pname: Vec<PName>,
        pub coord: Option<Coord>,
        pub placeref: Option<Vec<shared::PlaceRef>>,
        pub location: Option<Vec<Location>>,
        pub objref: Option<Vec<shared::ObjRef>>,
        pub url: Option<Vec<shared::Url>>,
        pub noteref: Option<Vec<shared::HLink>>,
        pub citationref: Option<Vec<shared::HLink>>,
        pub tagref: Option<Vec<shared::HLink>>,
    }

    // <!ELEMENT pname (daterange|datespan|dateval|datestr)?>
    // <!ATTLIST pname
    //         lang CDATA #IMPLIED
    //         value CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PName {
        // attributes
        //
        #[serde(rename = "@lang")]
        pub lang: Option<String>,
        #[serde(rename = "@value")]
        pub value: String,

        // child tags
        //

        // FIXME: this should be an enum
        pub daterange: Option<date::DateRange>,
        pub datespan: Option<date::DateSpan>,
        pub dateval: Option<date::DateVal>,
        pub datestr: Option<date::DateStr>,
    }

    // <!ELEMENT coord EMPTY>
    // <!ATTLIST coord
    //         long CDATA #REQUIRED
    //         lat  CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Coord {
        #[serde(rename = "@long")]
        pub long: String,
        #[serde(rename = "@lat")]
        pub lat: String,
    }

    // <!ELEMENT location EMPTY>
    // <!ATTLIST location
    //         street   CDATA #IMPLIED
    //         locality CDATA #IMPLIED
    //         city     CDATA #IMPLIED
    //         parish   CDATA #IMPLIED
    //         county   CDATA #IMPLIED
    //         state    CDATA #IMPLIED
    //         country  CDATA #IMPLIED
    //         postal   CDATA #IMPLIED
    //         phone    CDATA #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(rename = "@street")]
        pub street: Option<String>,
        #[serde(rename = "@locality")]
        pub locality: Option<String>,
        #[serde(rename = "@city")]
        pub city: Option<String>,
        #[serde(rename = "@parish")]
        pub parish: Option<String>,
        #[serde(rename = "@county")]
        pub county: Option<String>,
        #[serde(rename = "@state")]
        pub state: Option<String>,
        #[serde(rename = "@country")]
        pub country: Option<String>,
        #[serde(rename = "@postal")]
        pub postal: Option<String>,
        #[serde(rename = "@phone")]
        pub phone: Option<String>,
    }
}

pub mod repositories {

    use super::*;

    // <!ELEMENT repositories (repository)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Repositories {
        pub repository: Option<Vec<Repository>>,
    }

    // <!ELEMENT repository (rname, type, address*, url*, noteref*, tagref*)>
    // <!ATTLIST repository
    //         id        CDATA #IMPLIED
    //         handle    ID    #REQUIRED
    //         priv      (0|1) #IMPLIED
    //         change    CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Repository {
        // attributes
        //
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "@handle")]
        pub handle: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@change")]
        pub change: String,

        // child tags
        //
        pub rname: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub address: Option<Vec<people::Address>>,
        pub url: Option<Vec<shared::Url>>,
        pub noteref: Option<Vec<shared::HLink>>,
        pub tagref: Option<Vec<shared::HLink>>,
    }
}

pub mod sources {
    use super::*;

    // <!ELEMENT sources (source)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Sources {
        pub source: Option<Vec<Source>>,
    }

    // <!ELEMENT source (stitle?, sauthor?, spubinfo?, sabbrev?,
    //                   noteref*, objref*, srcattribute*, reporef*, tagref*)>
    // <!ATTLIST source
    //         id        CDATA #IMPLIED
    //         handle    ID    #REQUIRED
    //         priv      (0|1) #IMPLIED
    //         change    CDATA #REQUIRED
    // >
    // <!ELEMENT stitle   (#PCDATA)>
    // <!ELEMENT sauthor  (#PCDATA)>
    // <!ELEMENT spubinfo (#PCDATA)>
    // <!ELEMENT sabbrev  (#PCDATA)>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Source {
        // attributes
        //
        #[serde(rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "@handle")]
        pub handle: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@change")]
        pub change: String,

        // child tags
        //
        pub stitle: Option<String>,
        pub sauthor: Option<String>,
        pub spubinfo: Option<String>,
        pub sabbrev: Option<String>,
        pub noteref: Option<Vec<shared::HLink>>,
        pub objref: Option<Vec<shared::ObjRef>>,
        pub srcattribute: Option<Vec<shared::Attribute>>,
        pub reporef: Option<Vec<shared::RepoRef>>,
        pub tagref: Option<Vec<shared::HLink>>,
    }
}

pub mod tags {
    use super::*;

    // <!ELEMENT tags (tag)*>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Tags {
        pub tag: Option<Vec<Tag>>,
    }

    // <!ELEMENT tag EMPTY>
    // <!ATTLIST tag
    //         handle    ID    #REQUIRED
    //         name      CDATA #REQUIRED
    //         color     CDATA #REQUIRED
    //         priority  CDATA #REQUIRED
    //         change    CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Tag {
        // attributes
        //
        #[serde(rename = "@handle")]
        pub handle: String,
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "@color")]
        pub color: String,
        #[serde(rename = "@priority")]
        pub priority: String,
        #[serde(rename = "@change")]
        pub change: String,
    }
}

pub mod shared {
    use super::*;

    // <!ELEMENT attribute (citationref*, noteref*)>
    // <!ATTLIST attribute
    //         priv    (0|1)   #IMPLIED
    //         type    CDATA   #REQUIRED
    //         value   CDATA   #REQUIRED
    // >
    //
    // <!ELEMENT srcattribute EMPTY>
    // <!ATTLIST srcattribute
    //         priv    (0|1)   #IMPLIED
    //         type    CDATA   #REQUIRED
    //         value   CDATA   #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Attribute {
        // attributes
        //
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@type")]
        pub type_: String,
        #[serde(rename = "@value")]
        pub value: String,

        // child tags
        //
        pub citationref: Option<Vec<HLink>>,
        pub noteref: Option<Vec<HLink>>,
    }

    // <!ELEMENT childof EMPTY>
    // <!ATTLIST childof hlink IDREF  #REQUIRED>
    //
    // <!ELEMENT parentin EMPTY>
    // <!ATTLIST parentin hlink IDREF #REQUIRED>
    //
    // <!ELEMENT father EMPTY>
    // <!ATTLIST father hlink IDREF #REQUIRED>
    //
    // <!ELEMENT mother EMPTY>
    // <!ATTLIST mother hlink IDREF #REQUIRED>
    //
    // <!ELEMENT citationref EMPTY>
    // <!ATTLIST citationref hlink IDREF #REQUIRED>
    //
    // <!ELEMENT sourceref EMPTY>
    // <!ATTLIST sourceref hlink IDREF #REQUIRED>
    //
    // <!ELEMENT noteref EMPTY>
    // <!ATTLIST noteref hlink IDREF #REQUIRED>
    //
    // <!ELEMENT tagref EMPTY>
    // <!ATTLIST tagref hlink IDREF #REQUIRED>
    //
    // <!ELEMENT place EMPTY>
    // <!ATTLIST place hlink IDREF #REQUIRED>
    //
    // <!ELEMENT sealed_to EMPTY>
    // <!ATTLIST sealed_to hlink IDREF #REQUIRED>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HLink {
        #[serde(rename = "@hlink")]
        pub hlink: String,
    }

    // <!ELEMENT eventref (attribute*, noteref*, citationref*)>
    // <!ATTLIST eventref
    //         hlink IDREF #REQUIRED
    //         priv  (0|1) #IMPLIED
    //         role  CDATA #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EventRef {
        // attributes
        //
        #[serde(rename = "@hlink")]
        pub hlink: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@role")]
        pub role: Option<String>,

        // child tags
        //
        pub attribute: Option<Vec<Attribute>>,
        pub noteref: Option<Vec<HLink>>,
        pub citationref: Option<Vec<HLink>>,
    }

    // <!ELEMENT reporef (noteref*)>
    // <!ATTLIST reporef
    //         hlink  IDREF #REQUIRED
    //         priv   (0|1) #IMPLIED
    //         callno CDATA #IMPLIED
    //         medium CDATA #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RepoRef {
        // attributes
        //
        #[serde(rename = "@hlink")]
        pub hlink: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@callno")]
        pub callno: Option<String>,
        #[serde(rename = "@medium")]
        pub medium: Option<String>,

        // child tags
        //
        pub noteref: Option<Vec<HLink>>,
    }

    // <!ELEMENT objref (region?, attribute*, citationref*, noteref*)>
    // <!ATTLIST objref
    //         hlink IDREF #REQUIRED
    //         priv (0|1)  #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ObjRef {
        // attributes
        //
        #[serde(rename = "@hlink")]
        pub hlink: String,
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,

        // child tags
        //
        pub region: Option<Region>,
        pub attribute: Option<Vec<Attribute>>,
        pub citationref: Option<Vec<HLink>>,
        pub noteref: Option<Vec<HLink>>,
    }

    // <!ELEMENT placeref ((daterange|datespan|dateval|datestr)?)>
    // <!ATTLIST placeref hlink IDREF #REQUIRED >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PlaceRef {
        // attributes
        //
        #[serde(rename = "@hlink")]
        pub hlink: String,

        // child tags
        //

        // FIXME: this should be an enum
        pub daterange: Option<date::DateRange>,
        pub datespan: Option<date::DateSpan>,
        pub dateval: Option<date::DateVal>,
        pub datestr: Option<date::DateStr>,
    }

    // <!ELEMENT region EMPTY>
    // <!ATTLIST region
    //         corner1_x  CDATA #REQUIRED
    //         corner1_y  CDATA #REQUIRED
    //         corner2_x  CDATA #REQUIRED
    //         corner2_y  CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Region {
        #[serde(rename = "@corner1_x")]
        pub corner1_x: String,
        #[serde(rename = "@corner1_y")]
        pub corner1_y: String,
        #[serde(rename = "@corner2_x")]
        pub corner2_x: String,
        #[serde(rename = "@corner2_y")]
        pub corner2_y: String,
    }

    // <!ELEMENT url EMPTY>
    // <!ATTLIST url
    //         priv        (0|1) #IMPLIED
    //         type        CDATA #IMPLIED
    //         href        CDATA #REQUIRED
    //         description CDATA #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Url {
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@type")]
        pub type_: Option<String>,
        #[serde(rename = "@href")]
        pub href: String,
        #[serde(rename = "@description")]
        pub description: Option<String>,
    }
}

pub mod date {
    use super::*;

    // <!ELEMENT daterange EMPTY>
    // <!ATTLIST daterange
    //         start     CDATA                  #REQUIRED
    //         stop      CDATA                  #REQUIRED
    //         quality   (estimated|calculated) #IMPLIED
    //         cformat   CDATA                  #IMPLIED
    //         dualdated (0|1)                  #IMPLIED
    //         newyear   CDATA                  #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DateRange {
        #[serde(rename = "@start")]
        pub start: String,
        #[serde(rename = "@stop")]
        pub stop: String,

        #[serde(rename = "@quality")]
        pub quality: Option<String>,
        #[serde(rename = "@cformat")]
        pub cformat: Option<String>,
        #[serde(rename = "@dualdated")]
        pub dualdated: Option<bool>,
        #[serde(rename = "@newyear")]
        pub newyear: Option<String>,
    }

    // <!ELEMENT datespan EMPTY>
    // <!ATTLIST datespan
    //         start     CDATA                  #REQUIRED
    //         stop      CDATA                  #REQUIRED
    //         quality   (estimated|calculated) #IMPLIED
    //         cformat   CDATA                  #IMPLIED
    //         dualdated (0|1)                  #IMPLIED
    //         newyear   CDATA                  #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DateSpan {
        #[serde(rename = "@start")]
        pub start: String,
        #[serde(rename = "@stop")]
        pub stop: String,

        #[serde(rename = "@quality")]
        pub quality: Option<String>,
        #[serde(rename = "@cformat")]
        pub cformat: Option<String>,
        #[serde(rename = "@dualdated")]
        pub dualdated: Option<bool>,
        #[serde(rename = "@newyear")]
        pub newyear: Option<String>,
    }

    // <!ELEMENT dateval EMPTY>
    // <!ATTLIST dateval
    //         val       CDATA                  #REQUIRED
    //         type      (before|after|about)   #IMPLIED
    //         quality   (estimated|calculated) #IMPLIED
    //         cformat   CDATA                  #IMPLIED
    //         dualdated (0|1)                  #IMPLIED
    //         newyear   CDATA                  #IMPLIED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DateVal {
        #[serde(rename = "@val")]
        pub val: String,
        #[serde(rename = "@type")]
        pub type_: Option<String>,

        #[serde(rename = "@quality")]
        pub quality: Option<String>,
        #[serde(rename = "@cformat")]
        pub cformat: Option<String>,
        #[serde(rename = "@dualdated")]
        pub dualdated: Option<bool>,
        #[serde(rename = "@newyear")]
        pub newyear: Option<String>,
    }

    // <!ELEMENT datestr EMPTY>
    // <!ATTLIST datestr val CDATA #REQUIRED>
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DateStr {
        #[serde(rename = "@val")]
        pub val: String,
    }
}

pub mod lds {
    use super::*;

    // <!ELEMENT lds_ord ((daterange|datespan|dateval|datestr)?, temple?, place?,
    //                    status?, sealed_to?, noteref*, citationref*)>
    // <!ATTLIST lds_ord
    //         priv   (0|1) #IMPLIED
    //         type   CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LdsOrd {
        // attributes
        //
        #[serde(rename = "@priv")]
        pub priv_: Option<bool>,
        #[serde(rename = "@type")]
        pub type_: String,

        // child tags
        //
        pub temple: Option<Val>,
        pub place: Option<shared::HLink>,
        pub status: Option<Val>,
        pub sealed_to: Option<shared::HLink>,
        pub noteref: Option<Vec<shared::HLink>>,
        pub citationref: Option<Vec<shared::HLink>>,

        // FIXME: this should be an enum
        pub daterange: Option<date::DateRange>,
        pub datespan: Option<date::DateSpan>,
        pub dateval: Option<date::DateVal>,
        pub datestr: Option<date::DateStr>,
    }

    // <!ELEMENT temple EMPTY>
    // <!ATTLIST temple
    //         val   CDATA #REQUIRED
    // >
    //
    // <!ELEMENT status EMPTY>
    // <!ATTLIST status
    //         val   CDATA #REQUIRED
    // >
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Val {
        #[serde(rename = "@val")]
        pub val: String,
    }
}
