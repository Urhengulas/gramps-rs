# gramps-xml

Rust library to work with Gramps XML files.

Implemented according to `GRAMPS XML format 1.7.2` (See [Document Type definition](https://github.com/gramps-project/gramps/blob/ffcaf4b1b3099e7f384118007479eb57a7fff21f/data/grampsxml.dtd)).

## Learnings about translating dtd file to Rust types

- `CDATA` is `String`
- `(0|1)` is `bool`
- `T?` is zero or one `T`, `T*` is zero or more `T`s, `T+` is 1 or more `T`s and `T` is exactly one `T`
- `ELEMENT`
    ```text
    <!ELEMENT header (created, researcher?, mediapath*)>
    ```
    That is `struct Header` with fields `created: Created`, `researcher: Option<Researcher>` and `mediapath: Option<Vec<MediaPath>>`.
- `ATTLIST`
    ```text
    <!ELEMENT created EMPTY>
    <!ATTLIST created
            date     CDATA #REQUIRED
            version  CDATA #IMPLIED
    >
    ```
    That is `struct Created` with fields `date: String` and `version: Option<String>`. Both have to be serde renamed to start with `@`.
- ` (daterange|datespan|dateval|datestr)?`
    This could be any of those types or none. This should be represented as an enum but it is currently unclear how to.
