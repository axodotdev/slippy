use axohtml::{elements::section, html};

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
