#![feature(proc_macro_hygiene, decl_macro)]

// extern crate handlebars;
// extern crate fluent_bundle;

extern crate rocket_contrib;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
// #[macro_use] extern crate lazy_static;

use rocket::{
    request::{Request},
    response::{Redirect, NamedFile},
};
use rocket_contrib::templates::Template;

mod asset;
use asset::AssetFiles;

#[derive(Serialize)]
struct Context<T: serde::Serialize> {
    page: String,
    title: String,
    parent: &'static str,
    is_landing: bool,
    data: T,
    // lang: String,
    baseurl: String,
    // pontoon_enabled: bool,
    assets: AssetFiles,
    // locales: &'static [LocaleInfo],
}
impl<T: serde::Serialize> Context<T> {
    fn new(page: String, title: String, is_landing: bool, data: T) -> Self {
        Context {
            page,
            title,
            parent : "components/layout",
            is_landing,
            data,
            baseurl : "".into(), //baseurl(&lang),
            // lang,
            // pontoon_enabled: *PONTOON_ENABLED,
            assets : AssetFiles::new(),
            // locales: EXPLICIT_LOCALE_INFO,
        }
    }
}

use std::path::{Path, PathBuf};
#[get("/static/<res..>", rank = 1)]
fn statics(res: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(res)).ok()
}

// ------------------------------------------

#[get("/")]
fn index() -> Template {
    let page = "index".to_string();

    let context = Context::new(page.clone(), "GM language".into(), false, ());
    Template::render(page, &context)
}

#[get("/learn")]
fn learn() -> String {
    "unimplemented".into()
}
#[get("/learn/get-started")]
fn get_started() -> String {
    "unimplemented".into()
}

// ------------------------------------------

#[get("/files")]
fn files() -> Redirect {
    Redirect::moved(format!("https://file.gm-lang.org"))
}
#[get("/files/<file>")]
fn files_with_subs(file: String) -> Redirect {
    Redirect::permanent(format!("https://file.gm-lang.org/{}", file))
}

#[get("/docs")]
fn docs() -> Redirect {
    Redirect::moved(format!("https://doc.gm-lang.org"))
}
#[get("/docs/<docs>")]
fn docs_with_subs(docs: String) -> Redirect {
    Redirect::permanent(format!("https://doc.gm-lang.org/{}", docs))
}

// ------------------------------------------

#[catch(404)]
fn not_found(_req: &Request) -> Template {
    let page = "404".to_string();
    let context = Context::new(page.clone(), "error404-page-title".into(), false, ());
    Template::render(page, &context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![
            index,
            statics,

            learn,
            get_started,

            files,
            files_with_subs,
            docs,
            docs_with_subs,
        ])
        .register(catchers![
            not_found,
            // catch_error
        ])
        .launch();
}