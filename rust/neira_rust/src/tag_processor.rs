use pyo3::prelude::*;
use once_cell::sync::Lazy;
use std::collections::{HashSet};
use std::fs;
use std::sync::Mutex;
use serde_json::Value;

static ENTITIES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[pyclass]
#[derive(Clone, Debug)]
pub struct Tag {
    #[pyo3(get, set)]
    pub r#type: String,
    #[pyo3(get, set)]
    pub subject: String,
    #[pyo3(get, set)]
    pub commands: Vec<String>,
}

impl Tag {
    fn empty() -> Self {
        Tag { r#type: String::new(), subject: String::new(), commands: Vec::new() }
    }
}

fn register_entity(name: &str) {
    if name.is_empty() {
        return;
    }
    let mut entities = ENTITIES.lock().unwrap();
    if !entities.iter().any(|e| e == name) {
        entities.push(name.to_string());
    }
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn render(tag: &Tag) -> String {
    let mut base = format!("@{}: {}", capitalize_first(&tag.r#type), tag.subject);
    if !tag.commands.is_empty() {
        let cmds = tag
            .commands
            .iter()
            .map(|c| format!("/{}", c))
            .collect::<Vec<_>>()
            .join(" ");
        base.push(' ');
        base.push_str(&cmds);
    }
    base.push('@');
    base
}

fn find_char(chars: &[char], mut i: usize, target: char) -> Option<usize> {
    while i < chars.len() {
        if chars[i] == target {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn parse_tag(chars: &[char], start: usize) -> (Tag, Vec<Tag>, usize) {
    let mut i = start + 1;
    let colon = find_char(chars, i, ':');
    if colon.is_none() {
        return (Tag::empty(), vec![], 1);
    }
    let colon_idx = colon.unwrap();
    let tag_type = chars[i..colon_idx]
        .iter()
        .collect::<String>()
        .trim()
        .to_lowercase();
    i = colon_idx + 1;
    let mut content_chars: Vec<char> = Vec::new();
    let mut inner: Vec<Tag> = Vec::new();
    while i < chars.len() {
        let ch = chars[i];
        if ch == '@' {
            if i + 1 < chars.len() && chars[i + 1] == '@' {
                content_chars.push('@');
                i += 2;
                continue;
            }
            let next_at = find_char(chars, i + 1, '@');
            let next_colon = find_char(chars, i + 1, ':');
            if let Some(nc) = next_colon {
                if next_at.is_none() || nc < next_at.unwrap() {
                    let (nested_tag, nested_inner, consumed) = parse_tag(chars, i);
                    inner.extend(nested_inner);
                    inner.push(nested_tag.clone());
                    for rc in render(&nested_tag).chars() {
                        content_chars.push(rc);
                    }
                    i += consumed;
                    continue;
                }
            }
            i += 1;
            break;
        } else {
            content_chars.push(ch);
            i += 1;
        }
    }
    let content = content_chars.iter().collect::<String>().replace("@@", "@");
    let parts: Vec<String> = content
        .split('/')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect();
    let first = parts.first().cloned().unwrap_or_default();
    let mut commands: Vec<String> = Vec::new();
    let mut subject = first.clone();
    if let Some((s, cmd)) = first
        .split_once('—')
        .or_else(|| first.split_once('-'))
    {
        subject = s.trim().to_string();
        let cmd = cmd.trim();
        if !cmd.is_empty() {
            commands.push(cmd.to_string());
        }
    }
    for p in parts.iter().skip(1) {
        commands.push(p.clone());
    }
    let tag = Tag { r#type: tag_type, subject: subject.clone(), commands };
    register_entity(&subject);
    (tag, inner, i - start)
}

#[pyfunction]
pub fn parse(text: &str) -> PyResult<Vec<Tag>> {
    let chars: Vec<char> = text.chars().collect();
    let mut tags: Vec<Tag> = Vec::new();
    let mut i: usize = 0;
    while i < chars.len() {
        if chars[i] == '@' {
            let (tag, mut inner, consumed) = parse_tag(&chars, i);
            tags.append(&mut inner);
            if !tag.r#type.is_empty() {
                tags.push(tag);
            }
            i += consumed;
        } else {
            i += 1;
        }
    }
    Ok(tags)
}

#[pyfunction]
pub fn suggest_entities(prefix: &str) -> PyResult<Vec<String>> {
    let lower = prefix.to_lowercase();
    let mut suggestions: Vec<String> = {
        let entities = ENTITIES.lock().unwrap();
        entities
            .iter()
            .filter(|e| e.to_lowercase().starts_with(&lower))
            .cloned()
            .collect()
    };

    if let Ok(text) = fs::read_to_string("data/knowledge_base/characters.json") {
        if let Ok(val) = serde_json::from_str::<Value>(&text) {
            if let Some(obj) = val.as_object() {
                for info in obj.values() {
                    if let Some(name) = info.get("name").and_then(|v| v.as_str()) {
                        if name.to_lowercase().starts_with(&lower) {
                            suggestions.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    let mut seen = HashSet::new();
    suggestions.retain(|e| seen.insert(e.clone()));
    Ok(suggestions)
}

