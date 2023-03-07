import {parse} from "../mod.ts"

Deno.test("Simple", () => {
console.log(parse(`
        <!DOCTYPE html>
        <html>
            <head>
                {$ a1, a2, a3 from \"some-library\" as DEFAULT}
                {$ a1, a2, a3 from \"./some-file\" as DEFAULT}
            </head>
            <body>
                {#const a = b}
                {!if (a)}
                {a * 2}
                {/else}
                No variable
                {/}
                \"I'm the best hacker jajajajaj\"
            </body>
        </html>
        `, {
          input_path: "/a.html",
          output_path: ""
        }))
})
