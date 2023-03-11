#![allow(clippy::uninlined_format_args)]

mod errors;
mod message;
use crate::errors::*;
use axoasset::LocalAsset;
use axohtml::{elements::section, html, text, unsafe_text};
use clap::Parser;
use markdown::{CompileOptions, ParseOptions, Constructs};
use message::Message;
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
        Message::new(message::MessageType::Info, "Creating your slideshow").print();
        let mut slides_html: Vec<Box<section<String>>> = vec![];
        let content = fs::read_to_string(file_path)?;
        let slides: Vec<&str> = content.as_str().split("\n---\n").collect();

        for slide in slides {
            // Live dangerously / trust the author:
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
  
            let slide_html = markdown::to_html_with_options(slide, &markdown::Options {  parse: ParseOptions {constructs: custom, ..ParseOptions::default()}, compile: compile_options }).unwrap();
            slides_html.extend(html!(<section><div>{unsafe_text!(slide_html)}</div></section>))
        }

        let final_html = html!(<html><head>
            <title>{text!("su")}</title>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <link rel="stylesheet" href="styles.css" />
            
            </head><body>{slides_html}<div id="progress"></div></body><script src="index.js"></script><script src="https://unpkg.com/prismjs@1.29.0/prism.js" /></html>)
        .to_string();
        create_files(final_html)?;
        Message::new(message::MessageType::Success, "Your slides are done!").print();
    } else {
        Message::new(message::MessageType::Error, "File does not exist").print();
    }

    Ok(())
}

pub fn create_files(html: String) -> Result<()> {
    let dist = "public".to_owned();
    create_dist_dir(dist.clone())?;
    LocalAsset::new("index.js", JS.into()).write(&dist)?;
    LocalAsset::new("styles.css", CSS.into()).write(&dist)?;
    LocalAsset::new("index.html", html.into()).write(&dist)?;

    Ok(())
}

pub fn create_dist_dir(dist_path: String) -> Result<()> {
    if !Path::new(&dist_path).exists() {
        std::fs::create_dir_all(dist_path)?;
    }

    Ok(())
}
