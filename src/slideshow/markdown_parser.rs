use markdown::{CompileOptions, Constructs, ParseOptions};

pub fn transform_markdown(slide: &str) -> String {
    let compile_options = CompileOptions {
        allow_dangerous_html: true,
        allow_dangerous_protocol: true,
        ..CompileOptions::default()
    };

    let custom = Constructs {
        autolink: true,
        code_fenced: true,
        gfm_strikethrough: true,
        gfm_table: true,
        ..Constructs::gfm()
    };
    markdown::to_html_with_options(
        slide,
        &markdown::Options {
            parse: ParseOptions {
                constructs: custom,
                ..ParseOptions::default()
            },
            compile: compile_options,
        },
    )
    // https://docs.rs/markdown/1.0.0-alpha.7/markdown/fn.to_html_with_options.html#errors
    .unwrap()
}

#[test]
fn parses_markdown_headings() {
    let html = transform_markdown("## hello");
    println!("{}", html);
    assert!(html.eq("<h2>hello</h2>"));
}

#[cfg(test)]
fn remove_whitespace(s: String) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn parses_markdown_tables() {
    let html = transform_markdown(
        r#"
| one | two | three |
| --- | --- | ----- |
| yes | no  | yes   |
    "#,
    );
    assert!(remove_whitespace(html).eq("<table><thead><tr><th>one</th><th>two</th><th>three</th></tr></thead><tbody><tr><td>yes</td><td>no</td><td>yes</td></tr></tbody></table>"));
}

#[test]
fn parses_markdown_code() {
    let html = transform_markdown(
        r#"
```bash
sh uwu.sh
```
    "#,
    );
    assert!(remove_whitespace(html).eq("<pre><codeclass=\"language-bash\">shuwu.sh</code></pre>"));
}
