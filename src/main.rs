#![allow(clippy::uninlined_format_args)]

mod errors;
use crate::errors::*;
use axoasset::LocalAsset;
use axohtml::{elements::section, html, text, unsafe_text};
use clap::Parser;
use std::{fs, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: String,
}

const CSS: &str = include_str!("assets/styles.css");
const JS: &str = include_str!("assets/slide.js");

fn main() -> Result<()> {
    let args = Args::parse();
    let file_path = Path::new(&args.file);
    if Path::exists(file_path) && file_path.extension().unwrap().to_str() == Some("md") {
        let mut slides_html: Vec<Box<section<String>>> = vec![];
        let content = fs::read_to_string(file_path)?;
        let slides: Vec<&str> = content.as_str().split("---").collect();

        for slide in slides {
            let slide_html = markdown::to_html(slide);
            slides_html.extend(html!(<section>{unsafe_text!(slide_html)}</section>))
        }

        let final_html = html!(<html><head>
            <title>{text!("su")}</title>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <link rel="stylesheet" href="styles.css" />
            
            </head><body>{slides_html}<div id="progress"></div></body><script src="index.js"></script><script src="https://unpkg.com/prismjs@1.29.0/prism.js" /></html>)
        .to_string();
        create_dist_dir(String::from("public".to_owned()))?;
        LocalAsset::new("index.js", JS.into()).write(String::from("public".to_owned()).as_str())?;
        LocalAsset::new("styles.css", CSS.into())
            .write(String::from("public".to_owned()).as_str())?;
        LocalAsset::new("index.html", final_html.into())
            .write(String::from("public".to_owned()).as_str())?;
    }

    Ok(())
}

pub fn create_dist_dir(dist_path: String) -> Result<()> {
    if !Path::new(&dist_path).exists() {
        std::fs::create_dir_all(dist_path)?;
    }

    Ok(())
}
