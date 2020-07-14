// source: www.rust-lang.org

extern crate sass_rs;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::fs;

use sass_rs::{compile_file, Options};

fn hash_css(css: &str) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(css.as_bytes());
    hasher.finish().to_string()
}

fn compile_sass(filename: &str) -> String {
    let scss_file = format!("./src/styles/{}.scss", filename);

    let css = compile_file(&scss_file, Options::default())
        .unwrap_or_else(|_| panic!("couldn't compile sass: {}", &scss_file));

    let css_sha = format!("{}_{}", filename, hash_css(&css));
    let css_file = format!("./static/styles/{}.css", css_sha);

    fs::write(&css_file, css.into_bytes())
        .unwrap_or_else(|_| panic!("couldn't write css file: {}", &css_file));

    String::from(&css_file[1..])
}

fn concat_vendor_css(files: Vec<&str>) -> String {
    let mut concatted = String::new();
    for filestem in files {
        let vendor_path = format!("./static/styles/{}.css", filestem);
        let contents = fs::read_to_string(vendor_path).expect("couldn't read vendor css");
        concatted.push_str(&contents);
    }

    let css_sha = format!("vendor_{}", hash_css(&concatted));
    let css_path = format!("./static/styles/{}.css", &css_sha);

    fs::write(&css_path, &concatted).expect("couldn't write vendor css");

    String::from(&css_path[1..])
}

fn concat_app_js(files: Vec<&str>) -> String {
    let mut concatted = String::new();
    for filestem in files {
        let vendor_path = format!("./static/scripts/{}.js", filestem);
        let contents = fs::read_to_string(vendor_path).expect("couldn't read app js");
        concatted.push_str(&contents);
    }

    let js_sha = format!("app_{}", hash_css(&concatted));
    let js_path = format!("./static/scripts/{}.js", &js_sha);

    fs::write(&js_path, &concatted).expect("couldn't write app js");

    String::from(&js_path[1..])
}

#[derive(Clone, Serialize)]
struct CSSFiles {
    app: String,
    fonts: String,
    vendor: String,
}
#[derive(Clone, Serialize)]
struct JSFiles {
    app: String,
}
#[derive(Clone, Serialize)]
pub struct AssetFiles {
    css: CSSFiles,
    js: JSFiles,
}
impl AssetFiles {
    pub fn new() -> AssetFiles {
        AssetFiles {
            css: CSSFiles {
                app: compile_sass("app"),
                fonts: compile_sass("fonts"),
                vendor: concat_vendor_css(vec!["tachyons"]),
            },
            js: JSFiles {
                app: concat_app_js(vec!["tools-install"])
            },
        }
    }
}