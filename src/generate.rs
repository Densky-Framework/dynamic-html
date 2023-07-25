#[cfg(feature = "wasm")]
use serde::Deserialize;

#[cfg(feature = "wasm")]
use serde_json::Value;

#[cfg(feature = "wasm")]
#[derive(Deserialize)]
pub struct GenerateOptionsWasm {
    pub input_path: String,
    pub output_path: String,

    pub header: String,
    pub pre_imports: String,
    pub data_varname: String,
    pub escape_function: String,
}

pub struct GenerateOptions {
    pub input_path: String,
    pub output_path: String,

    pub header: String,
    pub pre_imports: String,
    pub data_varname: String,
    pub escape_function: String,
}

impl GenerateOptions {
    pub fn new(input_path: String, output_path: String) -> GenerateOptions {
        GenerateOptions {
            input_path,
            output_path,

            header: "".to_owned(),
            pre_imports: "".to_owned(),
            data_varname: "data".to_owned(),
            escape_function: "".to_owned(),
        }
    }

    pub fn set_header(&mut self, header: String) -> &mut GenerateOptions {
        self.header = header;
        self
    }

    pub fn set_data_varname(&mut self, data_varname: String) -> &mut GenerateOptions {
        self.data_varname = data_varname;
        self
    }

    pub fn set_escape_function(&mut self, escape_function: String) -> &mut GenerateOptions {
        self.escape_function = escape_function;
        self
    }

    pub fn add_import(&mut self, import: String) -> &mut GenerateOptions {
        self.pre_imports += "\n";
        self.pre_imports += &import;
        self.pre_imports += ";";
        self
    }

    #[cfg(feature = "wasm")]
    pub fn from_json(wasm: String) -> GenerateOptions {
        let wasm_value: Value = serde_json::from_str(&wasm).unwrap();
        let wasm_value = match wasm_value {
            Value::Object(obj) => obj,
            _ => panic!("No object"),
        };

        let input_path = match wasm_value.get("input_path") {
            Some(Value::String(s)) => s,
            _ => panic!("No string"),
        };

        GenerateOptions::new(input_path.clone(), "/b.html".to_string())
    }
}

pub fn escape(data: &String) -> String {
    return data
        .replace("\\", "\\\\")
        .replace("\n", "\\n")
        .replace("\"", "\\\"");
}
