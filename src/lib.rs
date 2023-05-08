pub mod generate;
pub mod parser;

use std::path;

use generate::escape;
pub use generate::GenerateOptions;
use parser::{handle_expression, normalize, search_delimiter};
use parser::{HtmlImportPart, HtmlPart, ParseError, CLOSE_DELIMITER, OPEN_DELIMITER};
use pathdiff::diff_paths;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct DynamicHtml {
    pub imports: Vec<HtmlImportPart>,
    pub parts: Vec<HtmlPart>,
}
impl DynamicHtml {
    pub fn parse(content: &String) -> Result<DynamicHtml, ParseError> {
        let mut last_index = 0;
        let mut parts: Vec<HtmlPart> = vec![];

        while last_index < content.len() {
            let open_index = search_delimiter(OPEN_DELIMITER, last_index, &content);
            let has_delimiter = open_index.is_some();
            let open_index = open_index.unwrap_or_else(|| content.len());

            let expression = &content[last_index..open_index].trim();
            if expression.len() != 0 {
                parts.push(HtmlPart::Literal(expression.to_string()));
            }

            if !has_delimiter {
                break;
            }

            last_index = open_index + 1;
            let close_index = search_delimiter(CLOSE_DELIMITER, last_index, &content);

            if let Some(close_index) = close_index {
                let expression = &content[last_index..close_index];
                let part = handle_expression(expression);

                if part.is_some() {
                    parts.push(part.unwrap());
                }

                last_index = close_index + 1;
            } else {
                return Err(ParseError::Unclosed);
            }
        }

        normalize(parts)
    }

    pub fn generate(&self, options: &GenerateOptions) -> String {
        let GenerateOptions {
            input_path,
            output_path,

            header,
            pre_imports,
            data_varname,
            escape_function,
        } = options;

        let input_path = path::Path::new(&input_path);
        let input_path = input_path.parent().unwrap();

        let output_path = path::Path::new(&output_path);
        let output_path = output_path.parent().unwrap();

        let imports = self
            .imports
            .iter()
            .map(|import| {
                let imports = match &import.imports {
                    Some(expr) => format!("{} {} {}", "{", expr, "}"),
                    None => "".to_string(),
                };

                let imports = match &import.default_name {
                    Some(expr) => format!("{}, {}", expr, imports),
                    None => imports,
                };

                let relative_path =
                    diff_paths(input_path.join(import.filename.clone()), output_path).unwrap();
                let relative_path = relative_path.to_str().unwrap();

                let relative_path = if &relative_path.chars().nth(0) == &Some('.') {
                    relative_path.to_string()
                } else {
                    format!("./{}", relative_path)
                };

                let resolved_path = if &import.filename.chars().nth(0) == &Some('.') {
                    relative_path.as_str()
                } else {
                    &import.filename
                };

                format!("import {} from \"{}\"", imports, resolved_path)
            })
            .reduce(|acc, e| acc + ";\n" + &e)
            .unwrap_or("".to_owned())
            + ";";

        let body = self
            .parts
            .iter()
            .map(|part| match part {
                HtmlPart::Literal(content) => format!("__output__ += \"{}\"", escape(content)),
                HtmlPart::Eval(content) => {
                    format!("__output__ += {}(({}))", escape_function, content)
                }
                HtmlPart::Unescaped(content) => format!("__output__ += String((\"{}\"))", content),
                HtmlPart::Block(content) => content.to_string(),
                _ => "// INVALID TYPE".to_string(),
            })
            .reduce(|acc, e| acc + ";\n  " + &e)
            .unwrap_or("".to_owned())
            + ";";

        format!(
            "{}\n{}\n{}

export default function({}: any, __output__: string = \"\"): string {}
  {}
  return __output__;
{}",
            header, pre_imports, imports, data_varname, "{", body, "}"
        )
    }
}

#[wasm_bindgen]
pub fn parse(content: String, options: String) -> Result<String, JsValue> {
    println!("{:?}", &options);
    match DynamicHtml::parse(&content) {
        Ok(out) => Ok(out.generate(&GenerateOptions::from_json(options))),
        Err(_) => Err(JsValue::null()),
    }
}
