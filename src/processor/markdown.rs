use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MarkdownMeta {
    pub title: String,
    pub created: String,
    pub modified: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct Markdown {
    pub meta: MarkdownMeta,
    pub content: String,
}

impl fmt::Display for Markdown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", serialize_markdown(self).unwrap())
    }
}

fn serialize_markdown(markdown: &Markdown) -> Result<String, serde_yaml::Error> {
    match serde_yaml::to_string(&markdown.meta) {
        Ok(m) => Ok(format!("{}{}\n{}\n", m, "---", markdown.content)),
        Err(e) => Err(e),
    }
}

fn title_to_filepath(dest_dir: &PathBuf, title: &str) -> Result<PathBuf, std::io::Error> {
    if "".eq(title) {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            format!("title: '{}' is not valid for a filename", title),
        ))
    } else {
        let title_part = match title.rsplit_once("/") {
            Some(s) => s.1,
            None => title
        };
        let trimmed_title = title_part.trim();
        let mut file_path = dest_dir.clone();
        file_path.push(trimmed_title);
        file_path.set_extension("md");
        Ok(file_path)
    }
}

pub fn write_markdown(markdown: Markdown, dest_dir: &PathBuf) -> Result<(), std::io::Error> {
    match title_to_filepath(dest_dir, &markdown.meta.title) {
        Ok(file_path) => match fs::File::create(file_path) {
            Ok(mut f) => match serialize_markdown(&markdown) {
                Err(e) => Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    format!("YAML ERROR: {}", e),
                )),
                Ok(text) => match f.write_all(text.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                },
            },
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_meta_with_all_fields() {
        // this just demonstrates how a fully populated MarkdownMeta will render
        let source = MarkdownMeta {
            title: String::from("A title"),
            created: String::from("2022-01-13T22:36:18.906Z"),
            modified: String::from("2022-01-14T07:36:50.656Z"),
            deleted: Some(true),
            favorited: Some(true),
            pinned: Some(true),
            tags: Some(vec![String::from("Personal"), String::from("Business")]),
        };
        let expected = r#"---
title: A title
created: "2022-01-13T22:36:18.906Z"
modified: "2022-01-14T07:36:50.656Z"
deleted: true
favorited: true
pinned: true
tags:
  - Personal
  - Business
"#;
        let actual = serde_yaml::to_string(&source).unwrap();
        println!("{}", expected);
        println!("{}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_meta_with_minimal_fields() {
        // this just demonstrates how a minimally populated MarkdownMeta will render
        let source = MarkdownMeta {
            title: String::from("A title"),
            created: String::from("2022-01-13T22:36:18.906Z"),
            modified: String::from("2022-01-14T07:36:50.656Z"),
            deleted: None,
            favorited: None,
            pinned: None,
            tags: None,
        };
        let expected = r#"---
title: A title
created: "2022-01-13T22:36:18.906Z"
modified: "2022-01-14T07:36:50.656Z"
"#;
        let actual = serde_yaml::to_string(&source).unwrap();
        println!("{}", expected);
        println!("{}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_markdown_with_all_meta_fields() {
        // this just demonstrates how a fully populated MarkdownMeta will render
        let meta = MarkdownMeta {
            title: String::from("A title"),
            created: String::from("2022-01-13T22:36:18.906Z"),
            modified: String::from("2022-01-14T07:36:50.656Z"),
            deleted: Some(true),
            favorited: Some(true),
            pinned: Some(true),
            tags: Some(vec![String::from("Personal"), String::from("Business")]),
        };
        let source = Markdown {
            meta: meta,
            content: String::from("This is a\ngreat piece of\nsample content!"),
        };
        let expected = r#"---
title: A title
created: "2022-01-13T22:36:18.906Z"
modified: "2022-01-14T07:36:50.656Z"
deleted: true
favorited: true
pinned: true
tags:
  - Personal
  - Business
---
This is a
great piece of
sample content!
"#;
        let actual = serialize_markdown(&source).unwrap();
        println!("{}", expected);
        println!("{}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_markdown_with_minimal_meta_fields() {
        // this just demonstrates how a fully populated MarkdownMeta will render
        let meta = MarkdownMeta {
            title: String::from("A title"),
            created: String::from("2022-01-13T22:36:18.906Z"),
            modified: String::from("2022-01-14T07:36:50.656Z"),
            deleted: None,
            favorited: None,
            pinned: None,
            tags: None,
        };
        let source = Markdown {
            meta: meta,
            content: String::from("This is a\ngreat piece of\nsample content!"),
        };
        let expected = r#"---
title: A title
created: "2022-01-13T22:36:18.906Z"
modified: "2022-01-14T07:36:50.656Z"
---
This is a
great piece of
sample content!
"#;
        let actual = serialize_markdown(&source).unwrap();
        println!("{}", expected);
        println!("{}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn filepath_invalid_empty() {
        let path = PathBuf::from("/tmp");
        let title = "";
        let error = title_to_filepath(&path, title).unwrap_err();
        assert_eq!(ErrorKind::InvalidData, error.kind());
        assert_eq!(
            format!("title: '{}' is not valid for a filename", title),
            format!("{}", error)
        );
    }

    #[test]
    fn filename_strips_leading_trailing_spaces() {
        let path = PathBuf::from("/tmp");
        let title = "  A Title With Spaces  ";
        let actual = title_to_filepath(&path, title).unwrap();
        let mut expected = PathBuf::from(path.to_str().unwrap());
        expected.push(title.trim_start_matches(" ").trim_end_matches(" "));
        expected.set_extension("md");
        assert_eq!(actual, expected);
    }

    #[test]
    fn filename_simple_success() {
        let path = PathBuf::from("/tmp");
        let title = "A Simple Filename";
        let actual = title_to_filepath(&path, title).unwrap();
        let mut expected = PathBuf::from(path.to_str().unwrap());
        expected.push(title.trim_start_matches(" ").trim_end_matches(" "));
        expected.set_extension("md");
        assert_eq!(actual, expected);
    }

    #[test]
    fn filename_uses_last_slash_part() {
        let path = PathBuf::from("/tmp");
        let title = "https://www.rust-lang.org/learn/get-started";
        let actual = title_to_filepath(&path, title).unwrap();
        let mut expected = PathBuf::from(path.to_str().unwrap());
        expected.push("get-started");
        expected.set_extension("md");
        assert_eq!(actual, expected);
    }


    #[test]
    fn markdown_writes_correct_content_to_expected_file() {
        // this demonstrates how a fully populated Markdown will render into a file
        // don't need to test all kinds of options since we already covered this
        // with verification of dest_dir writability, etc
        let expected: String =
            String::from_utf8_lossy(&fs::read("test_data/expected_1.md").unwrap())
                .parse()
                .unwrap();

        let meta = MarkdownMeta {
            title: String::from("A title"),
            created: String::from("2022-01-13T22:36:18.906Z"),
            modified: String::from("2022-01-14T07:36:50.656Z"),
            deleted: Some(true),
            favorited: Some(true),
            pinned: Some(true),
            tags: Some(vec![String::from("Personal"), String::from("Business")]),
        };
        let source = Markdown {
            meta: meta,
            content: String::from("This is a\ngreat piece of\nsample content!"),
        };
        let path = PathBuf::from("test_data/out");
        write_markdown(source, &path).unwrap();

        let actual: String =
            String::from_utf8_lossy(&fs::read("test_data/out/A title.md").unwrap())
                .parse()
                .unwrap();
                
        println!("{}", expected);
        println!("{}", actual);
        assert_eq!(expected, actual);
    }
}
