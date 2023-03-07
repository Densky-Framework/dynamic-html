import { instantiate } from "./lib/dynamic_html.generated.js";

const { parse: parse_org } = await instantiate();

export interface GenerateOptions {
  input_path: string;
  output_path: string;

  header?: string;
  pre_imports?: string;
  data_varname?: string;
  escape_function?: string;
}

export function parse(content: string, options: GenerateOptions): string {
  if (options.input_path === "") {
    throw new Error("Empty input path");
  }
  if (options.output_path === "") {
    throw new Error("Empty output path");
  }
  return parse_org(
    content,
    JSON.stringify({
      header: "",
      pre_imports: "",
      data_varname: "data",
      escape_function: "",

      ...options,
    })
  );
}
