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
