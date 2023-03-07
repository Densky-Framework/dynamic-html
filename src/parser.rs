use regex::Regex;

use crate::DynamicHtml;

#[derive(Debug, Clone)]
pub enum HtmlPart {
    Literal(String),
    Eval(String),
    Unescaped(String),
    Block(String),
    Import(String),
}

#[derive(Debug, Clone)]
pub struct HtmlImportPart {
    pub content: String,
    pub filename: String,
    pub default_name: Option<String>,
    pub imports: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    Unclosed,
    InvalidImport(String),
}

/* ----------- *\
  PREFIX
    ▼
   {@ EXPR }
   ▲       ▲
   DELIMITER
\* ----------- */
pub const OPEN_DELIMITER: char = '{';
pub const CLOSE_DELIMITER: char = '}';

pub const UNESCAPED_PREFIX: char = '@';
pub const OPEN_BLOCK_PREFIX: char = '!';
pub const CLOSE_BLOCK_PREFIX: char = '/';
pub const BLOCK_PREFIX: char = '#';
pub const IMPORT_PREFIX: char = '$';

/*
  https://regex101.com/r/oP95kB/1

  Matches:
  func1, Func2 as f2, a from "./file.ts" as DEFAULT
  "./file.ts" as DEFAULT
  func1, Func2 as f2, a from "./file.ts"
  a from "./file.ts"
  b as c, a from "./file.ts"
  a as b from "./file.ts"
*/
const IMPORT_REGEX: &str = r#"(?:((?:(?:,\s*)?[a-zA-Z_$][a-zA-Z0-9_$]*(?:\s+as\s+[a-zA-Z_$][a-zA-Z0-9_$]*)?)+) from\s+)?"([^"]+)"(?:\s+as\s+([a-zA-Z_$][a-zA-Z0-9_$]*))?"#;

impl HtmlPart {
    #[inline]
    pub fn from_prefix(prefix: char, content: String) -> HtmlPart {
        match prefix {
            UNESCAPED_PREFIX => Self::Unescaped(content),
            OPEN_BLOCK_PREFIX => Self::Block(content),
            CLOSE_BLOCK_PREFIX => Self::Block(content),
            BLOCK_PREFIX => Self::Block(content),
            IMPORT_PREFIX => Self::Import(content),
            _ => Self::Eval(content),
        }
    }

    #[inline]
    pub fn is_eval(prefix: char) -> bool {
        match prefix {
            UNESCAPED_PREFIX | OPEN_BLOCK_PREFIX | CLOSE_BLOCK_PREFIX | BLOCK_PREFIX
            | IMPORT_PREFIX => false,
            _ => true,
        }
    }
}

pub fn search_delimiter(delimiter: char, last_index: usize, content: &String) -> Option<usize> {
    let mut tmp_index = last_index;
    while tmp_index < content.len() {
        let index = match content[tmp_index..].find(delimiter) {
            Some(0) => return Some(last_index),
            None => return None,
            Some(index) => index,
        };

        // When the char before the delimiter is `\`
        // then skip it
        let before_char = content.chars().nth(index - 1);
        if before_char == Some('\\') {
            let delimiter_len = 1;
            tmp_index = index + delimiter_len;
            continue;
        }

        return Some(last_index + index);
    }

    None
}

pub fn handle_expression(expr: &str) -> Option<HtmlPart> {
    let expr = expr.trim();
    if expr.len() == 0 {
        return None;
    }

    let prefix = expr.chars().nth(0).unwrap();

    // The CLOSE_BLOCK_PREFIX has special output,
    // always is a block with "}" as content
    if prefix == CLOSE_BLOCK_PREFIX {
        let content = if expr.len() > 1 {
            format!("{} {} {}", "}", expr[1..].trim_start(), "{")
        } else {
            "}".to_string()
        };

        return Some(HtmlPart::from_prefix(prefix, content));
    }

    let expr = if HtmlPart::is_eval(prefix) {
        expr
    } else {
        &expr[1..].trim_start()
    };

    if expr.len() == 0 {
        return None;
    }

    let mut expr = expr.to_string();

    if prefix == OPEN_BLOCK_PREFIX {
        expr += " {";
    }

    Some(HtmlPart::from_prefix(prefix, expr))
}

pub fn normalize(dirty_parts: Vec<HtmlPart>) -> Result<DynamicHtml, ParseError> {
    let regex = Regex::new(IMPORT_REGEX).unwrap();
    let mut imports: Vec<HtmlImportPart> = vec![];
    let mut parts: Vec<HtmlPart> = vec![];

    for part in dirty_parts.iter() {
        match part {
            HtmlPart::Import(content) => {
                let captures = regex.captures(content);
                let captures = match captures {
                    Some(expr) => expr,
                    None => return Err(ParseError::InvalidImport(content.clone())),
                };

                let (import, filename, default_name) =
                    (captures.get(1), captures.get(2), captures.get(3));
                let import = match import {
                    Some(imports) => Some(imports.as_str().to_string()),
                    None => None,
                };
                let default_name = match default_name {
                    Some(default_name) => Some(default_name.as_str().to_string()),
                    None => None,
                };

                let import_part = HtmlImportPart {
                    content: content.to_string(),
                    default_name,
                    filename: filename.unwrap().as_str().to_string(),
                    imports: import,
                };

                imports.push(import_part);
            }
            part => parts.push(part.clone()),
        }
    }

    Ok(DynamicHtml { imports, parts })
}
