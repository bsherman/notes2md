use serde::{Deserialize, Serialize};
use std::fmt;
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

pub fn write_markdown(markdown: Markdown, _dest_dir: PathBuf) -> Result<(), std::io::Error> {
    println!("{}", markdown);
    Ok(())
}

fn serialize_markdown(markdown: &Markdown) -> Result<String, serde_yaml::Error> {
    match serde_yaml::to_string(&markdown.meta) {
        Ok(m) => Ok(format!("{}{}\n{}\n", m, "---", markdown.content)),
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
}
