use super::markdown::{write_markdown, Markdown, MarkdownMeta};
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::str;
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SimpleNotes {
    #[serde(rename(deserialize = "activeNotes"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    active_notes: Option<Vec<SimpleNote>>,
    #[serde(rename(deserialize = "trashedNotes"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    trashed_notes: Option<Vec<SimpleNote>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SimpleNote {
    id: String,
    content: String,
    #[serde(rename(deserialize = "creationDate"))]
    creation_date: String,
    #[serde(rename(deserialize = "lastModified"))]
    last_modified: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    markdown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pinned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
}

pub fn process(source_file: PathBuf, _dest_dir: PathBuf) -> Result<(), std::io::Error> {
    println!("Simplenote conversion not yet implemented.");
    println!("Parsing the data works... writing it has not begun.");
    let source_text = load_file(&source_file).unwrap();
    let all_notes = deserialize_notes(source_text).unwrap();

    println!(
        "Simplenote active_notes:{}, trashed_notes:{}.",
        all_notes.active_notes.unwrap().len(),
        all_notes.trashed_notes.unwrap().len()
    );

    let note = Markdown {
        meta: MarkdownMeta {
            title: String::from("A title"),
            created: String::from("2022-01-13T22:36:18.906Z"),
            modified: String::from("2022-01-14T07:36:50.656Z"),
            deleted: None,
            favorited: None,
            pinned: None,
            tags: None,
        },
        content: String::from("This is a\ngreat piece of\nsample content!"),
    };
    write_markdown(note, _dest_dir)
}

fn load_file(source_file: &PathBuf) -> Result<String, std::io::Error> {
    // this function is well guarded by `verify_source`, so we'll assume that IO is not a problem here
    let bytes = fs::read(&source_file)?;
    let text = String::from_utf8(bytes);
    match text {
        Ok(t) => Ok(t),
        Err(f) => {
            eprintln!("Error: {}", f);
            Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "source_file: '{}' contains data which is not UTF8",
                    source_file.to_str().unwrap()
                ),
            ))
        }
    }
}

fn deserialize_notes(source_text: String) -> Result<SimpleNotes, serde_json::Error> {
    match serde_json::from_str(&source_text) {
        Ok(notes) => Ok(notes),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file_success_for_text() {
        let path = PathBuf::from("test_data/happy.txt");
        let text = load_file(&path).unwrap();
        assert_eq!("this is a happy string", format!("{}", text));
    }

    #[test]
    fn load_file_fails_for_non_text() {
        let path = PathBuf::from("test_data/not_text.bin");
        let error = load_file(&path).unwrap_err();
        assert_eq!(ErrorKind::InvalidData, error.kind());
        assert_eq!(
            format!(
                "source_file: '{}' contains data which is not UTF8",
                String::from(path.to_string_lossy())
            ),
            format!("{}", error)
        );
    }

    #[test]
    fn deserialize_success_with_empty_json() {
        let source = r#"
            {}"#;
        let expected = SimpleNotes {
            active_notes: None,
            trashed_notes: None,
        };
        let actual = deserialize_notes(String::from(source)).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_success_with_empty_notes_lists() {
        let source = r#"
            {
                "activeNotes": [],
                "trashedNotes": []
            }"#;
        let expected = SimpleNotes {
            active_notes: Some(Vec::new()),
            trashed_notes: Some(Vec::new()),
        };
        let actual = deserialize_notes(String::from(source)).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_fail_missing_field_id() {
        let source = r#"
            {
                "activeNotes": [
                    {
                        "content": "this is a note",
                        "creationDate": "2022-01-13T22:36:18.906Z",
                        "lastModified": "2022-01-14T07:36:50.656Z"
                    }
                ]
            }"#;
        let single = SimpleNote {
            id: String::from(""),
            content: String::from("this is a note"),
            creation_date: String::from("2022-01-13T22:36:18.906Z"),
            last_modified: String::from("2022-01-14T07:36:50.656Z"),
            markdown: None,
            pinned: None,
            tags: None,
        };
        let _expected = SimpleNotes {
            active_notes: Some(vec![single]),
            trashed_notes: None,
        };
        let error = deserialize_notes(String::from(source)).unwrap_err();
        assert!(format!("{}", error).contains("missing field `id`"));
    }

    #[test]
    fn deserialize_fail_missing_field_content() {
        let source = r#"
            {
                "activeNotes": [
                    {
                        "id": "someid",
                        "creationDate": "2022-01-13T22:36:18.906Z",
                        "lastModified": "2022-01-14T07:36:50.656Z"
                    }
                ]
            }"#;
        let single = SimpleNote {
            id: String::from("someid"),
            content: String::from(""),
            creation_date: String::from("2022-01-13T22:36:18.906Z"),
            last_modified: String::from("2022-01-14T07:36:50.656Z"),
            markdown: None,
            pinned: None,
            tags: None,
        };
        let _expected = SimpleNotes {
            active_notes: Some(vec![single]),
            trashed_notes: None,
        };
        let error = deserialize_notes(String::from(source)).unwrap_err();
        assert!(format!("{}", error).contains("missing field `content`"));
    }
    // with 2 tests verifying that required fields fail deserialization, that's good enough

    #[test]
    fn deserialize_success_only_required() {
        let source = r#"
            {
                "activeNotes": [
                    {
                        "id": "someid",
                        "content": "this is a note",
                        "creationDate": "2022-01-13T22:36:18.906Z",
                        "lastModified": "2022-01-14T07:36:50.656Z"
                    }
                ]
            }"#;
        let single = SimpleNote {
            id: String::from("someid"),
            content: String::from("this is a note"),
            creation_date: String::from("2022-01-13T22:36:18.906Z"),
            last_modified: String::from("2022-01-14T07:36:50.656Z"),
            markdown: None,
            pinned: None,
            tags: None,
        };
        let expected = SimpleNotes {
            active_notes: Some(vec![single]),
            trashed_notes: None,
        };
        let actual = deserialize_notes(String::from(source)).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_success_with_optional() {
        let source = r#"
            {
                "activeNotes": [
                    {
                        "id": "someid",
                        "content": "this is a note",
                        "creationDate": "2022-01-13T22:36:18.906Z",
                        "lastModified": "2022-01-14T07:36:50.656Z",
                        "markdown": true,
                        "pinned": true,
                        "tags": ["Personal","Business"]
                    }
                ]
            }"#;
        let single = SimpleNote {
            id: String::from("someid"),
            content: String::from("this is a note"),
            creation_date: String::from("2022-01-13T22:36:18.906Z"),
            last_modified: String::from("2022-01-14T07:36:50.656Z"),
            markdown: Some(true),
            pinned: Some(true),
            tags: Some(vec![String::from("Personal"), String::from("Business")]),
        };
        let expected = SimpleNotes {
            active_notes: Some(vec![single]),
            trashed_notes: None,
        };
        let actual = deserialize_notes(String::from(source)).unwrap();
        assert_eq!(expected, actual);
    }
}
