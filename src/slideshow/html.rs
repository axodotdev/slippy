use axohtml::{elements::section, html};

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
pub fn create_html(slides_html: Vec<Box<section<String>>>) -> String {
    html!(
    <html>
        <head>
            <title></title>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <link rel="stylesheet" href="styles.css" />
        </head>
        <body>
            {slides_html}
            <div class="progress-bar"></div>
        </body>
        <script src="index.js"></script>
        <script src="https://unpkg.com/prismjs@1.29.0/prism.js" />
    </html>
        )
    .to_string()
}
#[cfg(test)]
use axohtml::text;
#[test]
fn creates_html_from_slides() {
    let html = create_html(vec![html!(<section><h1>{text!("uwu")}</h1></section>)]);
    assert!(html.eq("<html><head><title></title><meta charset=\"UTF-8\"/><meta content=\"width=device-width, initial-scale=1.0\" name=\"viewport\"/><link href=\"styles.css\" rel=\"stylesheet\"/></head><body><section><h1>uwu</h1></section><div class=\"progress-bar\"></div></body><script src=\"index.js\"></script><script src=\"https://unpkg.com/prismjs@1.29.0/prism.js\"></script></html>"));
}
