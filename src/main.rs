#![feature(proc_macro_hygiene, decl_macro)]

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[macro_use]
extern crate rocket;

// Static Serving
#[get("/<file..>")]
fn public(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

#[post("/user", format="application/json")]
fn user() {

}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/index.html")).ok()
}


fn main() {
    rocket::ignite().mount("/", routes![index, public]).launch();
}
