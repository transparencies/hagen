use failure::Error;
use log::{debug, info};
use std::path::Path;

type Result<T> = std::result::Result<T, Error>;

use crate::loader::{Content, JsonBodyProvider, Loader, Metadata};

use serde_json::{Map, Value};
use std::fs::read_to_string;

pub struct MarkdownLoader<P: AsRef<Path>> {
    path: P,
}

impl<P: AsRef<Path>> MarkdownLoader<P> {
    pub fn new(path: P) -> Self {
        MarkdownLoader { path }
    }
}

impl<P: AsRef<Path>> Loader for MarkdownLoader<P> {
    fn load_from(&self) -> Result<Content> {
        let path = self.path.as_ref();
        info!("Loading - Markdown: {:?}", path);

        let data = read_to_string(path)?;

        let front_matter = parse_front_matter(&data)?;

        Ok(Content {
            metadata: Metadata::from_path(path, path.file_stem(), "md"),
            front_matter: front_matter.1.unwrap_or_default(),
            content: Box::new(JsonBodyProvider::new(serde_json::Value::String(
                front_matter.0,
            ))),
        })
    }
}

fn is_marker(line: Option<&str>) -> bool {
    if let Some(s) = line {
        s.trim().eq("---")
    } else {
        false
    }
}

fn parse_front_matter(data: &String) -> Result<(String, Option<Map<String, Value>>)> {
    let mut lines = data.lines();

    if !is_marker(lines.next()) {
        return Ok((data.clone(), None));
    }

    let mut front_matter: Vec<String> = Vec::new();

    while let Some(s) = lines.next() {
        if is_marker(Some(s)) {
            break;
        }
        front_matter.push(s.into());
    }

    let front_matter = front_matter.join("\n");

    debug!("front matter: {}", front_matter);

    let front_matter = serde_yaml::from_str::<Map<String, Value>>(&front_matter)?;
    let remainder = lines.collect::<Vec<_>>().join("\n");

    debug!("front matter: {:?} -> {}", front_matter, remainder);

    Ok((remainder, Some(front_matter)))
}