// pub mod lib;
//
// use dynamic_html::{DynamicHtml, GenerateOptions};
//
//
// fn main() {
//     let html = "\
//         <!DOCTYPE html>\
//         <html>\
//             <head>\
//                 {$ a1, a2, a3 from \"some-library\" as DEFAULT}
//                 {$ a1, a2, a3 from \"./some-file\" as DEFAULT}
//             </head>\
//             <body>\
//                 {#const a = b}\
//                 {!if (a)}\
//                 {a * 2}\
//                 {/else}\
//                 No variable\
//                 {/}\
//                 \"I'm the best hacker jajajajaj\"\
//             </body>\
//         </html>\
//         ";
//
//     let dynamic = DynamicHtml::parse(&html.to_string());
//
//     println!("{:#?}", &dynamic);
//
//     println!(
//         "{}",
//         &dynamic.unwrap().generate(
//             &GenerateOptions::new("/a/a.html".to_string(), "/b/b.html".to_string())
//                 .set_header("// deno-lint-ignore-file".to_string())
//                 .add_import("import some_module from 'a file'".to_string())
//                 .add_import("import some_module from 'a file'".to_string())
//                 .add_import("import some_module from 'a file'".to_string())
//                 .add_import("import some_module from 'a file'".to_string())
//                 .add_import("import some_module from 'a file'".to_string())
//                 .add_import("import some_module from 'a file'".to_string())
//         )
//     );
// }

fn main() {
    unimplemented!()
}
