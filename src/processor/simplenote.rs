use super::markdown::{write_markdown, Markdown, MarkdownMeta};
use lazy_static::lazy_static;
use regex::Regex;
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

pub fn process(source_file: PathBuf, dest_dir: PathBuf) -> Result<(), std::io::Error> {
    let source_text = load_file(&source_file)?;
    let all_notes = deserialize_notes(source_text)?;

    let active_result = process_notes(all_notes.active_notes, false, &dest_dir);
    if active_result.is_err() {
        println!("{}", active_result.unwrap_err());
    }

    let trashed_result = process_notes(all_notes.trashed_notes, false, &dest_dir);
    if trashed_result.is_err() {
        println!("{}", trashed_result.unwrap_err());
    }

    Ok(())
}

fn process_notes(
    notes: Option<Vec<SimpleNote>>,
    trashed: bool,
    dest_dir: &PathBuf,
) -> Result<(), std::io::Error> {
    match notes {
        Some(n) => {
            for note in n {
                let md = convert_to_markdown(note, trashed);
                let result = write_markdown(md, dest_dir);
                if result.is_err() {
                    println!("{}", result.unwrap_err());
                }
            }
        }
        None => {
            let note_type = match trashed {
                true => "trashed",
                false => "active",
            };
            println!("No {} notes found to process.", note_type);
        }
    };
    Ok(())
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
                    source_file.to_string_lossy()
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

fn title_from_content(content: &String) -> String {
    lazy_static! {
        static ref RE_MD_URL: Regex = Regex::new(r"\([^)]*\)").unwrap();
        static ref RE_BOGUS_TITLE_CHARS: Regex = Regex::new(r#"['"`#()!~>_\[\]\*]"#).unwrap();
    }

    let mut first_line = "";
    for line in content.lines() {
        if "" != line.trim() {
            first_line = line;
            break;
        }
    }

    // nuke any markdown style URL definitions
    let line_no_url: String = RE_MD_URL.replace_all(&first_line, "").to_string();

    // nuke some bogus characters
    let line_no_bogos: String = RE_BOGUS_TITLE_CHARS
        .replace_all(&line_no_url, "")
        .to_string();

    // leading dots/spaces stripped and trimmed
    let line_trim = line_no_bogos.trim_start_matches([' ', '.']).trim();

    // ensure not longer than 200 chars
    if 200 < line_trim.chars().count() {
        line_trim[0..200].to_string()
    } else {
        line_trim.to_string()
    }
}

fn convert_to_markdown(source: SimpleNote, trashed: bool) -> Markdown {
    Markdown {
        meta: MarkdownMeta {
            title: title_from_content(&source.content),
            created: source.creation_date,
            modified: source.last_modified,
            deleted: if trashed { Some(true) } else { None },
            favorited: None,
            pinned: source.pinned,
            tags: source.tags,
        },
        content: source.content.replace("\r\n", "\n"),
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

    #[test]
    fn title_from_content_empty() {
        let source = String::from("");
        let expected = String::from("");

        let actual = title_from_content(&source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn title_from_content_oneline() {
        let source = String::from("This is a simple one liner");
        let expected = String::from("This is a simple one liner");

        let actual = title_from_content(&source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn title_from_content_multiline() {
        let source = String::from("Mulitple lines\r\n can comprise\r\na note, too.");
        let expected = String::from("Mulitple lines");

        let actual = title_from_content(&source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn title_from_content_multiline_with_blanks() {
        let source = String::from("\r\n\r\n   \r\n\r\nMulitple lines\r\n can be present with spaces in front for\r\nthe note, too.");
        let expected = String::from("Mulitple lines");

        let actual = title_from_content(&source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn title_from_content_max_length_200() {
        let source = String::from("CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCXXXXXXXXXX");
        let expected = String::from("CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC");

        let actual = title_from_content(&source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn title_from_content_strip_markdown() {
        let source =
            String::from("# ~ _ * ![`Test Code Markdown Document`](http://google.com) * _ ~ ");
        let expected = String::from("Test Code Markdown Document");

        let actual = title_from_content(&source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn title_from_content_url_path() {
        let source = String::from("https://www.rust-lang.org/learn/get-started");
        let expected = String::from("https://www.rust-lang.org/learn/get-started");

        let actual = title_from_content(&source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn title_from_content_with_leading_dots() {
        let source = String::from(". .. Some Title");
        let expected = String::from("Some Title");

        let actual = title_from_content(&source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_active_simplenote_to_markdown_minimal_fields() {
        let source = SimpleNote {
            id: String::from("someid"),
            content: String::from("this is a note\nand stuff"),
            creation_date: String::from("2022-01-13T22:36:18.906Z"),
            last_modified: String::from("2022-01-14T07:36:50.656Z"),
            markdown: None,
            pinned: None,
            tags: None,
        };
        let expected = Markdown {
            meta: MarkdownMeta {
                title: String::from("this is a note"),
                created: String::from("2022-01-13T22:36:18.906Z"),
                modified: String::from("2022-01-14T07:36:50.656Z"),
                deleted: None,
                favorited: None,
                pinned: None,
                tags: None,
            },
            content: String::from("this is a note\nand stuff"),
        };

        let actual: Markdown = convert_to_markdown(source, false);
        println!("{}", expected);
        println!("{}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn simplenote_converted_and_written_to_expected_file() {
        // this demonstrates how a fully populated Simplenote will render into a Markdown file
        let expected: String = String::from_utf8_lossy(
            &fs::read("test_data/expected_2-simplenote-single.md").unwrap(),
        )
        .parse()
        .unwrap();

        let dest_dir = PathBuf::from("test_data/out");
        let source_file = PathBuf::from("test_data/simplenote-single.json");
        process(source_file, dest_dir).unwrap();

        let actual: String =
            String::from_utf8_lossy(&fs::read("test_data/out/Sample Document.md").unwrap())
                .parse()
                .unwrap();

        println!("{}", expected);
        println!("{}", actual);
        assert_eq!(expected, actual);
    }
}
