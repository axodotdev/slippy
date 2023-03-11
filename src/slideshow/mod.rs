pub mod html;
pub mod markdown;

use std::path::Path;

use crate::{errors::*, Theme};
use axoasset::LocalAsset;
use css_minify::optimizations::{Level, Minifier};

const CSS: &str = include_str!("../assets/styles.css");
const LIGHT_THEME: &str = include_str!("../assets/themes/light.css");
const DARK_THEME: &str = include_str!("../assets/themes/dark.css");
const CUPCAKE_THEME: &str = include_str!("../assets/themes/cupcake.css");
const JS: &str = include_str!("../assets/slide.js");
const DIST: &str = "public";

pub fn create_files(html: String, theme_d: Option<Theme>) -> Result<()> {
    let theme_css = match theme_d {
        Some(theme) => match theme {
            Theme::Light => LIGHT_THEME,
            Theme::Dark => DARK_THEME,
            Theme::Cupcake => CUPCAKE_THEME,
        },
        None => LIGHT_THEME,
    };
    let full_css = format!("{} {}", CSS, theme_css);
    let minified_css = Minifier::default().minify(full_css.as_str(), Level::Three);

    match minified_css {
        Err(_) => Err(AxoSlidesError::CSSMinificationError {}),
        Ok(css) => {
            create_dist_dir()?;
            LocalAsset::new("index.js", JS.into()).write(DIST)?;
            LocalAsset::new("styles.css", css.into()).write(DIST)?;
            LocalAsset::new("index.html", html.into()).write(DIST)?;

            Ok(())
        }
    }
}

pub fn create_dist_dir() -> Result<()> {
    if !Path::new(&DIST).exists() {
        std::fs::create_dir_all(DIST)?;
    }

    Ok(())
}
